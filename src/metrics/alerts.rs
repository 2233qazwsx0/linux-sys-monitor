use crate::metrics::{Alert, EmailConfig, WebhookConfig, SlackConfig, TelegramConfig, NotificationConfig};
use std::collections::HashMap;

pub async fn send_email_alert(config: &EmailConfig, subject: &str, body: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    
    let mail_body = format!(
        "From: {}\r\nTo: {}\r\nSubject: {}\r\n\r\n{}",
        config.from_addr,
        config.to_addrs.join(", "),
        subject,
        body
    );
    
    let encoded = encode_base64(mail_body.as_bytes());
    
    let body_json = serde_json::json!({
        "raw": encoded
    });
    
    client.post(&format!("https://www.googleapis.com/gmail/v1/users/me/messages/send"))
        .header("Authorization", format!("Bearer {}", config.username))
        .json(&body_json)
        .send()
        .await
        .map_err(|e| format!("Failed to send email: {}", e))?;
    
    Ok(())
}

pub async fn send_webhook_notification(config: &WebhookConfig, payload: &std::collections::HashMap<String, serde_json::Value>) -> Result<(), String> {
    let client = reqwest::Client::new();
    
    let mut request = client.post(&config.url);
    
    if let Some(ref secret) = config.secret {
        let signature = calculate_hmac_sha256(
            serde_json::to_string(payload).unwrap_or_default(),
            secret
        );
        request = request.header("X-Signature", signature);
    }
    
    for (key, value) in &config.headers {
        request = request.header(key, value.to_string());
    }
    
    request
        .json(payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send webhook: {}", e))?;
    
    Ok(())
}

pub async fn send_slack_notification(config: &SlackConfig, message: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    
    let mut payload: std::collections::HashMap<String, serde_json::Value> = std::collections::HashMap::new();
    payload.insert("text".to_string(), serde_json::json!(message));
    
    if let Some(ref channel) = config.channel {
        payload.insert("channel".to_string(), serde_json::json!(channel));
    }
    
    if let Some(ref username) = config.username {
        payload.insert("username".to_string(), serde_json::json!(username));
    }
    
    client.post(&config.webhook_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send Slack notification: {}", e))?;
    
    Ok(())
}

pub async fn send_telegram_notification(config: &TelegramConfig, message: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        config.bot_token
    );
    
    let payload = serde_json::json!({
        "chat_id": config.chat_id,
        "text": message,
        "parse_mode": "HTML"
    });
    
    client.post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send Telegram message: {}", e))?;
    
    Ok(())
}

pub fn format_alert_message(alert: &Alert) -> String {
    format!(
        "🚨 <b>系统告警</b>\n\n📊 类型: {}\n📈 当前值: {:.2}\n⚠️ 阈值: {:.2}\n🕐 时间: {}",
        alert.alert_type,
        alert.value,
        alert.threshold,
        chrono::DateTime::from_timestamp(alert.timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    )
}

pub fn format_batch_alert_message(alerts: &[Alert]) -> String {
    if alerts.is_empty() {
        return "✅ 系统状态正常".to_string();
    }
    
    let mut message = String::from("🚨 <b>系统告警报告</b>\n\n");
    
    for alert in alerts {
        message.push_str(&format!(
            "• {}: {:.2} (阈值: {:.2})\n",
            alert.alert_type,
            alert.value,
            alert.threshold
        ));
    }
    
    message
}

pub fn create_notification_payload(alert: &Alert, notification_type: &str) -> HashMap<String, serde_json::Value> {
    let mut payload = HashMap::new();
    payload.insert("type".to_string(), serde_json::json!(notification_type));
    payload.insert("alert_type".to_string(), serde_json::json!(alert.alert_type));
    payload.insert("value".to_string(), serde_json::json!(alert.value));
    payload.insert("threshold".to_string(), serde_json::json!(alert.threshold));
    payload.insert("timestamp".to_string(), serde_json::json!(alert.timestamp));
    payload.insert("severity".to_string(), serde_json::json!(calculate_severity(alert)));
    payload
}

pub fn calculate_severity(alert: &Alert) -> String {
    let ratio = alert.value / alert.threshold;
    if ratio > 1.2 {
        "critical".to_string()
    } else if ratio > 1.1 {
        "high".to_string()
    } else if ratio > 1.0 {
        "medium".to_string()
    } else {
        "low".to_string()
    }
}

fn calculate_hmac_sha256(message: String, secret: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let combined = format!("{}:{}", message, secret);
    let mut hasher = DefaultHasher::new();
    combined.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub async fn send_all_notifications(
    config: &NotificationConfig,
    alerts: &[Alert]
) -> HashMap<String, Result<(), String>> {
    let mut results = HashMap::new();
    
    if let Some(ref email_config) = config.email {
        let message = format_batch_alert_message(alerts);
        results.insert(
            "email".to_string(),
            send_email_alert(email_config, "System Monitor Alert", &message).await
        );
    }
    
    if let Some(ref webhook_config) = config.webhook {
        let payload = create_notification_payload(
            &alerts.first().unwrap_or(&Alert {
                alert_type: "batch".to_string(),
                value: 0.0,
                threshold: 0.0,
                timestamp: chrono::Utc::now().timestamp(),
            }),
            "batch_alert"
        );
        results.insert(
            "webhook".to_string(),
            send_webhook_notification(webhook_config, &payload).await
        );
    }
    
    if let Some(ref slack_config) = config.slack {
        let message = format_batch_alert_message(alerts);
        results.insert(
            "slack".to_string(),
            send_slack_notification(slack_config, &message).await
        );
    }
    
    if let Some(ref telegram_config) = config.telegram {
        let message = format_batch_alert_message(alerts);
        results.insert(
            "telegram".to_string(),
            send_telegram_notification(telegram_config, &message).await
        );
    }
    
    results
}

fn encode_base64(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::new();
    for chunk in input.chunks(3) {
        let mut n: u32 = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            n |= (byte as u32) << (16 - i * 8);
        }
        
        let chars_to_emit = chunk.len() + 1;
        for i in 0..chars_to_emit {
            let idx = ((n >> (18 - i * 6)) & 0x3F) as usize;
            result.push(CHARS[idx] as char);
        }
        for _ in chars_to_emit..4 {
            result.push('=');
        }
    }
    result
}

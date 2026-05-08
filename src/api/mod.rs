pub mod http;

use axum::{
    extract::ws::WebSocketUpgrade,
    extract::State,
    response::{Html, IntoResponse},
    Json,
    routing::{get, post},
    Router,
};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};

use crate::metrics::{
    AlertConfig, AlertResponse, Alert, SystemMetrics, RingBuffer,
    TimeRange, AggregationResult, AnomalyResult, TrendResult, PeakResult,
    ScheduledExport, NotificationConfig, DataRetentionConfig, ComparisonResult,
    ReportData,
};
use crate::metrics::export;
use crate::metrics::analysis;
use crate::metrics::scheduling;
use crate::metrics::storage;

#[derive(Clone)]
pub struct SharedState {
    pub tx: Arc<broadcast::Sender<SystemMetrics>>,
    pub alert_config: Arc<Mutex<AlertConfig>>,
    pub history: Arc<Mutex<RingBuffer<SystemMetrics>>>,
    pub notification_config: Arc<Mutex<Option<NotificationConfig>>>,
    pub scheduled_exports: Arc<Mutex<Vec<ScheduledExport>>>,
    pub retention_config: Arc<Mutex<DataRetentionConfig>>,
}

impl Default for SharedState {
    fn default() -> Self {
        Self::new()
    }
}

impl SharedState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel::<SystemMetrics>(100);
        Self {
            tx: Arc::new(tx),
            alert_config: Arc::new(Mutex::new(AlertConfig::default())),
            history: Arc::new(Mutex::new(RingBuffer::new(3600))),
            notification_config: Arc::new(Mutex::new(None)),
            scheduled_exports: Arc::new(Mutex::new(Vec::new())),
            retention_config: Arc::new(Mutex::new(DataRetentionConfig::default())),
        }
    }
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
    pub metric: Option<String>,
    pub start: Option<i64>,
    pub end: Option<i64>,
}

#[derive(Deserialize)]
pub struct TimeRangeQuery {
    pub start: i64,
    pub end: i64,
}

#[derive(Deserialize)]
pub struct AggregateQuery {
    pub metric: String,
}

#[derive(Deserialize)]
pub struct AnomalyQuery {
    pub metric: String,
    pub threshold_std: Option<f64>,
}

#[derive(Deserialize)]
pub struct CompareQuery {
    pub metrics: String,
}

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn get_history(
    State(state): State<Arc<SharedState>>,
) -> Json<Vec<SystemMetrics>> {
    let history = state.history.lock().unwrap();
    Json(history.get_all())
}

pub async fn get_alerts(
    State(state): State<Arc<SharedState>>,
) -> Json<AlertResponse> {
    let config = state.alert_config.lock().unwrap().clone();
    let history = state.history.lock().unwrap();
    let latest = history.get_all();
    
    let mut alerts = Vec::new();
    if let Some(metrics) = latest.last() {
        if metrics.cpu.usage > config.cpu_threshold {
            alerts.push(Alert {
                alert_type: "cpu".to_string(),
                value: metrics.cpu.usage,
                threshold: config.cpu_threshold,
                timestamp: metrics.timestamp,
            });
        }
        if metrics.memory.usage_percent > config.memory_threshold {
            alerts.push(Alert {
                alert_type: "memory".to_string(),
                value: metrics.memory.usage_percent,
                threshold: config.memory_threshold,
                timestamp: metrics.timestamp,
            });
        }
    }
    
    Json(AlertResponse { alerts, config })
}

pub async fn update_alert_config(
    State(state): State<Arc<SharedState>>,
    Json(config): Json<AlertConfig>,
) -> Json<AlertConfig> {
    let mut current = state.alert_config.lock().unwrap();
    *current = config.clone();
    Json(config)
}

pub async fn export_json(
    State(state): State<Arc<SharedState>>,
) -> String {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    export::export_json(&data)
}

pub async fn export_json_data(
    State(state): State<Arc<SharedState>>,
    Json(range): Json<TimeRange>,
) -> Json<ApiResponse<String>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let filtered = export::filter_by_time_range(&data, &range);
    Json(ApiResponse::success(export::export_json(&filtered)))
}

pub async fn export_csv_data(
    State(state): State<Arc<SharedState>>,
    Json(range): Json<TimeRange>,
) -> Json<ApiResponse<String>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let filtered = export::filter_by_time_range(&data, &range);
    Json(ApiResponse::success(export::export_csv(&filtered)))
}

pub async fn export_prometheus_data(
    State(state): State<Arc<SharedState>>,
) -> String {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    export::export_prometheus(&data)
}

pub async fn export_influxdb_data(
    State(state): State<Arc<SharedState>>,
    Json(range): Json<TimeRange>,
) -> Json<ApiResponse<String>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let filtered = export::filter_by_time_range(&data, &range);
    Json(ApiResponse::success(export::export_influxdb(&filtered, "system_metrics")))
}

pub async fn export_graphite_data(
    State(state): State<Arc<SharedState>>,
    Json(range): Json<TimeRange>,
) -> Json<ApiResponse<String>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let filtered = export::filter_by_time_range(&data, &range);
    Json(ApiResponse::success(export::export_graphite(&filtered, "system")))
}

pub async fn query_time_range(
    State(state): State<Arc<SharedState>>,
    Json(range): Json<TimeRange>,
) -> Json<ApiResponse<Vec<SystemMetrics>>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let filtered = export::filter_by_time_range(&data, &range);
    Json(ApiResponse::success(filtered))
}

pub async fn aggregate_data(
    State(state): State<Arc<SharedState>>,
    Json(query): Json<AggregateQuery>,
) -> Json<ApiResponse<Option<AggregationResult>>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let result = analysis::aggregate(&data, &query.metric);
    Json(ApiResponse::success(result))
}

pub async fn aggregate_all_data(
    State(state): State<Arc<SharedState>>,
) -> Json<ApiResponse<std::collections::HashMap<String, AggregationResult>>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let result = analysis::aggregate_all(&data);
    Json(ApiResponse::success(result))
}

pub async fn detect_anomalies(
    State(state): State<Arc<SharedState>>,
    Json(query): Json<AnomalyQuery>,
) -> Json<ApiResponse<Vec<AnomalyResult>>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let threshold = query.threshold_std.unwrap_or(2.0);
    let result = analysis::detect_anomalies(&data, &query.metric, threshold);
    Json(ApiResponse::success(result))
}

pub async fn detect_all_anomalies(
    State(state): State<Arc<SharedState>>,
    Json(query): Json<serde_json::Value>,
) -> Json<ApiResponse<Vec<AnomalyResult>>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let threshold = query.get("threshold_std")
        .and_then(|v| v.as_f64())
        .unwrap_or(2.0);
    let result = analysis::detect_all_anomalies(&data, threshold);
    Json(ApiResponse::success(result))
}

pub async fn analyze_trend(
    State(state): State<Arc<SharedState>>,
    Json(query): Json<AggregateQuery>,
) -> Json<ApiResponse<Option<TrendResult>>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let result = analysis::analyze_trend(&data, &query.metric);
    Json(ApiResponse::success(result))
}

pub async fn analyze_all_trends(
    State(state): State<Arc<SharedState>>,
) -> Json<ApiResponse<Vec<TrendResult>>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let result = analysis::analyze_all_trends(&data);
    Json(ApiResponse::success(result))
}

pub async fn detect_peaks(
    State(state): State<Arc<SharedState>>,
    Json(query): Json<serde_json::Value>,
) -> Json<ApiResponse<Vec<PeakResult>>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let metric = query.get("metric")
        .and_then(|v| v.as_str())
        .unwrap_or("cpu.usage")
        .to_string();
    let threshold = query.get("threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.2);
    let result = analysis::detect_peaks(&data, &metric, threshold);
    Json(ApiResponse::success(result))
}

pub async fn detect_all_peaks(
    State(state): State<Arc<SharedState>>,
    Json(query): Json<serde_json::Value>,
) -> Json<ApiResponse<Vec<PeakResult>>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let threshold = query.get("threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.2);
    let result = analysis::detect_all_peaks(&data, threshold);
    Json(ApiResponse::success(result))
}

pub async fn generate_report(
    State(state): State<Arc<SharedState>>,
    Json(range): Json<TimeRange>,
) -> Json<ApiResponse<ReportData>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    let filtered = export::filter_by_time_range(&data, &range);
    
    let recommendations = analysis::generate_recommendations(&filtered);
    
    let report = ReportData {
        title: "System Monitoring Report".to_string(),
        generated_at: chrono::Utc::now().timestamp(),
        time_range: range,
        summary: analysis::aggregate_all(&filtered),
        anomalies: analysis::detect_all_anomalies(&filtered, 2.0),
        trends: analysis::analyze_all_trends(&filtered),
        peaks: analysis::detect_all_peaks(&filtered, 0.2),
        recommendations,
    };
    
    Json(ApiResponse::success(report))
}

pub async fn create_scheduled_export(
    State(state): State<Arc<SharedState>>,
    Json(export): Json<ScheduledExport>,
) -> Json<ApiResponse<ScheduledExport>> {
    let id = scheduling::generate_export_id();
    let new_export = ScheduledExport {
        id: id.clone(),
        name: export.name,
        format: export.format,
        cron_expression: export.cron_expression,
        enabled: export.enabled,
        destination: export.destination,
        last_run: None,
        next_run: scheduling::parse_cron_next_run(&export.cron_expression, chrono::Utc::now().timestamp()),
    };
    
    if let Err(e) = scheduling::validate_export_config(&new_export) {
        return Json(ApiResponse::error(&e));
    }
    
    state.scheduled_exports.lock().unwrap().push(new_export.clone());
    Json(ApiResponse::success(new_export))
}

pub async fn list_scheduled_exports(
    State(state): State<Arc<SharedState>>,
) -> Json<ApiResponse<Vec<ScheduledExport>>> {
    let exports = state.scheduled_exports.lock().unwrap().clone();
    Json(ApiResponse::success(exports))
}

pub async fn update_scheduled_export(
    State(state): State<Arc<SharedState>>,
    Json(export): Json<ScheduledExport>,
) -> Json<ApiResponse<bool>> {
    let mut exports = state.scheduled_exports.lock().unwrap();
    if let Some(idx) = exports.iter().position(|e| e.id == export.id) {
        exports[idx] = export;
        Json(ApiResponse::success(true))
    } else {
        Json(ApiResponse::error("Export not found"))
    }
}

pub async fn delete_scheduled_export(
    State(state): State<Arc<SharedState>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<ApiResponse<bool>> {
    let mut exports = state.scheduled_exports.lock().unwrap();
    let len_before = exports.len();
    exports.retain(|e| e.id != id);
    Json(ApiResponse::success(exports.len() < len_before))
}

pub async fn update_notification_config(
    State(state): State<Arc<SharedState>>,
    Json(config): Json<NotificationConfig>,
) -> Json<ApiResponse<NotificationConfig>> {
    let mut current = state.notification_config.lock().unwrap();
    *current = Some(config.clone());
    Json(ApiResponse::success(config))
}

pub async fn get_notification_config(
    State(state): State<Arc<SharedState>>,
) -> Json<ApiResponse<Option<NotificationConfig>>> {
    let config = state.notification_config.lock().unwrap().clone();
    Json(ApiResponse::success(config))
}

pub async fn send_email_alert(
    State(state): State<Arc<SharedState>>,
    Json(alert_data): Json<serde_json::Value>,
) -> Json<ApiResponse<String>> {
    let config = state.notification_config.lock().unwrap().clone();
    if let Some(ref email_config) = config.email {
        let subject = alert_data.get("subject")
            .and_then(|v| v.as_str())
            .unwrap_or("System Monitor Alert");
        let body = alert_data.get("body")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match crate::metrics::alerts::send_email_alert(email_config, subject, body).await {
            Ok(_) => Json(ApiResponse::success("Email sent successfully".to_string())),
            Err(e) => Json(ApiResponse::error(&e)),
        }
    } else {
        Json(ApiResponse::error("Email configuration not set"))
    }
}

pub async fn send_webhook_alert(
    State(state): State<Arc<SharedState>>,
    Json(payload): Json<serde_json::Value>,
) -> Json<ApiResponse<String>> {
    let config = state.notification_config.lock().unwrap().clone();
    if let Some(ref webhook_config) = config.webhook {
        let mut map = std::collections::HashMap::new();
        if let Ok(obj) = serde_json::from_value(payload.clone()) {
            map = obj;
        }
        
        match crate::metrics::alerts::send_webhook_notification(webhook_config, &map).await {
            Ok(_) => Json(ApiResponse::success("Webhook sent successfully".to_string())),
            Err(e) => Json(ApiResponse::error(&e)),
        }
    } else {
        Json(ApiResponse::error("Webhook configuration not set"))
    }
}

pub async fn send_slack_alert(
    State(state): State<Arc<SharedState>>,
    Json(message_data): Json<serde_json::Value>,
) -> Json<ApiResponse<String>> {
    let config = state.notification_config.lock().unwrap().clone();
    if let Some(ref slack_config) = config.slack {
        let message = message_data.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("System Monitor Alert");
        
        match crate::metrics::alerts::send_slack_notification(slack_config, message).await {
            Ok(_) => Json(ApiResponse::success("Slack message sent successfully".to_string())),
            Err(e) => Json(ApiResponse::error(&e)),
        }
    } else {
        Json(ApiResponse::error("Slack configuration not set"))
    }
}

pub async fn send_telegram_alert(
    State(state): State<Arc<SharedState>>,
    Json(message_data): Json<serde_json::Value>,
) -> Json<ApiResponse<String>> {
    let config = state.notification_config.lock().unwrap().clone();
    if let Some(ref telegram_config) = config.telegram {
        let message = message_data.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("System Monitor Alert");
        
        match crate::metrics::alerts::send_telegram_notification(telegram_config, message).await {
            Ok(_) => Json(ApiResponse::success("Telegram message sent successfully".to_string())),
            Err(e) => Json(ApiResponse::error(&e)),
        }
    } else {
        Json(ApiResponse::error("Telegram configuration not set"))
    }
}

pub async fn update_retention_config(
    State(state): State<Arc<SharedState>>,
    Json(config): Json<DataRetentionConfig>,
) -> Json<ApiResponse<DataRetentionConfig>> {
    let mut current = state.retention_config.lock().unwrap();
    *current = config.clone();
    Json(ApiResponse::success(config))
}

pub async fn get_retention_config(
    State(state): State<Arc<SharedState>>,
) -> Json<ApiResponse<DataRetentionConfig>> {
    let config = state.retention_config.lock().unwrap().clone();
    Json(ApiResponse::success(config))
}

pub async fn cleanup_old_data(
    State(_state): State<Arc<SharedState>>,
) -> Json<ApiResponse<usize>> {
    Json(ApiResponse::success(0))
}

pub async fn compare_metrics_data(
    State(state): State<Arc<SharedState>>,
    Json(query): Json<serde_json::Value>,
) -> Json<ApiResponse<ComparisonResult>> {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    
    let metrics_str = query.get("metrics")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    let pairs: Vec<(String, String)> = metrics_str.split(',')
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.split(':').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect();
    
    let result = analysis::compare_metrics(&data, &pairs);
    Json(ApiResponse::success(result))
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<SharedState>>,
) -> impl axum::response::IntoResponse {
    let tx = state.tx.clone();
    ws.on_upgrade(move |socket| http::handle_socket(socket, tx))
}

pub async fn serve_frontend() -> Html<String> {
    Html(include_str!("../frontend_dist/index.html").to_string())
}

pub fn create_router(state: Arc<SharedState>) -> Router {
    Router::new()
        .route("/", get(serve_frontend))
        .route("/health", get(health_check))
        .route("/api/history", get(get_history))
        .route("/api/alerts", get(get_alerts))
        .route("/api/alerts/config", post(update_alert_config))
        .route("/api/export/json", get(export_json))
        .route("/api/export/json", post(export_json_data))
        .route("/api/export/csv", post(export_csv_data))
        .route("/api/export/prometheus", get(export_prometheus_data))
        .route("/api/export/influxdb", post(export_influxdb_data))
        .route("/api/export/graphite", post(export_graphite_data))
        .route("/api/query/time-range", post(query_time_range))
        .route("/api/aggregate", post(aggregate_data))
        .route("/api/aggregate/all", get(aggregate_all_data))
        .route("/api/analyze/anomalies", post(detect_anomalies))
        .route("/api/analyze/anomalies/all", post(detect_all_anomalies))
        .route("/api/analyze/trend", post(analyze_trend))
        .route("/api/analyze/trends/all", get(analyze_all_trends))
        .route("/api/analyze/peaks", post(detect_peaks))
        .route("/api/analyze/peaks/all", post(detect_all_peaks))
        .route("/api/report/generate", post(generate_report))
        .route("/api/scheduled-exports", get(list_scheduled_exports))
        .route("/api/scheduled-exports", post(create_scheduled_export))
        .route("/api/scheduled-exports", put(update_scheduled_export))
        .route("/api/scheduled-exports/:id", delete(delete_scheduled_export))
        .route("/api/notifications/config", get(get_notification_config))
        .route("/api/notifications/config", post(update_notification_config))
        .route("/api/notifications/email", post(send_email_alert))
        .route("/api/notifications/webhook", post(send_webhook_alert))
        .route("/api/notifications/slack", post(send_slack_alert))
        .route("/api/notifications/telegram", post(send_telegram_alert))
        .route("/api/retention/config", get(get_retention_config))
        .route("/api/retention/config", post(update_retention_config))
        .route("/api/retention/cleanup", post(cleanup_old_data))
        .route("/api/compare", post(compare_metrics_data))
        .route("/ws", get(websocket_handler))
        .with_state(state)
}

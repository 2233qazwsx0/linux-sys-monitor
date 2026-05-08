pub mod collector;
pub mod ring_buffer;
pub mod export;
pub mod analysis;
pub mod alerts;
pub mod scheduling;
pub mod storage;

pub use collector::{SystemMetrics, MetricsCollector};
pub use ring_buffer::RingBuffer;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub cpu_threshold: f32,
    pub memory_threshold: f32,
    pub disk_threshold: f32,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_type: String,
    pub value: f32,
    pub threshold: f32,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertResponse {
    pub alerts: Vec<Alert>,
    pub config: AlertConfig,
}

#[derive(Clone)]
pub struct AppState {
    pub tx: Arc<broadcast::Sender<SystemMetrics>>,
    pub alert_config: Arc<Mutex<AlertConfig>>,
    pub history: Arc<Mutex<RingBuffer<SystemMetrics>>>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel::<SystemMetrics>(100);
        Self {
            tx: Arc::new(tx),
            alert_config: Arc::new(Mutex::new(AlertConfig::default())),
            history: Arc::new(Mutex::new(RingBuffer::new(3600))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: i64,
    pub end: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationResult {
    pub metric_name: String,
    pub count: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub sum: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub timestamp: i64,
    pub metric_name: String,
    pub value: f64,
    pub expected: f64,
    pub deviation: f64,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendResult {
    pub metric_name: String,
    pub direction: String,
    pub slope: f64,
    pub correlation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakResult {
    pub timestamp: i64,
    pub metric_name: String,
    pub value: f64,
    pub peak_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledExport {
    pub id: String,
    pub name: String,
    pub format: String,
    pub cron_expression: String,
    pub enabled: bool,
    pub destination: String,
    pub last_run: Option<i64>,
    pub next_run: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub email: Option<EmailConfig>,
    pub webhook: Option<WebhookConfig>,
    pub slack: Option<SlackConfig>,
    pub telegram: Option<TelegramConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_addr: String,
    pub to_addrs: Vec<String>,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub secret: Option<String>,
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfig {
    pub webhook_url: String,
    pub channel: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionConfig {
    pub max_age_days: u32,
    pub max_records: usize,
    pub compression_enabled: bool,
    pub cleanup_interval_hours: u32,
}

impl Default for DataRetentionConfig {
    fn default() -> Self {
        Self {
            max_age_days: 30,
            max_records: 86400,
            compression_enabled: true,
            cleanup_interval_hours: 6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub metrics: Vec<String>,
    pub correlations: std::collections::HashMap<String, f64>,
    pub insights: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub title: String,
    pub generated_at: i64,
    pub time_range: TimeRange,
    pub summary: std::collections::HashMap<String, AggregationResult>,
    pub anomalies: Vec<AnomalyResult>,
    pub trends: Vec<TrendResult>,
    pub peaks: Vec<PeakResult>,
    pub recommendations: Vec<String>,
}

pub mod collector;
pub mod ring_buffer;

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
pub struct SharedState {
    pub tx: Arc<broadcast::Sender<SystemMetrics>>,
    pub alert_config: Arc<Mutex<AlertConfig>>,
    pub history: Arc<Mutex<RingBuffer<SystemMetrics>>>,
}

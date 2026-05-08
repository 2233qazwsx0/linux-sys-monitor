pub mod collector;
pub mod ring_buffer;

pub use collector::{SystemMetrics, MetricsCollector, SmallDiskInfo, SmallProcessInfo, DiskInfo, ProcessInfo, SmallF32Array, SmallU64Array};
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
    pub alert_type: heapless::String<16>,
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
            history: Arc::new(Mutex::new(RingBuffer::new(300))),
        }
    }
}

pub mod http;

use axum::{
    extract::ws::WebSocketUpgrade,
    extract::State,
    response::Html,
    Json,
};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

use crate::metrics::{AlertConfig, AlertResponse, Alert, SystemMetrics};

#[derive(Clone)]
pub struct SharedState {
    pub tx: Arc<broadcast::Sender<SystemMetrics>>,
    pub alert_config: Arc<Mutex<AlertConfig>>,
    pub history: Arc<Mutex<crate::metrics::RingBuffer<SystemMetrics>>>,
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

pub async fn export_data(
    State(state): State<Arc<SharedState>>,
) -> String {
    let history = state.history.lock().unwrap();
    let data = history.get_all();
    serde_json::to_string(&data).unwrap_or_default()
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

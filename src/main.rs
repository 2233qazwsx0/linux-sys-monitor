use std::net::SocketAddr;
use axum::{Router, routing::get, routing::post};
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber;
use tokio::sync::broadcast;
use std::sync::{Arc, Mutex};

mod metrics;
mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let state = Arc::new(api::SharedState {
        tx: Arc::new(broadcast::channel::<metrics::SystemMetrics>(100).0),
        alert_config: Arc::new(Mutex::new(metrics::AlertConfig::default())),
        history: Arc::new(Mutex::new(metrics::RingBuffer::new(3600))),
    });
    let state_clone = state.clone();
    let tx_clone = state.tx.clone();
    
    tokio::spawn(async move {
        let mut collector = metrics::MetricsCollector::new();
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let metrics_data = collector.collect();
            state_clone.history.lock().unwrap().push(metrics_data.clone());
            let _ = tx_clone.send(metrics_data);
        }
    });
    
    let app = Router::new()
        .route("/api/health", get(api::health_check))
        .route("/api/history", get(api::get_history))
        .route("/api/alerts", get(api::get_alerts))
        .route("/api/alerts/config", post(api::update_alert_config))
        .route("/api/export", get(api::export_data))
        .route("/ws", get(api::websocket_handler))
        .route("/", get(api::serve_frontend))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(state);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server starting on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

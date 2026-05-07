use std::net::SocketAddr;
use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber;
use std::sync::Arc;
use tokio::sync::broadcast;

mod metrics;
mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let (tx, _rx) = broadcast::channel::<metrics::SystemMetrics>(100);
    let tx_clone = tx.clone();
    
    tokio::spawn(async move {
        let mut collector = metrics::MetricsCollector::new();
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let metrics = collector.collect();
            let _ = tx_clone.send(metrics);
        }
    });
    
    let app = Router::new()
        .route("/api/health", get(api::health_check))
        .route("/api/history", get(api::get_history))
        .route("/ws", get(api::websocket_handler))
        .route("/", get(api::serve_frontend))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(tx);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server starting on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

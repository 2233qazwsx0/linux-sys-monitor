use std::net::SocketAddr;
use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber;
use tokio::sync::broadcast;
use std::sync::Arc;

mod metrics;
mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port: u16 = std::env::var("MONITOR_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string());

    tracing::info!("Starting Linux System Monitor");
    tracing::info!("Port: {}", port);
    tracing::info!("Log level: {}", log_level);

    let (tx, _rx) = broadcast::channel::<metrics::SystemMetrics>(100);
    let tx = Arc::new(tx);
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        let mut collector = metrics::MetricsCollector::new();
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let metrics_data = collector.collect();
            let _ = tx_clone.send(metrics_data);
        }
    });

    let app = Router::new()
        .route("/api/health", get(api::health_check))
        .route("/api/history", get(api::get_history))
        .route("/ws", get(api::websocket_handler))
        .route("/", get(api::serve_frontend))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(tx);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

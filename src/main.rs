use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber;
use std::sync::Arc;

mod metrics;
mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let state = Arc::new(api::SharedState::new());
    
    let app = api::create_router(state)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));
    
    let port = std::env::var("MONITOR_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    let platform = if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "android") {
        "Android"
    } else {
        "Linux"
    };
    
    tracing::info!("{} System Monitor v{} starting on http://{}", platform, env!("CARGO_PKG_VERSION"), addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

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
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server starting on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

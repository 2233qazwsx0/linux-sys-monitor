pub mod http;

use axum::{
    extract::ws::{WebSocket, Message, WebSocketUpgrade},
    extract::State,
    response::Html,
    TypedHeader,
};
use tokio::sync::broadcast;
use std::sync::Arc;

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn get_history(
    State(_state): State<Arc<broadcast::Sender<crate::metrics::SystemMetrics>>>,
) -> String {
    "History endpoint - use WebSocket for real-time data".to_string()
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(tx): State<Arc<broadcast::Sender<crate::metrics::SystemMetrics>>>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| http::handle_socket(socket, tx))
}

pub async fn serve_frontend() -> Html<String> {
    Html(include_str!("../frontend_dist/index.html").to_string())
}

use axum::extract::ws::{WebSocket, Message};
use tokio::sync::broadcast;
use std::sync::Arc;

pub async fn handle_socket(
    socket: WebSocket,
    sender: Arc<broadcast::Sender<crate::metrics::SystemMetrics>>,
) {
    let mut receiver = sender.subscribe();
    let (mut ws_sender, mut ws_receiver) = socket.split();
    
    loop {
        tokio::select! {
            metrics = receiver.recv() => {
                match metrics {
                    Ok(data) => {
                        let json = serde_json::to_string(&data).unwrap();
                        if ws_sender.send(Message::Text(json)).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }
            msg = ws_receiver.recv() => {
                if msg.is_err() {
                    return;
                }
            }
        }
    }
}

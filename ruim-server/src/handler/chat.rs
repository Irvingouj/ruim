use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};

use crate::service::auth::UserTokenExtractor;


pub async fn websocket_handler(
    UserTokenExtractor { user_id: _ }: UserTokenExtractor, 
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            if socket.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
    }
}

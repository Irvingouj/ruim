use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use uuid::Uuid;

use crate::service::auth::UserTokenExtractor;

pub async fn websocket_handler(
    UserTokenExtractor { user_id }: UserTokenExtractor,
    State(_db): State<crate::db::Database>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |ws| async move {
        handle_socket(ws, user_id).await;
    })
}

async fn handle_socket(_socket: WebSocket, _sender: Uuid) {
    todo!()
}

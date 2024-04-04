use anyhow::Context;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{db::Database, service::auth::UserTokenExtractor};

pub async fn websocket_handler(
    UserTokenExtractor { user_id }: UserTokenExtractor,
    State(db): State<crate::db::Database>,
    State(session_manager): State<crate::core::session_manager::SessionManager>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |ws| async move {
        handle_socket(ws, user_id, db, session_manager).await;
    })
}

async fn handle_socket(
    socket: WebSocket,
    user_id: Uuid,
    db: Database,
    session_manager: crate::core::session_manager::SessionManager,
) {
    let (websocket_task_handle, mut client_receiver) =
        session_manager.add_websocket(user_id, socket);

    let session_manager_clone = session_manager.clone();
    let client_receive_handle = tokio::spawn(async move {
        while let Some(msg) = client_receiver.recv().await {
            let crate::core::session_manager::WebsocketClientMessage::Message(msg) = msg else {
                tracing::error!("Error receiving message from session manager");
                break;
            };

            let axum::extract::ws::Message::Text(msg) = msg else {
                tracing::error!("received unexpected msg or closed");
                break;
            };

            let msg: api_models::chat::ClientMessage = serde_json::from_str(&msg)?;

            match msg {
                api_models::chat::ClientMessage::Regular(msg) => {
                    db.add_chat_message(user_id, Uuid::parse_str(&msg.receiver_id)?, &msg.message)
                        .await
                        .context("Failed to add chat message")
                        .inspect_err(|err| {
                            tracing::error!(?err);
                        })?;

                    let server_msg = api_models::chat::ServerMessage::Regular(
                        api_models::chat::ServerMessageBody {
                            message: msg.message,
                            created_at: msg.created_at,
                            sender_id: user_id.to_string(),
                        },
                    );

                    let msg = axum::extract::ws::Message::Text(serde_json::to_string(&server_msg)?);
                    let _ = session_manager_clone
                        .send_control_command(
                            user_id,
                            crate::core::session_manager::WebsocketControlMessage::SendMessage(msg),
                        )
                        .await;
                }
            }
        }

        anyhow::Ok(())
    });

    tokio::select! {
        _ = websocket_task_handle => {
            tracing::info!("Websocket handler finished");
        }
        _ = client_receive_handle => {
            // client receiver finished, which means the websocket is closed
            // session mannager failed to send the control message, which means the other end is closed
            let _ = session_manager.send_control_command(user_id, crate::core::session_manager::WebsocketControlMessage::Close).await;
        }
    }
}

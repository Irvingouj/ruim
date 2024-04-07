use anyhow::Context;
use axum::extract::{ws::WebSocket, FromRef};

use dashmap::DashMap;
use std::sync::Arc;

use uuid::Uuid;

use crate::context::RuimContext;

/// To safely close the websocket, websocket need to take ownership.
/// This means that it is impossible to have websocket in multiple places.
/// hence we spwan a task to handle the websocket and communicate with it using channels.
#[derive(Debug, Clone)]
pub struct SafeWebsocket {
    command_sender: tokio::sync::mpsc::Sender<WebsocketControlMessage>,
}

impl SafeWebsocket {
    pub async fn send_command(&self, msg: WebsocketControlMessage) -> anyhow::Result<()> {
        self.command_sender.send(msg).await?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct SessionManager {
    // Maybe Mutex<HashMap> would be better?
    pub websockets: Arc<DashMap<Uuid, SafeWebsocket>>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            websockets: Arc::new(DashMap::new()),
        }
    }
}

impl SessionManager {
    pub fn add_websocket(
        &self,
        user_id: Uuid,
        mut websocket: WebSocket,
    ) -> (
        tokio::task::JoinHandle<()>,
        tokio::sync::mpsc::Receiver<WebsocketClientMessage>,
    ) {
        let (command_sender, mut command_receiver) =
            tokio::sync::mpsc::channel::<WebsocketControlMessage>(1);
        let (client_sender, client_receiver) =
            tokio::sync::mpsc::channel::<WebsocketClientMessage>(10);

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = websocket.recv() => {
                        let Some(msg) = msg else {
                            let _ = client_sender.send(WebsocketClientMessage::Error).await.inspect_err(|err| {
                                tracing::trace!("Error sending message to session manager: {:?}", err);
                            });
                            break;
                        };

                        let Ok(msg) = msg.inspect_err(|err| {
                            tracing::error!("Error receiving message from websocket: {:?}", err);
                        }) else {
                            break;
                        };

                        let _ = client_sender.send(WebsocketClientMessage::Message(msg)).await.inspect_err(|err| {
                            tracing::error!("Error sending message to session manager: {:?}", err);
                        });
                    }
                    command = command_receiver.recv() => {
                        let Some(command) = command else{
                            tracing::trace!("Command channel closed");
                            return;
                        };
                        match command {
                            WebsocketControlMessage::SendMessage(msg) => {
                                let _ = websocket.send(msg).await.inspect_err(|err| {
                                    tracing::error!("Error sending message to websocket: {:?}", err);
                                });
                            },
                            WebsocketControlMessage::Close => {
                                let _ = websocket.close().await.inspect_err(|err| {
                                    tracing::error!("Error closing websocket: {:?}", err);
                                });
                                return;
                            }
                        }
                    }
                }
            }
        });

        self.websockets
            .insert(user_id, SafeWebsocket { command_sender });

        (handle, client_receiver)
    }

    pub fn remove_websocket(&self, user_id: Uuid) {
        self.websockets.remove(&user_id);
    }

    pub async fn send_control_command(
        &self,
        user_id: Uuid,
        command: WebsocketControlMessage,
    ) -> anyhow::Result<()> {
        self.websockets
            .get(&user_id)
            .context("missing websocket")?
            .send_command(command)
            .await?;

        Ok(())
    }
}

pub enum WebsocketControlMessage {
    SendMessage(axum::extract::ws::Message),
    Close,
}

pub enum WebsocketClientMessage {
    Message(axum::extract::ws::Message),
    Error,
}

pub struct CloseMessage {
    pub code: u16,
    pub reason: String,
}

impl FromRef<RuimContext> for SessionManager {
    fn from_ref(input: &RuimContext) -> Self {
        input.session_manager.clone()
    }
}

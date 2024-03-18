use axum::extract::{ws::WebSocket, FromRef};

use dashmap::DashMap;
use serde::Serialize;
use std::sync::Arc;

use uuid::Uuid;

use crate::{context::RuimContext, db};

/// To safely close the websocket, websocket need to take ownership.
/// This means that it is impossible to have websocket in multiple places.
/// hence we spwan a task to handle the websocket and communicate with it using channels.
#[derive(Debug)]
pub struct SafeWebsocket {
    handle: tokio::task::JoinHandle<()>,
    command_sender: tokio::sync::mpsc::Sender<WebsocketControlMessage>,
    client_receiver: tokio::sync::mpsc::Receiver<WebsocketClientMessage>,
}

impl SafeWebsocket {
    pub async fn send_msg<T: Serialize>(&self, msg: T) -> anyhow::Result<()> {
        let msg = serde_json::to_string(&msg)?;
        self.command_sender
            .send(WebsocketControlMessage::SendMsg(msg))
            .await?;

        Ok(())
    }

    pub async fn recv_msg(&mut self) -> Option<WebsocketClientMessage> {
        self.client_receiver.recv().await
    }
}

impl Drop for SafeWebsocket {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

#[derive(Clone)]
pub struct SessionManager {
    // Maybe Mutex<HashMap> would be better?
    pub websockets: Arc<DashMap<Uuid, SafeWebsocket>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            websockets: Arc::new(DashMap::new()),
        }
    }
}

impl SessionManager {
    pub fn add_websocket(&self, user_id: Uuid, mut websocket: WebSocket) {
        let (command_sender, mut command_receiver) =
            tokio::sync::mpsc::channel::<WebsocketControlMessage>(1);
        let (client_sender, client_receiver) =
            tokio::sync::mpsc::channel::<WebsocketClientMessage>(10);
        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = command_receiver.recv() => {
                        let Some(msg) = msg else{
                            let _ = websocket.close().await.inspect_err(|err| {
                                tracing::error!("Error closing websocket: {:?}", err);
                            });
                            return;
                        };

                        match msg {
                            WebsocketControlMessage::SendMsg(msg) => {
                                let _ = websocket.send(axum::extract::ws::Message::Text(serde_json::to_string(&msg).unwrap())).await.inspect_err(|err| {
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
                    msg = websocket.recv() => {
                        let Some(msg) = msg else {
                            let _ = client_sender.send(WebsocketClientMessage::Closed(None)).await.inspect_err(|err| {
                                tracing::trace!("Error sending message to session manager: {:?}", err);
                            });
                            return;
                        };

                        let Ok(msg) = msg.inspect_err(|err| {
                            tracing::error!("Error receiving message from websocket: {:?}", err);
                        }) else {
                            return;
                        };

                        match msg {
                            axum::extract::ws::Message::Text(text) => {
                                let _ = client_sender.send(WebsocketClientMessage::Msg(text)).await.inspect_err(|err| {
                                    tracing::error!("Error sending message to session manager: {:?}", err);
                                });
                            },
                            axum::extract::ws::Message::Close(reason) => {
                                let _ = client_sender.send(WebsocketClientMessage::Closed(reason.map(|msg|CloseMessage{
                                    code: msg.code,
                                    reason: msg.reason.to_string(),
                                }))).await.inspect_err(|err| {
                                    tracing::error!("Error sending message to session manager: {:?}", err);
                                });
                                return;
                            },
                            _ => {
                                tracing::error!("Unsupported message type");
                            }
                        }


                    }
                }
            }
        });
        self.websockets.insert(
            user_id,
            SafeWebsocket {
                handle,
                command_sender,
                client_receiver,
            },
        );
    }

    pub fn remove_websocket(&self, user_id: Uuid) {
        self.websockets.remove(&user_id);
    }
}

pub enum WebsocketControlMessage {
    SendMsg(String),
    Close,
}

pub enum WebsocketClientMessage {
    Msg(String),
    Closed(Option<CloseMessage>),
    Err(anyhow::Error),
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
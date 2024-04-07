use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMessageBody {
    pub message: String,
    pub created_at: String,
    pub receiver_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Regular(ClientMessageBody),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerMessageBody {
    pub message: String,
    pub created_at: String,
    pub sender_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Regular(ServerMessageBody),
    Notify,
}

use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Message {
    pub message_id: i32,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub content: String,
    pub created_at: sqlx::types::time::OffsetDateTime,
}
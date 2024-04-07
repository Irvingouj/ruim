use uuid::Uuid;



impl super::Database {

    pub async fn add_chat_message(&self, user_id: Uuid, receiver_id: Uuid, message: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO messages (sender_id, receiver_id, content)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            receiver_id,
            message
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    
}
use uuid::Uuid;

impl crate::db::Database {
    pub async fn create_friend_application(
        &self,
        sender_id: Uuid,
        receiver_id: Uuid,
    ) -> Result<i32, crate::db::DBError> {
        let res = sqlx::query_as!(
            crate::model::user::FriendApplication,
            r#"
            INSERT INTO friend_applications (sender_id, receiver_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            sender_id,
            receiver_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(crate::db::DBError::Sqlx)?;

        Ok(res.application_id)
    }

    pub async fn query_all_friend(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<crate::model::user::Friendship>, crate::db::DBError> {
        let res = sqlx::query_as!(
            crate::model::user::Friendship,
            r#"
            SELECT * FROM friendships
            WHERE user1_id = $1 OR user2_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::db::DBError::Sqlx)?;

        Ok(res)
    }
}

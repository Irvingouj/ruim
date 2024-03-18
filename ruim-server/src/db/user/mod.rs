use uuid::Uuid;

use crate::model::user::User;
pub mod friendship;

impl super::Database {
    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<(), super::DBError> {
        sqlx::query!(
            r#"
            INSERT INTO users (username, hashed_password, email)
            VALUES ($1, $2, $3)
            "#,
            username,
            password,
            email
        )
        .execute(&self.pool)
        .await
        .map_err(super::DBError::Sqlx)?;

        Ok(())
    }

    pub async fn get_user_by_id(&self, uuid: &Uuid) -> Result<Option<User>, super::DBError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE user_id = $1
            "#,
            uuid
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(super::DBError::Sqlx)?;

        Ok(user)
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>, super::DBError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE username = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(super::DBError::Sqlx)?;

        Ok(user)
    }

    pub async fn get_public_users(
        &self,
        page: i64,
        limit: i64,
    ) -> Result<Vec<User>, super::DBError> {
        let offset = (page - 1) * limit;

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE accept_public_chat = true
            ORDER BY created_at
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(super::DBError::Sqlx)?; // (1)

        Ok(users)
    }
}

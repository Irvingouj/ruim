use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: Option<sqlx::types::time::OffsetDateTime>,
    pub updated_at: Option<sqlx::types::time::OffsetDateTime>,
}

#[derive(Debug, Serialize)]
pub struct ApiUser {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<User> for ApiUser {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            created_at: user.created_at.map(|t| t.to_string()),
            updated_at: user.updated_at.map(|t| t.to_string()),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct FriendApplication {
    pub application_id: i32,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub status: i16,
    pub created_at: Option<sqlx::types::time::OffsetDateTime>,
}

#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(i16)]
pub enum FriendApplicationStatus {
    Pending = 1,
    Accepted = 2,
    Rejected = 3,
}

#[derive(Debug)]
pub struct Friendship {
    pub friendship_id: i32,
    pub user1_id: Uuid,
    pub user2_id: Uuid,
    pub status: i16,
    pub created_at: Option<sqlx::types::time::OffsetDateTime>,
}

#[derive(Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(i16)]
pub enum FriendShipStatus {
    Friend = 1,
    Pending = 2,
    Blocked = 3,
}

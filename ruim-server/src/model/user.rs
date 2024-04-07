use api_models::user::ApiUser;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: Option<sqlx::types::time::OffsetDateTime>,
    pub updated_at: Option<sqlx::types::time::OffsetDateTime>,
    pub accept_public_chat: bool,
    pub show_in_public_chat: bool,
    pub last_seen: Option<sqlx::types::time::OffsetDateTime>,
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

impl From<User> for ApiUser {
    fn from(val: User) -> Self {
        ApiUser {
            user_id: val.user_id,
            username: val.username,
            email: val.email,
            created_at: val.created_at.map(|t| t.to_string()),
            updated_at: val.updated_at.map(|t| t.to_string()),
        }
    }
}

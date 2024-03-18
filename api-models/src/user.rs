use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Debug, Serialize)]
pub struct ApiUser {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginBody {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AddFriendRequest {
    pub friend_id: Uuid,
}

#[derive(Serialize)]
pub struct AddFriendResponse {
    pub application_id: i32,
    pub friend_id: Uuid,
}

#[derive(Serialize,Deserialize)]
pub struct RegisterBody {
    pub username: String,
    pub email: String,
    pub password: String,
}
use super::{ApiError, RegisterBody};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{handler::GenericResponse, service::auth::UserTokenExtractor};

pub(crate) fn router() -> axum::Router<crate::RuimContext> {
    Router::new()
        .route("/application", post(add_friend))
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


pub async fn add_friend(
    UserTokenExtractor { user_id }: UserTokenExtractor,
    State(db): State<crate::db::Database>,
    // Wow: the Json Extractor must be the last extractor as parameter
    Json(AddFriendRequest { friend_id }): Json<AddFriendRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let application_id = db
        .create_friend_application(user_id, friend_id)
        .await
        .inspect_err(|e| tracing::error!("Failed to create friend application: {:?}", e))
        .map_err(|_| ApiError::msg("Failed to create friend application"))?;

    Ok(Json(AddFriendResponse {
        application_id,
        friend_id,
    }))
}

pub async fn remove_friend(
    UserTokenExtractor { user_id }: UserTokenExtractor,
    State(db): State<crate::db::Database>,
) -> Result<impl IntoResponse, crate::handler::ApiError> {
    Ok(GenericResponse::default().msg("Friend removed successfully"))
}

pub async fn accept_friend_request(
    UserTokenExtractor { user_id }: UserTokenExtractor,
    State(db): State<crate::db::Database>,
    Json(AddFriendRequest { friend_id }): Json<AddFriendRequest>,
) -> Result<impl IntoResponse, crate::handler::ApiError> {
    Ok(GenericResponse::default().msg("Friend request accepted"))
}
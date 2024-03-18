use super::{ApiError};
use api_models::user::{AddFriendRequest, AddFriendResponse};
use axum::{
    extract::State,
    response::{IntoResponse},
    routing::post,
    Json, Router,
};

use crate::{context::RuimContext, handler::GenericResponse, service::auth::UserTokenExtractor};

pub(crate) fn router() -> axum::Router<RuimContext> {
    Router::new()
        .route("/application", post(add_friend))
}

pub async fn add_friend(
    UserTokenExtractor { user_id }: UserTokenExtractor,
    State(db): State<crate::db::Database>,
    // Wow: the Json Extractor must be the last extractor as parameter
    Json(AddFriendRequest { friend_id }): Json<AddFriendRequest>,
) -> Result< Json<AddFriendResponse>, ApiError> {
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
    UserTokenExtractor { user_id: _ }: UserTokenExtractor,
    State(_db): State<crate::db::Database>,
) -> Result<impl IntoResponse, crate::handler::ApiError> {
    Ok(GenericResponse::default().msg("Friend removed successfully"))
}

pub async fn accept_friend_request(
    UserTokenExtractor { user_id: _ }: UserTokenExtractor,
    State(_db): State<crate::db::Database>,
    Json(AddFriendRequest { friend_id: _ }): Json<AddFriendRequest>,
) -> Result<impl IntoResponse, crate::handler::ApiError> {
    Ok(GenericResponse::default().msg("Friend request accepted"))
}
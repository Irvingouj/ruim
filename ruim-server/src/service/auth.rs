use crate::{handler::ApiError, jwt::Jwt};
use axum::{
    async_trait, extract::{FromRef, FromRequestParts, Request, State}, http::{self, request::Parts}, middleware::Next, response::{IntoResponse, Response}
};
use uuid::Uuid;

pub async fn guard(State(jwt): State<Jwt>, request: Request, next: Next) -> impl IntoResponse {
    let err_res = http::Response::builder()
        .status(http::StatusCode::UNAUTHORIZED)
        .body("Unauthorized".into())
        .unwrap();

    let token = match request.headers().get("Authorization") {
        Some(token) => token,
        None => return err_res,
    };

    let Some(token) = token.to_str().unwrap().split_whitespace().last() else {
        return err_res;
    };

    if let Ok(_) = jwt.verify_token(token) {
        return next.run(request).await;
    }

    err_res
}

#[derive(Debug, Clone)]
pub struct UserTokenExtractor {
    pub user_id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for UserTokenExtractor
where
    Jwt: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = match parts.headers.get("Authorization") {
            Some(token) => token,
            None => return Err(ApiError::msg("Unauthorized")),
        };

        let token = match token.to_str() {
            Ok(token) => token,
            Err(_) => return Err(ApiError::msg("Unauthorized")),
        };

        let token = match token.split_whitespace().last() {
            Some(token) => token,
            None => return Err(ApiError::msg("Unauthorized")),
        };

        let jwt = Jwt::from_ref(state);

        let claim = match jwt.verify_token(token) {
            Ok(claim) => claim,
            Err(_) => return Err(ApiError::msg("Unauthorized")),
        };

        Ok(UserTokenExtractor {
            user_id: claim.user_id,
        })

    }
}


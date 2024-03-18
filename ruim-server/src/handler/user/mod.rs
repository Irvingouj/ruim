use api_models::user::{ApiUser, LoginBody, RegisterBody};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde_json::json;

use crate::{
    context::RuimContext, db::Database, handler::ApiError, jwt::UserTokenClaims,
    service::auth::UserTokenExtractor,
};

use super::GenericResponse;

pub mod friendship;

pub fn router(_state: RuimContext) -> Router<RuimContext> {
    Router::new()
        .route("/signup", put(register))
        .route("/login", get(login))
        .route("/detail", get(get_user))
        .nest("/friend", friendship::router())
}

async fn register(
    State(db): State<Database>,
    Json(RegisterBody {
        username,
        email,
        password,
    }): Json<RegisterBody>,
) -> Result<GenericResponse, ApiError> {
    // hash password
    let password = hash_password(&password)?;

    db.create_user(&username, &password, &email)
        .await
        .map_err(|_| ApiError::msg("Failed to create user"))?;

    Ok(GenericResponse::default().msg("User created successfully"))
}

async fn login(
    State(RuimContext { db, jwt, .. }): State<RuimContext>,
    Json(LoginBody { username, password }): Json<LoginBody>,
) -> Result<impl IntoResponse, ApiError> {
    let user = db
        .get_user_by_name(&username)
        .await
        .map_err(|_| ApiError::msg("Failed to get user"))?;

    let Some(user) = user else {
        return Err(ApiError::msg("User not found").code(StatusCode::NOT_FOUND));
    };

    verify_password(&password, &user.hashed_password)
        .map_err(|_| ApiError::msg("Invalid password").code(StatusCode::UNAUTHORIZED))?;

    let token = jwt.generate_token(UserTokenClaims {
        user_id: user.user_id,
    })?;

    Ok(Json(json!({
        "msg": "Login successful",
        "token": token,
    })))
}

fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| anyhow::anyhow!("Failed to hash password"))?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, hash: &str) -> Result<(), anyhow::Error> {
    let argon2 = Argon2::default();
    let parsed_hash =
        PasswordHash::new(hash).map_err(|_| anyhow::anyhow!("Failed to parse hash"))?;
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| anyhow::anyhow!("Failed to verify password"))?;
    Ok(())
}

pub async fn get_user(
    State(db): State<Database>,
    UserTokenExtractor { user_id }: UserTokenExtractor,
) -> Result<impl IntoResponse, ApiError> {
    let user = db
        .get_user_by_id(&user_id)
        .await
        .map_err(|_| ApiError::msg("Failed to get user"))?;

    let user = user.ok_or_else(|| ApiError::msg("User not found").code(StatusCode::NOT_FOUND))?;
    let api_user: ApiUser = user.into();
    Ok(Json(api_user))
}

pub async fn get_public_users(
    State(db): State<Database>,
    page: i64,
    limit: i64,
) -> Result<impl IntoResponse, ApiError> {
    let users = db
        .get_public_users(page, limit)
        .await
        .map_err(|_| ApiError::msg("Failed to get users"))?;

    let api_users: Vec<ApiUser> = users.into_iter().map(|u| u.into()).collect();
    Ok(Json(api_users))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "hunter42";
        let hashed_password = hash_password(password).unwrap();
        assert_ne!(password, hashed_password);
    }

    #[test]
    fn test_verify_password() {
        let password = "hunter42";
        let hashed_password = hash_password(password).unwrap();
        assert!(verify_password(password, &hashed_password).is_ok());
    }
}

use std::net::SocketAddr;

use axum::{middleware, Router};

mod db;
mod handler;
mod jwt;
mod model;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();
    let state = RuimContext::new().await?;

    let app = Router::new()
        .route(
            "/api/chat",
            axum::routing::get(handler::chat::websocket_handler).route_layer(
                middleware::from_fn_with_state(state.clone(), service::auth::guard),
            ),
        )
        .nest("/api/user", handler::user::router(state.clone()))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct RuimContext {
    db: db::Database,
    jwt: jwt::Jwt,
}

impl RuimContext {
    pub async fn new() -> anyhow::Result<Self> {
        let db = db::Database::new().await?;
        let jwt = jwt::Jwt::new_from_env()?;
        Ok(Self { db, jwt })
    }
}

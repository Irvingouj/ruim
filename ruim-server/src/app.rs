use std::net::SocketAddr;

use axum::{middleware, Router};

use crate::{context::RuimContext, handler, service};

pub fn create_app(state: RuimContext) -> Router {
    Router::new()
        .route(
            "/api/chat",
            axum::routing::get(handler::chat::websocket_handler).route_layer(
                middleware::from_fn_with_state(state.clone(), service::auth::guard),
            ),
        )
        .nest("/api/user", handler::user::router(state.clone()))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}

pub async fn start_ruim_server() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = RuimContext::new().await?;
    let app = create_app(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

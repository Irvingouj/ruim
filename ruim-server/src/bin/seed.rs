use api_models::user::RegisterBody;
use axum_test::TestServer;
use ruim_server_lib::{app::create_app, context::RuimContext};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();
    let app = create_app(RuimContext::new().await?);
    let server = TestServer::new(app)?;
    let users = create_users();

    for user in users {
        let response = server.put("/api/user/signup").json(&user).await;

        tracing::info!(?response);
    }
    Ok(())
}

pub fn create_users() -> Vec<RegisterBody> {
    let api_user1 = RegisterBody {
        username: String::from("JohnDoe"),
        email: String::from("johndoe@example.com"),
        password: String::from("password1"),
    };

    let api_user2 = RegisterBody {
        username: String::from("JaneSmith"),
        email: String::from("janesmith@example.com"),
        password: String::from("password2"),
    };

    let api_user3 = RegisterBody {
        username: String::from("AliceJohnson"),
        email: String::from("alicejohnson@example.com"),
        password: String::from("password3"),
    };

    let api_user4 = RegisterBody {
        username: String::from("BobWilliams"),
        email: String::from("bobwilliams@example.com"),
        password: String::from("password4"),
    };

    let api_user5 = RegisterBody {
        username: String::from("EmilyBrown"),
        email: String::from("emilybrown@example.com"),
        password: String::from("password5"),
    };

    let users = vec![api_user1, api_user2, api_user3, api_user4, api_user5];
    users
}

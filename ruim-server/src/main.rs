use ruim_server_lib::app::start_ruim_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_ruim_server().await
}

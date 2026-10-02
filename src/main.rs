use axum_template::{config::shared::SharedConfig, server::Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    axum_template::tracing::init();

    let config = SharedConfig::from_env()?;
    let server = Server::new(config).await?;

    server.run().await?;

    Ok(())
}

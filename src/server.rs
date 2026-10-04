use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use tokio::net::TcpListener;

use crate::{
    config::shared::SharedConfig, context::ServerContext, middleware::apply_middlewares,
    routes::build_axum_router,
};

pub struct Server {
    router: Router,
    listener: TcpListener,
}

impl Server {
    pub async fn new(config: SharedConfig) -> Result<Self, ServerInitError> {
        let address = config.bind.address();
        let listener = TcpListener::bind(address).await?;

        let cx = ServerContext::builder().config(Arc::new(config)).build();

        let router = build_axum_router(cx.clone());
        let router = apply_middlewares(router, cx);

        Ok(Self { router, listener })
    }

    pub fn address(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub async fn run(self) -> std::io::Result<()> {
        axum::serve(self.listener, self.router.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServerInitError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl-c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::SignalKind;

        tokio::signal::unix::signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
    _ = ctrl_c => {},
    _ = terminate => {}
    }

    tracing::info!("received termination signal, shutting down...");
}

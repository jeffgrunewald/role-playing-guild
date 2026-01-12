use anyhow::Result;
use axum::{Router, routing::get};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

pub struct ApiServer {
    listen_addr: SocketAddr,
    state: ApiState,
}

#[derive(Clone)]
pub struct ApiState {
    db: SqlitePool,
}

impl ApiServer {
    pub fn new(db: SqlitePool, listen_addr: SocketAddr) -> Self {
        Self {
            listen_addr,
            state: ApiState::new(db),
        }
    }

    pub async fn run(self, shutdown: triggered::Listener) -> Result<()> {
        let router = Router::new()
            .route("/health", get(health))
            .with_state(self.state.clone());

        let listener = TcpListener::bind(self.listen_addr)
            .await
            .expect("Failed to bind tcp address");

        info!(addr = %self.listen_addr, "API server listening");

        axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(shutdown)
            .await
            .map_err(anyhow::Error::from)
    }
}

impl ApiState {
    fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

async fn health() -> &'static str {
    "OK"
}

use crate::routes::{create_router, AppState};
use anyhow::{Context, Result};
use domain::AppConfig;
use indexer::Database;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn run_server(config: AppConfig, db: Database) -> Result<()> {
    let state = AppState {
        db,
        config: Arc::new(config.clone()),
    };

    let app = create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .context("Invalid host/port configuration")?;

    info!("Starting Axum HTTP server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await.context("Server crashed")?;

    Ok(())
}

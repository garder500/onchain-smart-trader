pub mod health;
pub mod performance;
pub mod portfolio;
pub mod positions;
pub mod signals;
pub mod strategies;
pub mod tokens;
pub mod wallets;

use axum::Router;
use domain::AppConfig;
use indexer::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub config: Arc<AppConfig>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(health::router())
        .merge(wallets::router())
        .merge(tokens::router())
        .merge(signals::router())
        .merge(positions::router())
        .merge(portfolio::router())
        .merge(performance::router())
        .merge(strategies::router())
        .with_state(state)
}

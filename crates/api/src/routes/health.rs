use crate::routes::AppState;
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "onchain-smart-trader",
        "mode": "paper-trading-simulation",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

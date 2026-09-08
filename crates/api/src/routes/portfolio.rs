use crate::routes::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct StrategyQuery {
    pub strategy_id: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/portfolio", get(get_portfolio))
}

async fn get_portfolio(
    State(state): State<AppState>,
    Query(query): Query<StrategyQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let strat_id = query
        .strategy_id
        .unwrap_or_else(|| "Smart Wallet Copy".to_string());

    let snapshot = state
        .db
        .get_latest_portfolio_snapshot(&strat_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let positions = state
        .db
        .list_positions(50, 0)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "strategy_id": strat_id,
        "initial_balance": state.config.initial_paper_balance,
        "latest_snapshot": snapshot,
        "positions": positions
    })))
}

use crate::routes::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/wallets", get(list_wallets))
        .route("/wallets/{address}", get(get_wallet))
}

async fn list_wallets(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let limit = pagination.limit.unwrap_or(50).min(100);
    let offset = pagination.offset.unwrap_or(0);

    let wallets = state
        .db
        .list_wallets(limit, offset)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "limit": limit,
        "offset": offset,
        "count": wallets.len(),
        "wallets": wallets
    })))
}

async fn get_wallet(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let wallet = state
        .db
        .get_wallet(&address)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("Wallet {} not found", address),
        ))?;

    let latest_score = state
        .db
        .get_latest_wallet_score(&address)
        .await
        .unwrap_or(None);

    Ok(Json(json!({
        "wallet": wallet,
        "latest_score": latest_score
    })))
}

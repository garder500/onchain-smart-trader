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
        .route("/tokens", get(list_tokens))
        .route("/tokens/{address}", get(get_token))
}

async fn list_tokens(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let limit = pagination.limit.unwrap_or(50).min(100);
    let offset = pagination.offset.unwrap_or(0);

    let tokens = state
        .db
        .list_tokens(limit, offset)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "limit": limit,
        "offset": offset,
        "count": tokens.len(),
        "tokens": tokens
    })))
}

async fn get_token(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = state
        .db
        .get_token(&address)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((
            StatusCode::NOT_FOUND,
            format!("Token {} not found", address),
        ))?;

    let latest_risk_score = state
        .db
        .get_latest_token_risk_score(&address)
        .await
        .unwrap_or(None);

    Ok(Json(json!({
        "token": token,
        "risk_score": latest_risk_score
    })))
}

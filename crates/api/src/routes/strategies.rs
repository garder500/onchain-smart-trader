use crate::routes::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/strategies", get(list_strategies))
        .route("/strategies/{id}", get(get_strategy))
}

async fn list_strategies(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let rows = sqlx::query("SELECT id, strategy_type, config, initial_balance, current_balance, started_at, ended_at FROM strategy_runs ORDER BY started_at DESC")
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let strategies: Vec<Value> = rows
        .into_iter()
        .map(|r| {
            use sqlx::Row;
            json!({
                "id": r.get::<String, _>("id"),
                "strategy_type": r.get::<String, _>("strategy_type"),
                "config": r.get::<Value, _>("config"),
                "initial_balance": r.get::<rust_decimal::Decimal, _>("initial_balance"),
                "current_balance": r.get::<rust_decimal::Decimal, _>("current_balance"),
                "started_at": r.get::<chrono::DateTime<chrono::Utc>, _>("started_at"),
                "ended_at": r.get::<Option<chrono::DateTime<chrono::Utc>>, _>("ended_at"),
            })
        })
        .collect();

    Ok(Json(json!({ "strategies": strategies })))
}

async fn get_strategy(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let row = sqlx::query("SELECT id, strategy_type, config, initial_balance, current_balance, started_at, ended_at FROM strategy_runs WHERE id = $1")
        .bind(&id)
        .fetch_optional(state.db.pool())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, format!("Strategy {} not found", id)))?;

    use sqlx::Row;
    Ok(Json(json!({
        "id": row.get::<String, _>("id"),
        "strategy_type": row.get::<String, _>("strategy_type"),
        "config": row.get::<Value, _>("config"),
        "initial_balance": row.get::<rust_decimal::Decimal, _>("initial_balance"),
        "current_balance": row.get::<rust_decimal::Decimal, _>("current_balance"),
        "started_at": row.get::<chrono::DateTime<chrono::Utc>, _>("started_at"),
        "ended_at": row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("ended_at"),
    })))
}

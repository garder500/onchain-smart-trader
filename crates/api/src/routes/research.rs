use crate::routes::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use domain::Trade;
use research::{
    CopiabilityEngine, ExperimentConfig, ExperimentReport, ReportGenerator, ResearchRunner,
    ScalabilityEngine,
};
use rust_decimal::Decimal;
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/research/experiments", get(get_or_run_experiment))
        .route("/api/v1/research/report", get(get_markdown_report))
        .route("/api/v1/research/copyability", get(get_copyability))
        .route("/api/v1/research/scalability", get(get_scalability))
        .route("/api/v1/research/run", post(run_custom_experiment))
}

#[derive(Debug, Deserialize)]
pub struct ResearchQuery {
    pub limit: Option<i64>,
}

async fn fetch_or_synthesize_trades(
    state: &AppState,
    limit: i64,
) -> (Vec<Trade>, research::DataSource) {
    let db_trades = state.db.get_all_trades(limit).await.unwrap_or_default();
    if !db_trades.is_empty() {
        (db_trades, research::DataSource::Real)
    } else {
        (
            generate_synthetic_research_dataset(),
            research::DataSource::Synthetic,
        )
    }
}

pub async fn get_or_run_experiment(
    State(state): State<AppState>,
    Query(query): Query<ResearchQuery>,
) -> Result<Json<ExperimentReport>, StatusCode> {
    let limit = query.limit.unwrap_or(1000);
    let (trades, data_source) = fetch_or_synthesize_trades(&state, limit).await;

    let config = ExperimentConfig {
        data_source,
        permutation_iterations: 100,
        bootstrap_iterations: 200,
        ..Default::default()
    };

    let report = ResearchRunner::run_experiment(&trades, &config, "git-head");
    Ok(Json(report))
}

pub async fn get_markdown_report(
    State(state): State<AppState>,
    Query(query): Query<ResearchQuery>,
) -> Response {
    let limit = query.limit.unwrap_or(1000);
    let (trades, data_source) = fetch_or_synthesize_trades(&state, limit).await;

    let config = ExperimentConfig {
        data_source,
        permutation_iterations: 100,
        bootstrap_iterations: 200,
        ..Default::default()
    };

    let report = ResearchRunner::run_experiment(&trades, &config, "git-head");
    let markdown = ReportGenerator::generate_markdown(&report);

    (
        StatusCode::OK,
        [("content-type", "text/markdown; charset=utf-8")],
        markdown,
    )
        .into_response()
}

pub async fn get_copyability(
    State(state): State<AppState>,
    Query(query): Query<ResearchQuery>,
) -> Result<Json<Vec<research::DelayImpactPoint>>, StatusCode> {
    let limit = query.limit.unwrap_or(1000);
    let (trades, _) = fetch_or_synthesize_trades(&state, limit).await;

    let delays = vec![0, 1, 2, 5, 10, 15, 30, 60, 120];
    let results =
        CopiabilityEngine::evaluate_latency_matrix(&trades, &delays, Decimal::from(1000), 30);

    Ok(Json(results))
}

pub async fn get_scalability(
    State(state): State<AppState>,
    Query(query): Query<ResearchQuery>,
) -> Result<Json<Vec<research::ScalabilityImpactPoint>>, StatusCode> {
    let limit = query.limit.unwrap_or(1000);
    let (trades, _) = fetch_or_synthesize_trades(&state, limit).await;

    let capitals = vec![
        Decimal::from(100),
        Decimal::from(500),
        Decimal::from(1000),
        Decimal::from(5000),
        Decimal::from(10000),
        Decimal::from(50000),
        Decimal::from(100000),
    ];

    let results =
        ScalabilityEngine::evaluate_scalability(&trades, &capitals, Decimal::from(100_000), 30);

    Ok(Json(results))
}

pub async fn run_custom_experiment(
    State(state): State<AppState>,
    Json(config): Json<ExperimentConfig>,
) -> Result<Json<ExperimentReport>, StatusCode> {
    let (trades, _) = fetch_or_synthesize_trades(&state, 2000).await;
    let report = ResearchRunner::run_experiment(&trades, &config, "git-head");
    Ok(Json(report))
}

pub fn generate_synthetic_research_dataset() -> Vec<Trade> {
    use chrono::{Duration, Utc};
    use domain::{TradeSide, TxHash, WalletAddress};
    use uuid::Uuid;

    let now = Utc::now();
    let smart_wallet = WalletAddress::new("0x1111111111111111111111111111111111111111");
    let swing_wallet = WalletAddress::new("0x2222222222222222222222222222222222222222");
    let token = "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

    let mut trades = Vec::new();
    for i in 0..15 {
        let buy_time = now - Duration::days(30) + Duration::days(i * 2);
        trades.push(Trade {
            id: Uuid::new_v4(),
            wallet_address: smart_wallet.clone(),
            token_address: token.into(),
            side: TradeSide::Buy,
            amount_tokens: Decimal::from(100),
            price_usd: Decimal::from(10),
            volume_usd: Decimal::from(1000),
            fee_usd: Decimal::from(3),
            tx_hash: TxHash::new(format!("0xtx_b_{}", i)),
            block_number: 1000 + i as u64,
            timestamp: buy_time,
        });

        trades.push(Trade {
            id: Uuid::new_v4(),
            wallet_address: smart_wallet.clone(),
            token_address: token.into(),
            side: TradeSide::Sell,
            amount_tokens: Decimal::from(100),
            price_usd: Decimal::from(15),
            volume_usd: Decimal::from(1500),
            fee_usd: Decimal::from(4),
            tx_hash: TxHash::new(format!("0xtx_s_{}", i)),
            block_number: 1050 + i as u64,
            timestamp: buy_time + Duration::hours(12),
        });
    }

    for i in 0..10 {
        let buy_time = now - Duration::days(25) + Duration::days(i * 2);
        trades.push(Trade {
            id: Uuid::new_v4(),
            wallet_address: swing_wallet.clone(),
            token_address: token.into(),
            side: TradeSide::Buy,
            amount_tokens: Decimal::from(50),
            price_usd: Decimal::from(20),
            volume_usd: Decimal::from(1000),
            fee_usd: Decimal::from(3),
            tx_hash: TxHash::new(format!("0xsw_b_{}", i)),
            block_number: 2000 + i as u64,
            timestamp: buy_time,
        });

        trades.push(Trade {
            id: Uuid::new_v4(),
            wallet_address: swing_wallet.clone(),
            token_address: token.into(),
            side: TradeSide::Sell,
            amount_tokens: Decimal::from(50),
            price_usd: Decimal::from(28),
            volume_usd: Decimal::from(1400),
            fee_usd: Decimal::from(4),
            tx_hash: TxHash::new(format!("0xsw_s_{}", i)),
            block_number: 2050 + i as u64,
            timestamp: buy_time + Duration::days(2),
        });
    }

    trades
}

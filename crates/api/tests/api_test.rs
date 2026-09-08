use api::{create_router, AppState};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use domain::AppConfig;
use http_body_util::BodyExt;
use indexer::Database;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_api_health_endpoint() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgrespassword@localhost:5432/trading_bot".into()
    });

    let db = match Database::connect(&db_url).await {
        Ok(db) => db,
        Err(_) => return,
    };
    db.run_migrations().await.unwrap();

    let config = AppConfig::from_env().unwrap_or(AppConfig {
        database_url: db_url,
        rpc_http_url: "https://eth.llamarpc.com".into(),
        rpc_ws_url: "wss://eth.llamarpc.com".into(),
        chain_id: 1,
        initial_paper_balance: rust_decimal::Decimal::from(1000),
        min_liquidity: rust_decimal::Decimal::from(10000),
        max_position_percent: rust_decimal::Decimal::from_str_radix("0.1", 10).unwrap(),
        max_open_positions: 5,
        slippage_bps: 50,
        trading_fee_bps: 30,
        min_wallet_score: rust_decimal::Decimal::from(65),
        min_wallet_trades: 5,
        host: "0.0.0.0".into(),
        port: 3000,
        token_risk: domain::TokenRiskWeights::default(),
    });

    let app = create_router(AppState {
        db,
        config: Arc::new(config),
    });

    // 1. Health check test
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body_json["status"], "ok");
    assert_eq!(body_json["service"], "onchain-smart-trader");

    // 2. Wallets list test
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/wallets?limit=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Tokens list test
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/tokens?limit=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 4. Portfolio test
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/portfolio")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 5. Research Experiments endpoint test
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/research/experiments")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let report_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(report_json["experiment_id"].is_string());
    assert!(report_json["verdict"]["data_source"].is_string());

    // 6. Research Copiability endpoint test
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/research/copyability")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 7. Research Markdown Report endpoint test
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/research/report")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let md_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let md_str = String::from_utf8_lossy(&md_bytes);
    assert!(md_str.contains("Research Experiment Report"));
    assert!(md_str.contains("DATA SOURCE"));
}

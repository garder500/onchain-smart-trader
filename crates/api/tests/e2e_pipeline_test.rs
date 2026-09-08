use chrono::{Duration, Utc};
use domain::{
    Portfolio, PositionStatus, SignalAction, SignalReason, Token, TokenAddress, TokenContext,
    Trade, TradeSide, TxHash, WalletAddress,
};
use indexer::Database;
use paper_trader::{OrderExecutor, PaperExecutor, PaperExecutorConfig};
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;
use strategy::{
    CompositeExitStrategy, PositionSizer, PositionSizerConfig, RiskManager, RiskManagerConfig,
    SmartWalletCopyStrategy, StrategyConfig,
};
use token_risk::TokenRiskEngine;
use uuid::Uuid;

#[tokio::test]
async fn test_full_pipeline_indexer_strategy_order_portfolio() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgrespassword@localhost:5432/trading_bot".into()
    });

    let db = match Database::connect(&db_url).await {
        Ok(db) => db,
        Err(_) => return,
    };
    db.run_migrations().await.unwrap();

    let now = Utc::now();
    let unique_suffix = Uuid::new_v4().as_u128();
    let wallet_addr = WalletAddress::new(format!("0x{:032x}aabbccdd", unique_suffix));
    let token_addr = TokenAddress::new(format!("0x{:032x}eeff0011", unique_suffix));

    // 1. Ingest historical smart wallet track record into Database (Indexer -> DB)
    let mut historical_trades = Vec::new();
    for i in 0..6 {
        let t = now - Duration::days(10 - i);
        let buy = Trade {
            id: Uuid::new_v4(),
            wallet_address: wallet_addr.clone(),
            token_address: format!("0x{:032x}11111111", i).into(),
            side: TradeSide::Buy,
            amount_tokens: Decimal::from(100),
            price_usd: Decimal::from(5),
            volume_usd: Decimal::from(500),
            fee_usd: Decimal::from(1),
            tx_hash: TxHash::new(format!("0x{:064x}", Uuid::new_v4().as_u128())),
            block_number: 100 + i as u64,
            timestamp: t,
        };
        db.insert_trade(&buy).await.unwrap();
        historical_trades.push(buy);

        let sell = Trade {
            id: Uuid::new_v4(),
            wallet_address: wallet_addr.clone(),
            token_address: format!("0x{:032x}11111111", i).into(),
            side: TradeSide::Sell,
            amount_tokens: Decimal::from(100),
            price_usd: Decimal::from(10), // 100% gain
            volume_usd: Decimal::from(1000),
            fee_usd: Decimal::from(2),
            tx_hash: TxHash::new(format!("0x{:064x}", Uuid::new_v4().as_u128())),
            block_number: 105 + i as u64,
            timestamp: t + Duration::hours(2),
        };
        db.insert_trade(&sell).await.unwrap();
        historical_trades.push(sell);
    }

    // 2. Discovered token context
    let token = Token {
        address: token_addr.clone(),
        deployer: None,
        creation_block: Some(1000),
        creation_timestamp: Some(now - Duration::hours(1)),
        symbol: Some("ALPHA".into()),
        name: Some("Alpha Token".into()),
        decimals: 18,
        total_supply: Some(Decimal::from(1000000)),
        liquidity_usd: Some(Decimal::from(80000)),
        holders_count: Some(200),
        top_holders: Vec::new(),
        top_10_holder_concentration: Some(Decimal::from_str("0.20").unwrap()),
        mint_capability: Some(false),
        pause_freeze_capability: Some(false),
        liquidity_lock_info: Some("Locked".into()),
        is_honeypot: Some(false),
        created_at: now - Duration::hours(1),
        updated_at: now,
    };
    db.upsert_token(&token).await.unwrap();

    let token_ctx = TokenContext {
        token,
        current_timestamp: now,
        pool_liquidity_usd: Decimal::from(80000),
        volume_24h_usd: Decimal::from(20000),
        deployer_historic_rugs: 0,
        deployer_total_launches: 2,
    };

    // 3. Smart wallet executes a new BUY
    let live_buy = Trade {
        id: Uuid::new_v4(),
        wallet_address: wallet_addr.clone(),
        token_address: token_addr.clone(),
        side: TradeSide::Buy,
        amount_tokens: Decimal::from(500),
        price_usd: Decimal::from(2),
        volume_usd: Decimal::from(1000),
        fee_usd: Decimal::from(3),
        tx_hash: TxHash::new(format!("0x{:064x}", Uuid::new_v4().as_u128())),
        block_number: 2000,
        timestamp: now,
    };
    db.insert_trade(&live_buy).await.unwrap();

    // 4. Strategy Engine evaluates signal (Strategy -> Signal)
    let strat_config = StrategyConfig {
        strategy_id: "E2E_Smart_Copy".into(),
        min_wallet_score: Decimal::from(65),
        min_wallet_trades: 5,
    };
    let sizer = PositionSizer::new(PositionSizerConfig::default());
    let exit_strategy = CompositeExitStrategy::default_rules(
        Decimal::from_str("0.50").unwrap(),
        Decimal::from_str("0.20").unwrap(),
        86400,
    );
    let risk_manager = RiskManager::new(RiskManagerConfig::default());
    let strategy = SmartWalletCopyStrategy::new(
        strat_config,
        Arc::new(TokenRiskEngine::default()),
        sizer,
        exit_strategy,
        risk_manager,
    );

    let mut portfolio = Portfolio::new(Decimal::from(1000), now);
    let signal = strategy
        .on_trade_event(&live_buy, &historical_trades, &token_ctx, &portfolio)
        .expect("Signal should be generated and approved");

    assert_eq!(signal.action, SignalAction::Buy);
    assert_eq!(signal.token_address, token_addr);
    assert_eq!(signal.reason, SignalReason::SmartWalletFollow);
    db.save_signal(&signal).await.unwrap();

    // 5. Paper Trader executes signal (Signal -> Paper Order -> Portfolio)
    let executor = PaperExecutor::new(PaperExecutorConfig::default());
    let order = executor
        .execute_order(
            &signal,
            Decimal::from(2),
            token_ctx.pool_liquidity_usd,
            &mut portfolio,
        )
        .await
        .expect("Order execution should succeed");

    assert_eq!(order.action, SignalAction::Buy);
    assert!(order.execution_price > Decimal::from(2)); // slippage applied
    assert_eq!(portfolio.open_positions_count(), 1);
    db.save_paper_order(&order, "E2E_Smart_Copy").await.unwrap();

    for pos in portfolio.positions.values() {
        db.save_paper_position(pos, "E2E_Smart_Copy").await.unwrap();
    }

    // 6. Verify positions and state in Database
    let open_positions = db.list_positions(10, 0).await.unwrap();
    assert!(!open_positions.is_empty());
    assert_eq!(open_positions[0].status, PositionStatus::Open);
    assert_eq!(open_positions[0].token_address, token_addr);
}

use chrono::{Duration, Utc};
use domain::{Token, TokenAddress, Trade, TradeSide, TxHash, Wallet, WalletAddress};
use indexer::Database;
use rust_decimal::Decimal;
use std::str::FromStr;
use uuid::Uuid;

#[tokio::test]
async fn test_database_idempotent_operations() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgrespassword@localhost:5432/trading_bot".into()
    });

    let db = match Database::connect(&db_url).await {
        Ok(db) => db,
        Err(_) => {
            println!("Skipping DB test, database not reachable");
            return;
        }
    };

    // Run migrations
    db.run_migrations().await.expect("Failed to run migrations");

    // 1. Checkpoint test
    let test_cp_id = format!("test_cp_{}", Uuid::new_v4());
    db.save_checkpoint(&test_cp_id, 100)
        .await
        .expect("Save checkpoint failed");
    let cp = db
        .get_checkpoint(&test_cp_id)
        .await
        .expect("Get checkpoint failed");
    assert_eq!(cp, Some(100));

    // Update checkpoint
    db.save_checkpoint(&test_cp_id, 101)
        .await
        .expect("Update checkpoint failed");
    let cp2 = db
        .get_checkpoint(&test_cp_id)
        .await
        .expect("Get checkpoint failed");
    assert_eq!(cp2, Some(101));

    // 2. Wallet idempotency test
    let now = Utc::now();
    let wallet_addr = WalletAddress::new(format!("0x{:040x}", 12345));
    let wallet = Wallet::new(wallet_addr.clone(), now);

    db.upsert_wallet(&wallet)
        .await
        .expect("Upsert wallet failed");
    // Second upsert should succeed without error
    db.upsert_wallet(&wallet)
        .await
        .expect("Second upsert wallet failed");

    let loaded_wallet = db
        .get_wallet(wallet_addr.as_str())
        .await
        .expect("Get wallet failed");
    assert!(loaded_wallet.is_some());

    // 3. Token upsert test
    let token_addr = TokenAddress::new(format!("0x{:040x}", 67890));
    let token = Token {
        address: token_addr.clone(),
        deployer: Some(wallet_addr.clone()),
        creation_block: Some(100),
        creation_timestamp: Some(now),
        symbol: Some("SMART".into()),
        name: Some("Smart Token".into()),
        decimals: 18,
        total_supply: Some(Decimal::from(1000000)),
        liquidity_usd: Some(Decimal::from(50000)),
        holders_count: Some(42),
        top_holders: Vec::new(),
        top_10_holder_concentration: Some(Decimal::from_str("0.25").unwrap()),
        mint_capability: Some(false),
        pause_freeze_capability: Some(false),
        liquidity_lock_info: Some("Locked for 1 year".into()),
        is_honeypot: Some(false),
        created_at: now,
        updated_at: now,
    };

    db.upsert_token(&token).await.expect("Upsert token failed");
    db.upsert_token(&token)
        .await
        .expect("Second upsert token failed");

    let loaded_token = db
        .get_token(token_addr.as_str())
        .await
        .expect("Get token failed");
    assert!(loaded_token.is_some());
    assert_eq!(loaded_token.unwrap().symbol, Some("SMART".into()));

    // 4. Trade insert and anti-look-ahead filtering test
    let tx_hash = TxHash::new(format!("0x{:064x}", 99999));
    let t1_time = now - Duration::minutes(10);
    let future_time = now + Duration::minutes(5);

    let trade1 = Trade {
        id: Uuid::new_v4(),
        wallet_address: wallet_addr.clone(),
        token_address: token_addr.clone(),
        side: TradeSide::Buy,
        amount_tokens: Decimal::from(100),
        price_usd: Decimal::from(2),
        volume_usd: Decimal::from(200),
        fee_usd: Decimal::from_str("0.6").unwrap(),
        tx_hash: tx_hash.clone(),
        block_number: 100,
        timestamp: t1_time,
    };

    db.insert_trade(&trade1)
        .await
        .expect("Insert trade 1 failed");

    let trade2 = Trade {
        id: Uuid::new_v4(),
        wallet_address: wallet_addr.clone(),
        token_address: token_addr.clone(),
        side: TradeSide::Sell,
        amount_tokens: Decimal::from(100),
        price_usd: Decimal::from(3),
        volume_usd: Decimal::from(300),
        fee_usd: Decimal::from_str("0.9").unwrap(),
        tx_hash: TxHash::new(format!("0x{:064x}", 88888)),
        block_number: 101,
        timestamp: future_time,
    };

    db.insert_trade(&trade2)
        .await
        .expect("Insert trade 2 failed");

    // Strictly fetch trades before now (eval timestamp)
    let trades_before_now = db
        .get_wallet_trades_before(&wallet_addr, now)
        .await
        .expect("Fetch trades failed");

    // Only trade 1 should be included! (Anti-look-ahead check)
    assert_eq!(trades_before_now.len(), 1);
    assert_eq!(trades_before_now[0].side, TradeSide::Buy);
}

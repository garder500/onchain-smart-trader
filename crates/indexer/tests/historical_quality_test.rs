use chrono::{Duration, Utc};
use domain::{TokenAddress, Trade, TradeSide, TxHash, WalletAddress};
use indexer::historical::HistoricalIngestionService;
use rust_decimal::Decimal;
use uuid::Uuid;

#[test]
fn test_data_quality_verification_passes_on_valid_dataset() {
    let base_time = Utc::now() - Duration::days(35);
    let mut trades = Vec::new();

    // Create 550 trades across 60 wallets spanning 35 days
    for i in 0..550 {
        let wallet_idx = i % 60;
        let wallet_addr = format!("0x{:040x}", wallet_idx + 1);
        let trade = Trade {
            id: Uuid::new_v4(),
            wallet_address: WalletAddress::new(wallet_addr),
            token_address: TokenAddress::new("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            side: if i % 2 == 0 {
                TradeSide::Buy
            } else {
                TradeSide::Sell
            },
            amount_tokens: Decimal::from(1),
            price_usd: Decimal::from(2000),
            volume_usd: Decimal::from(2000),
            fee_usd: Decimal::from(5),
            tx_hash: TxHash::new(format!("0x{:064x}", i + 1)),
            block_number: 25000000 + i as u64,
            timestamp: base_time + Duration::hours(i as i64 * 35 * 24 / 550),
        };
        trades.push(trade);
    }

    let report = HistoricalIngestionService::evaluate_quality(&trades);
    assert_eq!(report.total_records, 550);
    assert_eq!(report.duplicates_count, 0);
    assert_eq!(report.null_or_zero_prices, 0);
    assert_eq!(report.null_or_zero_amounts, 0);
    assert_eq!(report.negative_fees, 0);
    assert_eq!(report.invalid_addresses, 0);
    assert_eq!(report.monotonic_timestamp_violations, 0);
    assert!(report.timespan_days >= 30.0);
    assert!(report.unique_wallets >= 50);
    assert!(report.passed_all_checks);
}

#[test]
fn test_data_quality_verification_catches_anomalies() {
    let base_time = Utc::now();
    let bad_trades = vec![
        // Trade with zero price
        Trade {
            id: Uuid::new_v4(),
            wallet_address: WalletAddress::new("0x0000000000000000000000000000000000000001"),
            token_address: TokenAddress::new("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            side: TradeSide::Buy,
            amount_tokens: Decimal::from(1),
            price_usd: Decimal::ZERO, // Zero price!
            volume_usd: Decimal::from(100),
            fee_usd: Decimal::from(5),
            tx_hash: TxHash::new(
                "0x0000000000000000000000000000000000000000000000000000000000000001",
            ),
            block_number: 100,
            timestamp: base_time,
        },
        // Trade with negative fee
        Trade {
            id: Uuid::new_v4(),
            wallet_address: WalletAddress::new("0x0000000000000000000000000000000000000002"),
            token_address: TokenAddress::new("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            side: TradeSide::Sell,
            amount_tokens: Decimal::from(1),
            price_usd: Decimal::from(2000),
            volume_usd: Decimal::from(2000),
            fee_usd: Decimal::from(-1), // Negative fee!
            tx_hash: TxHash::new(
                "0x0000000000000000000000000000000000000000000000000000000000000002",
            ),
            block_number: 101,
            timestamp: base_time + Duration::seconds(10),
        },
        // Trade with invalid EVM address
        Trade {
            id: Uuid::new_v4(),
            wallet_address: WalletAddress::new("invalid_not_hex"), // Invalid address!
            token_address: TokenAddress::new("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
            side: TradeSide::Buy,
            amount_tokens: Decimal::from(1),
            price_usd: Decimal::from(2000),
            volume_usd: Decimal::from(2000),
            fee_usd: Decimal::from(5),
            tx_hash: TxHash::new(
                "0x0000000000000000000000000000000000000000000000000000000000000003",
            ),
            block_number: 102,
            timestamp: base_time + Duration::seconds(20),
        },
    ];

    let report = HistoricalIngestionService::evaluate_quality(&bad_trades);
    assert_eq!(report.null_or_zero_prices, 1);
    assert_eq!(report.negative_fees, 1);
    assert_eq!(report.invalid_addresses, 1);
    assert!(!report.passed_all_checks);
}

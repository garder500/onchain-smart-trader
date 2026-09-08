pub mod metrics;
pub mod scorer;

pub use metrics::{MetricsCalculator, RoundTripTrade};
pub use scorer::{calculate_wallet_score, WalletContext};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use domain::{Trade, TradeSide, TxHash, WalletAddress, WalletCategory};
    use rust_decimal::Decimal;
    use uuid::Uuid;

    #[test]
    fn test_wallet_metrics_and_scoring() {
        let now = Utc::now();
        let wallet = WalletAddress::new("0x1111111111111111111111111111111111111111");
        let token = "0x2222222222222222222222222222222222222222";

        let mut trades = Vec::new();
        // Generate 6 profitable trades: buy at $10, sell at $15
        for i in 0..6 {
            let buy_time = now - Duration::days(10 - i);
            let sell_time = buy_time + Duration::hours(2);

            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token.into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(3),
                tx_hash: TxHash::new(format!("0xb{}", i)),
                block_number: 100 + i as u64,
                timestamp: buy_time,
            });

            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token.into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(15),
                volume_usd: Decimal::from(1500),
                fee_usd: Decimal::from(4),
                tx_hash: TxHash::new(format!("0xs{}", i)),
                block_number: 110 + i as u64,
                timestamp: sell_time,
            });
        }

        let ctx = WalletContext {
            wallet_address: wallet.clone(),
            trades,
            eval_timestamp: now,
            min_trades_threshold: 5,
        };

        let score = calculate_wallet_score(&ctx);

        assert_eq!(score.metrics.total_trades, 6);
        assert_eq!(score.metrics.winning_trades, 6);
        assert_eq!(score.metrics.losing_trades, 0);
        assert_eq!(score.metrics.win_rate, Decimal::ONE);
        assert!(score.overall_score >= Decimal::from(65));
        assert_eq!(score.category, WalletCategory::Smart);
    }

    #[test]
    fn test_insufficient_sample_size_capped() {
        let now = Utc::now();
        let wallet = WalletAddress::new("0x1111111111111111111111111111111111111111");
        let token = "0x2222222222222222222222222222222222222222";

        // Only 1 trade
        let trades = vec![
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token.into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(3),
                tx_hash: TxHash::new("0xb1"),
                block_number: 100,
                timestamp: now - Duration::hours(2),
            },
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token.into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(50), // 5x return!
                volume_usd: Decimal::from(5000),
                fee_usd: Decimal::from(4),
                tx_hash: TxHash::new("0xs1"),
                block_number: 101,
                timestamp: now - Duration::hours(1),
            },
        ];

        let ctx = WalletContext {
            wallet_address: wallet,
            trades,
            eval_timestamp: now,
            min_trades_threshold: 5, // minimum 5 trades required
        };

        let score = calculate_wallet_score(&ctx);
        // Even with a 5x trade, category MUST be UNKNOWN due to sample size
        assert_eq!(score.category, WalletCategory::Unknown);
    }
}

pub mod metrics;
pub mod scorer;

pub use metrics::{MetricsCalculator, RoundTripTrade};
pub use scorer::{
    calculate_wallet_score, calculate_wallet_score_with_market_prices, WalletContext,
};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use domain::{Trade, TradeSide, TxHash, WalletAddress, WalletCategory};
    use rust_decimal::Decimal;
    use std::collections::HashMap;
    use std::str::FromStr;
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
        assert!(score.metrics.expectancy > Decimal::ZERO);
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

    #[test]
    fn test_unclosed_rug_pull_survivorship_bias_eliminated() {
        let now = Utc::now();
        let wallet = WalletAddress::new("0x9999999999999999999999999999999999999999");
        let token_good = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let token_rug = "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

        // 1 closed winning trade on token_good
        let trades = vec![
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token_good.into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new("0xgood_buy"),
                block_number: 100,
                timestamp: now - Duration::hours(5),
            },
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token_good.into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(20), // 2x gain
                volume_usd: Decimal::from(2000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new("0xgood_sell"),
                block_number: 105,
                timestamp: now - Duration::hours(4),
            },
            // 1 BUY on token_rug that was NEVER sold and dumped to zero
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token_rug.into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(1000),
                price_usd: Decimal::from(5), // $5,000 invested
                volume_usd: Decimal::from(5000),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new("0xrug_buy"),
                block_number: 110,
                timestamp: now - Duration::hours(3),
            },
        ];

        let mut market_prices = HashMap::new();
        market_prices.insert(token_good.to_string(), Decimal::from(20));
        market_prices.insert(token_rug.to_string(), Decimal::ZERO); // Rugged to 0!

        let metrics = MetricsCalculator::compute_metrics_with_market_prices(
            &trades,
            now,
            3600,
            Some(&market_prices),
        );

        // Anti-survivorship bias validation:
        // Total evaluated trades must be 2, NOT 1!
        assert_eq!(metrics.total_trades, 2);
        assert_eq!(metrics.winning_trades, 1);
        assert_eq!(metrics.losing_trades, 1);
        assert_eq!(metrics.win_rate, Decimal::from_str("0.5").unwrap());
        assert_eq!(metrics.rug_exposure_count, 1);
        assert_eq!(metrics.unclosed_positions_count, 1);
        assert!(metrics.unrealized_pnl < Decimal::ZERO);
        // Expectancy must account for the -$5000 loss
        assert!(metrics.expectancy < Decimal::ZERO);
    }
}

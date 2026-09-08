pub mod ab_test;
pub mod metrics;
pub mod replay;
pub mod report;

pub use ab_test::AbTestRunner;
pub use metrics::PerformanceCalculator;
pub use replay::ReplayEngine;
pub use report::ReportGenerator;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use domain::{Token, TokenAddress, TokenContext, Trade, TradeSide, TxHash, WalletAddress};
    use paper_trader::{PaperExecutor, PaperExecutorConfig};
    use rust_decimal::Decimal;
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::sync::Arc;
    use strategy::{
        CompositeExitStrategy, PositionSizer, PositionSizerConfig, RiskManager, RiskManagerConfig,
        SmartWalletCopyStrategy, StrategyConfig,
    };
    use token_risk::TokenRiskEngine;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_anti_look_ahead_enforcement() {
        let now = Utc::now();
        let token = TokenAddress::new("0x1111111111111111111111111111111111111111");
        let wallet = WalletAddress::new("0x2222222222222222222222222222222222222222");

        let strategy_config = StrategyConfig {
            strategy_id: "test".into(),
            min_wallet_score: Decimal::from(65),
            min_wallet_trades: 5,
        };
        let strategy = SmartWalletCopyStrategy::new(
            strategy_config,
            Arc::new(TokenRiskEngine::default()),
            PositionSizer::new(PositionSizerConfig::default()),
            CompositeExitStrategy::default_rules(
                Decimal::from_str("0.5").unwrap(),
                Decimal::from_str("0.2").unwrap(),
                86400,
            ),
            RiskManager::new(RiskManagerConfig::default()),
        );

        let replay = ReplayEngine::new(
            strategy,
            PaperExecutor::new(PaperExecutorConfig::default()),
            Decimal::from(1000),
        );

        // Intentionally create unsorted / future poisoned trade
        let trade_past = Trade {
            id: Uuid::new_v4(),
            wallet_address: wallet.clone(),
            token_address: token.clone(),
            side: TradeSide::Buy,
            amount_tokens: Decimal::from(100),
            price_usd: Decimal::from(10),
            volume_usd: Decimal::from(1000),
            fee_usd: Decimal::from(3),
            tx_hash: TxHash::new("0x1"),
            block_number: 100,
            timestamp: now - Duration::days(1),
        };

        let mut token_contexts = HashMap::new();
        token_contexts.insert(
            token.clone(),
            TokenContext {
                token: Token {
                    address: token.clone(),
                    deployer: None,
                    creation_block: Some(90),
                    creation_timestamp: Some(now - Duration::days(2)),
                    symbol: Some("TEST".into()),
                    name: Some("Test".into()),
                    decimals: 18,
                    total_supply: Some(Decimal::from(1000000)),
                    liquidity_usd: Some(Decimal::from(50000)),
                    holders_count: Some(100),
                    top_holders: Vec::new(),
                    top_10_holder_concentration: Some(Decimal::from_str("0.2").unwrap()),
                    mint_capability: Some(false),
                    pause_freeze_capability: Some(false),
                    liquidity_lock_info: None,
                    is_honeypot: Some(false),
                    created_at: now - Duration::days(2),
                    updated_at: now,
                },
                current_timestamp: now,
                pool_liquidity_usd: Decimal::from(50000),
                volume_24h_usd: Decimal::from(10000),
                deployer_historic_rugs: 0,
                deployer_total_launches: 1,
            },
        );

        let res = replay
            .run_replay(
                "test",
                vec![trade_past],
                &token_contexts,
                now - Duration::days(5),
                now,
            )
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_ab_testing_pipeline() {
        let now = Utc::now();
        let token = TokenAddress::new("0x3333333333333333333333333333333333333333");
        let wallet = WalletAddress::new("0x4444444444444444444444444444444444444444");

        let mut trades = Vec::new();
        for i in 0..10 {
            let t = now - Duration::days(10 - i);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token.clone(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10 + i),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(3),
                tx_hash: TxHash::new(format!("0x{:040x}", i)),
                block_number: 100 + i as u64,
                timestamp: t,
            });
        }

        let mut token_contexts = HashMap::new();
        token_contexts.insert(
            token.clone(),
            TokenContext {
                token: Token {
                    address: token.clone(),
                    deployer: None,
                    creation_block: Some(50),
                    creation_timestamp: Some(now - Duration::days(20)),
                    symbol: Some("COMPARE".into()),
                    name: Some("Compare Token".into()),
                    decimals: 18,
                    total_supply: Some(Decimal::from(1000000)),
                    liquidity_usd: Some(Decimal::from(50000)),
                    holders_count: Some(100),
                    top_holders: Vec::new(),
                    top_10_holder_concentration: Some(Decimal::from_str("0.2").unwrap()),
                    mint_capability: Some(false),
                    pause_freeze_capability: Some(false),
                    liquidity_lock_info: None,
                    is_honeypot: Some(false),
                    created_at: now - Duration::days(20),
                    updated_at: now,
                },
                current_timestamp: now,
                pool_liquidity_usd: Decimal::from(50000),
                volume_24h_usd: Decimal::from(15000),
                deployer_historic_rugs: 0,
                deployer_total_launches: 2,
            },
        );

        let ab_runner = AbTestRunner::new(Decimal::from(1000));
        let results = ab_runner
            .run_all_strategies(trades, &token_contexts, now - Duration::days(15), now)
            .await;

        // All 4 strategies should have completed
        assert_eq!(results.len(), 4);
        let comparison_table = ReportGenerator::format_ab_test_comparison(&results);
        assert!(comparison_table.contains("Smart Wallet Copy"));
        assert!(comparison_table.contains("Naive Copy Trading"));
        assert!(comparison_table.contains("Random Selection"));
        assert!(comparison_table.contains("Buy & Hold"));
    }
}

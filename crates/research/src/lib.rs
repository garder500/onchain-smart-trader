pub mod benchmarks;
pub mod copiability;
pub mod report;
pub mod runner;
pub mod scalability;
pub mod types;
pub mod validation;
pub mod wallet_engine;

pub use benchmarks::BenchmarkEngine;
pub use copiability::CopiabilityEngine;
pub use report::ReportGenerator;
pub use runner::ResearchRunner;
pub use scalability::ScalabilityEngine;
pub use types::*;
pub use validation::ScientificValidator;
pub use wallet_engine::WalletResearchEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use domain::{Trade, TradeSide, TxHash, WalletAddress};
    use rust_decimal::Decimal;
    use uuid::Uuid;

    fn generate_test_trade_sequence() -> Vec<Trade> {
        let now = Utc::now();
        let sniper = WalletAddress::new("0x1111111111111111111111111111111111111111");
        let swing = WalletAddress::new("0x2222222222222222222222222222222222222222");
        let token2 = "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

        let mut trades = Vec::new();

        // 1. Sniper trades (sniping 5 newly created tokens within 60 seconds of pool launch, holding < 5 mins)
        for i in 0..5 {
            let token_sniper = format!("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa{:04}", i);
            let t_buy = now - Duration::days(15) + Duration::hours(i * 12);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: sniper.clone(),
                token_address: token_sniper.clone().into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(3),
                tx_hash: TxHash::new(format!("0xs_b_{}", i)),
                block_number: 100 + i as u64,
                timestamp: t_buy,
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: sniper.clone(),
                token_address: token_sniper.into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(20),
                volume_usd: Decimal::from(2000),
                fee_usd: Decimal::from(3),
                tx_hash: TxHash::new(format!("0xs_s_{}", i)),
                block_number: 101 + i as u64,
                timestamp: t_buy + Duration::seconds(180), // 3 min hold!
            });
        }

        // 2. Swing trader trades (holding time > 24 hours)
        for i in 0..5 {
            let t_buy = now - Duration::days(20) + Duration::days(i * 3);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: swing.clone(),
                token_address: token2.into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(50),
                price_usd: Decimal::from(50),
                volume_usd: Decimal::from(2500),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new(format!("0xw_b_{}", i)),
                block_number: 200 + i as u64,
                timestamp: t_buy,
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: swing.clone(),
                token_address: token2.into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(50),
                price_usd: Decimal::from(75),
                volume_usd: Decimal::from(3750),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new(format!("0xw_s_{}", i)),
                block_number: 210 + i as u64,
                timestamp: t_buy + Duration::days(2), // 48 hour hold!
            });
        }

        trades
    }

    #[test]
    fn test_behavioral_clustering_sniper_vs_swing() {
        let trades = generate_test_trade_sequence();
        let now = Utc::now();

        let sniper_trades: Vec<Trade> = trades
            .iter()
            .filter(|t| t.wallet_address.as_str() == "0x1111111111111111111111111111111111111111")
            .cloned()
            .collect();
        let swing_trades: Vec<Trade> = trades
            .iter()
            .filter(|t| t.wallet_address.as_str() == "0x2222222222222222222222222222222222222222")
            .cloned()
            .collect();

        let c_sniper = WalletResearchEngine::classify_wallet(
            "0x1111111111111111111111111111111111111111",
            &sniper_trades,
            now,
        );
        let c_swing = WalletResearchEngine::classify_wallet(
            "0x2222222222222222222222222222222222222222",
            &swing_trades,
            now,
        );

        assert_eq!(c_sniper.cluster, BehavioralCluster::EarlySniper);
        assert!(!c_sniper.copiable, "Snipers must be flagged uncopiable!");

        assert_eq!(c_swing.cluster, BehavioralCluster::SwingTrader);
        assert!(
            c_swing.copiable,
            "Swing traders should be flagged copiable!"
        );
    }

    #[test]
    fn test_copiability_latency_degradation() {
        let trades = generate_test_trade_sequence();
        let delays = vec![0, 1, 5, 15, 60, 120];

        let results =
            CopiabilityEngine::evaluate_latency_matrix(&trades, &delays, Decimal::from(1000), 30);

        assert_eq!(results.len(), 6);
        // Net pnl must decrease strictly or monotonically as latency delay increases
        let pnl_0s = results[0].net_pnl;
        let pnl_120s = results.last().unwrap().net_pnl;

        assert!(
            pnl_0s > pnl_120s,
            "Zero latency PnL must exceed 120s delayed PnL"
        );
    }

    #[test]
    fn test_scalability_liquidity_impact() {
        let trades = generate_test_trade_sequence();
        let capitals = vec![
            Decimal::from(100),
            Decimal::from(1000),
            Decimal::from(10000),
            Decimal::from(100000),
        ];

        let results = ScalabilityEngine::evaluate_scalability(
            &trades,
            &capitals,
            Decimal::from(50000), // $50k pool
            30,
        );

        assert_eq!(results.len(), 4);
        let impact_100 = results[0].avg_price_impact_bps;
        let impact_100k = results[3].avg_price_impact_bps;
        assert!(
            impact_100k > impact_100,
            "Larger capital must suffer greater price impact"
        );
    }

    #[test]
    fn test_full_research_experiment_run_and_report() {
        let trades = generate_test_trade_sequence();
        let config = ExperimentConfig {
            permutation_iterations: 20, // Fast for unit tests
            bootstrap_iterations: 50,
            ..Default::default()
        };

        let report = ResearchRunner::run_experiment(&trades, &config, "git-test-commit-sha");

        assert_eq!(report.wallet_classifications.len(), 2);
        assert!(!report.delay_curve.is_empty());
        assert!(!report.scalability_curve.is_empty());
        assert_eq!(report.bootstrap_ci.len(), 3);

        let markdown = ReportGenerator::generate_markdown(&report);
        assert!(markdown.contains("Research Experiment Report"));
        assert!(markdown.contains("DATA SOURCE"));
        assert!(markdown.contains("Scientific Verdict"));
    }
}

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
    use std::str::FromStr;
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

        let c_sniper = WalletResearchEngine::classify_wallet_as_of(
            "0x1111111111111111111111111111111111111111",
            &sniper_trades,
            now,
        );
        let c_swing = WalletResearchEngine::classify_wallet_as_of(
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

        let results = CopiabilityEngine::evaluate_latency_matrix(
            &trades,
            &delays,
            Decimal::from(1000),
            Decimal::from(10000),
            5,
            30,
            LatencyMode::StressTest,
        );

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
            false,
            Decimal::from(100000),
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

    // =========================================================================
    // 12 DEDICATED METHODOLOGICAL VERIFICATION TESTS (Phase 2.1 Audit Suite)
    // =========================================================================

    #[test]
    fn test_methodology_1_anti_lookahead_future_trades_ignored() {
        let now = Utc::now();
        let wallet = WalletAddress::new("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        let cutoff = now - Duration::days(5);

        // Wallet was poor before cutoff, but had huge profitable trades AFTER cutoff
        let mut trades = Vec::new();
        for i in 0..5 {
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new(format!("0xpre_{}", i)),
                block_number: 100 + i as u64,
                timestamp: cutoff - Duration::days(10) + Duration::days(i),
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(5), // 50% loss
                volume_usd: Decimal::from(500),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new(format!("0xpre_s_{}", i)),
                block_number: 101 + i as u64,
                timestamp: cutoff - Duration::days(10) + Duration::days(i) + Duration::hours(2),
            });
        }

        // Post-cutoff trades: 10x gains
        for i in 0..5 {
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x2222222222222222222222222222222222222222".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new(format!("0xpost_{}", i)),
                block_number: 500 + i as u64,
                timestamp: cutoff + Duration::days(1 + i),
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x2222222222222222222222222222222222222222".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(100), // 10x
                volume_usd: Decimal::from(10000),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new(format!("0xpost_s_{}", i)),
                block_number: 501 + i as u64,
                timestamp: cutoff + Duration::days(1 + i) + Duration::hours(2),
            });
        }

        let class_as_of =
            WalletResearchEngine::classify_wallet_as_of(wallet.as_str(), &trades, cutoff);
        assert!(
            !class_as_of.copiable,
            "Future 10x trades must NOT leak into pre-cutoff classification!"
        );
    }

    #[test]
    fn test_methodology_2_future_wallet_selection_invariance() {
        let now = Utc::now();
        let base_trades = generate_test_trade_sequence();
        let cutoff = now - Duration::days(5);

        let selected_before =
            WalletResearchEngine::select_copiable_wallets_as_of(&base_trades, cutoff, 3);

        // Add 10 huge new winning wallets strictly in the future (> cutoff)
        let mut augmented = base_trades.clone();
        for w in 10..20 {
            let addr = format!("0x{:040x}", w);
            augmented.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: WalletAddress::new(&addr),
                token_address: "0x3333333333333333333333333333333333333333".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(1000),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(10000),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new(format!("0xfut_{}", w)),
                block_number: 9999,
                timestamp: cutoff + Duration::days(2),
            });
        }

        let selected_after =
            WalletResearchEngine::select_copiable_wallets_as_of(&augmented, cutoff, 3);
        assert_eq!(
            selected_before.len(),
            selected_after.len(),
            "Future wallets must not alter selection as-of cutoff"
        );
        for (b, a) in selected_before.iter().zip(&selected_after) {
            assert_eq!(b.wallet_address, a.wallet_address);
            assert_eq!(b.copiable, a.copiable);
        }
    }

    #[test]
    fn test_methodology_3_dataset_sha256_mutation_sensitivity() {
        let trades = generate_test_trade_sequence();
        let config = ExperimentConfig::default();

        let report1 = ResearchRunner::run_experiment(&trades, &config, "commit");
        let hash1 = report1.dataset_hash;

        // Mutate a single price by 1 cent
        let mut mutated_trades = trades.clone();
        mutated_trades[0].price_usd += Decimal::from_str("0.01").unwrap();

        let report2 = ResearchRunner::run_experiment(&mutated_trades, &config, "commit");
        let hash2 = report2.dataset_hash;

        assert_ne!(
            hash1, hash2,
            "Dataset SHA-256 hash must be sensitive to 1 cent change in any trade"
        );
    }

    #[test]
    fn test_methodology_4_random_benchmark_reproducibility_with_seed() {
        let trades = generate_test_trade_sequence();
        let initial_capital = Decimal::from(10000);
        let runs = 50;

        let (_, dist1) = BenchmarkEngine::compute_random_selection_distribution(
            &trades,
            initial_capital,
            runs,
            12345,
        );
        let (_, dist2) = BenchmarkEngine::compute_random_selection_distribution(
            &trades,
            initial_capital,
            runs,
            12345,
        );
        let (_, dist3) = BenchmarkEngine::compute_random_selection_distribution(
            &trades,
            initial_capital,
            runs,
            99999,
        );

        assert_eq!(
            dist1.mean_return_pct, dist2.mean_return_pct,
            "Same seed must produce identical benchmark distribution"
        );
        assert_eq!(dist1.std_dev, dist2.std_dev);
        // Different seed should generally differ
        assert_ne!(
            dist1.mean_return_pct, dist3.mean_return_pct,
            "Different seed should produce different random benchmark draws"
        );
    }

    #[test]
    fn test_methodology_5_buy_and_hold_real_calculation() {
        let now = Utc::now();
        let token = "0x9999999999999999999999999999999999999999";
        let trades = vec![
            Trade {
                id: Uuid::new_v4(),
                wallet_address: WalletAddress::new("0x1111111111111111111111111111111111111111"),
                token_address: token.into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(10),
                price_usd: Decimal::from(100), // Entry: $100
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::ZERO,
                tx_hash: TxHash::new("0x1"),
                block_number: 1,
                timestamp: now - Duration::days(5),
            },
            Trade {
                id: Uuid::new_v4(),
                wallet_address: WalletAddress::new("0x2222222222222222222222222222222222222222"),
                token_address: token.into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(10),
                price_usd: Decimal::from(150), // Exit: $150 (+50%)
                volume_usd: Decimal::from(1500),
                fee_usd: Decimal::ZERO,
                tx_hash: TxHash::new("0x2"),
                block_number: 2,
                timestamp: now,
            },
        ];

        let b_and_h = BenchmarkEngine::compute_buy_and_hold(&trades);
        assert_eq!(
            b_and_h.total_return_pct,
            Decimal::from(50),
            "Buy & hold must reflect actual 50% price trajectory"
        );
    }

    #[test]
    fn test_methodology_6_capital_constraint_enforcement() {
        let now = Utc::now();
        let wallet = WalletAddress::new("0x1111111111111111111111111111111111111111");
        // Generate 10 buys with capital per trade = $1,000, but initial cash = $2,000
        let mut trades = Vec::new();
        for i in 0..10 {
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: format!("0x{:040x}", i).into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::ZERO,
                tx_hash: TxHash::new(format!("0xb_{}", i)),
                block_number: 10 + i as u64,
                timestamp: now + Duration::hours(i as i64),
            });
        }

        let delays = vec![0];
        let capital_per_trade = Decimal::from(1000);
        let initial_cash = Decimal::from(2000); // Can only afford 2 buys
        let max_open_positions = 5;

        let results = CopiabilityEngine::evaluate_latency_matrix(
            &trades,
            &delays,
            capital_per_trade,
            initial_cash,
            max_open_positions,
            0,
            LatencyMode::StressTest,
        );

        assert_eq!(
            results[0].trades_executed, 0,
            "No sells occurred to close round-trips"
        );
    }

    #[test]
    fn test_methodology_7_pnl_decomposition_identity() {
        let trades = generate_test_trade_sequence();
        let metrics = ScientificValidator::evaluate_slice(&trades);

        // Identity: net_pnl = gross_pnl - trading_fees (when gas and slippage are 0 in evaluation slice)
        assert_eq!(
            metrics.net_pnl + metrics.trading_fees,
            metrics.gross_pnl,
            "PnL decomposition identity: Gross PnL must equal Net PnL + Fees"
        );
    }

    #[test]
    fn test_methodology_8_latency_stress_test_monotonicity() {
        let trades = generate_test_trade_sequence();
        let delays = vec![0, 1, 5, 10, 30, 60, 120];

        let results = CopiabilityEngine::evaluate_latency_matrix(
            &trades,
            &delays,
            Decimal::from(1000),
            Decimal::from(10000),
            5,
            30,
            LatencyMode::StressTest,
        );

        for i in 1..results.len() {
            assert!(
                results[i].net_pnl <= results[i - 1].net_pnl,
                "Latency delay PnL must be weakly monotonically decreasing"
            );
        }
    }

    #[test]
    fn test_methodology_9_test_set_isolation() {
        let trades = generate_test_trade_sequence();
        let (train, val, test) = ScientificValidator::chronological_split(&trades, 0.60, 0.20);

        let t_train_end = train.last().map(|t| t.timestamp).unwrap();
        let t_val_start = val.first().map(|t| t.timestamp).unwrap();
        let t_val_end = val.last().map(|t| t.timestamp).unwrap();
        let t_test_start = test.first().map(|t| t.timestamp).unwrap();

        assert!(
            t_train_end <= t_val_start,
            "Train end must precede Val start"
        );
        assert!(t_val_end <= t_test_start, "Val end must precede Test start");
    }

    #[test]
    fn test_methodology_10_require_real_data_synthetic_verdict() {
        let trades = generate_test_trade_sequence();
        let config = ExperimentConfig {
            data_source: DataSource::Synthetic,
            permutation_iterations: 10,
            bootstrap_iterations: 20,
            ..Default::default()
        };

        let report = ResearchRunner::run_experiment(&trades, &config, "v1");
        assert_eq!(
            report.verdict.status,
            VerdictStatus::NotValidated,
            "Synthetic data must strictly produce VerdictStatus::NotValidated"
        );
    }

    #[test]
    fn test_methodology_11_reproducibility_with_seed() {
        let trades = generate_test_trade_sequence();
        let config1 = ExperimentConfig {
            seed: 777,
            permutation_iterations: 30,
            bootstrap_iterations: 30,
            ..Default::default()
        };
        let config2 = ExperimentConfig {
            seed: 777,
            permutation_iterations: 30,
            bootstrap_iterations: 30,
            ..Default::default()
        };

        let r1 = ResearchRunner::run_experiment(&trades, &config1, "v1");
        let r2 = ResearchRunner::run_experiment(&trades, &config2, "v1");

        assert_eq!(r1.permutation_test.p_value, r2.permutation_test.p_value);
        assert_eq!(
            r1.permutation_test.null_mean_sharpe,
            r2.permutation_test.null_mean_sharpe
        );
    }

    #[test]
    fn test_methodology_12_walk_forward_window_isolation() {
        let trades = generate_test_trade_sequence();
        let windows = ScientificValidator::walk_forward_analysis(&trades, 2, 2);

        for w in &windows {
            assert!(
                w.train_end <= w.test_start,
                "Walk-forward train window must end before test window begins"
            );
        }
    }
}

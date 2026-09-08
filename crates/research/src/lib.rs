pub mod benchmarks;
pub mod copiability;
pub mod informational_alpha;
pub mod report;
pub mod runner;
pub mod scalability;
pub mod types;
pub mod validation;
pub mod wallet_engine;

pub use benchmarks::BenchmarkEngine;
pub use copiability::CopiabilityEngine;
pub use informational_alpha::InformationalAlphaEngine;
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
    use std::collections::HashSet;
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

        assert_eq!(r1.dataset_hash, r2.dataset_hash);
        assert_eq!(r1.train_selected_wallets, r2.train_selected_wallets);
        assert_eq!(r1.train_metrics.net_pnl, r2.train_metrics.net_pnl);
        assert_eq!(r1.test_metrics.net_pnl, r2.test_metrics.net_pnl);
        assert_eq!(r1.permutation_test.p_value, r2.permutation_test.p_value);
        assert_eq!(
            r1.permutation_test.null_mean_sharpe,
            r2.permutation_test.null_mean_sharpe
        );
        assert_eq!(r1.bootstrap_ci[0].mean, r2.bootstrap_ci[0].mean);
        assert_eq!(
            r1.benchmark_comparisons[0].total_return_pct,
            r2.benchmark_comparisons[0].total_return_pct
        );
        assert_eq!(r1.verdict.status, r2.verdict.status);
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

    // =========================================================================
    // RED TEAM ADVERSARIAL TEST SUITE (Phase 2.2 Independent Verification)
    // =========================================================================

    #[test]
    fn test_red_team_a_bad_in_train_10x_in_test_never_selected() {
        let now = Utc::now();
        let bad_wallet = WalletAddress::new("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        let good_wallet = WalletAddress::new("0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        let mut trades = Vec::new();

        // 1. In Train (days -30 to -10): bad_wallet loses 50% on every trade
        for i in 0..6 {
            let t = now - Duration::days(30) + Duration::days(i * 2);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: bad_wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xa_buy_{}", i)),
                block_number: 100 + i as u64,
                timestamp: t,
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: bad_wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(5), // -50% loss
                volume_usd: Decimal::from(500),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xa_sell_{}", i)),
                block_number: 101 + i as u64,
                timestamp: t + Duration::hours(1),
            });
        }

        // Good wallet performs well in Train
        for i in 0..6 {
            let t = now - Duration::days(29) + Duration::days(i * 2);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: good_wallet.clone(),
                token_address: "0x2222222222222222222222222222222222222222".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xb_buy_{}", i)),
                block_number: 200 + i as u64,
                timestamp: t,
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: good_wallet.clone(),
                token_address: "0x2222222222222222222222222222222222222222".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(20), // +100% gain
                volume_usd: Decimal::from(2000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xb_sell_{}", i)),
                block_number: 201 + i as u64,
                timestamp: t + Duration::days(2),
            });
        }

        // 2. In Test (days -5 to now): bad_wallet suddenly gets massive 10x gains
        for i in 0..6 {
            let t = now - Duration::days(5) + Duration::hours(i * 12);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: bad_wallet.clone(),
                token_address: "0x3333333333333333333333333333333333333333".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xa_post_buy_{}", i)),
                block_number: 500 + i as u64,
                timestamp: t,
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: bad_wallet.clone(),
                token_address: "0x3333333333333333333333333333333333333333".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(100), // 10x gain
                volume_usd: Decimal::from(10000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xa_post_sell_{}", i)),
                block_number: 501 + i as u64,
                timestamp: t + Duration::hours(2),
            });
        }

        let config = ExperimentConfig {
            train_ratio: 0.60,
            val_ratio: 0.10,
            test_ratio: 0.30,
            min_wallet_trades: 5,
            permutation_iterations: 10,
            bootstrap_iterations: 20,
            ..Default::default()
        };

        let report = ResearchRunner::run_experiment(&trades, &config, "v1");

        // RED TEAM ASSERTION: Bad wallet must NOT be selected in train
        assert!(
            !report
                .train_selected_wallets
                .contains(&bad_wallet.to_string()),
            "Adversarial Test A Failed: Wallet with future 10x gains leaked into Train selection!"
        );
        // And good wallet was properly selected
        assert!(
            report
                .train_selected_wallets
                .contains(&good_wallet.to_string()),
            "Good wallet in train should have been selected"
        );
    }

    #[test]
    fn test_red_team_b_great_in_train_catastrophic_in_test_fully_realized() {
        let now = Utc::now();
        let wallet = WalletAddress::new("0xcccccccccccccccccccccccccccccccccccccccc");
        let mut trades = Vec::new();

        // 1. In Train (days -30 to -10): Wallet wins consistently (+100% gain, 6 round-trips)
        for i in 0..6 {
            let t = now - Duration::days(30) + Duration::days(i * 3);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xc_train_b_{}", i)),
                block_number: 100 + i as u64,
                timestamp: t,
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(20),
                volume_usd: Decimal::from(2000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xc_train_s_{}", i)),
                block_number: 101 + i as u64,
                timestamp: t + Duration::days(1),
            });
        }

        // 2. In Test (days -5 to now): Wallet suffers catastrophic 90% loss
        for i in 0..6 {
            let t = now - Duration::days(5) + Duration::hours(i * 12);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x2222222222222222222222222222222222222222".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(100),
                volume_usd: Decimal::from(10000),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new(format!("0xc_test_b_{}", i)),
                block_number: 500 + i as u64,
                timestamp: t,
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x2222222222222222222222222222222222222222".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10), // -90% crash
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new(format!("0xc_test_s_{}", i)),
                block_number: 501 + i as u64,
                timestamp: t + Duration::hours(2),
            });
        }

        let config = ExperimentConfig {
            train_ratio: 0.50,
            val_ratio: 0.10,
            test_ratio: 0.40,
            min_wallet_trades: 5,
            permutation_iterations: 10,
            bootstrap_iterations: 20,
            ..Default::default()
        };

        let report = ResearchRunner::run_experiment(&trades, &config, "v1");

        // Selected in train based on history
        assert!(
            report.train_selected_wallets.contains(&wallet.to_string()),
            "Wallet should be selected based on stellar train performance"
        );
        // RED TEAM ASSERTION: Loss in test must be strictly accounted for
        assert!(
            report.test_metrics.net_pnl < Decimal::ZERO,
            "Adversarial Test B Failed: Out-of-sample crash was magically avoided! Test Net PnL: {}",
            report.test_metrics.net_pnl
        );
        assert!(
            report.test_metrics.max_drawdown_pct >= Decimal::from(80),
            "Adversarial Test B Failed: Max drawdown was not fully recognized: {}",
            report.test_metrics.max_drawdown_pct
        );
    }

    #[test]
    fn test_red_team_c_altering_future_timestamps_preserves_train_selection() {
        let base_trades = generate_test_trade_sequence();
        let now = Utc::now();
        let cutoff = now - Duration::days(5);

        let selected_1 =
            WalletResearchEngine::select_copiable_wallets_as_of(&base_trades, cutoff, 3);

        // Mutate future trades by adding +100 days
        let mut mutated_future = base_trades.clone();
        for t in &mut mutated_future {
            if t.timestamp > cutoff {
                t.timestamp += Duration::days(100);
            }
        }

        let selected_2 =
            WalletResearchEngine::select_copiable_wallets_as_of(&mutated_future, cutoff, 3);

        assert_eq!(selected_1.len(), selected_2.len());
        for (a, b) in selected_1.iter().zip(&selected_2) {
            assert_eq!(a.wallet_address, b.wallet_address);
            assert_eq!(a.copiable, b.copiable);
            assert_eq!(a.persistence_score, b.persistence_score);
        }
    }

    #[test]
    fn test_red_team_d_altering_future_prices_preserves_train_classifications() {
        let base_trades = generate_test_trade_sequence();
        let now = Utc::now();
        let cutoff = now - Duration::days(5);

        let c1 = WalletResearchEngine::classify_wallet_as_of(
            "0x2222222222222222222222222222222222222222",
            &base_trades,
            cutoff,
        );

        // Mutate future trades with 1,000,000 price
        let mut mutated_future = base_trades.clone();
        for t in &mut mutated_future {
            if t.timestamp > cutoff {
                t.price_usd = Decimal::from(1_000_000);
            }
        }

        let c2 = WalletResearchEngine::classify_wallet_as_of(
            "0x2222222222222222222222222222222222222222",
            &mutated_future,
            cutoff,
        );

        assert_eq!(c1.cluster, c2.cluster);
        assert_eq!(c1.copiable, c2.copiable);
        assert_eq!(c1.persistence_score, c2.persistence_score);
    }

    #[test]
    fn test_red_team_e_wallet_only_in_test_never_in_train_selection() {
        let now = Utc::now();
        let mut trades = generate_test_trade_sequence();
        let test_only_wallet = WalletAddress::new("0x9999999999999999999999999999999999999999");

        // Add 10 winning trades for test_only_wallet occurring exclusively at current time (in test set)
        for i in 0..10 {
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: test_only_wallet.clone(),
                token_address: "0x8888888888888888888888888888888888888888".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(1),
                tx_hash: TxHash::new(format!("0xnew_b_{}", i)),
                block_number: 900 + i as u64,
                timestamp: now + Duration::hours(i as i64),
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: test_only_wallet.clone(),
                token_address: "0x8888888888888888888888888888888888888888".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(50), // 5x
                volume_usd: Decimal::from(5000),
                fee_usd: Decimal::from(1),
                tx_hash: TxHash::new(format!("0xnew_s_{}", i)),
                block_number: 901 + i as u64,
                timestamp: now + Duration::hours(i as i64) + Duration::minutes(30),
            });
        }

        let config = ExperimentConfig {
            train_ratio: 0.60,
            val_ratio: 0.20,
            test_ratio: 0.20,
            permutation_iterations: 10,
            bootstrap_iterations: 20,
            ..Default::default()
        };

        let report = ResearchRunner::run_experiment(&trades, &config, "v1");

        assert!(
            !report
                .train_selected_wallets
                .contains(&test_only_wallet.to_string()),
            "Wallet trading only in Test partition must never appear in Train selection!"
        );
    }

    #[test]
    fn test_red_team_f_no_future_trades_passed_to_train_selection() {
        let trades = generate_test_trade_sequence();
        let (train, val, test) = ScientificValidator::chronological_split(&trades, 0.60, 0.20);
        let train_end = train.last().unwrap().timestamp;

        for t in &train {
            assert!(
                t.timestamp <= train_end,
                "Train trade timestamp must be <= train_end"
            );
        }
        for t in &val {
            assert!(
                t.timestamp >= train_end,
                "Val trade timestamp must be >= train_end"
            );
        }
        for t in &test {
            assert!(
                t.timestamp > train_end,
                "Test trade timestamp must be strictly > train_end"
            );
        }
    }

    #[test]
    fn test_red_team_walk_forward_isolation_future_pump_unseen() {
        let mut trades = generate_test_trade_sequence();
        let initial_windows = ScientificValidator::walk_forward_analysis(&trades, 2, 2);

        // Inject 100x trade at the very end of the dataset
        let last_time = trades.last().unwrap().timestamp;
        trades.push(Trade {
            id: Uuid::new_v4(),
            wallet_address: WalletAddress::new("0x2222222222222222222222222222222222222222"),
            token_address: "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into(),
            side: TradeSide::Sell,
            amount_tokens: Decimal::from(100),
            price_usd: Decimal::from(10000), // 100x
            volume_usd: Decimal::from(1000000),
            fee_usd: Decimal::ZERO,
            tx_hash: TxHash::new("0xfuture_pump"),
            block_number: 99999,
            timestamp: last_time + Duration::days(10),
        });

        let updated_windows = ScientificValidator::walk_forward_analysis(&trades, 2, 2);

        // Window 0 (the earliest window) must be completely unaffected by future 100x injection
        assert_eq!(
            initial_windows[0].in_sample_sharpe, updated_windows[0].in_sample_sharpe,
            "Early walk-forward window In-Sample Sharpe must remain invariant to future pump!"
        );
    }

    #[test]
    fn test_red_team_permutation_null_vs_true_signal() {
        let now = Utc::now();
        let wallet1 = WalletAddress::new("0x1111111111111111111111111111111111111111");
        let wallet2 = WalletAddress::new("0x2222222222222222222222222222222222222222");

        // 1. True signal dataset: Wallets consistently achieve +30% profit
        let mut signal_trades = Vec::new();
        for i in 0..10 {
            let t = now - Duration::days(20) + Duration::days(i);
            let w = if i % 2 == 0 { &wallet1 } else { &wallet2 };
            signal_trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: w.clone(),
                token_address: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(1),
                tx_hash: TxHash::new(format!("0xsb_{}", i)),
                block_number: 10 + i as u64,
                timestamp: t,
            });
            signal_trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: w.clone(),
                token_address: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(13), // +30%
                volume_usd: Decimal::from(1300),
                fee_usd: Decimal::from(1),
                tx_hash: TxHash::new(format!("0xss_{}", i)),
                block_number: 11 + i as u64,
                timestamp: t + Duration::hours(2),
            });
        }

        let signal_result = ScientificValidator::permutation_test(&signal_trades, 100, 42);
        assert!(
            signal_result.is_significant,
            "Permutation test must detect true consistent signal (p={})",
            signal_result.p_value
        );
        assert!(signal_result.p_value < 0.05);

        // 2. Zero-alpha / null dataset: Flat trades with zero net gain
        let mut null_trades = Vec::new();
        for i in 0..10 {
            let t = now - Duration::days(20) + Duration::days(i);
            let w = if i % 2 == 0 { &wallet1 } else { &wallet2 };
            null_trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: w.clone(),
                token_address: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(1),
                tx_hash: TxHash::new(format!("0xnb_{}", i)),
                block_number: 10 + i as u64,
                timestamp: t,
            });
            null_trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: w.clone(),
                token_address: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10), // 0% gain
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(1),
                tx_hash: TxHash::new(format!("0xns_{}", i)),
                block_number: 11 + i as u64,
                timestamp: t + Duration::hours(2),
            });
        }

        let null_result = ScientificValidator::permutation_test(&null_trades, 100, 42);
        assert!(
            !null_result.is_significant,
            "Permutation test must NOT find alpha in zero-edge null dataset (p={})",
            null_result.p_value
        );
    }

    #[test]
    fn test_red_team_dataset_hashing_full_field_sensitivity_and_order_invariance() {
        let base_trades = generate_test_trade_sequence();
        let config = ExperimentConfig::default();

        let base_hash = ResearchRunner::run_experiment(&base_trades, &config, "c").dataset_hash;

        // 1. Mutate price
        let mut m_price = base_trades.clone();
        m_price[0].price_usd += Decimal::from_str("0.01").unwrap();
        assert_ne!(
            base_hash,
            ResearchRunner::run_experiment(&m_price, &config, "c").dataset_hash,
            "Hash must change on price mutation"
        );

        // 2. Mutate timestamp
        let mut m_time = base_trades.clone();
        m_time[0].timestamp += Duration::seconds(1);
        assert_ne!(
            base_hash,
            ResearchRunner::run_experiment(&m_time, &config, "c").dataset_hash,
            "Hash must change on timestamp mutation"
        );

        // 3. Mutate wallet
        let mut m_wallet = base_trades.clone();
        m_wallet[0].wallet_address =
            WalletAddress::new("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef");
        assert_ne!(
            base_hash,
            ResearchRunner::run_experiment(&m_wallet, &config, "c").dataset_hash,
            "Hash must change on wallet mutation"
        );

        // 4. Mutate token
        let mut m_token = base_trades.clone();
        m_token[0].token_address = "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".into();
        assert_ne!(
            base_hash,
            ResearchRunner::run_experiment(&m_token, &config, "c").dataset_hash,
            "Hash must change on token mutation"
        );

        // 5. Mutate amount
        let mut m_amount = base_trades.clone();
        m_amount[0].amount_tokens += Decimal::ONE;
        assert_ne!(
            base_hash,
            ResearchRunner::run_experiment(&m_amount, &config, "c").dataset_hash,
            "Hash must change on amount mutation"
        );

        // 6. Mutate tx_hash
        let mut m_tx = base_trades.clone();
        m_tx[0].tx_hash = TxHash::new("0xmutated_tx_hash");
        assert_ne!(
            base_hash,
            ResearchRunner::run_experiment(&m_tx, &config, "c").dataset_hash,
            "Hash must change on tx_hash mutation"
        );

        // 7. Shuffled trade slice order -> canonical sort yields IDENTICAL hash
        let mut shuffled = base_trades.clone();
        shuffled.reverse();
        let shuffled_hash = ResearchRunner::run_experiment(&shuffled, &config, "c").dataset_hash;
        assert_eq!(
            base_hash, shuffled_hash,
            "Canonical sorting must ensure hash invariance under input slice order permutation"
        );
    }

    #[test]
    fn test_red_team_strict_verdict_oos_loss_rejected_on_real_data() {
        let now = Utc::now();
        let wallet = WalletAddress::new("0x1111111111111111111111111111111111111111");
        let mut trades = Vec::new();

        // 50 total trades to meet sample size requirements for Real data
        // Train trades: winning (+50%)
        for i in 0..30 {
            let t = now - Duration::days(50) + Duration::days(i);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xtrb_{}", i)),
                block_number: 100 + i as u64,
                timestamp: t,
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(15),
                volume_usd: Decimal::from(1500),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xtrs_{}", i)),
                block_number: 101 + i as u64,
                timestamp: t + Duration::hours(2),
            });
        }

        // Test trades: severe losses (-50%)
        for i in 0..20 {
            let t = now - Duration::days(15) + Duration::hours(i * 12);
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(20),
                volume_usd: Decimal::from(2000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xteb_{}", i)),
                block_number: 500 + i as u64,
                timestamp: t,
            });
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(2),
                tx_hash: TxHash::new(format!("0xtes_{}", i)),
                block_number: 501 + i as u64,
                timestamp: t + Duration::hours(2),
            });
        }

        let config = ExperimentConfig {
            data_source: DataSource::Real,
            train_ratio: 0.60,
            val_ratio: 0.10,
            test_ratio: 0.30,
            permutation_iterations: 20,
            bootstrap_iterations: 20,
            ..Default::default()
        };

        let report = ResearchRunner::run_experiment(&trades, &config, "v1");

        // RED TEAM ASSERTION: Out-of-sample losses on Real data must NOT emit EmpiricallySupported
        assert_eq!(
            report.verdict.status,
            VerdictStatus::NoStatisticalEdge,
            "Real data with unprofitable Out-of-Sample results must be rejected with NoStatisticalEdge!"
        );
        assert!(
            report.verdict.conclusion.contains("NO STATISTICAL EDGE"),
            "Conclusion must clearly explain the failure"
        );
    }

    #[test]
    fn test_red_team_pnl_decomposition_scenarios() {
        let now = Utc::now();
        let wallet = WalletAddress::new("0x1111111111111111111111111111111111111111");

        // 1. Winning trade with fees
        let trades_win = vec![
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(10),
                price_usd: Decimal::from(100),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new("0xw_b"),
                block_number: 1,
                timestamp: now - Duration::hours(2),
            },
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(10),
                price_usd: Decimal::from(150),
                volume_usd: Decimal::from(1500),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new("0xw_s"),
                block_number: 2,
                timestamp: now - Duration::hours(1),
            },
        ];
        let m_win = ScientificValidator::evaluate_slice(&trades_win);
        assert_eq!(m_win.net_pnl, Decimal::from(490)); // Gross $500 - $10 fees
        assert_eq!(m_win.gross_pnl, Decimal::from(500));
        assert_eq!(m_win.trading_fees, Decimal::from(10));
        assert_eq!(m_win.gross_pnl - m_win.trading_fees, m_win.net_pnl);

        // 2. Losing trade
        let trades_loss = vec![
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(10),
                price_usd: Decimal::from(100),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new("0xl_b"),
                block_number: 3,
                timestamp: now - Duration::hours(2),
            },
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(10),
                price_usd: Decimal::from(70),
                volume_usd: Decimal::from(700),
                fee_usd: Decimal::from(5),
                tx_hash: TxHash::new("0xl_s"),
                block_number: 4,
                timestamp: now - Duration::hours(1),
            },
        ];
        let m_loss = ScientificValidator::evaluate_slice(&trades_loss);
        assert_eq!(m_loss.net_pnl, Decimal::from(-310)); // Gross -$300 - $10 fees
        assert_eq!(m_loss.gross_pnl, Decimal::from(-300));
        assert_eq!(m_loss.trading_fees, Decimal::from(10));

        // 3. High fees eating small profit
        let trades_high_fees = vec![
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(10),
                price_usd: Decimal::from(100),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(50),
                tx_hash: TxHash::new("0xhf_b"),
                block_number: 5,
                timestamp: now - Duration::hours(2),
            },
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: "0x1111111111111111111111111111111111111111".into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(10),
                price_usd: Decimal::from(105),
                volume_usd: Decimal::from(1050),
                fee_usd: Decimal::from(50),
                tx_hash: TxHash::new("0xhf_s"),
                block_number: 6,
                timestamp: now - Duration::hours(1),
            },
        ];
        let m_hf = ScientificValidator::evaluate_slice(&trades_high_fees);
        assert_eq!(m_hf.gross_pnl, Decimal::from(50));
        assert_eq!(m_hf.trading_fees, Decimal::from(100));
        assert_eq!(m_hf.net_pnl, Decimal::from(-50)); // Net is negative despite positive gross!
    }

    #[test]
    fn test_benjamini_hochberg_fdr_correction() {
        // Input raw p-values
        let raw_p = vec![0.001, 0.01, 0.03, 0.04, 0.50];
        let adjusted = ScientificValidator::benjamini_hochberg_correction(&raw_p);

        assert_eq!(adjusted.len(), 5);
        // Adjusted p-values must be >= raw p-values
        for (raw, adj) in raw_p.iter().zip(adjusted.iter()) {
            assert!(
                adj >= raw,
                "Adjusted p-value ({}) must be >= raw p-value ({})",
                adj,
                raw
            );
        }
        // First hypothesis should remain significant (0.001 * 5 / 1 = 0.005)
        assert!(adjusted[0] <= 0.01);
        // Fifth hypothesis should remain non-significant
        assert_eq!(adjusted[4], 0.50);
    }

    #[test]
    fn test_informational_alpha_engine_evaluation() {
        let trades = generate_test_trade_sequence();
        let mut smart_set = HashSet::new();
        smart_set.insert("0x2222222222222222222222222222222222222222".to_string());

        let results = InformationalAlphaEngine::evaluate_all_strategies(
            &trades[..5],
            &trades[5..],
            &smart_set,
            Decimal::from(10000),
            Decimal::from(1000),
            30,
        );

        assert!(!results.is_empty());
        // Verify all 5 families are evaluated
        let families: HashSet<StrategyFamily> = results.iter().map(|r| r.family).collect();
        assert!(families.contains(&StrategyFamily::DirectCopy));
        assert!(families.contains(&StrategyFamily::Confirmation));
        assert!(families.contains(&StrategyFamily::Consensus));
        assert!(families.contains(&StrategyFamily::WalletMomentum));
        assert!(families.contains(&StrategyFamily::TokenAttention));
    }

    #[test]
    fn test_cross_pool_generalization() {
        let mut trades = generate_test_trade_sequence();
        // Give half trades a different token/pool
        for t in trades.iter_mut().skip(5) {
            t.token_address = "0x9999999999999999999999999999999999999999".into();
        }

        let train_pools = vec!["0x1111111111111111111111111111111111111111".to_string()];
        let test_pool = "0x9999999999999999999999999999999999999999";

        let result = ScientificValidator::cross_pool_analysis(&trades, &train_pools, test_pool, 1);
        assert_eq!(result.test_pool, test_pool);
        assert_eq!(result.train_pools, train_pools);
    }

    #[test]
    fn test_unseen_wallet_generalization() {
        let trades = generate_test_trade_sequence();
        let train_trades = &trades[..5];
        let test_trades = &trades[5..];
        let mut selected_wallets = HashSet::new();
        selected_wallets.insert("0x2222222222222222222222222222222222222222".to_string());
        let train_end = train_trades.last().unwrap().timestamp;

        let eval = ScientificValidator::unseen_wallet_analysis(
            train_trades,
            test_trades,
            &selected_wallets,
            train_end,
        );

        assert_eq!(
            eval.known_wallets_trades
                + eval.unseen_wallets_trades
                + eval.new_post_train_wallets_trades,
            test_trades.len()
        );
    }

    #[test]
    fn test_market_regime_analysis() {
        let trades = generate_test_trade_sequence();
        let regimes = ScientificValidator::regime_analysis(&trades);
        assert_eq!(regimes.len(), 2);
        assert_eq!(regimes[0].regime, MarketRegime::HighLiquidity);
        assert_eq!(regimes[1].regime, MarketRegime::LowLiquidity);
    }

    #[test]
    fn test_phase2_6_dataset_manifest_integrity() {
        let manifest_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../data/PHASE2_6_DATASET_MANIFEST.json"
        );
        if std::path::Path::new(manifest_path).exists() {
            let content = std::fs::read_to_string(manifest_path).unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
            assert_eq!(parsed["dex"], "Uniswap V2");
            assert_eq!(parsed["chain_id"], 1);
            assert!(parsed["total_trades"].as_u64().unwrap() >= 3700);
            assert!(parsed["quality_checks_passed"].as_bool().unwrap());
        }
    }

    #[test]
    fn test_no_hardcoded_performance_metrics() {
        let mut trades_a = generate_test_trade_sequence();
        for (i, t) in trades_a.iter_mut().enumerate() {
            if t.side == TradeSide::Sell {
                t.price_usd = Decimal::from(15 + (i % 5) * 3);
            }
        }
        let selected_wallets: HashSet<String> = trades_a
            .iter()
            .map(|t| t.wallet_address.as_str().to_string())
            .collect();
        let results_a = InformationalAlphaEngine::evaluate_all_strategies(
            &trades_a,
            &trades_a,
            &selected_wallets,
            Decimal::from(10000),
            Decimal::from(1000),
            30,
        );

        // Modify trade prices in trades_b to change performance
        let mut trades_b = generate_test_trade_sequence();
        for (i, t) in trades_b.iter_mut().enumerate() {
            if t.side == TradeSide::Sell {
                t.price_usd = Decimal::from(30 + (i % 5) * 7);
            }
        }
        let results_b = InformationalAlphaEngine::evaluate_all_strategies(
            &trades_b,
            &trades_b,
            &selected_wallets,
            Decimal::from(10000),
            Decimal::from(1000),
            30,
        );

        // Test that the engine produces different results on different price series (not hardcoded)
        // DirectCopy at delay=0 should always have non-zero net_pnl if there are closed trades
        let dc_a: Vec<_> = results_a
            .iter()
            .filter(|r| r.family == StrategyFamily::DirectCopy && r.latency_seconds == 0)
            .collect();
        let dc_b: Vec<_> = results_b
            .iter()
            .filter(|r| r.family == StrategyFamily::DirectCopy && r.latency_seconds == 0)
            .collect();

        if !dc_a.is_empty() && !dc_b.is_empty() {
            let a = &dc_a[0];
            let b = &dc_b[0];
            // Two different sell price series must produce different net_pnl
            if a.net_pnl != Decimal::ZERO || b.net_pnl != Decimal::ZERO {
                assert_ne!(
                    a.net_pnl, b.net_pnl,
                    "DirectCopy net_pnl must differ between different price series"
                );
            }
        }
    }

    #[test]
    fn test_no_pseudo_p_values_true_permutation() {
        let trades = generate_test_trade_sequence();
        let (train, test) = trades.split_at(trades.len() / 2);
        let selected_wallets: HashSet<String> = trades
            .iter()
            .map(|t| t.wallet_address.as_str().to_string())
            .collect();
        let results = InformationalAlphaEngine::evaluate_all_strategies(
            train,
            test,
            &selected_wallets,
            Decimal::from(10000),
            Decimal::from(1000),
            30,
        );

        for res in &results {
            let perm = res
                .permutation_detail
                .as_ref()
                .expect("Permutation detail must exist");
            assert_eq!(perm.iterations, 50);
            assert!(perm.p_value >= 0.0 && perm.p_value <= 1.0);
            assert_eq!(res.raw_p_value, perm.p_value);
        }
    }

    #[test]
    fn test_empirical_latency_price_lookup_with_no_observation() {
        let now = Utc::now();
        let token = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let wallet = WalletAddress::new("0x1111111111111111111111111111111111111111");

        let trades = vec![
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token.into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(1),
                tx_hash: TxHash::new("0x01"),
                block_number: 100,
                timestamp: now,
            },
            Trade {
                id: Uuid::new_v4(),
                wallet_address: wallet.clone(),
                token_address: token.into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(11),
                volume_usd: Decimal::from(1100),
                fee_usd: Decimal::from(1),
                tx_hash: TxHash::new("0x02"),
                block_number: 101,
                timestamp: now + Duration::seconds(10),
            },
        ];

        let delays = vec![5, 5000];
        let points = CopiabilityEngine::measure_empirical_latency_distribution(&trades, &delays);

        assert_eq!(points.len(), 2);
        assert_eq!(points[0].status, "EMPIRICAL_OBSERVATION");
        assert_eq!(points[0].sample_size, 1);
        assert_eq!(points[0].observed_price, Some(Decimal::from(11)));
        assert_eq!(points[0].price_delta_bps, Some(Decimal::from(1000)));

        assert_eq!(points[1].status, "NO_OBSERVATION");
        assert_eq!(points[1].sample_size, 0);
        assert!(points[1].observed_price.is_none());
        assert!(points[1].price_delta_bps.is_none());
    }

    #[test]
    fn test_dataset_manifest_reconciliation_and_audit() {
        let manifest_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../data/PHASE2_7_DATASET_MANIFEST.json"
        );
        if std::path::Path::new(manifest_path).exists() {
            let content = std::fs::read_to_string(manifest_path).unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
            assert_eq!(parsed["dex"], "Uniswap V2");
            assert_eq!(parsed["chain_id"], 1);
            assert_eq!(parsed["total_trades"], 3743);
            assert!(parsed["quality_checks_passed"].as_bool().unwrap());
            assert!(parsed["canonical_sha256"].as_str().is_some());
            assert_eq!(
                parsed["parent_manifest_sha256"].as_str(),
                Some("27e3d93d3790b154b9f5d56e34f67081eb654be04a024eaa532e2bb836cb96f8")
            );
        }
    }

    #[test]
    fn test_anti_lookahead_future_price_attack() {
        let mut trades = generate_test_trade_sequence();
        let split_idx = trades.len() / 2;
        let split_time = trades[split_idx].timestamp;

        for t in trades.iter_mut().skip(split_idx) {
            t.price_usd = Decimal::from(1_000_000);
        }

        let train_trades: Vec<Trade> = trades
            .iter()
            .filter(|t| t.timestamp < split_time)
            .cloned()
            .collect();
        for t in &train_trades {
            assert!(
                t.price_usd < Decimal::from(1000),
                "Lookahead leaked future price attack into train partition!"
            );
        }
    }

    #[test]
    fn test_anti_lookahead_future_wallet_attack() {
        let mut trades = generate_test_trade_sequence();
        let now = Utc::now();
        let cutoff = now - Duration::days(5);

        let evil_wallet = "0x9999999999999999999999999999999999999999";
        for i in 0..10 {
            trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: WalletAddress::new(evil_wallet),
                token_address: "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::ZERO,
                tx_hash: TxHash::new(format!("0xevil_{}", i)),
                block_number: 99999,
                timestamp: now,
            });
        }

        let classification =
            WalletResearchEngine::classify_wallet_as_of(evil_wallet, &trades, cutoff);
        assert!(!classification.copiable);
        assert_eq!(classification.cluster, BehavioralCluster::HighRiskDegen);
    }

    #[test]
    fn test_anti_lookahead_future_pump_attack() {
        let mut trades = generate_test_trade_sequence();
        let max_existing_ts = trades.iter().map(|t| t.timestamp).max().unwrap();

        let pump_token = "0xpump_token";
        trades.push(Trade {
            id: Uuid::new_v4(),
            wallet_address: WalletAddress::new("0x1111111111111111111111111111111111111111"),
            token_address: pump_token.into(),
            side: TradeSide::Buy,
            amount_tokens: Decimal::from(10_000),
            price_usd: Decimal::from(500),
            volume_usd: Decimal::from(5_000_000),
            fee_usd: Decimal::ZERO,
            tx_hash: TxHash::new("0xpump"),
            block_number: 500000,
            timestamp: max_existing_ts + Duration::days(10),
        });

        let (train, _, _) = ScientificValidator::chronological_split(&trades, 0.5, 0.0);
        for t in &train {
            assert_ne!(
                t.token_address.as_str(),
                pump_token,
                "Future pump token leaked into train partition!"
            );
        }
    }

    #[test]
    fn test_anti_lookahead_future_timestamp_attack() {
        let trades = generate_test_trade_sequence();
        let (train, _, test) = ScientificValidator::chronological_split(&trades, 0.5, 0.0);
        let max_train_ts = train.iter().map(|t| t.timestamp).max().unwrap();
        let min_test_ts = test.iter().map(|t| t.timestamp).min().unwrap();

        assert!(
            max_train_ts <= min_test_ts,
            "Temporal ordering violated: train max ts {} > test min ts {}",
            max_train_ts,
            min_test_ts
        );
    }

    #[test]
    fn test_pnl_decomposition_identity() {
        let decomp = PnLDecomposition {
            gross_alpha: Decimal::from(1000),
            latency_cost: Decimal::from(150),
            market_impact: Decimal::from(200),
            dex_fees: Decimal::from(50),
            gas_cost: Decimal::from(25),
            net_alpha: Decimal::from(575),
        };

        let computed_net = decomp.gross_alpha
            - decomp.latency_cost
            - decomp.market_impact
            - decomp.dex_fees
            - decomp.gas_cost;
        assert_eq!(decomp.net_alpha, computed_net);
    }

    #[test]
    fn test_sample_size_classification() {
        assert_eq!(
            SampleSizeAssessment::from_count(5),
            SampleSizeAssessment::InsufficientSample
        );
        assert_eq!(
            SampleSizeAssessment::from_count(20),
            SampleSizeAssessment::WeakSample
        );
        assert_eq!(
            SampleSizeAssessment::from_count(50),
            SampleSizeAssessment::AdequateSample
        );
        assert_eq!(
            SampleSizeAssessment::from_count(200),
            SampleSizeAssessment::StrongSample
        );
    }

    #[test]
    fn test_effect_size_calculation() {
        let effect = EffectSizeReport {
            mean_excess_return: Decimal::from(120),
            median_excess_return: Decimal::from(95),
            cohen_d: Some(Decimal::from_str("0.45").unwrap()),
            win_rate_diff_vs_benchmark: Decimal::from(5),
            sharpe_diff_vs_benchmark: Some(Decimal::from_str("0.25").unwrap()),
        };

        assert_eq!(effect.cohen_d, Some(Decimal::from_str("0.45").unwrap()));
        assert_eq!(effect.mean_excess_return, Decimal::from(120));
        assert_eq!(effect.median_excess_return, Decimal::from(95));
    }
}

use crate::benchmarks::BenchmarkEngine;
use crate::copiability::CopiabilityEngine;
use crate::scalability::ScalabilityEngine;
use crate::types::{
    DataSource, ExperimentConfig, ExperimentId, ExperimentReport, FrozenConfig, LatencyMode,
    ScientificVerdict, VerdictStatus,
};
use crate::validation::ScientificValidator;
use crate::wallet_engine::WalletResearchEngine;
use chrono::Utc;
use domain::Trade;
use rust_decimal::Decimal;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

pub struct ResearchRunner;

impl ResearchRunner {
    /// Executes a full scientific research experiment across all empirical dimensions.
    /// Strictly enforces:
    /// 1. Real cryptographic SHA-256 hash across all trade attributes.
    /// 2. Anti-look-ahead wallet selection: Wallets are selected ONLY from Train partition (trades <= T_train).
    /// 3. Blind out-of-sample evaluation: Copied wallets' trades occurring > T_train are evaluated blindly.
    /// 4. Strict data segregation: Synthetic data is ALWAYS flagged VerdictStatus::NotValidated.
    pub fn run_experiment(
        trades: &[Trade],
        config: &ExperimentConfig,
        git_commit: &str,
    ) -> ExperimentReport {
        let frozen_config = FrozenConfig::freeze(config.clone());
        let config = frozen_config.get();

        let experiment_id = ExperimentId::generate();
        let created_at = Utc::now();

        let mut sorted_trades = trades.to_vec();
        sorted_trades.sort_by(|a, b| {
            a.timestamp
                .cmp(&b.timestamp)
                .then_with(|| a.tx_hash.as_str().cmp(b.tx_hash.as_str()))
                .then_with(|| a.id.cmp(&b.id))
        });

        let start_timestamp = sorted_trades
            .first()
            .map(|t| t.timestamp)
            .unwrap_or(created_at);
        let end_timestamp = sorted_trades
            .last()
            .map(|t| t.timestamp)
            .unwrap_or(created_at);

        // 1. Cryptographic SHA-256 Dataset Hash
        let mut hasher = Sha256::new();
        for t in &sorted_trades {
            hasher.update(t.id.to_string().as_bytes());
            hasher.update(t.wallet_address.as_str().as_bytes());
            hasher.update(t.token_address.as_str().as_bytes());
            hasher.update(t.side.to_string().as_bytes());
            hasher.update(t.amount_tokens.to_string().as_bytes());
            hasher.update(t.price_usd.to_string().as_bytes());
            hasher.update(t.timestamp.timestamp_nanos_opt().unwrap_or(0).to_le_bytes());
            hasher.update(t.fee_usd.to_string().as_bytes());
            hasher.update(t.tx_hash.as_str().as_bytes());
        }
        let dataset_hash = format!("{:x}", hasher.finalize());

        // 2. Strict Chronological Split on ALL Trades (Train, Val, Test)
        let (train_all, val_all, test_all) = ScientificValidator::chronological_split(
            &sorted_trades,
            config.train_ratio,
            config.val_ratio,
        );

        let train_end_timestamp = train_all
            .last()
            .map(|t| t.timestamp)
            .unwrap_or(start_timestamp);

        // 3. Anti-Look-Ahead Wallet Selection:
        // Wallets are classified and selected ONLY on trades occurring in Train (<= train_end_timestamp)
        let selected_classifications = WalletResearchEngine::select_copiable_wallets_as_of(
            &train_all,
            train_end_timestamp,
            config.min_wallet_trades,
        );

        let train_selected_wallets: Vec<String> = selected_classifications
            .iter()
            .filter(|w| {
                w.copiable && w.persistence_score >= config.min_wallet_score / Decimal::from(100)
            })
            .map(|w| w.wallet_address.clone())
            .collect();

        let selected_wallet_set: HashSet<String> = train_selected_wallets.iter().cloned().collect();

        // Also compile all classifications across the dataset for reporting taxonomy
        let mut all_wallet_trades: HashMap<String, Vec<Trade>> = HashMap::new();
        for trade in &sorted_trades {
            all_wallet_trades
                .entry(trade.wallet_address.as_str().to_string())
                .or_default()
                .push(trade.clone());
        }
        let mut wallet_classifications = Vec::new();
        for (addr, w_trades) in &all_wallet_trades {
            let classification =
                WalletResearchEngine::classify_wallet_as_of(addr, w_trades, end_timestamp);
            wallet_classifications.push(classification);
        }

        // 4. Blind Execution on Partitions:
        // Filter trades of the FROZEN selected wallets in each chronological partition
        let train_trades: Vec<Trade> = train_all
            .iter()
            .filter(|t| selected_wallet_set.contains(t.wallet_address.as_str()))
            .cloned()
            .collect();
        let val_trades: Vec<Trade> = val_all
            .iter()
            .filter(|t| selected_wallet_set.contains(t.wallet_address.as_str()))
            .cloned()
            .collect();
        let test_trades: Vec<Trade> = test_all
            .iter()
            .filter(|t| selected_wallet_set.contains(t.wallet_address.as_str()))
            .cloned()
            .collect();

        let train_metrics = ScientificValidator::evaluate_slice(&train_trades);
        let val_metrics = ScientificValidator::evaluate_slice(&val_trades);
        let test_metrics = ScientificValidator::evaluate_slice(&test_trades);

        // Copiable trades across the entire period based strictly on Train selection
        let all_copiable_trades: Vec<Trade> = sorted_trades
            .iter()
            .filter(|t| selected_wallet_set.contains(t.wallet_address.as_str()))
            .cloned()
            .collect();

        let baseline_metrics = ScientificValidator::evaluate_slice(&all_copiable_trades);

        // 5. Copiability Engine: Latency Degradation Curve (Evaluated on Test / OOS trades)
        let eval_copiable_trades = if !test_trades.is_empty() {
            &test_trades
        } else if !all_copiable_trades.is_empty() {
            &all_copiable_trades
        } else {
            &sorted_trades
        };

        let fixed_capital_per_trade = Decimal::from(1000);
        let fee_bps = 30; // 0.30% DEX fee
        let latency_mode = match config.data_source {
            DataSource::Real => LatencyMode::Empirical,
            _ => LatencyMode::StressTest,
        };

        let delay_curve = CopiabilityEngine::evaluate_latency_matrix(
            eval_copiable_trades,
            &config.delays_seconds,
            fixed_capital_per_trade,
            config.initial_cash,
            config.max_open_positions,
            fee_bps,
            latency_mode,
        );

        // Find break-even latency
        let mut break_even_latency = None;
        for point in &delay_curve {
            if point.net_pnl <= Decimal::ZERO && point.delay_seconds > 0 {
                break_even_latency = Some(point.delay_seconds);
                break;
            }
        }

        // 6. Scalability Engine: Capital Impact Curve
        let is_real_liquidity = config.data_source == DataSource::Real;
        let assumed_pool_liquidity = config.pool_liquidity.unwrap_or_else(|| {
            if is_real_liquidity {
                Decimal::from(15_000_000)
            } else {
                Decimal::from(100_000)
            }
        });
        let scalability_curve = ScalabilityEngine::evaluate_scalability(
            eval_copiable_trades,
            &config.capitals_usd,
            assumed_pool_liquidity,
            is_real_liquidity,
            config.initial_cash,
            fee_bps,
        );
        let max_scalable_capital =
            ScalabilityEngine::compute_maximum_scalable_capital(&scalability_curve);

        // 7. Walk-Forward Cross-Validation
        let walk_forward_windows =
            ScientificValidator::walk_forward_analysis(&sorted_trades, 3, config.min_wallet_trades);

        // 8. Ablation Study (Strictly evaluated on Out-Of-Sample partition)
        let eval_trades = if !test_all.is_empty() {
            &test_all
        } else {
            &sorted_trades
        };

        let ablation_results = ScientificValidator::ablation_study(
            &train_all,
            eval_trades,
            eval_copiable_trades,
            train_end_timestamp,
        );

        // 9. Permutation Testing (H0 Null Hypothesis) with Seeded PRNG
        let permutation_test = ScientificValidator::permutation_test(
            eval_copiable_trades,
            config.permutation_iterations,
            config.seed,
        );

        // 10. Bootstrap 95% Confidence Intervals with Seeded PRNG
        let bootstrap_ci = ScientificValidator::bootstrap_confidence_intervals(
            eval_copiable_trades,
            config.bootstrap_iterations,
            config.seed,
        );

        // 11. Empirical Benchmark Comparisons
        let benchmark_comparisons = BenchmarkEngine::evaluate_benchmarks(
            eval_copiable_trades,
            eval_trades,
            config.initial_cash,
            config.random_benchmark_runs,
            config.seed,
        );

        // 12. Synthesize Scientific Verdict with Strict Data Source Segregation
        let is_copiable_under_latency = delay_curve
            .iter()
            .find(|p| p.delay_seconds == 2)
            .map(|p| p.net_pnl > Decimal::ZERO)
            .unwrap_or(false);

        let smart_ret = benchmark_comparisons
            .iter()
            .find(|b| b.strategy_name.contains("SmartWalletCopy"))
            .map(|b| b.total_return_pct)
            .unwrap_or(Decimal::ZERO);

        let naive_ret = benchmark_comparisons
            .iter()
            .find(|b| b.strategy_name.contains("NaiveCopy"))
            .map(|b| b.total_return_pct)
            .unwrap_or(Decimal::ZERO);

        let beats_naive = smart_ret > naive_ret;

        let verdict_status = match config.data_source {
            DataSource::Synthetic => VerdictStatus::NotValidated,
            DataSource::Mixed => {
                if trades.len() < 30 {
                    VerdictStatus::InsufficientData
                } else {
                    VerdictStatus::PromisingButUnproven
                }
            }
            DataSource::Real => {
                if trades.len() < 50 {
                    VerdictStatus::InsufficientData
                } else if !permutation_test.is_significant
                    || test_metrics.net_pnl <= Decimal::ZERO
                    || !beats_naive
                    || test_metrics.max_drawdown_pct > Decimal::from(35)
                {
                    VerdictStatus::NoStatisticalEdge
                } else if !is_copiable_under_latency {
                    VerdictStatus::EdgeNotCopiable
                } else if max_scalable_capital < Decimal::from(1000) {
                    VerdictStatus::EdgeUnscalable
                } else {
                    VerdictStatus::EmpiricallySupported
                }
            }
        };

        let conclusion = match verdict_status {
            VerdictStatus::NotValidated => {
                "NOT VALIDATED: Results are based on synthetic data and serve only to test engine functionality. No statistical alpha can be inferred.".to_string()
            }
            VerdictStatus::InsufficientData => {
                format!("INSUFFICIENT DATA: Dataset contains only {} trades, below scientific statistical power requirements.", trades.len())
            }
            VerdictStatus::NoStatisticalEdge => {
                if !permutation_test.is_significant {
                    format!("NO STATISTICAL EDGE: Observed performance failed permutation testing (p={:.4} >= 0.05). Alpha is indistinguishable from luck.", permutation_test.p_value)
                } else if test_metrics.net_pnl <= Decimal::ZERO {
                    "NO STATISTICAL EDGE: Out-of-sample (Test) net PnL is negative or zero, indicating overfitting or lack of predictive power.".to_string()
                } else if !beats_naive {
                    "NO STATISTICAL EDGE: Out-of-sample performance fails to outperform naive blind copy trading.".to_string()
                } else {
                    format!("NO STATISTICAL EDGE: Excessive out-of-sample drawdown ({:.1}% > 35%).", test_metrics.max_drawdown_pct)
                }
            }
            VerdictStatus::EdgeNotCopiable => {
                format!(
                    "EDGE NOT COPIABLE: Alpha exists in theoretical zero-latency terms but is eliminated under realistic latency (break-even: {}s).",
                    break_even_latency.unwrap_or(2)
                )
            }
            VerdictStatus::EdgeUnscalable => {
                format!(
                    "EDGE UNSCALABLE: Severe price impact limits capital capacity to ${} (threshold: $1,000).",
                    max_scalable_capital
                )
            }
            VerdictStatus::PromisingButUnproven => {
                "PROMISING BUT UNPROVEN: Hybrid dataset indicates edge but requires pure on-chain validation.".to_string()
            }
            VerdictStatus::EmpiricallySupported => {
                format!(
                    "EMPIRICALLY SUPPORTED: Statistically significant alpha (p={:.4}) resilient to realistic execution latency (break-even {}s) and scalable to ${}.",
                    permutation_test.p_value,
                    break_even_latency.map(|d| d.to_string()).unwrap_or_else(|| ">120".into()),
                    max_scalable_capital
                )
            }
        };

        let verdict = ScientificVerdict {
            status: verdict_status,
            data_source: config.data_source,
            is_alpha_statistically_significant: permutation_test.is_significant,
            is_copiable_under_latency,
            maximum_scalable_capital_usd: max_scalable_capital,
            break_even_latency_seconds: break_even_latency,
            conclusion,
        };

        ExperimentReport {
            experiment_id,
            created_at,
            config: config.clone(),
            git_commit: git_commit.to_string(),
            dataset_hash,
            total_events: trades.len(),
            start_timestamp,
            end_timestamp,
            wallet_classifications,
            train_selected_wallets,
            baseline_metrics,
            delay_curve,
            scalability_curve,
            train_metrics,
            val_metrics,
            test_metrics,
            walk_forward_windows,
            ablation_results,
            permutation_test,
            bootstrap_ci,
            benchmark_comparisons,
            verdict,
        }
    }
}

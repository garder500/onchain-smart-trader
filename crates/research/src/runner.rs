use crate::benchmarks::BenchmarkEngine;
use crate::copiability::CopiabilityEngine;
use crate::scalability::ScalabilityEngine;
use crate::types::{ExperimentConfig, ExperimentId, ExperimentReport, ScientificVerdict};
use crate::validation::ScientificValidator;
use crate::wallet_engine::WalletResearchEngine;
use chrono::Utc;
use domain::Trade;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};

pub struct ResearchRunner;

impl ResearchRunner {
    /// Executes a full scientific research experiment across all empirical dimensions.
    pub fn run_experiment(
        trades: &[Trade],
        config: &ExperimentConfig,
        git_commit: &str,
    ) -> ExperimentReport {
        let experiment_id = ExperimentId::generate();
        let created_at = Utc::now();

        let mut sorted_trades = trades.to_vec();
        sorted_trades.sort_by_key(|t| t.timestamp);

        let start_timestamp = sorted_trades
            .first()
            .map(|t| t.timestamp)
            .unwrap_or(created_at);
        let end_timestamp = sorted_trades
            .last()
            .map(|t| t.timestamp)
            .unwrap_or(created_at);

        // Compute dataset hash
        let dataset_str = format!("{}_{}_{}", trades.len(), start_timestamp, end_timestamp);
        let dataset_hash = format!("{:x}", md5_like_hash(&dataset_str));

        // 1. Group trades by wallet
        let mut wallet_trades: HashMap<String, Vec<Trade>> = HashMap::new();
        for trade in &sorted_trades {
            let addr = trade.wallet_address.as_str().to_string();
            wallet_trades.entry(addr).or_default().push(trade.clone());
        }

        // 2. Wallet behavioral classification & filtering
        let mut wallet_classifications = Vec::new();
        let mut copiable_wallet_addrs = HashSet::new();

        for (addr, w_trades) in &wallet_trades {
            let classification =
                WalletResearchEngine::classify_wallet(addr, w_trades, end_timestamp);
            if classification.copiable {
                copiable_wallet_addrs.insert(addr.clone());
            }
            wallet_classifications.push(classification);
        }

        // Filter trades belonging to copiable wallets
        let copiable_trades: Vec<Trade> = sorted_trades
            .iter()
            .filter(|t| copiable_wallet_addrs.contains(t.wallet_address.as_str()))
            .cloned()
            .collect();

        // 3. Baseline metrics
        let baseline_metrics = ScientificValidator::evaluate_slice(&copiable_trades);

        // 4. Copiability Engine: Latency Degradation Curve
        let fixed_capital_per_trade = Decimal::from(1000);
        let fee_bps = 30; // 0.30% DEX fee
        let delay_curve = CopiabilityEngine::evaluate_latency_matrix(
            &copiable_trades,
            &config.delays_seconds,
            fixed_capital_per_trade,
            fee_bps,
        );

        // Find break-even latency
        let mut break_even_latency = None;
        for point in &delay_curve {
            if point.net_pnl <= Decimal::ZERO && point.delay_seconds > 0 {
                break_even_latency = Some(point.delay_seconds);
                break;
            }
        }

        // 5. Scalability Engine: Capital Impact Curve
        let assumed_pool_liquidity = Decimal::from(100_000); // $100k pool liquidity baseline
        let scalability_curve = ScalabilityEngine::evaluate_scalability(
            &copiable_trades,
            &config.capitals_usd,
            assumed_pool_liquidity,
            fee_bps,
        );
        let max_scalable_capital =
            ScalabilityEngine::compute_maximum_scalable_capital(&scalability_curve);

        // 6. Chronological Out-Of-Sample Splitting (Anti-Look-Ahead)
        let (train_trades, val_trades, test_trades) = ScientificValidator::chronological_split(
            &copiable_trades,
            config.train_ratio,
            config.val_ratio,
        );

        let train_metrics = ScientificValidator::evaluate_slice(&train_trades);
        let val_metrics = ScientificValidator::evaluate_slice(&val_trades);
        let test_metrics = ScientificValidator::evaluate_slice(&test_trades);

        // 7. Walk-Forward Cross-Validation
        let walk_forward_windows = ScientificValidator::walk_forward_analysis(&copiable_trades, 3);

        // 8. Ablation Study
        let ablation_results = ScientificValidator::ablation_study(&copiable_trades);

        // 9. Permutation Testing (H0 Null Hypothesis)
        let permutation_test =
            ScientificValidator::permutation_test(&copiable_trades, config.permutation_iterations);

        // 10. Bootstrap 95% Confidence Intervals
        let bootstrap_ci = ScientificValidator::bootstrap_confidence_intervals(
            &copiable_trades,
            config.bootstrap_iterations,
        );

        // 11. Benchmark Comparisons
        let benchmark_comparisons =
            BenchmarkEngine::evaluate_benchmarks(&copiable_trades, &sorted_trades);

        // 12. Synthesize Scientific Verdict
        let is_copiable_under_latency = delay_curve
            .iter()
            .find(|p| p.delay_seconds == 2)
            .map(|p| p.net_pnl > Decimal::ZERO)
            .unwrap_or(false);

        let conclusion = if !permutation_test.is_significant {
            "Statistical test failed: the observed alpha is indistinguishable from random luck (p >= 0.05). Copy-trading this wallet set has NO statistically proven edge.".to_string()
        } else if !is_copiable_under_latency {
            format!(
                "Alpha is statistically significant on paper at 0s, but is ENTIRELY DESTROYED by realistic execution latency. Break-even delay is {}s. Copy-trading is an operational illusion.",
                break_even_latency.unwrap_or(1)
            )
        } else if max_scalable_capital < Decimal::from(1000) {
            "Statistically valid and resilient to low latency, but unscalable: price impact erodes edge above micro-capital ($1,000).".to_string()
        } else {
            format!(
                "Statistically significant edge confirmed (p={:.4}). Resilient up to latency limits with capacity viable up to ${}.",
                permutation_test.p_value, max_scalable_capital
            )
        };

        let verdict = ScientificVerdict {
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

fn md5_like_hash(input: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

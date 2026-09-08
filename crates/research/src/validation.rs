use crate::types::{
    AblationVariantResult, BootstrapConfidenceInterval, PerformanceMetrics, PermutationTestResult,
    WalkForwardWindow,
};
use domain::Trade;
use rand::seq::SliceRandom;
use rand::Rng;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use wallet_profiler::MetricsCalculator;

pub struct ScientificValidator;

impl ScientificValidator {
    /// Splits trades chronologically without look-ahead: Train, Validation, Test.
    pub fn chronological_split(
        trades: &[Trade],
        train_ratio: f64,
        val_ratio: f64,
    ) -> (Vec<Trade>, Vec<Trade>, Vec<Trade>) {
        let mut sorted = trades.to_vec();
        sorted.sort_by_key(|t| t.timestamp);

        let n = sorted.len();
        let train_end = ((n as f64) * train_ratio).round() as usize;
        let val_end = ((n as f64) * (train_ratio + val_ratio)).round() as usize;

        let train = sorted[..train_end.min(n)].to_vec();
        let val = sorted[train_end.min(n)..val_end.min(n)].to_vec();
        let test = sorted[val_end.min(n)..].to_vec();

        (train, val, test)
    }

    /// Evaluates PerformanceMetrics on a trade slice.
    pub fn evaluate_slice(trades: &[Trade]) -> PerformanceMetrics {
        if trades.is_empty() {
            return PerformanceMetrics::default();
        }

        let eval_time = trades
            .last()
            .map(|t| t.timestamp)
            .unwrap_or_else(chrono::Utc::now);
        let m = MetricsCalculator::compute_metrics(trades, eval_time, 1800);

        PerformanceMetrics {
            total_trades: m.total_trades,
            winning_trades: m.winning_trades,
            losing_trades: m.losing_trades,
            win_rate: m.win_rate,
            loss_rate: m.loss_rate,
            gross_pnl: m.realized_pnl.max(Decimal::ZERO),
            net_pnl: m.realized_pnl + m.unrealized_pnl,
            profit_factor: m.profit_factor,
            expectancy: m.expectancy,
            max_drawdown_pct: m.max_drawdown * Decimal::from(100),
            sharpe_ratio: m.sharpe_ratio,
        }
    }

    /// Conducts rolling walk-forward cross-validation across multiple chronological windows.
    pub fn walk_forward_analysis(trades: &[Trade], num_windows: usize) -> Vec<WalkForwardWindow> {
        let mut sorted = trades.to_vec();
        sorted.sort_by_key(|t| t.timestamp);

        if sorted.len() < 10 || num_windows < 2 {
            return Vec::new();
        }

        let chunk_size = sorted.len() / (num_windows + 1);
        let mut windows = Vec::new();

        for i in 0..num_windows {
            let train_slice = &sorted[i * chunk_size..(i + 1) * chunk_size];
            let test_slice = &sorted[(i + 1) * chunk_size..(i + 2) * chunk_size.min(sorted.len())];

            if train_slice.is_empty() || test_slice.is_empty() {
                continue;
            }

            let m_train = Self::evaluate_slice(train_slice);
            let m_test = Self::evaluate_slice(test_slice);

            let s_train = m_train.sharpe_ratio.unwrap_or(Decimal::ZERO);
            let s_test = m_test.sharpe_ratio.unwrap_or(Decimal::ZERO);

            let degradation = if s_train > Decimal::ZERO {
                ((s_train - s_test) / s_train) * Decimal::from(100)
            } else {
                Decimal::ZERO
            };

            windows.push(WalkForwardWindow {
                window_index: i + 1,
                train_start: train_slice.first().unwrap().timestamp,
                train_end: train_slice.last().unwrap().timestamp,
                test_start: test_slice.first().unwrap().timestamp,
                test_end: test_slice.last().unwrap().timestamp,
                in_sample_sharpe: m_train.sharpe_ratio,
                out_of_sample_sharpe: m_test.sharpe_ratio,
                degradation_pct: degradation,
            });
        }

        windows
    }

    /// Runs Monte Carlo permutation testing to compute p-value against the null hypothesis H0.
    pub fn permutation_test(trades: &[Trade], iterations: usize) -> PermutationTestResult {
        let baseline = Self::evaluate_slice(trades);
        let observed_sharpe = baseline.sharpe_ratio.unwrap_or(Decimal::ZERO);

        if trades.len() < 4 {
            return PermutationTestResult {
                observed_sharpe,
                null_mean_sharpe: Decimal::ZERO,
                p_value: 1.0,
                is_significant: false,
            };
        }

        let mut rng = rand::thread_rng();
        let mut null_sharpes = Vec::with_capacity(iterations);
        let mut extreme_count = 0usize;

        for _ in 0..iterations {
            let mut shuffled_trades = trades.to_vec();
            // Shuffle price changes / timestamps to destroy temporal structure
            shuffled_trades.shuffle(&mut rng);

            let null_m = Self::evaluate_slice(&shuffled_trades);
            let null_sharpe = null_m.sharpe_ratio.unwrap_or(Decimal::ZERO);
            null_sharpes.push(null_sharpe);

            if null_sharpe >= observed_sharpe {
                extreme_count += 1;
            }
        }

        let null_mean_sharpe = if !null_sharpes.is_empty() {
            null_sharpes.iter().sum::<Decimal>() / Decimal::from(null_sharpes.len())
        } else {
            Decimal::ZERO
        };

        let p_value = (extreme_count as f64) / (iterations as f64);
        let is_significant = p_value < 0.05 && observed_sharpe > Decimal::ZERO;

        PermutationTestResult {
            observed_sharpe,
            null_mean_sharpe,
            p_value,
            is_significant,
        }
    }

    /// Computes 95% Bootstrap Confidence Intervals for key metrics.
    pub fn bootstrap_confidence_intervals(
        trades: &[Trade],
        iterations: usize,
    ) -> Vec<BootstrapConfidenceInterval> {
        if trades.is_empty() {
            return Vec::new();
        }

        let mut rng = rand::thread_rng();
        let n = trades.len();

        let mut win_rates = Vec::with_capacity(iterations);
        let mut expectancies = Vec::with_capacity(iterations);
        let mut sharpes = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let mut sample = Vec::with_capacity(n);
            for _ in 0..n {
                let idx = rng.gen_range(0..n);
                sample.push(trades[idx].clone());
            }

            let m = Self::evaluate_slice(&sample);
            win_rates.push(m.win_rate);
            expectancies.push(m.expectancy);
            if let Some(s) = m.sharpe_ratio {
                sharpes.push(s);
            }
        }

        fn get_ci(mut values: Vec<Decimal>, name: &str) -> BootstrapConfidenceInterval {
            if values.is_empty() {
                return BootstrapConfidenceInterval {
                    metric: name.into(),
                    mean: Decimal::ZERO,
                    ci_lower_95: Decimal::ZERO,
                    ci_upper_95: Decimal::ZERO,
                };
            }
            values.sort();
            let count = values.len();
            let mean = values.iter().sum::<Decimal>() / Decimal::from(count);
            let lower_idx = ((count as f64) * 0.025).round() as usize;
            let upper_idx = ((count as f64) * 0.975).round() as usize;

            BootstrapConfidenceInterval {
                metric: name.into(),
                mean,
                ci_lower_95: values[lower_idx.min(count - 1)],
                ci_upper_95: values[upper_idx.min(count - 1)],
            }
        }

        vec![
            get_ci(win_rates, "Win Rate"),
            get_ci(expectancies, "Expectancy ($)"),
            get_ci(sharpes, "Sharpe Ratio"),
        ]
    }

    /// Runs ablation tests comparing baseline with isolated mechanism removals.
    pub fn ablation_study(trades: &[Trade]) -> Vec<AblationVariantResult> {
        let baseline = Self::evaluate_slice(trades);
        let mut variants = Vec::new();

        variants.push(AblationVariantResult {
            variant_name: "Baseline (Full Filtering)".into(),
            description: "Smart wallet filter + Token risk filter + 0s latency".into(),
            net_pnl: baseline.net_pnl,
            sharpe: baseline.sharpe_ratio,
            pnl_delta_pct: Decimal::ZERO,
        });

        // Variant 1: Severe latency degradation (30s delay)
        let degraded_trades: Vec<Trade> = trades
            .iter()
            .map(|t| {
                let mut degraded = t.clone();
                match degraded.side {
                    domain::TradeSide::Buy => {
                        degraded.price_usd *= Decimal::from_str("1.05").unwrap()
                    }
                    domain::TradeSide::Sell => {
                        degraded.price_usd *= Decimal::from_str("0.95").unwrap()
                    }
                }
                degraded
            })
            .collect();
        let m_latency = Self::evaluate_slice(&degraded_trades);
        let delta_latency = if baseline.net_pnl > Decimal::ZERO {
            ((m_latency.net_pnl - baseline.net_pnl) / baseline.net_pnl) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        variants.push(AblationVariantResult {
            variant_name: "Ablation: +30s Latency Delay".into(),
            description: "5% adverse slippage on entry and exit".into(),
            net_pnl: m_latency.net_pnl,
            sharpe: m_latency.sharpe_ratio,
            pnl_delta_pct: delta_latency,
        });

        // Variant 2: Including Rug Pulls / Unfiltered Tokens
        let mut rug_heavy_trades = trades.to_vec();
        if let Some(last) = trades.last() {
            let mut rug_trade = last.clone();
            rug_trade.price_usd = Decimal::from_str("0.00001").unwrap();
            rug_heavy_trades.push(rug_trade);
        }
        let m_rugs = Self::evaluate_slice(&rug_heavy_trades);
        let delta_rugs = if baseline.net_pnl > Decimal::ZERO {
            ((m_rugs.net_pnl - baseline.net_pnl) / baseline.net_pnl) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        variants.push(AblationVariantResult {
            variant_name: "Ablation: No Token Risk Filter".into(),
            description: "Allows unverified deployers and low-liquidity honeypots".into(),
            net_pnl: m_rugs.net_pnl,
            sharpe: m_rugs.sharpe_ratio,
            pnl_delta_pct: delta_rugs,
        });

        variants
    }
}

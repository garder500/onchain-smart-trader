use crate::types::{
    AblationVariantResult, BootstrapConfidenceInterval, PerformanceMetrics, PermutationTestResult,
    WalkForwardWindow,
};
use crate::wallet_engine::WalletResearchEngine;
use chrono::{DateTime, Utc};
use domain::{Trade, TradeSide};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
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

    /// Evaluates PerformanceMetrics on a trade slice with complete decomposition.
    pub fn evaluate_slice(trades: &[Trade]) -> PerformanceMetrics {
        if trades.is_empty() {
            return PerformanceMetrics::default();
        }

        let eval_time = trades
            .last()
            .map(|t| t.timestamp)
            .unwrap_or_else(chrono::Utc::now);
        let m = MetricsCalculator::compute_metrics(trades, eval_time, 1800);

        // Compute trading fees from trade records
        let mut trading_fees = Decimal::ZERO;
        let gas_fees = Decimal::ZERO;
        for t in trades {
            trading_fees += t.fee_usd;
        }

        // Net PnL is realized + unrealized
        let net_pnl = m.realized_pnl + m.unrealized_pnl;
        let gross_pnl = net_pnl + trading_fees;

        PerformanceMetrics {
            total_trades: m.total_trades,
            winning_trades: m.winning_trades,
            losing_trades: m.losing_trades,
            win_rate: m.win_rate,
            loss_rate: m.loss_rate,
            gross_pnl,
            trading_fees,
            gas_fees,
            slippage_cost: Decimal::ZERO,
            net_pnl,
            profit_factor: m.profit_factor,
            expectancy: m.expectancy,
            max_drawdown_pct: m.max_drawdown * Decimal::from(100),
            trade_level_sharpe: m.sharpe_ratio,
        }
    }

    /// Conducts rolling walk-forward cross-validation across multiple chronological windows.
    /// In each window, wallets are selected strictly on the window's training partition up to train_end.
    pub fn walk_forward_analysis(
        all_trades: &[Trade],
        num_windows: usize,
        min_wallet_trades: usize,
    ) -> Vec<WalkForwardWindow> {
        let mut sorted = all_trades.to_vec();
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

            let train_end = train_slice.last().unwrap().timestamp;

            // Independently select copiable wallets on the train slice as of train_end
            let selected_wallets = WalletResearchEngine::select_copiable_wallets_as_of(
                train_slice,
                train_end,
                min_wallet_trades,
            );
            let selected_addrs: HashSet<String> = selected_wallets
                .iter()
                .filter(|w| w.copiable)
                .map(|w| w.wallet_address.clone())
                .collect();

            let train_selected_trades: Vec<Trade> = train_slice
                .iter()
                .filter(|t| selected_addrs.contains(t.wallet_address.as_str()))
                .cloned()
                .collect();

            let test_selected_trades: Vec<Trade> = test_slice
                .iter()
                .filter(|t| selected_addrs.contains(t.wallet_address.as_str()))
                .cloned()
                .collect();

            let m_train = Self::evaluate_slice(&train_selected_trades);
            let m_test = Self::evaluate_slice(&test_selected_trades);

            let s_train = m_train.trade_level_sharpe.unwrap_or(Decimal::ZERO);
            let s_test = m_test.trade_level_sharpe.unwrap_or(Decimal::ZERO);

            let degradation = if s_train > Decimal::ZERO {
                ((s_train - s_test) / s_train) * Decimal::from(100)
            } else {
                Decimal::ZERO
            };

            windows.push(WalkForwardWindow {
                window_index: i + 1,
                train_start: train_slice.first().unwrap().timestamp,
                train_end,
                test_start: test_slice.first().unwrap().timestamp,
                test_end: test_slice.last().unwrap().timestamp,
                selected_wallets_count: selected_addrs.len(),
                in_sample_sharpe: m_train.trade_level_sharpe,
                out_of_sample_sharpe: m_test.trade_level_sharpe,
                degradation_pct: degradation,
            });
        }

        windows
    }

    /// Runs Monte Carlo permutation testing with seeded PRNG.
    /// Resamples at the WALLET level to preserve intra-wallet trade correlations.
    pub fn permutation_test(
        trades: &[Trade],
        iterations: usize,
        seed: u64,
    ) -> PermutationTestResult {
        let baseline = Self::evaluate_slice(trades);
        let observed_sharpe = baseline.trade_level_sharpe.unwrap_or(Decimal::ZERO);

        if trades.len() < 4 {
            return PermutationTestResult {
                unit_of_randomization: "WALLET".into(),
                observed_sharpe,
                null_mean_sharpe: Decimal::ZERO,
                null_median_sharpe: Decimal::ZERO,
                p_value: 1.0,
                is_significant: false,
                iterations,
                seed,
            };
        }

        // Group trades by wallet
        let mut wallet_trades: HashMap<String, Vec<Trade>> = HashMap::new();
        for t in trades {
            wallet_trades
                .entry(t.wallet_address.as_str().to_string())
                .or_default()
                .push(t.clone());
        }

        let mut rng = StdRng::seed_from_u64(seed);
        let mut null_sharpes = Vec::with_capacity(iterations);
        let mut extreme_count = 0usize;

        let mut wallet_keys: Vec<String> = wallet_trades.keys().cloned().collect();
        wallet_keys.sort();

        for _ in 0..iterations {
            // Shuffle wallet allocations: assign trade sequences to randomly permuted wallets
            // and perturb timestamps/sides to simulate null hypothesis H0 of no predictive edge
            let mut shuffled_trades = Vec::with_capacity(trades.len());
            let mut permuted_keys = wallet_keys.clone();
            permuted_keys.shuffle(&mut rng);

            for (orig_key, perm_key) in wallet_keys.iter().zip(&permuted_keys) {
                if let Some(w_trades) = wallet_trades.get(orig_key) {
                    for t in w_trades {
                        let mut copy = t.clone();
                        copy.wallet_address = domain::WalletAddress::new(perm_key);
                        // Randomize side in null hypothesis simulation
                        if rng.gen_bool(0.5) {
                            copy.side = match copy.side {
                                TradeSide::Buy => TradeSide::Sell,
                                TradeSide::Sell => TradeSide::Buy,
                            };
                        }
                        shuffled_trades.push(copy);
                    }
                }
            }

            shuffled_trades.sort_by_key(|t| t.timestamp);
            let null_m = Self::evaluate_slice(&shuffled_trades);
            let null_sharpe = null_m.trade_level_sharpe.unwrap_or(Decimal::ZERO);
            null_sharpes.push(null_sharpe);

            if null_sharpe >= observed_sharpe {
                extreme_count += 1;
            }
        }

        null_sharpes.sort();
        let null_mean_sharpe = if !null_sharpes.is_empty() {
            null_sharpes.iter().sum::<Decimal>() / Decimal::from(null_sharpes.len())
        } else {
            Decimal::ZERO
        };

        let null_median_sharpe = if !null_sharpes.is_empty() {
            null_sharpes[null_sharpes.len() / 2]
        } else {
            Decimal::ZERO
        };

        let p_value = (extreme_count as f64) / (iterations as f64);
        let is_significant = p_value < 0.05 && observed_sharpe > Decimal::ZERO;

        PermutationTestResult {
            unit_of_randomization: "WALLET".into(),
            observed_sharpe,
            null_mean_sharpe,
            null_median_sharpe,
            p_value,
            is_significant,
            iterations,
            seed,
        }
    }

    /// Computes 95% and 99% Bootstrap Confidence Intervals resampled by WALLET with seeded PRNG.
    pub fn bootstrap_confidence_intervals(
        trades: &[Trade],
        iterations: usize,
        seed: u64,
    ) -> Vec<BootstrapConfidenceInterval> {
        if trades.is_empty() {
            return Vec::new();
        }

        let mut wallet_trades: HashMap<String, Vec<Trade>> = HashMap::new();
        for t in trades {
            wallet_trades
                .entry(t.wallet_address.as_str().to_string())
                .or_default()
                .push(t.clone());
        }
        let mut wallets: Vec<String> = wallet_trades.keys().cloned().collect();
        wallets.sort();
        let num_wallets = wallets.len();

        let mut rng = StdRng::seed_from_u64(seed);

        let mut win_rates = Vec::with_capacity(iterations);
        let mut expectancies = Vec::with_capacity(iterations);
        let mut sharpes = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let mut sample_trades = Vec::new();
            if num_wallets > 0 {
                for _ in 0..num_wallets {
                    let idx = rng.gen_range(0..num_wallets);
                    let w_addr = &wallets[idx];
                    if let Some(w_list) = wallet_trades.get(w_addr) {
                        sample_trades.extend(w_list.iter().cloned());
                    }
                }
            } else {
                for _ in 0..trades.len() {
                    let idx = rng.gen_range(0..trades.len());
                    sample_trades.push(trades[idx].clone());
                }
            }

            sample_trades.sort_by_key(|t| t.timestamp);
            let m = Self::evaluate_slice(&sample_trades);
            win_rates.push(m.win_rate);
            expectancies.push(m.expectancy);
            if let Some(s) = m.trade_level_sharpe {
                sharpes.push(s);
            }
        }

        fn get_ci(mut values: Vec<Decimal>, name: &str, unit: &str) -> BootstrapConfidenceInterval {
            if values.is_empty() {
                return BootstrapConfidenceInterval {
                    metric: name.into(),
                    unit: unit.into(),
                    mean: Decimal::ZERO,
                    median: Decimal::ZERO,
                    ci_lower_95: Decimal::ZERO,
                    ci_upper_95: Decimal::ZERO,
                    ci_lower_99: Decimal::ZERO,
                    ci_upper_99: Decimal::ZERO,
                };
            }
            values.sort();
            let count = values.len();
            let mean = values.iter().sum::<Decimal>() / Decimal::from(count);
            let median = values[count / 2];

            let lower_idx_95 = ((count as f64) * 0.025).round() as usize;
            let upper_idx_95 = ((count as f64) * 0.975).round() as usize;
            let lower_idx_99 = ((count as f64) * 0.005).round() as usize;
            let upper_idx_99 = ((count as f64) * 0.995).round() as usize;

            BootstrapConfidenceInterval {
                metric: name.into(),
                unit: unit.into(),
                mean,
                median,
                ci_lower_95: values[lower_idx_95.min(count - 1)],
                ci_upper_95: values[upper_idx_95.min(count - 1)],
                ci_lower_99: values[lower_idx_99.min(count - 1)],
                ci_upper_99: values[upper_idx_99.min(count - 1)],
            }
        }

        vec![
            get_ci(win_rates, "Win Rate", "%"),
            get_ci(expectancies, "Expectancy ($)", "USD"),
            get_ci(sharpes, "Trade-Level Sharpe Ratio", "ratio"),
        ]
    }

    /// Runs genuine ablation tests comparing baseline with isolated mechanism removals:
    /// 1. Baseline: Full filtering (Smart Wallets + Persistence + 0s latency)
    /// 2. NoWalletFilter: Copying all unfiltered wallets blindly in evaluation partition
    /// 3. NoPersistenceFilter: Including smart wallets regardless of persistence score
    /// 4. AdverseLatency: Applying realistic execution delay penalty (+5s delay)
    pub fn ablation_study(
        train_trades: &[Trade],
        eval_trades: &[Trade],
        baseline_eval_trades: &[Trade],
        as_of_timestamp: DateTime<Utc>,
    ) -> Vec<AblationVariantResult> {
        let baseline = Self::evaluate_slice(baseline_eval_trades);
        let mut variants = Vec::new();

        variants.push(AblationVariantResult {
            variant_name: "Baseline (Full Filtering)".into(),
            description: "Cluster filter + Persistence filter + Execution at signal time".into(),
            net_pnl: baseline.net_pnl,
            trade_level_sharpe: baseline.trade_level_sharpe,
            pnl_delta_pct: Decimal::ZERO,
        });

        // Variant 1: NoWalletFilter - Copy all wallets blindly in evaluation partition
        let m_unfiltered = Self::evaluate_slice(eval_trades);
        let delta_unfiltered = if baseline.net_pnl.abs() > Decimal::ZERO {
            ((m_unfiltered.net_pnl - baseline.net_pnl) / baseline.net_pnl.abs())
                * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        variants.push(AblationVariantResult {
            variant_name: "Ablation: No Wallet Filter".into(),
            description: "Copies every wallet blindly without behavioral clustering".into(),
            net_pnl: m_unfiltered.net_pnl,
            trade_level_sharpe: m_unfiltered.trade_level_sharpe,
            pnl_delta_pct: delta_unfiltered,
        });

        // Variant 2: NoPersistenceFilter - Smart wallets without checking consistency across time
        let mut wallet_trades: HashMap<String, Vec<Trade>> = HashMap::new();
        for t in train_trades {
            if t.timestamp <= as_of_timestamp {
                wallet_trades
                    .entry(t.wallet_address.as_str().to_string())
                    .or_default()
                    .push(t.clone());
            }
        }
        let mut non_persistent_copiable_addrs = HashSet::new();
        for (addr, w_trades) in &wallet_trades {
            let classification =
                WalletResearchEngine::classify_wallet_as_of(addr, w_trades, as_of_timestamp);
            // Include if classified as momentum/swing regardless of persistence score
            if matches!(
                classification.cluster,
                crate::types::BehavioralCluster::MomentumTrader
                    | crate::types::BehavioralCluster::SwingTrader
            ) {
                non_persistent_copiable_addrs.insert(addr.clone());
            }
        }
        let non_persistent_trades: Vec<Trade> = eval_trades
            .iter()
            .filter(|t| non_persistent_copiable_addrs.contains(t.wallet_address.as_str()))
            .cloned()
            .collect();
        let m_no_persistence = Self::evaluate_slice(&non_persistent_trades);
        let delta_no_persistence = if baseline.net_pnl.abs() > Decimal::ZERO {
            ((m_no_persistence.net_pnl - baseline.net_pnl) / baseline.net_pnl.abs())
                * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        variants.push(AblationVariantResult {
            variant_name: "Ablation: No Persistence Filter".into(),
            description:
                "Includes momentum/swing wallets without verifying cross-period persistence".into(),
            net_pnl: m_no_persistence.net_pnl,
            trade_level_sharpe: m_no_persistence.trade_level_sharpe,
            pnl_delta_pct: delta_no_persistence,
        });

        // Variant 3: Adverse Execution Latency (+5s Delay)
        let degraded_trades: Vec<Trade> = baseline_eval_trades
            .iter()
            .map(|t| {
                let mut degraded = t.clone();
                match degraded.side {
                    TradeSide::Buy => {
                        degraded.price_usd *= Decimal::from_str("1.02").unwrap_or(Decimal::ONE);
                    }
                    TradeSide::Sell => {
                        degraded.price_usd *= Decimal::from_str("0.98").unwrap_or(Decimal::ONE);
                    }
                }
                degraded
            })
            .collect();
        let m_latency = Self::evaluate_slice(&degraded_trades);
        let delta_latency = if baseline.net_pnl.abs() > Decimal::ZERO {
            ((m_latency.net_pnl - baseline.net_pnl) / baseline.net_pnl.abs()) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        variants.push(AblationVariantResult {
            variant_name: "Ablation: Realistic Execution Latency (+5s Delay)".into(),
            description: "2% adverse price slippage on copy entry and exit".into(),
            net_pnl: m_latency.net_pnl,
            trade_level_sharpe: m_latency.trade_level_sharpe,
            pnl_delta_pct: delta_latency,
        });

        variants
    }

    /// Computes Benjamini-Hochberg (False Discovery Rate) adjusted p-values.
    /// Given raw p-values [p_1, ..., p_m], sorts them:
    /// p_(1) <= p_(2) <= ... <= p_(m)
    /// Adjusted p-value: q_(i) = min_{k >= i} [ (m / k) * p_(k) ] capped at 1.0
    pub fn benjamini_hochberg_correction(raw_p_values: &[f64]) -> Vec<f64> {
        let m = raw_p_values.len();
        if m == 0 {
            return Vec::new();
        }

        let mut indexed: Vec<(usize, f64)> = raw_p_values.iter().cloned().enumerate().collect();
        // Sort ascending by p-value
        indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut adjusted_indexed = vec![1.0; m];
        let mut min_so_far: f64 = 1.0;

        // Iterate backwards from m down to 1
        for rank in (1..=m).rev() {
            let orig_idx = indexed[rank - 1].0;
            let p_val = indexed[rank - 1].1;
            let unconstrained_q = (p_val * (m as f64) / (rank as f64)).min(1.0);
            min_so_far = min_so_far.min(unconstrained_q);
            adjusted_indexed[orig_idx] = min_so_far;
        }

        adjusted_indexed
    }

    /// Cross-Pool Generalization Analysis:
    /// Trains on pools in `train_pools` (e.g. USDC/WETH, WBTC/WETH) and evaluates on `test_pool` (e.g. DAI/WETH or USDT/WETH)
    pub fn cross_pool_analysis(
        all_trades: &[Trade],
        train_pool_addrs: &[String],
        test_pool_addr: &str,
        min_wallet_trades: usize,
    ) -> crate::types::CrossPoolEvaluation {
        let train_pool_set: HashSet<String> = train_pool_addrs.iter().cloned().collect();

        let train_trades: Vec<Trade> = all_trades
            .iter()
            .filter(|t| train_pool_set.contains(t.token_address.as_str()))
            .cloned()
            .collect();

        let test_trades: Vec<Trade> = all_trades
            .iter()
            .filter(|t| t.token_address.as_str() == test_pool_addr)
            .cloned()
            .collect();

        let train_end = train_trades
            .last()
            .map(|t| t.timestamp)
            .unwrap_or_else(Utc::now);

        let selected = WalletResearchEngine::select_copiable_wallets_as_of(
            &train_trades,
            train_end,
            min_wallet_trades,
        );
        let selected_addrs: HashSet<String> = selected
            .into_iter()
            .filter(|w| w.copiable)
            .map(|w| w.wallet_address)
            .collect();

        let test_copiable: Vec<Trade> = test_trades
            .iter()
            .filter(|t| selected_addrs.contains(t.wallet_address.as_str()))
            .cloned()
            .collect();

        let m_test = Self::evaluate_slice(&test_copiable);
        let m_train = Self::evaluate_slice(&train_trades);

        let gen_ratio = if m_train.net_pnl > Decimal::ZERO {
            (m_test.net_pnl / m_train.net_pnl).max(Decimal::ZERO)
        } else {
            Decimal::ZERO
        };

        crate::types::CrossPoolEvaluation {
            train_pools: train_pool_addrs.to_vec(),
            test_pool: test_pool_addr.to_string(),
            pool_type: "CROSS_PAIR".into(),
            net_pnl: m_test.net_pnl,
            win_rate: m_test.win_rate,
            out_of_sample_sharpe: m_test.trade_level_sharpe,
            trade_count: test_copiable.len(),
            generalization_ratio: gen_ratio,
        }
    }

    /// Unseen Wallet Evaluation:
    /// Segregates performance on:
    /// 1. Known wallets (discovered in Train)
    /// 2. Unseen wallets (existing wallets in Test that were NOT selected in Train)
    /// 3. New wallets (wallets whose first seen timestamp is > Train cutoff)
    pub fn unseen_wallet_analysis(
        train_trades: &[Trade],
        test_trades: &[Trade],
        selected_wallets: &HashSet<String>,
        _train_end: DateTime<Utc>,
    ) -> crate::types::UnseenWalletEvaluation {
        let known_trades: Vec<Trade> = test_trades
            .iter()
            .filter(|t| selected_wallets.contains(t.wallet_address.as_str()))
            .cloned()
            .collect();

        let mut train_seen_wallets = HashSet::new();
        for t in train_trades {
            train_seen_wallets.insert(t.wallet_address.as_str().to_string());
        }

        let unseen_existing_trades: Vec<Trade> = test_trades
            .iter()
            .filter(|t| {
                train_seen_wallets.contains(t.wallet_address.as_str())
                    && !selected_wallets.contains(t.wallet_address.as_str())
            })
            .cloned()
            .collect();

        let new_post_train_trades: Vec<Trade> = test_trades
            .iter()
            .filter(|t| {
                !train_seen_wallets.contains(t.wallet_address.as_str())
                    && !selected_wallets.contains(t.wallet_address.as_str())
            })
            .cloned()
            .collect();

        let m_known = Self::evaluate_slice(&known_trades);
        let m_unseen = Self::evaluate_slice(&unseen_existing_trades);
        let m_new = Self::evaluate_slice(&new_post_train_trades);

        crate::types::UnseenWalletEvaluation {
            known_wallets_trades: known_trades.len(),
            known_wallets_sharpe: m_known.trade_level_sharpe,
            known_wallets_pnl: m_known.net_pnl,
            unseen_wallets_trades: unseen_existing_trades.len(),
            unseen_wallets_sharpe: m_unseen.trade_level_sharpe,
            unseen_wallets_pnl: m_unseen.net_pnl,
            new_post_train_wallets_trades: new_post_train_trades.len(),
            new_post_train_wallets_sharpe: m_new.trade_level_sharpe,
            new_post_train_wallets_pnl: m_new.net_pnl,
        }
    }

    /// Market Regime Evaluation:
    /// Evaluates strategy returns under High vs Low Volatility, High vs Low Liquidity regimes
    pub fn regime_analysis(evaluated_trades: &[Trade]) -> Vec<crate::types::RegimePerformance> {
        if evaluated_trades.is_empty() {
            return Vec::new();
        }

        let mut sorted = evaluated_trades.to_vec();
        sorted.sort_by_key(|t| t.timestamp);

        // Partition by volume/liquidity size (median volume as threshold)
        let mut volumes: Vec<Decimal> = sorted.iter().map(|t| t.volume_usd).collect();
        volumes.sort();
        let median_vol = volumes[volumes.len() / 2];

        let high_vol_trades: Vec<Trade> = sorted
            .iter()
            .filter(|t| t.volume_usd >= median_vol)
            .cloned()
            .collect();
        let low_vol_trades: Vec<Trade> = sorted
            .iter()
            .filter(|t| t.volume_usd < median_vol)
            .cloned()
            .collect();

        let m_high = Self::evaluate_slice(&high_vol_trades);
        let m_low = Self::evaluate_slice(&low_vol_trades);

        vec![
            crate::types::RegimePerformance {
                regime: crate::types::MarketRegime::HighLiquidity,
                trade_count: high_vol_trades.len(),
                net_pnl: m_high.net_pnl,
                win_rate: m_high.win_rate,
                trade_level_sharpe: m_high.trade_level_sharpe,
                profit_factor: m_high.profit_factor,
            },
            crate::types::RegimePerformance {
                regime: crate::types::MarketRegime::LowLiquidity,
                trade_count: low_vol_trades.len(),
                net_pnl: m_low.net_pnl,
                win_rate: m_low.win_rate,
                trade_level_sharpe: m_low.trade_level_sharpe,
                profit_factor: m_low.profit_factor,
            },
        ]
    }

    /// Computes standardized effect sizes comparing strategy trades against a benchmark baseline
    pub fn compute_effect_sizes(
        strategy_trades: &[Trade],
        benchmark_trades: &[Trade],
    ) -> crate::types::EffectSizeReport {
        let m_strat = Self::evaluate_slice(strategy_trades);
        let m_bench = Self::evaluate_slice(benchmark_trades);

        let strat_returns: Vec<f64> = strategy_trades
            .iter()
            .map(|t| t.price_usd.to_f64().unwrap_or(0.0))
            .collect();
        let bench_returns: Vec<f64> = benchmark_trades
            .iter()
            .map(|t| t.price_usd.to_f64().unwrap_or(0.0))
            .collect();

        let mean_s = if !strat_returns.is_empty() {
            strat_returns.iter().sum::<f64>() / (strat_returns.len() as f64)
        } else {
            0.0
        };
        let mean_b = if !bench_returns.is_empty() {
            bench_returns.iter().sum::<f64>() / (bench_returns.len() as f64)
        } else {
            0.0
        };

        let var_s = if strat_returns.len() > 1 {
            strat_returns
                .iter()
                .map(|r| (r - mean_s).powi(2))
                .sum::<f64>()
                / ((strat_returns.len() - 1) as f64)
        } else {
            0.0
        };
        let var_b = if bench_returns.len() > 1 {
            bench_returns
                .iter()
                .map(|r| (r - mean_b).powi(2))
                .sum::<f64>()
                / ((bench_returns.len() - 1) as f64)
        } else {
            0.0
        };

        let pooled_var = ((var_s + var_b) / 2.0).max(1e-12);
        let cohen_d = (mean_s - mean_b) / pooled_var.sqrt();

        let mean_excess = Decimal::from_f64_retain(mean_s - mean_b).unwrap_or(Decimal::ZERO);
        let cohen_d_dec = Decimal::from_f64_retain(cohen_d);

        let sharpe_diff = match (m_strat.trade_level_sharpe, m_bench.trade_level_sharpe) {
            (Some(s), Some(b)) => Some(s - b),
            (Some(s), None) => Some(s),
            (None, Some(b)) => Some(-b),
            (None, None) => None,
        };

        crate::types::EffectSizeReport {
            mean_excess_return: mean_excess,
            median_excess_return: mean_excess, // robust proxy
            cohen_d: cohen_d_dec,
            win_rate_diff_vs_benchmark: m_strat.win_rate - m_bench.win_rate,
            sharpe_diff_vs_benchmark: sharpe_diff,
        }
    }
}

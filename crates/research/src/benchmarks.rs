use crate::types::{BenchmarkComparison, RandomBenchmarkDistribution};
use crate::validation::ScientificValidator;
use domain::Trade;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct BenchmarkEngine;

impl BenchmarkEngine {
    /// Compares SmartWalletCopy against standard empirical baseline benchmarks without hardcoded values.
    pub fn evaluate_benchmarks(
        smart_trades: &[Trade],
        all_unfiltered_trades: &[Trade],
        initial_capital: Decimal,
        random_runs: usize,
        seed: u64,
    ) -> Vec<BenchmarkComparison> {
        let mut comparisons = Vec::new();

        // 1. Smart Wallet Copy Strategy (Our candidate)
        let smart_m = ScientificValidator::evaluate_slice(smart_trades);
        let smart_ret = if initial_capital > Decimal::ZERO {
            (smart_m.net_pnl / initial_capital) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        comparisons.push(BenchmarkComparison {
            strategy_name: "SmartWalletCopy (Filtered OOS)".into(),
            total_return_pct: smart_ret,
            trade_level_sharpe: smart_m.trade_level_sharpe,
            max_drawdown_pct: smart_m.max_drawdown_pct,
            win_rate: smart_m.win_rate,
            random_distribution: None,
        });

        // 2. Naive Copy Strategy (Blindly copy every wallet transaction)
        let naive_m = ScientificValidator::evaluate_slice(all_unfiltered_trades);
        let naive_ret = if initial_capital > Decimal::ZERO {
            (naive_m.net_pnl / initial_capital) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        comparisons.push(BenchmarkComparison {
            strategy_name: "NaiveCopy (Unfiltered All Wallets)".into(),
            total_return_pct: naive_ret,
            trade_level_sharpe: naive_m.trade_level_sharpe,
            max_drawdown_pct: naive_m.max_drawdown_pct,
            win_rate: naive_m.win_rate,
            random_distribution: None,
        });

        // 3. Random Wallet Selection (Monte Carlo distribution over N runs)
        let (random_summary, random_dist) = Self::compute_random_selection_distribution(
            all_unfiltered_trades,
            initial_capital,
            random_runs,
            seed,
        );
        comparisons.push(BenchmarkComparison {
            strategy_name: format!("RandomSelection (Monte Carlo N={})", random_runs),
            total_return_pct: random_summary.total_return_pct,
            trade_level_sharpe: random_summary.trade_level_sharpe,
            max_drawdown_pct: random_summary.max_drawdown_pct,
            win_rate: random_summary.win_rate,
            random_distribution: Some(random_dist),
        });

        // 4. Buy and Hold: Computed from actual price trajectory across tokens in the dataset
        let buy_and_hold = Self::compute_buy_and_hold(all_unfiltered_trades);
        comparisons.push(buy_and_hold);

        comparisons
    }

    /// Computes empirical Buy & Hold performance from the actual price change of tokens in the dataset.
    /// Uses first price seen to last price seen over the evaluation window.
    pub fn compute_buy_and_hold(all_trades: &[Trade]) -> BenchmarkComparison {
        if all_trades.is_empty() {
            return BenchmarkComparison {
                strategy_name: "Buy & Hold (Empirical Market Basket)".into(),
                total_return_pct: Decimal::ZERO,
                trade_level_sharpe: None,
                max_drawdown_pct: Decimal::ZERO,
                win_rate: Decimal::ZERO,
                random_distribution: None,
            };
        }

        let mut sorted = all_trades.to_vec();
        sorted.sort_by_key(|t| t.timestamp);

        // Map token -> (first_price, last_price)
        let mut token_prices: HashMap<String, (Decimal, Decimal)> = HashMap::new();
        for t in &sorted {
            let key = t.token_address.as_str().to_string();
            token_prices
                .entry(key)
                .and_modify(|e| e.1 = t.price_usd)
                .or_insert((t.price_usd, t.price_usd));
        }

        let mut token_returns = Vec::new();
        let mut wins = 0usize;
        for (p_first, p_last) in token_prices.values() {
            if *p_first > Decimal::ZERO {
                let ret = (*p_last - *p_first) / *p_first;
                if ret > Decimal::ZERO {
                    wins += 1;
                }
                token_returns.push(ret);
            }
        }

        let count = token_returns.len();
        if count == 0 {
            return BenchmarkComparison {
                strategy_name: "Buy & Hold (Empirical Market Basket)".into(),
                total_return_pct: Decimal::ZERO,
                trade_level_sharpe: None,
                max_drawdown_pct: Decimal::ZERO,
                win_rate: Decimal::ZERO,
                random_distribution: None,
            };
        }

        let avg_return = token_returns.iter().sum::<Decimal>() / Decimal::from(count);
        let total_return_pct = avg_return * Decimal::from(100);
        let win_rate = Decimal::from(wins) / Decimal::from(count);

        // Trade-level Sharpe of the buy-and-hold basket
        let sharpe = if count >= 2 {
            let n = count as f64;
            let mean: f64 = token_returns.iter().filter_map(|r| r.to_f64()).sum::<f64>() / n;
            let var: f64 = token_returns
                .iter()
                .filter_map(|r| r.to_f64())
                .map(|r| (r - mean).powi(2))
                .sum::<f64>()
                / (n - 1.0);
            let stdev = var.sqrt();
            if stdev > 1e-6 {
                Decimal::from_f64_retain(mean / stdev)
            } else {
                None
            }
        } else {
            None
        };

        BenchmarkComparison {
            strategy_name: "Buy & Hold (Empirical Market Basket)".into(),
            total_return_pct,
            trade_level_sharpe: sharpe,
            max_drawdown_pct: Decimal::ZERO, // No active round-trip drawdown in basket
            win_rate,
            random_distribution: None,
        }
    }

    /// Computes the empirical distribution of random wallet selections across N Monte Carlo runs.
    pub fn compute_random_selection_distribution(
        all_trades: &[Trade],
        initial_capital: Decimal,
        runs: usize,
        seed: u64,
    ) -> (BenchmarkComparison, RandomBenchmarkDistribution) {
        let mut wallet_trades: HashMap<String, Vec<Trade>> = HashMap::new();
        for t in all_trades {
            wallet_trades
                .entry(t.wallet_address.as_str().to_string())
                .or_default()
                .push(t.clone());
        }
        let mut wallets: Vec<String> = wallet_trades.keys().cloned().collect();
        wallets.sort();

        if wallets.is_empty() || runs == 0 {
            let dummy_dist = RandomBenchmarkDistribution {
                runs: 0,
                mean_return_pct: Decimal::ZERO,
                median_return_pct: Decimal::ZERO,
                std_dev: Decimal::ZERO,
                p25_return_pct: Decimal::ZERO,
                p75_return_pct: Decimal::ZERO,
                ci_lower_95: Decimal::ZERO,
                ci_upper_95: Decimal::ZERO,
                mean_win_rate: Decimal::ZERO,
                mean_trade_sharpe: None,
            };
            let dummy_comp = BenchmarkComparison {
                strategy_name: "RandomSelection".into(),
                total_return_pct: Decimal::ZERO,
                trade_level_sharpe: None,
                max_drawdown_pct: Decimal::ZERO,
                win_rate: Decimal::ZERO,
                random_distribution: Some(dummy_dist.clone()),
            };
            return (dummy_comp, dummy_dist);
        }

        let mut rng = StdRng::seed_from_u64(seed);
        let sample_size = (wallets.len() / 3).max(1);

        let mut returns_pct = Vec::with_capacity(runs);
        let mut win_rates = Vec::with_capacity(runs);
        let mut sharpes = Vec::with_capacity(runs);
        let mut drawdowns = Vec::with_capacity(runs);

        for _ in 0..runs {
            let mut chosen = wallets.clone();
            chosen.shuffle(&mut rng);
            let selected_wallets: std::collections::HashSet<String> =
                chosen.into_iter().take(sample_size).collect();

            let sample_trades: Vec<Trade> = all_trades
                .iter()
                .filter(|t| selected_wallets.contains(t.wallet_address.as_str()))
                .cloned()
                .collect();

            let m = ScientificValidator::evaluate_slice(&sample_trades);
            let ret = if initial_capital > Decimal::ZERO {
                (m.net_pnl / initial_capital) * Decimal::from(100)
            } else {
                Decimal::ZERO
            };

            returns_pct.push(ret);
            win_rates.push(m.win_rate);
            drawdowns.push(m.max_drawdown_pct);
            if let Some(s) = m.trade_level_sharpe {
                sharpes.push(s);
            }
        }

        returns_pct.sort();
        let count_dec = Decimal::from(runs);
        let mean_ret = returns_pct.iter().sum::<Decimal>() / count_dec;
        let median_ret = returns_pct[runs / 2];

        // Std dev of returns
        let mean_f = mean_ret.to_f64().unwrap_or(0.0);
        let var_f = returns_pct
            .iter()
            .map(|r| (r.to_f64().unwrap_or(0.0) - mean_f).powi(2))
            .sum::<f64>()
            / (runs as f64).max(1.0);
        let std_dev = Decimal::from_f64_retain(var_f.sqrt()).unwrap_or(Decimal::ZERO);

        let p25 = returns_pct[((runs as f64) * 0.25).round() as usize];
        let p75 = returns_pct[((runs as f64) * 0.75).round() as usize];
        let ci_lower_95 = returns_pct[((runs as f64) * 0.025).round() as usize];
        let ci_upper_95 =
            returns_pct[((runs as f64) * 0.975).round().min(runs as f64 - 1.0) as usize];

        let mean_win_rate = win_rates.iter().sum::<Decimal>() / count_dec;
        let mean_drawdown = drawdowns.iter().sum::<Decimal>() / count_dec;
        let mean_sharpe = if !sharpes.is_empty() {
            Some(sharpes.iter().sum::<Decimal>() / Decimal::from(sharpes.len()))
        } else {
            None
        };

        let distribution = RandomBenchmarkDistribution {
            runs,
            mean_return_pct: mean_ret,
            median_return_pct: median_ret,
            std_dev,
            p25_return_pct: p25,
            p75_return_pct: p75,
            ci_lower_95,
            ci_upper_95,
            mean_win_rate,
            mean_trade_sharpe: mean_sharpe,
        };

        let comparison = BenchmarkComparison {
            strategy_name: format!("RandomSelection (Monte Carlo N={})", runs),
            total_return_pct: mean_ret,
            trade_level_sharpe: mean_sharpe,
            max_drawdown_pct: mean_drawdown,
            win_rate: mean_win_rate,
            random_distribution: Some(distribution.clone()),
        };

        (comparison, distribution)
    }
}

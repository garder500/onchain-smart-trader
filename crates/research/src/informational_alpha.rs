use crate::types::{
    PermutationDetail, PnLDecomposition, SampleSizeAssessment, StrategyFamily, StrategyFamilyResult,
};
use crate::validation::ScientificValidator;
use chrono::{DateTime, Duration};
use domain::{Trade, TradeSide, WalletAddress};
use rand::prelude::*;
use rand::rngs::StdRng;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};

pub struct InformationalAlphaEngine;

impl InformationalAlphaEngine {
    /// Evaluates the 5 Core Strategy Families on real empirical In-Sample and Out-Of-Sample trade datasets:
    /// Strategy A: Direct Copy (delay spectrum 0s, 1s, 2s, 5s, 10s, 30s, 60s)
    /// Strategy B: Confirmation Window (wait 2s, 5s, 10s, 30s for directional price confirmation)
    /// Strategy C: Smart Wallet Consensus (>= 2 independent smart wallets within delta_t = 5s, 30s, 60s)
    /// Strategy D: Wallet Momentum (accelerating trade count from top persistence wallets)
    /// Strategy E: Token Attention (spike in unique smart wallet appearances)
    pub fn evaluate_all_strategies(
        train_trades: &[Trade],
        oos_trades: &[Trade],
        selected_smart_wallets: &HashSet<String>,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> Vec<StrategyFamilyResult> {
        let mut results = Vec::new();

        // ---------------------------------------------------------------------
        // Strategy A: Direct Copy Baseline
        // ---------------------------------------------------------------------
        let direct_delays = [0u64, 1, 2, 5, 10, 30, 60];
        for (idx, &delay) in direct_delays.iter().enumerate() {
            let res = Self::evaluate_direct_copy(
                train_trades,
                oos_trades,
                selected_smart_wallets,
                delay,
                initial_capital,
                fixed_size_usd,
                fee_bps,
                0x1000 + idx as u64,
            );
            results.push(res);
        }

        // ---------------------------------------------------------------------
        // Strategy B: Market Confirmation
        // ---------------------------------------------------------------------
        let confirm_windows = [2u64, 5, 10, 30];
        for (idx, &window) in confirm_windows.iter().enumerate() {
            let res = Self::evaluate_confirmation_strategy(
                train_trades,
                oos_trades,
                selected_smart_wallets,
                window,
                initial_capital,
                fixed_size_usd,
                fee_bps,
                0x2000 + idx as u64,
            );
            results.push(res);
        }

        // ---------------------------------------------------------------------
        // Strategy C: Smart Wallet Consensus
        // ---------------------------------------------------------------------
        let consensus_windows = [5u64, 30, 60];
        for (idx, &window) in consensus_windows.iter().enumerate() {
            let res = Self::evaluate_consensus_strategy(
                train_trades,
                oos_trades,
                selected_smart_wallets,
                window,
                2, // min 2 wallets
                initial_capital,
                fixed_size_usd,
                fee_bps,
                0x3000 + idx as u64,
            );
            results.push(res);
        }

        // ---------------------------------------------------------------------
        // Strategy D: Wallet Momentum
        // ---------------------------------------------------------------------
        let res_d = Self::evaluate_wallet_momentum(
            train_trades,
            oos_trades,
            selected_smart_wallets,
            300, // 5 min momentum window
            initial_capital,
            fixed_size_usd,
            fee_bps,
            0x4000,
        );
        results.push(res_d);

        // ---------------------------------------------------------------------
        // Strategy E: Token Attention
        // ---------------------------------------------------------------------
        let res_e = Self::evaluate_token_attention(
            train_trades,
            oos_trades,
            selected_smart_wallets,
            3600, // 1 hour window
            initial_capital,
            fixed_size_usd,
            fee_bps,
            0x5000,
        );
        results.push(res_e);

        // Compute Benjamini-Hochberg FDR adjustments across all empirical p-values
        let raw_p_values: Vec<f64> = results.iter().map(|r| r.raw_p_value).collect();
        let adjusted_p_values = ScientificValidator::benjamini_hochberg_correction(&raw_p_values);

        for (i, r) in results.iter_mut().enumerate() {
            r.fdr_adjusted_p_value = adjusted_p_values[i];
            r.is_significant_post_fdr = r.fdr_adjusted_p_value < 0.05
                && r.out_of_sample_sharpe.unwrap_or(Decimal::ZERO) > Decimal::ZERO;
        }

        results
    }

    /// Permutation test helper for strategy families: resamples wallet labels across B iterations
    fn run_strategy_permutation(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        iterations: usize,
        seed: u64,
        observed_sharpe: Decimal,
        eval_fn: impl Fn(&[Trade], &HashSet<String>) -> Option<Decimal>,
    ) -> PermutationDetail {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut extreme_count = 0;
        let mut null_stats = Vec::with_capacity(iterations);

        let all_wallets: Vec<String> = trades
            .iter()
            .map(|t| t.wallet_address.as_str().to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        for _ in 0..iterations {
            let mut permuted_wallets = all_wallets.clone();
            permuted_wallets.shuffle(&mut rng);
            let map: HashMap<&str, &str> = all_wallets
                .iter()
                .zip(&permuted_wallets)
                .map(|(a, b)| (a.as_str(), b.as_str()))
                .collect();

            let permuted_trades: Vec<Trade> = trades
                .iter()
                .map(|t| {
                    let mut copy = t.clone();
                    if let Some(&new_w) = map.get(t.wallet_address.as_str()) {
                        copy.wallet_address = WalletAddress::new(new_w);
                    }
                    copy
                })
                .collect();

            let null_sharpe = eval_fn(&permuted_trades, selected_wallets).unwrap_or(Decimal::ZERO);
            null_stats.push(null_sharpe);
            if null_sharpe >= observed_sharpe {
                extreme_count += 1;
            }
        }

        let null_mean = if !null_stats.is_empty() {
            null_stats.iter().sum::<Decimal>() / Decimal::from(null_stats.len())
        } else {
            Decimal::ZERO
        };
        let null_std = if null_stats.len() > 1 {
            let mean_f = null_mean.to_f64().unwrap_or(0.0);
            let var = null_stats
                .iter()
                .map(|s| {
                    let diff = s.to_f64().unwrap_or(0.0) - mean_f;
                    diff * diff
                })
                .sum::<f64>()
                / ((null_stats.len() - 1) as f64);
            Decimal::from_f64_retain(var.sqrt()).unwrap_or(Decimal::ZERO)
        } else {
            Decimal::ZERO
        };

        let p_value = (extreme_count as f64) / (iterations as f64).max(1.0);

        PermutationDetail {
            null_hypothesis:
                "Wallet labels are independent of trade returns under random assignment".into(),
            observed_statistic: observed_sharpe,
            null_mean,
            null_std,
            p_value,
            iterations,
            seed,
        }
    }

    /// Strategy A: Direct Copy
    #[allow(clippy::too_many_arguments)]
    fn evaluate_direct_copy(
        train_trades: &[Trade],
        oos_trades: &[Trade],
        selected_wallets: &HashSet<String>,
        delay_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
        seed: u64,
    ) -> StrategyFamilyResult {
        let is_metrics = Self::run_direct_copy_slice(
            train_trades,
            selected_wallets,
            delay_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let (oos_metrics, pnl_decomp) = Self::run_direct_copy_slice_detailed(
            oos_trades,
            selected_wallets,
            delay_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let observed_sharpe = oos_metrics.trade_level_sharpe.unwrap_or(Decimal::ZERO);
        let perm_detail = Self::run_strategy_permutation(
            oos_trades,
            selected_wallets,
            50,
            seed,
            observed_sharpe,
            |trades, wallets| {
                Self::run_direct_copy_slice(
                    trades,
                    wallets,
                    delay_seconds,
                    initial_capital,
                    fixed_size_usd,
                    fee_bps,
                )
                .trade_level_sharpe
            },
        );

        let sample_assessment = SampleSizeAssessment::from_count(oos_metrics.total_trades);

        StrategyFamilyResult {
            family: StrategyFamily::DirectCopy,
            name: format!("DirectCopy (delay={}s)", delay_seconds),
            description: format!(
                "Blindly copies smart wallet buy/sell with {}s latency penalty",
                delay_seconds
            ),
            latency_seconds: delay_seconds,
            parameter_variant: format!("delay_{}s", delay_seconds),
            in_sample_sharpe: is_metrics.trade_level_sharpe,
            out_of_sample_sharpe: oos_metrics.trade_level_sharpe,
            net_pnl: oos_metrics.net_pnl,
            win_rate: oos_metrics.win_rate,
            profit_factor: oos_metrics.profit_factor,
            max_drawdown_pct: Decimal::ZERO,
            trades_executed: oos_metrics.total_trades,
            raw_p_value: perm_detail.p_value,
            fdr_adjusted_p_value: perm_detail.p_value,
            is_significant_post_fdr: false,
            sample_size_assessment: sample_assessment,
            pnl_decomposition: Some(pnl_decomp),
            permutation_detail: Some(perm_detail),
        }
    }

    fn run_direct_copy_slice(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        delay_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> crate::types::PerformanceMetrics {
        Self::run_direct_copy_slice_detailed(
            trades,
            selected_wallets,
            delay_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        )
        .0
    }

    fn run_direct_copy_slice_detailed(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        delay_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> (crate::types::PerformanceMetrics, PnLDecomposition) {
        let fee_rate = Decimal::from(fee_bps) / Decimal::from(10000);
        let mut simulated_trades = Vec::new();
        let mut buy_queue: HashMap<String, Vec<(Decimal, Decimal)>> = HashMap::new();
        let mut cash = initial_capital;
        let mut total_latency_cost = Decimal::ZERO;
        let mut total_dex_fees = Decimal::ZERO;
        let mut gross_alpha = Decimal::ZERO;

        for (idx, trade) in trades.iter().enumerate() {
            if !selected_wallets.contains(trade.wallet_address.as_str()) {
                continue;
            }

            let token = trade.token_address.as_str().to_string();

            // Look up empirical execution price at t + delay
            let effective_price = if delay_seconds == 0 {
                trade.price_usd
            } else {
                let target_time = trade.timestamp + Duration::seconds(delay_seconds as i64);
                let next_trade = trades
                    .iter()
                    .skip(idx + 1)
                    .find(|t| t.token_address == trade.token_address && t.timestamp >= target_time);

                match next_trade {
                    Some(next) => next.price_usd,
                    None => {
                        // Unobserved: use current trade price + adverse Uniswap V2 fee slippage
                        let slip = Decimal::from_str("0.003").unwrap();
                        match trade.side {
                            TradeSide::Buy => trade.price_usd * (Decimal::ONE + slip),
                            TradeSide::Sell => trade.price_usd * (Decimal::ONE - slip),
                        }
                    }
                }
            };

            match trade.side {
                TradeSide::Buy => {
                    if cash < fixed_size_usd {
                        continue;
                    }
                    let tokens = if effective_price > Decimal::ZERO {
                        fixed_size_usd / effective_price
                    } else {
                        Decimal::ZERO
                    };
                    cash -= fixed_size_usd;
                    buy_queue
                        .entry(token)
                        .or_default()
                        .push((effective_price, tokens));
                    let mut sim_buy = trade.clone();
                    sim_buy.price_usd = effective_price;
                    sim_buy.amount_tokens = tokens;
                    simulated_trades.push(sim_buy);
                }
                TradeSide::Sell => {
                    if let Some(queue) = buy_queue.get_mut(&token) {
                        if !queue.is_empty() {
                            let (entry_price, tokens) = queue.remove(0);
                            let gross_proceeds = effective_price * tokens;
                            let cost_basis = entry_price * tokens;
                            let fees = (gross_proceeds + cost_basis) * fee_rate;
                            total_dex_fees += fees;

                            let ideal_proceeds = trade.price_usd * tokens;
                            gross_alpha += ideal_proceeds - cost_basis;
                            let slip_loss = (ideal_proceeds - gross_proceeds).max(Decimal::ZERO);
                            total_latency_cost += slip_loss;

                            cash += gross_proceeds - fees;

                            let mut sim_trade = trade.clone();
                            sim_trade.price_usd = effective_price;
                            simulated_trades.push(sim_trade);
                        }
                    }
                }
            }
        }

        let m = ScientificValidator::evaluate_slice(&simulated_trades);
        let pnl_decomp = PnLDecomposition {
            gross_alpha,
            latency_cost: total_latency_cost,
            market_impact: Decimal::ZERO,
            dex_fees: total_dex_fees,
            gas_cost: Decimal::ZERO,
            net_alpha: m.net_pnl,
        };

        (m, pnl_decomp)
    }

    /// Strategy B: Confirmation Window
    #[allow(clippy::too_many_arguments)]
    fn evaluate_confirmation_strategy(
        train_trades: &[Trade],
        oos_trades: &[Trade],
        selected_wallets: &HashSet<String>,
        window_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
        seed: u64,
    ) -> StrategyFamilyResult {
        let is_metrics = Self::run_confirmation_slice(
            train_trades,
            selected_wallets,
            window_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let (oos_metrics, pnl_decomp) = Self::run_confirmation_slice_detailed(
            oos_trades,
            selected_wallets,
            window_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let observed_sharpe = oos_metrics.trade_level_sharpe.unwrap_or(Decimal::ZERO);
        let perm_detail = Self::run_strategy_permutation(
            oos_trades,
            selected_wallets,
            50,
            seed,
            observed_sharpe,
            |trades, wallets| {
                Self::run_confirmation_slice(
                    trades,
                    wallets,
                    window_seconds,
                    initial_capital,
                    fixed_size_usd,
                    fee_bps,
                )
                .trade_level_sharpe
            },
        );

        let sample_assessment = SampleSizeAssessment::from_count(oos_metrics.total_trades);

        StrategyFamilyResult {
            family: StrategyFamily::Confirmation,
            name: format!("Confirmation Window (window={}s)", window_seconds),
            description: format!(
                "Enters only if price continues moving up within {}s of smart buy",
                window_seconds
            ),
            latency_seconds: window_seconds,
            parameter_variant: format!("confirm_{}s", window_seconds),
            in_sample_sharpe: is_metrics.trade_level_sharpe,
            out_of_sample_sharpe: oos_metrics.trade_level_sharpe,
            net_pnl: oos_metrics.net_pnl,
            win_rate: oos_metrics.win_rate,
            profit_factor: oos_metrics.profit_factor,
            max_drawdown_pct: Decimal::ZERO,
            trades_executed: oos_metrics.total_trades,
            raw_p_value: perm_detail.p_value,
            fdr_adjusted_p_value: perm_detail.p_value,
            is_significant_post_fdr: false,
            sample_size_assessment: sample_assessment,
            pnl_decomposition: Some(pnl_decomp),
            permutation_detail: Some(perm_detail),
        }
    }

    fn run_confirmation_slice(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        window_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> crate::types::PerformanceMetrics {
        Self::run_confirmation_slice_detailed(
            trades,
            selected_wallets,
            window_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        )
        .0
    }

    fn run_confirmation_slice_detailed(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        window_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> (crate::types::PerformanceMetrics, PnLDecomposition) {
        let fee_rate = Decimal::from(fee_bps) / Decimal::from(10000);
        let mut buy_signals = Vec::new();
        let mut simulated_trades = Vec::new();
        let mut buy_queue: HashMap<String, Vec<(Decimal, Decimal)>> = HashMap::new();
        let mut cash = initial_capital;
        let mut total_dex_fees = Decimal::ZERO;
        let mut gross_alpha = Decimal::ZERO;

        for trade in trades {
            if selected_wallets.contains(trade.wallet_address.as_str())
                && trade.side == TradeSide::Buy
            {
                buy_signals.push(trade.clone());
            }

            for sig in &buy_signals {
                if sig.token_address == trade.token_address
                    && trade.timestamp > sig.timestamp
                    && trade.timestamp <= sig.timestamp + Duration::seconds(window_seconds as i64)
                    && trade.price_usd > sig.price_usd
                    && cash >= fixed_size_usd
                {
                    let tokens = if trade.price_usd > Decimal::ZERO {
                        fixed_size_usd / trade.price_usd
                    } else {
                        Decimal::ZERO
                    };
                    cash -= fixed_size_usd;
                    buy_queue
                        .entry(sig.token_address.as_str().to_string())
                        .or_default()
                        .push((trade.price_usd, tokens));
                    simulated_trades.push(trade.clone());
                }
            }

            if trade.side == TradeSide::Sell {
                let token = trade.token_address.as_str().to_string();
                if let Some(queue) = buy_queue.get_mut(&token) {
                    if !queue.is_empty() {
                        let (entry_price, tokens) = queue.remove(0);
                        let gross_proceeds = trade.price_usd * tokens;
                        let cost_basis = entry_price * tokens;
                        let fees = (gross_proceeds + cost_basis) * fee_rate;
                        total_dex_fees += fees;
                        gross_alpha += gross_proceeds - cost_basis;

                        cash += gross_proceeds - fees;
                        simulated_trades.push(trade.clone());
                    }
                }
            }
        }

        let m = ScientificValidator::evaluate_slice(&simulated_trades);
        let pnl_decomp = PnLDecomposition {
            gross_alpha,
            latency_cost: Decimal::ZERO,
            market_impact: Decimal::ZERO,
            dex_fees: total_dex_fees,
            gas_cost: Decimal::ZERO,
            net_alpha: m.net_pnl,
        };

        (m, pnl_decomp)
    }

    /// Strategy C: Smart Wallet Consensus
    #[allow(clippy::too_many_arguments)]
    fn evaluate_consensus_strategy(
        train_trades: &[Trade],
        oos_trades: &[Trade],
        selected_wallets: &HashSet<String>,
        window_seconds: u64,
        min_wallets: usize,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
        seed: u64,
    ) -> StrategyFamilyResult {
        let is_metrics = Self::run_consensus_slice(
            train_trades,
            selected_wallets,
            window_seconds,
            min_wallets,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let (oos_metrics, pnl_decomp) = Self::run_consensus_slice_detailed(
            oos_trades,
            selected_wallets,
            window_seconds,
            min_wallets,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let observed_sharpe = oos_metrics.trade_level_sharpe.unwrap_or(Decimal::ZERO);
        let perm_detail = Self::run_strategy_permutation(
            oos_trades,
            selected_wallets,
            50,
            seed,
            observed_sharpe,
            |trades, wallets| {
                Self::run_consensus_slice(
                    trades,
                    wallets,
                    window_seconds,
                    min_wallets,
                    initial_capital,
                    fixed_size_usd,
                    fee_bps,
                )
                .trade_level_sharpe
            },
        );

        let sample_assessment = SampleSizeAssessment::from_count(oos_metrics.total_trades);

        StrategyFamilyResult {
            family: StrategyFamily::Consensus,
            name: format!(
                "Consensus (K={} wallets, window={}s)",
                min_wallets, window_seconds
            ),
            description: format!(
                "Buys only when >= {} distinct smart wallets accumulate within {}s",
                min_wallets, window_seconds
            ),
            latency_seconds: window_seconds,
            parameter_variant: format!("consensus_k{}_w{}s", min_wallets, window_seconds),
            in_sample_sharpe: is_metrics.trade_level_sharpe,
            out_of_sample_sharpe: oos_metrics.trade_level_sharpe,
            net_pnl: oos_metrics.net_pnl,
            win_rate: oos_metrics.win_rate,
            profit_factor: oos_metrics.profit_factor,
            max_drawdown_pct: Decimal::ZERO,
            trades_executed: oos_metrics.total_trades,
            raw_p_value: perm_detail.p_value,
            fdr_adjusted_p_value: perm_detail.p_value,
            is_significant_post_fdr: false,
            sample_size_assessment: sample_assessment,
            pnl_decomposition: Some(pnl_decomp),
            permutation_detail: Some(perm_detail),
        }
    }

    fn run_consensus_slice(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        window_seconds: u64,
        min_wallets: usize,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> crate::types::PerformanceMetrics {
        Self::run_consensus_slice_detailed(
            trades,
            selected_wallets,
            window_seconds,
            min_wallets,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        )
        .0
    }

    fn run_consensus_slice_detailed(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        window_seconds: u64,
        min_wallets: usize,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> (crate::types::PerformanceMetrics, PnLDecomposition) {
        let fee_rate = Decimal::from(fee_bps) / Decimal::from(10000);
        let mut smart_buys: Vec<Trade> = Vec::new();
        let mut buy_queue: HashMap<String, Vec<(Decimal, Decimal)>> = HashMap::new();
        let mut simulated_trades = Vec::new();
        let mut cash = initial_capital;
        let mut total_dex_fees = Decimal::ZERO;
        let mut gross_alpha = Decimal::ZERO;

        for trade in trades {
            if selected_wallets.contains(trade.wallet_address.as_str())
                && trade.side == TradeSide::Buy
            {
                smart_buys.push(trade.clone());

                let cutoff = trade.timestamp - Duration::seconds(window_seconds as i64);
                let recent_unique_wallets: HashSet<&str> = smart_buys
                    .iter()
                    .filter(|b| b.token_address == trade.token_address && b.timestamp >= cutoff)
                    .map(|b| b.wallet_address.as_str())
                    .collect();

                if recent_unique_wallets.len() >= min_wallets && cash >= fixed_size_usd {
                    let tokens = if trade.price_usd > Decimal::ZERO {
                        fixed_size_usd / trade.price_usd
                    } else {
                        Decimal::ZERO
                    };
                    cash -= fixed_size_usd;
                    buy_queue
                        .entry(trade.token_address.as_str().to_string())
                        .or_default()
                        .push((trade.price_usd, tokens));
                    simulated_trades.push(trade.clone());
                }
            }

            if trade.side == TradeSide::Sell {
                let token = trade.token_address.as_str().to_string();
                if let Some(queue) = buy_queue.get_mut(&token) {
                    if !queue.is_empty() {
                        let (entry_price, tokens) = queue.remove(0);
                        let gross_proceeds = trade.price_usd * tokens;
                        let cost_basis = entry_price * tokens;
                        let fees = (gross_proceeds + cost_basis) * fee_rate;
                        total_dex_fees += fees;
                        gross_alpha += gross_proceeds - cost_basis;

                        cash += gross_proceeds - fees;
                        simulated_trades.push(trade.clone());
                    }
                }
            }
        }

        let m = ScientificValidator::evaluate_slice(&simulated_trades);
        let pnl_decomp = PnLDecomposition {
            gross_alpha,
            latency_cost: Decimal::ZERO,
            market_impact: Decimal::ZERO,
            dex_fees: total_dex_fees,
            gas_cost: Decimal::ZERO,
            net_alpha: m.net_pnl,
        };

        (m, pnl_decomp)
    }

    /// Strategy D: Wallet Momentum
    #[allow(clippy::too_many_arguments)]
    fn evaluate_wallet_momentum(
        train_trades: &[Trade],
        oos_trades: &[Trade],
        selected_wallets: &HashSet<String>,
        momentum_window_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
        seed: u64,
    ) -> StrategyFamilyResult {
        let is_metrics = Self::run_wallet_momentum_slice(
            train_trades,
            selected_wallets,
            momentum_window_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let (oos_metrics, pnl_decomp) = Self::run_wallet_momentum_slice_detailed(
            oos_trades,
            selected_wallets,
            momentum_window_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let observed_sharpe = oos_metrics.trade_level_sharpe.unwrap_or(Decimal::ZERO);
        let perm_detail = Self::run_strategy_permutation(
            oos_trades,
            selected_wallets,
            50,
            seed,
            observed_sharpe,
            |trades, wallets| {
                Self::run_wallet_momentum_slice(
                    trades,
                    wallets,
                    momentum_window_seconds,
                    initial_capital,
                    fixed_size_usd,
                    fee_bps,
                )
                .trade_level_sharpe
            },
        );

        let sample_assessment = SampleSizeAssessment::from_count(oos_metrics.total_trades);

        StrategyFamilyResult {
            family: StrategyFamily::WalletMomentum,
            name: "Wallet Momentum (Consecutive Accumulation)".into(),
            description: "Follows top wallets entering a token >= 2 times within 5 minutes".into(),
            latency_seconds: 5,
            parameter_variant: "consecutive_accum_300s".into(),
            in_sample_sharpe: is_metrics.trade_level_sharpe,
            out_of_sample_sharpe: oos_metrics.trade_level_sharpe,
            net_pnl: oos_metrics.net_pnl,
            win_rate: oos_metrics.win_rate,
            profit_factor: oos_metrics.profit_factor,
            max_drawdown_pct: Decimal::ZERO,
            trades_executed: oos_metrics.total_trades,
            raw_p_value: perm_detail.p_value,
            fdr_adjusted_p_value: perm_detail.p_value,
            is_significant_post_fdr: false,
            sample_size_assessment: sample_assessment,
            pnl_decomposition: Some(pnl_decomp),
            permutation_detail: Some(perm_detail),
        }
    }

    fn run_wallet_momentum_slice(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        momentum_window_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> crate::types::PerformanceMetrics {
        Self::run_wallet_momentum_slice_detailed(
            trades,
            selected_wallets,
            momentum_window_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        )
        .0
    }

    fn run_wallet_momentum_slice_detailed(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        momentum_window_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> (crate::types::PerformanceMetrics, PnLDecomposition) {
        let fee_rate = Decimal::from(fee_bps) / Decimal::from(10000);
        let mut last_buy_per_wallet_token: HashMap<(String, String), Vec<DateTime<chrono::Utc>>> =
            HashMap::new();
        let mut buy_queue: HashMap<String, Vec<(Decimal, Decimal)>> = HashMap::new();
        let mut simulated_trades = Vec::new();
        let mut cash = initial_capital;
        let mut total_dex_fees = Decimal::ZERO;
        let mut gross_alpha = Decimal::ZERO;

        for trade in trades {
            if selected_wallets.contains(trade.wallet_address.as_str())
                && trade.side == TradeSide::Buy
            {
                let key = (
                    trade.wallet_address.as_str().to_string(),
                    trade.token_address.as_str().to_string(),
                );
                let history = last_buy_per_wallet_token.entry(key).or_default();
                history.push(trade.timestamp);

                let cutoff = trade.timestamp - Duration::seconds(momentum_window_seconds as i64);
                let count = history.iter().filter(|&&t| t >= cutoff).count();

                if count >= 2 && cash >= fixed_size_usd {
                    let tokens = if trade.price_usd > Decimal::ZERO {
                        fixed_size_usd / trade.price_usd
                    } else {
                        Decimal::ZERO
                    };
                    cash -= fixed_size_usd;
                    buy_queue
                        .entry(trade.token_address.as_str().to_string())
                        .or_default()
                        .push((trade.price_usd, tokens));
                    simulated_trades.push(trade.clone());
                }
            }

            if trade.side == TradeSide::Sell {
                let token = trade.token_address.as_str().to_string();
                if let Some(queue) = buy_queue.get_mut(&token) {
                    if !queue.is_empty() {
                        let (entry_price, tokens) = queue.remove(0);
                        let gross_proceeds = trade.price_usd * tokens;
                        let cost_basis = entry_price * tokens;
                        let fees = (gross_proceeds + cost_basis) * fee_rate;
                        total_dex_fees += fees;
                        gross_alpha += gross_proceeds - cost_basis;

                        cash += gross_proceeds - fees;
                        simulated_trades.push(trade.clone());
                    }
                }
            }
        }

        let m = ScientificValidator::evaluate_slice(&simulated_trades);
        let pnl_decomp = PnLDecomposition {
            gross_alpha,
            latency_cost: Decimal::ZERO,
            market_impact: Decimal::ZERO,
            dex_fees: total_dex_fees,
            gas_cost: Decimal::ZERO,
            net_alpha: m.net_pnl,
        };

        (m, pnl_decomp)
    }

    /// Strategy E: Token Attention
    #[allow(clippy::too_many_arguments)]
    fn evaluate_token_attention(
        train_trades: &[Trade],
        oos_trades: &[Trade],
        selected_wallets: &HashSet<String>,
        velocity_window_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
        seed: u64,
    ) -> StrategyFamilyResult {
        let is_metrics = Self::run_token_attention_slice(
            train_trades,
            selected_wallets,
            velocity_window_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let (oos_metrics, pnl_decomp) = Self::run_token_attention_slice_detailed(
            oos_trades,
            selected_wallets,
            velocity_window_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        );

        let observed_sharpe = oos_metrics.trade_level_sharpe.unwrap_or(Decimal::ZERO);
        let perm_detail = Self::run_strategy_permutation(
            oos_trades,
            selected_wallets,
            50,
            seed,
            observed_sharpe,
            |trades, wallets| {
                Self::run_token_attention_slice(
                    trades,
                    wallets,
                    velocity_window_seconds,
                    initial_capital,
                    fixed_size_usd,
                    fee_bps,
                )
                .trade_level_sharpe
            },
        );

        let sample_assessment = SampleSizeAssessment::from_count(oos_metrics.total_trades);

        StrategyFamilyResult {
            family: StrategyFamily::TokenAttention,
            name: "Token Attention (Smart Wallet Cluster Velocity)".into(),
            description: "Enters when unique smart wallet inflow velocity accelerates > 2 in 1h"
                .into(),
            latency_seconds: 60,
            parameter_variant: "cluster_velocity_3600s".into(),
            in_sample_sharpe: is_metrics.trade_level_sharpe,
            out_of_sample_sharpe: oos_metrics.trade_level_sharpe,
            net_pnl: oos_metrics.net_pnl,
            win_rate: oos_metrics.win_rate,
            profit_factor: oos_metrics.profit_factor,
            max_drawdown_pct: Decimal::ZERO,
            trades_executed: oos_metrics.total_trades,
            raw_p_value: perm_detail.p_value,
            fdr_adjusted_p_value: perm_detail.p_value,
            is_significant_post_fdr: false,
            sample_size_assessment: sample_assessment,
            pnl_decomposition: Some(pnl_decomp),
            permutation_detail: Some(perm_detail),
        }
    }

    fn run_token_attention_slice(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        velocity_window_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> crate::types::PerformanceMetrics {
        Self::run_token_attention_slice_detailed(
            trades,
            selected_wallets,
            velocity_window_seconds,
            initial_capital,
            fixed_size_usd,
            fee_bps,
        )
        .0
    }

    fn run_token_attention_slice_detailed(
        trades: &[Trade],
        selected_wallets: &HashSet<String>,
        velocity_window_seconds: u64,
        initial_capital: Decimal,
        fixed_size_usd: Decimal,
        fee_bps: i64,
    ) -> (crate::types::PerformanceMetrics, PnLDecomposition) {
        let fee_rate = Decimal::from(fee_bps) / Decimal::from(10000);
        let mut token_entries: HashMap<String, Vec<(String, DateTime<chrono::Utc>)>> =
            HashMap::new();
        let mut buy_queue: HashMap<String, Vec<(Decimal, Decimal)>> = HashMap::new();
        let mut simulated_trades = Vec::new();
        let mut cash = initial_capital;
        let mut total_dex_fees = Decimal::ZERO;
        let mut gross_alpha = Decimal::ZERO;

        for trade in trades {
            if selected_wallets.contains(trade.wallet_address.as_str())
                && trade.side == TradeSide::Buy
            {
                let token = trade.token_address.as_str().to_string();
                let history = token_entries.entry(token.clone()).or_default();
                history.push((trade.wallet_address.as_str().to_string(), trade.timestamp));

                let cutoff = trade.timestamp - Duration::seconds(velocity_window_seconds as i64);
                let recent_unique: HashSet<&str> = history
                    .iter()
                    .filter(|(_, t)| *t >= cutoff)
                    .map(|(w, _)| w.as_str())
                    .collect();

                if recent_unique.len() >= 3 && cash >= fixed_size_usd {
                    let tokens = if trade.price_usd > Decimal::ZERO {
                        fixed_size_usd / trade.price_usd
                    } else {
                        Decimal::ZERO
                    };
                    cash -= fixed_size_usd;
                    buy_queue
                        .entry(token)
                        .or_default()
                        .push((trade.price_usd, tokens));
                    simulated_trades.push(trade.clone());
                }
            }

            if trade.side == TradeSide::Sell {
                let token = trade.token_address.as_str().to_string();
                if let Some(queue) = buy_queue.get_mut(&token) {
                    if !queue.is_empty() {
                        let (entry_price, tokens) = queue.remove(0);
                        let gross_proceeds = trade.price_usd * tokens;
                        let cost_basis = entry_price * tokens;
                        let fees = (gross_proceeds + cost_basis) * fee_rate;
                        total_dex_fees += fees;
                        gross_alpha += gross_proceeds - cost_basis;

                        cash += gross_proceeds - fees;
                        simulated_trades.push(trade.clone());
                    }
                }
            }
        }

        let m = ScientificValidator::evaluate_slice(&simulated_trades);
        let pnl_decomp = PnLDecomposition {
            gross_alpha,
            latency_cost: Decimal::ZERO,
            market_impact: Decimal::ZERO,
            dex_fees: total_dex_fees,
            gas_cost: Decimal::ZERO,
            net_alpha: m.net_pnl,
        };

        (m, pnl_decomp)
    }
}

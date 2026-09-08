use crate::types::{DelayImpactPoint, EmpiricalLatencyPoint, LatencyMode};
use chrono::Duration;
use domain::{Trade, TradeSide};
use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct CopiabilityEngine;

impl CopiabilityEngine {
    /// Simulates the impact of execution latency on copy trading returns across a delay spectrum,
    /// enforcing finite capital budget and open position limits (no infinite cash).
    pub fn evaluate_latency_matrix(
        trades: &[Trade],
        delays: &[u64],
        fixed_capital_per_trade: Decimal,
        initial_cash: Decimal,
        max_open_positions: usize,
        fee_bps: i64,
        latency_mode: LatencyMode,
    ) -> Vec<DelayImpactPoint> {
        let mut results = Vec::new();

        // Baseline theoretical pnl at 0s latency
        let baseline_pnl = Self::simulate_copy_at_delay(
            trades,
            0,
            fixed_capital_per_trade,
            initial_cash,
            max_open_positions,
            fee_bps,
            LatencyMode::Empirical,
        )
        .0;

        for &delay in delays {
            let (net_pnl, win_rate, count, slippage_usd) = Self::simulate_copy_at_delay(
                trades,
                delay,
                fixed_capital_per_trade,
                initial_cash,
                max_open_positions,
                fee_bps,
                latency_mode,
            );

            let copy_efficiency = if baseline_pnl > Decimal::ZERO {
                (net_pnl / baseline_pnl)
                    .max(Decimal::from(-5))
                    .min(Decimal::from(5))
            } else if net_pnl > Decimal::ZERO {
                Decimal::ONE
            } else {
                Decimal::ZERO
            };

            results.push(DelayImpactPoint {
                delay_seconds: delay,
                mode: latency_mode,
                net_pnl,
                win_rate,
                copy_efficiency,
                trades_executed: count,
                slippage_incurred_usd: slippage_usd,
            });
        }

        results
    }

    /// Measures the empirical price change observed at subsequent real trade timestamps.
    /// If no trade occurred in the window, explicitly marks as NO_OBSERVATION.
    pub fn measure_empirical_latency_distribution(
        trades: &[Trade],
        delays: &[u64],
    ) -> Vec<EmpiricalLatencyPoint> {
        let mut points = Vec::new();

        for &d in delays {
            let mut deltas_bps = Vec::new();
            let mut elapsed_secs = Vec::new();
            let mut prices = Vec::new();

            for (i, t_i) in trades.iter().enumerate() {
                let target_time = t_i.timestamp + Duration::seconds(d as i64);

                // Look forward for subsequent trade in same token
                for t_j in trades.iter().skip(i + 1) {
                    if t_j.token_address == t_i.token_address && t_j.timestamp >= target_time {
                        let elapsed = (t_j.timestamp - t_i.timestamp).num_seconds();
                        if elapsed <= (d as i64 + 3600) && t_i.price_usd > Decimal::ZERO {
                            let ratio = t_j.price_usd / t_i.price_usd;
                            if ratio > Decimal::new(1, 1) && ratio < Decimal::from(10) {
                                let delta_bps = ((t_j.price_usd - t_i.price_usd) / t_i.price_usd)
                                    * Decimal::from(10000);
                                deltas_bps.push(delta_bps);
                                elapsed_secs.push(elapsed as f64);
                                prices.push(t_j.price_usd);
                            }
                        }
                        break;
                    }
                }
            }

            let n = deltas_bps.len();
            if n > 0 {
                let mean_delta = deltas_bps.iter().sum::<Decimal>() / Decimal::from(n);
                let mean_elapsed = elapsed_secs.iter().sum::<f64>() / (n as f64);
                let mean_price = prices.iter().sum::<Decimal>() / Decimal::from(n);

                points.push(EmpiricalLatencyPoint {
                    target_delay_seconds: d,
                    actual_elapsed_seconds: Some(mean_elapsed),
                    observed_price: Some(mean_price),
                    price_delta_bps: Some(mean_delta),
                    sample_size: n,
                    status: "EMPIRICAL_OBSERVATION".into(),
                });
            } else {
                points.push(EmpiricalLatencyPoint {
                    target_delay_seconds: d,
                    actual_elapsed_seconds: None,
                    observed_price: None,
                    price_delta_bps: None,
                    sample_size: 0,
                    status: "NO_OBSERVATION".into(),
                });
            }
        }

        points
    }

    fn simulate_copy_at_delay(
        trades: &[Trade],
        delay_seconds: u64,
        capital_per_trade: Decimal,
        initial_cash: Decimal,
        max_open_positions: usize,
        fee_bps: i64,
        latency_mode: LatencyMode,
    ) -> (Decimal, Decimal, usize, Decimal) {
        let mut cash = initial_cash;
        let mut buy_queue: HashMap<String, Vec<(Decimal, Decimal)>> = HashMap::new(); // (entry_price, tokens)
        let mut net_pnl = Decimal::ZERO;
        let mut wins = 0usize;
        let mut total_closed = 0usize;
        let mut total_slippage_usd = Decimal::ZERO;

        let fee_rate = Decimal::from(fee_bps) / Decimal::from(10000);

        for (idx, trade) in trades.iter().enumerate() {
            let token = trade.token_address.as_str().to_string();

            // Determine effective execution price
            let (effective_price, slippage_loss) = if delay_seconds == 0 {
                (trade.price_usd, Decimal::ZERO)
            } else if latency_mode == LatencyMode::Empirical {
                // Empirical price lookup: find next trade in the same pool occurring at or after delay
                let target_time = trade.timestamp + Duration::seconds(delay_seconds as i64);
                let next_subsequent_trade = trades
                    .iter()
                    .skip(idx + 1)
                    .find(|t| t.token_address == trade.token_address && t.timestamp >= target_time);

                match next_subsequent_trade {
                    Some(next) => {
                        if trade.price_usd <= Decimal::ZERO {
                            continue;
                        }
                        let ratio = next.price_usd / trade.price_usd;
                        if ratio < Decimal::new(1, 1) || ratio > Decimal::from(10) {
                            continue;
                        }
                        let diff = match trade.side {
                            TradeSide::Buy => next.price_usd - trade.price_usd,
                            TradeSide::Sell => trade.price_usd - next.price_usd,
                        };
                        let slip_cost =
                            (diff.max(Decimal::ZERO) / trade.price_usd) * capital_per_trade;
                        (next.price_usd, slip_cost)
                    }
                    None => {
                        // No subsequent trade observed in empirical dataset: skip trade
                        continue;
                    }
                }
            } else {
                // Stress test analytical formula
                let delay_f = delay_seconds as f64;
                let slip_f = (0.008 * delay_f.sqrt() + 0.001).min(0.25);
                let slip_rate = Decimal::from_f64_retain(slip_f).unwrap_or(Decimal::ZERO);
                let price = match trade.side {
                    TradeSide::Buy => trade.price_usd * (Decimal::ONE + slip_rate),
                    TradeSide::Sell => trade.price_usd * (Decimal::ONE - slip_rate),
                };
                let slip_cost =
                    (price - trade.price_usd).abs() * (capital_per_trade / price.max(Decimal::ONE));
                (price, slip_cost)
            };

            match trade.side {
                TradeSide::Buy => {
                    let current_open_positions: usize =
                        buy_queue.values().map(|lots| lots.len()).sum();
                    if cash < capital_per_trade || current_open_positions >= max_open_positions {
                        continue;
                    }

                    total_slippage_usd += slippage_loss;
                    let tokens_bought = if effective_price > Decimal::ZERO {
                        capital_per_trade / effective_price
                    } else {
                        Decimal::ZERO
                    };

                    cash -= capital_per_trade;

                    buy_queue
                        .entry(token)
                        .or_default()
                        .push((effective_price, tokens_bought));
                }
                TradeSide::Sell => {
                    if let Some(queue) = buy_queue.get_mut(&token) {
                        if !queue.is_empty() {
                            let (entry_price, tokens) = queue.remove(0);
                            let gross_proceeds = effective_price * tokens;
                            let cost_basis = entry_price * tokens;
                            let fees = (gross_proceeds + cost_basis) * fee_rate;
                            let trade_pnl = gross_proceeds - cost_basis - fees;

                            cash += gross_proceeds - fees;
                            net_pnl += trade_pnl;
                            total_closed += 1;
                            if trade_pnl > Decimal::ZERO {
                                wins += 1;
                            }
                        }
                    }
                }
            }
        }

        let win_rate = if total_closed > 0 {
            Decimal::from(wins) / Decimal::from(total_closed)
        } else {
            Decimal::ZERO
        };

        (net_pnl, win_rate, total_closed, total_slippage_usd)
    }
}

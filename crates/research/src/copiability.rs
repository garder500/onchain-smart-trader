use crate::types::DelayImpactPoint;
use domain::{Trade, TradeSide};
use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct CopiabilityEngine;

impl CopiabilityEngine {
    /// Simulates the impact of execution latency on copy trading returns across a delay spectrum.
    pub fn evaluate_latency_matrix(
        trades: &[Trade],
        delays: &[u64],
        fixed_capital_per_trade: Decimal,
        fee_bps: i64,
    ) -> Vec<DelayImpactPoint> {
        let mut results = Vec::new();

        // Baseline theoretical pnl at 0s latency
        let baseline_pnl =
            Self::simulate_copy_at_delay(trades, 0, fixed_capital_per_trade, fee_bps).0;

        for &delay in delays {
            let (net_pnl, win_rate, count, slippage_usd) =
                Self::simulate_copy_at_delay(trades, delay, fixed_capital_per_trade, fee_bps);

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
                net_pnl,
                win_rate,
                copy_efficiency,
                trades_executed: count,
                slippage_incurred_usd: slippage_usd,
            });
        }

        results
    }

    fn simulate_copy_at_delay(
        trades: &[Trade],
        delay_seconds: u64,
        capital_per_trade: Decimal,
        fee_bps: i64,
    ) -> (Decimal, Decimal, usize, Decimal) {
        let mut buy_queue: HashMap<String, Vec<(Decimal, Decimal)>> = HashMap::new(); // (entry_price, tokens)
        let mut net_pnl = Decimal::ZERO;
        let mut wins = 0usize;
        let mut total_closed = 0usize;
        let mut total_slippage_usd = Decimal::ZERO;

        let fee_rate = Decimal::from(fee_bps) / Decimal::from(10000);

        // Adverse slippage per delay: delta_p = min(25%, 0.008 * sqrt(delay) + 0.001)
        let adverse_slippage_rate = if delay_seconds == 0 {
            Decimal::ZERO
        } else {
            let delay_f = delay_seconds as f64;
            let slip_f = (0.008 * delay_f.sqrt() + 0.001).min(0.25);
            Decimal::from_f64_retain(slip_f).unwrap_or(Decimal::ZERO)
        };

        for trade in trades {
            let token = trade.token_address.as_str().to_string();

            match trade.side {
                TradeSide::Buy => {
                    // Entry price degrades higher (worse) due to front-runners
                    let effective_buy_price =
                        trade.price_usd * (Decimal::ONE + adverse_slippage_rate);
                    let slippage_loss = (effective_buy_price - trade.price_usd)
                        * (capital_per_trade / effective_buy_price);
                    total_slippage_usd += slippage_loss.max(Decimal::ZERO);

                    let tokens_bought = if effective_buy_price > Decimal::ZERO {
                        capital_per_trade / effective_buy_price
                    } else {
                        Decimal::ZERO
                    };

                    buy_queue
                        .entry(token)
                        .or_default()
                        .push((effective_buy_price, tokens_bought));
                }
                TradeSide::Sell => {
                    // Exit price degrades lower (worse) due to copy delay
                    let effective_sell_price =
                        trade.price_usd * (Decimal::ONE - adverse_slippage_rate);

                    if let Some(queue) = buy_queue.get_mut(&token) {
                        if !queue.is_empty() {
                            let (entry_price, tokens) = queue.remove(0);
                            let gross_proceeds = effective_sell_price * tokens;
                            let cost_basis = entry_price * tokens;
                            let fees = (gross_proceeds + cost_basis) * fee_rate;
                            let trade_pnl = gross_proceeds - cost_basis - fees;

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

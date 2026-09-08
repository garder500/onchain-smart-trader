use crate::types::ScalabilityImpactPoint;
use domain::{Trade, TradeSide};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub struct ScalabilityEngine;

impl ScalabilityEngine {
    /// Evaluates returns across varying capital allocations using constant-product AMM impact.
    pub fn evaluate_scalability(
        trades: &[Trade],
        capitals: &[Decimal],
        assumed_pool_liquidity: Decimal,
        fee_bps: i64,
    ) -> Vec<ScalabilityImpactPoint> {
        let mut points = Vec::new();
        let fee_rate = Decimal::from(fee_bps) / Decimal::from(10000);

        for &capital in capitals {
            // Price impact = capital / (pool_liquidity + capital)
            let impact_rate = if assumed_pool_liquidity > Decimal::ZERO {
                capital / (assumed_pool_liquidity + capital)
            } else {
                Decimal::from_str("0.10").unwrap()
            };

            let impact_bps = impact_rate * Decimal::from(10000);

            let mut buy_queue: HashMap<String, Vec<(Decimal, Decimal)>> = HashMap::new();
            let mut total_net_pnl = Decimal::ZERO;
            let mut total_capital_deployed = Decimal::ZERO;

            for trade in trades {
                let token = trade.token_address.as_str().to_string();

                match trade.side {
                    TradeSide::Buy => {
                        let executed_price = trade.price_usd * (Decimal::ONE + impact_rate);
                        let tokens = if executed_price > Decimal::ZERO {
                            capital / executed_price
                        } else {
                            Decimal::ZERO
                        };
                        total_capital_deployed += capital;
                        buy_queue
                            .entry(token)
                            .or_default()
                            .push((executed_price, tokens));
                    }
                    TradeSide::Sell => {
                        let executed_price = trade.price_usd * (Decimal::ONE - impact_rate);
                        if let Some(queue) = buy_queue.get_mut(&token) {
                            if !queue.is_empty() {
                                let (entry_price, tokens) = queue.remove(0);
                                let proceeds = executed_price * tokens;
                                let cost = entry_price * tokens;
                                let fees = (proceeds + cost) * fee_rate;
                                total_net_pnl += proceeds - cost - fees;
                            }
                        }
                    }
                }
            }

            let return_pct = if total_capital_deployed > Decimal::ZERO {
                total_net_pnl / total_capital_deployed
            } else {
                Decimal::ZERO
            };

            let capacity_exhausted = return_pct <= Decimal::ZERO;

            points.push(ScalabilityImpactPoint {
                capital_usd: capital,
                net_pnl: total_net_pnl,
                return_pct,
                avg_price_impact_bps: impact_bps,
                capacity_exhausted,
            });
        }

        points
    }

    /// Computes the maximum scalable capital before slippage exhausts profitability.
    pub fn compute_maximum_scalable_capital(points: &[ScalabilityImpactPoint]) -> Decimal {
        let mut max_profitable = Decimal::ZERO;
        for p in points {
            if p.net_pnl > Decimal::ZERO {
                max_profitable = p.capital_usd;
            } else {
                break; // Stop at first unprofitable capital tier
            }
        }
        max_profitable
    }
}

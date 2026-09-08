use chrono::{DateTime, Utc};
use domain::{Trade, TradeSide, WalletMetrics};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RoundTripTrade {
    pub token_address: String,
    pub buy_price: Decimal,
    pub sell_price: Decimal,
    pub quantity: Decimal,
    pub pnl_usd: Decimal,
    pub return_pct: Decimal,
    pub holding_seconds: u64,
    pub buy_time: DateTime<Utc>,
    pub sell_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct BuyLot {
    buy_price: Decimal,
    remaining_qty: Decimal,
    buy_time: DateTime<Utc>,
    fee_per_unit: Decimal,
}

pub struct MetricsCalculator;

impl MetricsCalculator {
    /// Reconstructs closed round-trip trades using FIFO matching on historical trades
    /// strictly <= eval_timestamp (anti-look-ahead compliant).
    pub fn compute_metrics(
        trades: &[Trade],
        eval_timestamp: DateTime<Utc>,
        min_early_entry_window_secs: i64,
    ) -> WalletMetrics {
        Self::compute_metrics_with_market_prices(
            trades,
            eval_timestamp,
            min_early_entry_window_secs,
            None,
        )
    }

    /// Computes metrics with explicit mark-to-market prices for unclosed positions,
    /// rigorously eliminating survivorship bias (unclosed rug pulls / abandoned bags).
    pub fn compute_metrics_with_market_prices(
        trades: &[Trade],
        eval_timestamp: DateTime<Utc>,
        min_early_entry_window_secs: i64,
        market_prices: Option<&HashMap<String, Decimal>>,
    ) -> WalletMetrics {
        // Strictly filter trades to eliminate look-ahead bias
        let mut valid_trades: Vec<&Trade> = trades
            .iter()
            .filter(|t| t.timestamp <= eval_timestamp)
            .collect();

        valid_trades.sort_by_key(|t| t.timestamp);

        let mut round_trips = Vec::new();
        // FIFO buy queue per token
        let mut buy_queues: HashMap<String, Vec<BuyLot>> = HashMap::new();

        let mut tokens_traded_set = std::collections::HashSet::new();
        let mut token_first_trade: HashMap<String, DateTime<Utc>> = HashMap::new();
        let mut token_last_price: HashMap<String, Decimal> = HashMap::new();

        let mut total_buys = 0usize;
        let mut early_buys = 0usize;

        // First pass: track earliest trade seen per token in the dataset
        for trade in &valid_trades {
            let token_key = trade.token_address.as_str().to_string();
            token_first_trade
                .entry(token_key.clone())
                .or_insert(trade.timestamp);
            token_last_price.insert(token_key, trade.price_usd);
        }

        for trade in &valid_trades {
            let token_key = trade.token_address.as_str().to_string();
            tokens_traded_set.insert(token_key.clone());

            match trade.side {
                TradeSide::Buy => {
                    total_buys += 1;
                    if let Some(&first_seen) = token_first_trade.get(&token_key) {
                        let diff_secs = (trade.timestamp - first_seen).num_seconds();
                        if diff_secs <= min_early_entry_window_secs {
                            early_buys += 1;
                        }
                    }

                    let fee_per_unit = if trade.amount_tokens > Decimal::ZERO {
                        trade.fee_usd / trade.amount_tokens
                    } else {
                        Decimal::ZERO
                    };
                    buy_queues.entry(token_key).or_default().push(BuyLot {
                        buy_price: trade.price_usd,
                        remaining_qty: trade.amount_tokens,
                        buy_time: trade.timestamp,
                        fee_per_unit,
                    });
                }
                TradeSide::Sell => {
                    let mut sell_qty_remaining = trade.amount_tokens;
                    let sell_fee_per_unit = if trade.amount_tokens > Decimal::ZERO {
                        trade.fee_usd / trade.amount_tokens
                    } else {
                        Decimal::ZERO
                    };

                    if let Some(queue) = buy_queues.get_mut(&token_key) {
                        while sell_qty_remaining > Decimal::ZERO && !queue.is_empty() {
                            let lot = &mut queue[0];
                            let matched_qty = sell_qty_remaining.min(lot.remaining_qty);

                            let gross_pnl = (trade.price_usd - lot.buy_price) * matched_qty;
                            let total_fee = (lot.fee_per_unit + sell_fee_per_unit) * matched_qty;
                            let net_pnl = gross_pnl - total_fee;

                            let return_pct = if lot.buy_price > Decimal::ZERO {
                                (trade.price_usd - lot.buy_price) / lot.buy_price
                            } else {
                                Decimal::ZERO
                            };

                            let holding_seconds =
                                (trade.timestamp - lot.buy_time).num_seconds().max(0) as u64;

                            round_trips.push(RoundTripTrade {
                                token_address: token_key.clone(),
                                buy_price: lot.buy_price,
                                sell_price: trade.price_usd,
                                quantity: matched_qty,
                                pnl_usd: net_pnl,
                                return_pct,
                                holding_seconds,
                                buy_time: lot.buy_time,
                                sell_time: trade.timestamp,
                            });

                            sell_qty_remaining -= matched_qty;
                            if matched_qty == lot.remaining_qty {
                                queue.remove(0);
                            } else {
                                queue[0].remaining_qty -= matched_qty;
                            }
                        }
                    }
                }
            }
        }

        // Evaluate unclosed positions to eliminate survivorship bias
        let mut unclosed_positions_count = 0usize;
        let mut total_unrealized_pnl = Decimal::ZERO;
        let mut rug_exposure_count = 0usize;
        let mut unclosed_returns = Vec::new();
        let mut unclosed_pnl_items = Vec::new();
        let mut unclosed_holding_time = 0u64;

        for (token_key, queue) in buy_queues.iter() {
            let current_price = market_prices
                .and_then(|m| m.get(token_key).copied())
                .or_else(|| token_last_price.get(token_key).copied())
                .unwrap_or(Decimal::ZERO);

            for lot in queue {
                if lot.remaining_qty <= Decimal::ZERO {
                    continue;
                }
                unclosed_positions_count += 1;
                let gross_pnl = (current_price - lot.buy_price) * lot.remaining_qty;
                let fees = lot.fee_per_unit * lot.remaining_qty;
                let net_pnl = gross_pnl - fees;
                total_unrealized_pnl += net_pnl;

                let return_pct = if lot.buy_price > Decimal::ZERO {
                    (current_price - lot.buy_price) / lot.buy_price
                } else {
                    Decimal::ZERO
                };

                let holding_secs = (eval_timestamp - lot.buy_time).num_seconds().max(0) as u64;
                unclosed_holding_time += holding_secs;

                unclosed_returns.push(return_pct);
                unclosed_pnl_items.push(net_pnl);

                // Flag rug pull: token dropped by >= 90% or price is 0
                if current_price == Decimal::ZERO
                    || (lot.buy_price > Decimal::ZERO
                        && current_price <= lot.buy_price * Decimal::from_str("0.10").unwrap())
                {
                    rug_exposure_count += 1;
                }
            }
        }

        let total_closed = round_trips.len();
        let total_evaluated = total_closed + unclosed_positions_count;

        if total_evaluated == 0 {
            return WalletMetrics {
                total_trades: 0,
                winning_trades: 0,
                losing_trades: 0,
                win_rate: Decimal::ZERO,
                average_return: Decimal::ZERO,
                median_return: Decimal::ZERO,
                realized_pnl: Decimal::ZERO,
                average_holding_time_seconds: 0,
                max_drawdown: Decimal::ZERO,
                profit_factor: Decimal::ZERO,
                tokens_traded: tokens_traded_set.len(),
                early_entry_ratio: Decimal::ZERO,
                rug_exposure_count: 0,
                loss_rate: Decimal::ZERO,
                expectancy: Decimal::ZERO,
                unrealized_pnl: Decimal::ZERO,
                unclosed_positions_count: 0,
                sharpe_ratio: None,
            };
        }

        let mut winning_trades = 0usize;
        let mut losing_trades = 0usize;
        let mut gross_profit = Decimal::ZERO;
        let mut gross_loss = Decimal::ZERO;
        let mut total_realized_pnl = Decimal::ZERO;
        let mut total_return = Decimal::ZERO;
        let mut total_holding_time = 0u64;
        let mut returns_vec = Vec::with_capacity(total_evaluated);

        // Process closed round-trips
        let mut cumulative_pnl = Decimal::ZERO;
        let mut peak_pnl = Decimal::ZERO;
        let mut max_drawdown_usd = Decimal::ZERO;

        for rt in &round_trips {
            total_realized_pnl += rt.pnl_usd;
            total_return += rt.return_pct;
            returns_vec.push(rt.return_pct);
            total_holding_time += rt.holding_seconds;

            if rt.pnl_usd > Decimal::ZERO {
                winning_trades += 1;
                gross_profit += rt.pnl_usd;
            } else {
                losing_trades += 1;
                gross_loss += rt.pnl_usd.abs();
            }

            cumulative_pnl += rt.pnl_usd;
            if cumulative_pnl > peak_pnl {
                peak_pnl = cumulative_pnl;
            }
            let drawdown = peak_pnl - cumulative_pnl;
            if drawdown > max_drawdown_usd {
                max_drawdown_usd = drawdown;
            }
        }

        // Factor unclosed positions into overall evaluation (anti-survivorship)
        for (ret, pnl) in unclosed_returns.into_iter().zip(unclosed_pnl_items) {
            total_return += ret;
            returns_vec.push(ret);

            if pnl > Decimal::ZERO {
                winning_trades += 1;
                gross_profit += pnl;
            } else {
                losing_trades += 1;
                gross_loss += pnl.abs();
            }
        }
        total_holding_time += unclosed_holding_time;

        let total_dec = Decimal::from(total_evaluated);
        let win_rate = Decimal::from(winning_trades) / total_dec;
        let loss_rate = Decimal::from(losing_trades) / total_dec;
        let average_return = total_return / total_dec;

        returns_vec.sort();
        let median_return = if total_evaluated % 2 == 1 {
            returns_vec[total_evaluated / 2]
        } else {
            (returns_vec[total_evaluated / 2 - 1] + returns_vec[total_evaluated / 2]) / Decimal::TWO
        };

        let profit_factor = if gross_loss > Decimal::ZERO {
            gross_profit / gross_loss
        } else if gross_profit > Decimal::ZERO {
            Decimal::from(999)
        } else {
            Decimal::ZERO
        };

        let capital_base = round_trips
            .iter()
            .map(|rt| rt.buy_price * rt.quantity)
            .max()
            .unwrap_or(Decimal::ZERO);

        let peak_equity = if peak_pnl > Decimal::ZERO {
            capital_base + peak_pnl
        } else if capital_base > Decimal::ZERO {
            capital_base
        } else {
            Decimal::ZERO
        };

        let max_drawdown = if peak_equity > Decimal::ZERO {
            (max_drawdown_usd / peak_equity).min(Decimal::ONE)
        } else {
            Decimal::ZERO
        };

        let average_holding_time_seconds = total_holding_time / (total_evaluated as u64);

        let early_entry_ratio = if total_buys > 0 {
            Decimal::from(early_buys) / Decimal::from(total_buys)
        } else {
            Decimal::ZERO
        };

        let avg_win = if winning_trades > 0 {
            gross_profit / Decimal::from(winning_trades)
        } else {
            Decimal::ZERO
        };
        let avg_loss = if losing_trades > 0 {
            gross_loss / Decimal::from(losing_trades)
        } else {
            Decimal::ZERO
        };
        let expectancy = (win_rate * avg_win) - (loss_rate * avg_loss);

        // Sharpe Ratio from trade returns distribution
        let sharpe_ratio = if returns_vec.len() >= 2 {
            let n = returns_vec.len() as f64;
            let mean: f64 = returns_vec.iter().filter_map(|r| r.to_f64()).sum::<f64>() / n;
            let var: f64 = returns_vec
                .iter()
                .filter_map(|r| r.to_f64())
                .map(|r| (r - mean).powi(2))
                .sum::<f64>()
                / (n - 1.0);
            let stdev = var.sqrt();
            if stdev > 1e-6 {
                Decimal::from_f64_retain(mean / stdev)
            } else if mean > 0.0 {
                Some(Decimal::from(999))
            } else if mean < 0.0 {
                Some(Decimal::from(-999))
            } else {
                None
            }
        } else {
            None
        };

        WalletMetrics {
            total_trades: total_evaluated,
            winning_trades,
            losing_trades,
            win_rate,
            average_return,
            median_return,
            realized_pnl: total_realized_pnl,
            average_holding_time_seconds,
            max_drawdown,
            profit_factor,
            tokens_traded: tokens_traded_set.len(),
            early_entry_ratio,
            rug_exposure_count,
            loss_rate,
            expectancy,
            unrealized_pnl: total_unrealized_pnl,
            unclosed_positions_count,
            sharpe_ratio,
        }
    }
}

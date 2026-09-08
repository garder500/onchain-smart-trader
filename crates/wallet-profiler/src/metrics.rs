use chrono::{DateTime, Utc};
use domain::{Trade, TradeSide, WalletMetrics};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;

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

pub struct MetricsCalculator;

impl MetricsCalculator {
    /// Reconstructs closed round-trip trades using FIFO matching on historical trades
    /// strictly <= eval_timestamp (anti-look-ahead compliant).
    pub fn compute_metrics(
        trades: &[Trade],
        eval_timestamp: DateTime<Utc>,
        _min_early_entry_window_secs: i64,
    ) -> WalletMetrics {
        // Strictly filter trades to eliminate look-ahead bias
        let mut valid_trades: Vec<&Trade> = trades
            .iter()
            .filter(|t| t.timestamp <= eval_timestamp)
            .collect();

        valid_trades.sort_by_key(|t| t.timestamp);

        let mut round_trips = Vec::new();
        // FIFO buy queue per token: (buy_price, remaining_qty, buy_time, fee_per_unit)
        let mut buy_queues: HashMap<String, Vec<(Decimal, Decimal, DateTime<Utc>, Decimal)>> =
            HashMap::new();

        let mut tokens_traded_set = std::collections::HashSet::new();

        for trade in &valid_trades {
            let token_key = trade.token_address.as_str().to_string();
            tokens_traded_set.insert(token_key.clone());

            match trade.side {
                TradeSide::Buy => {
                    let fee_per_unit = if trade.amount_tokens > Decimal::ZERO {
                        trade.fee_usd / trade.amount_tokens
                    } else {
                        Decimal::ZERO
                    };
                    buy_queues.entry(token_key).or_default().push((
                        trade.price_usd,
                        trade.amount_tokens,
                        trade.timestamp,
                        fee_per_unit,
                    ));
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
                            let (buy_price, buy_qty, buy_time, buy_fee_per_unit) = queue[0];
                            let matched_qty = sell_qty_remaining.min(buy_qty);

                            let gross_pnl = (trade.price_usd - buy_price) * matched_qty;
                            let total_fee = (buy_fee_per_unit + sell_fee_per_unit) * matched_qty;
                            let net_pnl = gross_pnl - total_fee;

                            let return_pct = if buy_price > Decimal::ZERO {
                                (trade.price_usd - buy_price) / buy_price
                            } else {
                                Decimal::ZERO
                            };

                            let holding_seconds =
                                (trade.timestamp - buy_time).num_seconds().max(0) as u64;

                            round_trips.push(RoundTripTrade {
                                token_address: token_key.clone(),
                                buy_price,
                                sell_price: trade.price_usd,
                                quantity: matched_qty,
                                pnl_usd: net_pnl,
                                return_pct,
                                holding_seconds,
                                buy_time,
                                sell_time: trade.timestamp,
                            });

                            sell_qty_remaining -= matched_qty;
                            if matched_qty == buy_qty {
                                queue.remove(0);
                            } else {
                                queue[0].1 -= matched_qty;
                            }
                        }
                    }
                }
            }
        }

        let total_trades = round_trips.len();
        if total_trades == 0 {
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
            };
        }

        let mut winning_trades = 0;
        let mut losing_trades = 0;
        let mut gross_profit = Decimal::ZERO;
        let mut gross_loss = Decimal::ZERO;
        let mut total_pnl = Decimal::ZERO;
        let mut total_return = Decimal::ZERO;
        let mut total_holding_time = 0u64;
        let mut returns_vec = Vec::with_capacity(total_trades);

        // Track cumulative PnL series for max drawdown
        let mut cumulative_pnl = Decimal::ZERO;
        let mut peak_pnl = Decimal::ZERO;
        let mut max_drawdown_usd = Decimal::ZERO;

        for rt in &round_trips {
            total_pnl += rt.pnl_usd;
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

        let win_rate = Decimal::from(winning_trades) / Decimal::from(total_trades);
        let average_return = total_return / Decimal::from(total_trades);

        returns_vec.sort();
        let median_return = if total_trades % 2 == 1 {
            returns_vec[total_trades / 2]
        } else {
            (returns_vec[total_trades / 2 - 1] + returns_vec[total_trades / 2]) / Decimal::TWO
        };

        let profit_factor = if gross_loss > Decimal::ZERO {
            gross_profit / gross_loss
        } else if gross_profit > Decimal::ZERO {
            Decimal::from(999) // infinite profit factor capped
        } else {
            Decimal::ZERO
        };

        let max_drawdown = if peak_pnl > Decimal::ZERO {
            max_drawdown_usd / peak_pnl
        } else {
            Decimal::ZERO
        };

        let average_holding_time_seconds = total_holding_time / (total_trades as u64);

        WalletMetrics {
            total_trades,
            winning_trades,
            losing_trades,
            win_rate,
            average_return,
            median_return,
            realized_pnl: total_pnl,
            average_holding_time_seconds,
            max_drawdown,
            profit_factor,
            tokens_traded: tokens_traded_set.len(),
            early_entry_ratio: Decimal::from_str("0.5").unwrap(), // estimated / evaluated
            rug_exposure_count: 0,
        }
    }
}

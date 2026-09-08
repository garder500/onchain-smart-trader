use chrono::{DateTime, Utc};
use domain::{PerformanceSnapshot, Portfolio, PositionStatus};
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

pub struct PerformanceCalculator;

impl PerformanceCalculator {
    pub fn compute_performance(
        strategy_id: &str,
        initial_balance: Decimal,
        portfolio: &Portfolio,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        equity_snapshots: &[Decimal],
    ) -> PerformanceSnapshot {
        let final_balance = portfolio.equity;
        let net_pnl = final_balance - initial_balance;
        let roi_percent = if initial_balance > Decimal::ZERO {
            (net_pnl / initial_balance) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        // Collect closed positions
        let closed_positions: Vec<&domain::Position> = portfolio
            .positions
            .values()
            .filter(|p| p.status == PositionStatus::Closed)
            .collect();

        let total_trades = closed_positions.len();
        let mut winning_trades = 0;
        let mut losing_trades = 0;
        let mut gross_profit = Decimal::ZERO;
        let mut gross_loss = Decimal::ZERO;

        for pos in &closed_positions {
            if pos.realized_pnl > Decimal::ZERO {
                winning_trades += 1;
                gross_profit += pos.realized_pnl;
            } else {
                losing_trades += 1;
                gross_loss += pos.realized_pnl.abs();
            }
        }

        let win_rate = if total_trades > 0 {
            (Decimal::from(winning_trades) / Decimal::from(total_trades)) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        let profit_factor = if gross_loss > Decimal::ZERO {
            gross_profit / gross_loss
        } else if gross_profit > Decimal::ZERO {
            Decimal::from(999)
        } else {
            Decimal::ZERO
        };

        // Compute Max Drawdown from equity series
        let mut max_drawdown_percent = Decimal::ZERO;
        let mut peak = initial_balance;

        for &equity in equity_snapshots {
            if equity > peak {
                peak = equity;
            }
            if peak > Decimal::ZERO {
                let dd = ((peak - equity) / peak) * Decimal::from(100);
                if dd > max_drawdown_percent {
                    max_drawdown_percent = dd;
                }
            }
        }

        // Compute Sharpe ratio from periodic return differentials
        let sharpe_ratio = if equity_snapshots.len() >= 2 {
            let mut returns = Vec::new();
            for i in 1..equity_snapshots.len() {
                let prev = equity_snapshots[i - 1];
                let curr = equity_snapshots[i];
                if prev > Decimal::ZERO {
                    let r = (curr - prev) / prev;
                    if let Some(rf) = r.to_f64() {
                        returns.push(rf);
                    }
                }
            }

            if !returns.is_empty() {
                let n = returns.len() as f64;
                let mean = returns.iter().sum::<f64>() / n;
                let variance = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / n;
                let std_dev = variance.sqrt();

                if std_dev > 0.00001 {
                    let annualized_sharpe = (mean / std_dev) * (365.0_f64).sqrt();
                    Decimal::from_str(&format!("{:.2}", annualized_sharpe)).ok()
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        PerformanceSnapshot {
            strategy_id: strategy_id.to_string(),
            period_start,
            period_end,
            initial_balance,
            final_balance,
            roi_percent,
            net_pnl,
            max_drawdown_percent,
            win_rate,
            profit_factor,
            sharpe_ratio,
            total_trades,
            winning_trades,
            losing_trades,
            total_fees: portfolio.total_fees_paid,
            total_slippage: portfolio.total_slippage_paid,
            calculated_at: Utc::now(),
        }
    }
}

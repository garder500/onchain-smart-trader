use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub strategy_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub initial_balance: Decimal,
    pub final_balance: Decimal,
    pub roi_percent: Decimal,
    pub net_pnl: Decimal,
    pub max_drawdown_percent: Decimal,
    pub win_rate: Decimal,
    pub profit_factor: Decimal,
    pub sharpe_ratio: Option<Decimal>,
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub total_fees: Decimal,
    pub total_slippage: Decimal,
    pub calculated_at: DateTime<Utc>,
}

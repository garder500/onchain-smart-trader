use crate::wallet::WalletAddress;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletCategory {
    Unknown,
    Promising,
    Smart,
    Excellent,
}

impl std::fmt::Display for WalletCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletCategory::Unknown => write!(f, "UNKNOWN"),
            WalletCategory::Promising => write!(f, "PROMISING"),
            WalletCategory::Smart => write!(f, "SMART"),
            WalletCategory::Excellent => write!(f, "EXCELLENT"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMetrics {
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: Decimal,
    pub average_return: Decimal,
    pub median_return: Decimal,
    pub realized_pnl: Decimal,
    pub average_holding_time_seconds: u64,
    pub max_drawdown: Decimal,
    pub profit_factor: Decimal,
    pub tokens_traded: usize,
    pub early_entry_ratio: Decimal,
    pub rug_exposure_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreFactors {
    pub profitability_factor: Decimal,
    pub consistency_factor: Decimal,
    pub early_entry_factor: Decimal,
    pub profit_factor_score: Decimal,
    pub sample_size_factor: Decimal,
    pub drawdown_penalty: Decimal,
    pub rug_penalty: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletScore {
    pub wallet_address: WalletAddress,
    pub overall_score: Decimal,
    pub category: WalletCategory,
    pub metrics: WalletMetrics,
    pub factors: ScoreFactors,
    pub explanation: Vec<String>,
    pub evaluated_at: DateTime<Utc>,
}

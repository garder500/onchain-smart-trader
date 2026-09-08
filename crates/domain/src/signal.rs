use crate::token::TokenAddress;
use crate::wallet::WalletAddress;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SignalAction {
    Buy,
    Sell,
    Hold,
}

impl std::fmt::Display for SignalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalAction::Buy => write!(f, "BUY"),
            SignalAction::Sell => write!(f, "SELL"),
            SignalAction::Hold => write!(f, "HOLD"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalReason {
    SmartWalletFollow,
    TakeProfit,
    StopLoss,
    TimeExit,
    WalletExit,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub id: Uuid,
    pub strategy_id: String,
    pub token_address: TokenAddress,
    pub action: SignalAction,
    pub suggested_size_usd: Decimal,
    pub wallet_address: Option<WalletAddress>,
    pub wallet_score: Option<Decimal>,
    pub token_risk_score: Option<Decimal>,
    pub reason: SignalReason,
    pub details: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperOrder {
    pub id: Uuid,
    pub signal_id: Option<Uuid>,
    pub token_address: TokenAddress,
    pub action: SignalAction,
    pub requested_price: Decimal,
    pub execution_price: Decimal,
    pub requested_quantity: Decimal,
    pub executed_quantity: Decimal,
    pub volume_usd: Decimal,
    pub fees: Decimal,
    pub slippage: Decimal,
    pub timestamp: DateTime<Utc>,
}

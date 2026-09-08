use crate::token::TokenAddress;
use crate::wallet::WalletAddress;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionStatus {
    Open,
    Closed,
    Cancelled,
}

impl std::fmt::Display for PositionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionStatus::Open => write!(f, "OPEN"),
            PositionStatus::Closed => write!(f, "CLOSED"),
            PositionStatus::Cancelled => write!(f, "CANCELLED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: Uuid,
    pub token_address: TokenAddress,
    pub status: PositionStatus,
    pub amount_tokens: Decimal,
    pub entry_price: Decimal,
    pub current_price: Decimal,
    pub exit_price: Option<Decimal>,
    pub invested_usd: Decimal,
    pub current_value_usd: Decimal,
    pub realized_pnl: Decimal,
    pub unrealized_pnl: Decimal,
    pub total_fees: Decimal,
    pub copied_wallet: Option<WalletAddress>,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

impl Position {
    pub fn new(
        token_address: TokenAddress,
        amount_tokens: Decimal,
        entry_price: Decimal,
        invested_usd: Decimal,
        fee: Decimal,
        copied_wallet: Option<WalletAddress>,
        opened_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            token_address,
            status: PositionStatus::Open,
            amount_tokens,
            entry_price,
            current_price: entry_price,
            exit_price: None,
            invested_usd,
            current_value_usd: invested_usd,
            realized_pnl: Decimal::ZERO,
            unrealized_pnl: Decimal::ZERO,
            total_fees: fee,
            copied_wallet,
            opened_at,
            closed_at: None,
            updated_at: opened_at,
        }
    }

    pub fn update_price(&mut self, new_price: Decimal, timestamp: DateTime<Utc>) {
        self.current_price = new_price;
        self.current_value_usd = self.amount_tokens * new_price;
        self.unrealized_pnl = self.current_value_usd - self.invested_usd;
        self.updated_at = timestamp;
    }

    pub fn close(&mut self, exit_price: Decimal, exit_fee: Decimal, timestamp: DateTime<Utc>) {
        self.status = PositionStatus::Closed;
        self.exit_price = Some(exit_price);
        self.current_price = exit_price;
        let gross_value = self.amount_tokens * exit_price;
        self.total_fees += exit_fee;
        self.realized_pnl = gross_value - self.invested_usd - exit_fee;
        self.unrealized_pnl = Decimal::ZERO;
        self.current_value_usd = Decimal::ZERO;
        self.closed_at = Some(timestamp);
        self.updated_at = timestamp;
    }
}

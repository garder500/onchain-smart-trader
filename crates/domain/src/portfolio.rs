use crate::position::Position;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub initial_cash: Decimal,
    pub cash: Decimal,
    pub equity: Decimal,
    pub realized_pnl: Decimal,
    pub unrealized_pnl: Decimal,
    pub total_fees_paid: Decimal,
    pub total_slippage_paid: Decimal,
    pub positions: HashMap<Uuid, Position>,
    pub updated_at: DateTime<Utc>,
}

impl Portfolio {
    pub fn new(initial_cash: Decimal, timestamp: DateTime<Utc>) -> Self {
        Self {
            initial_cash,
            cash: initial_cash,
            equity: initial_cash,
            realized_pnl: Decimal::ZERO,
            unrealized_pnl: Decimal::ZERO,
            total_fees_paid: Decimal::ZERO,
            total_slippage_paid: Decimal::ZERO,
            positions: HashMap::new(),
            updated_at: timestamp,
        }
    }

    pub fn recompute_equity(&mut self, timestamp: DateTime<Utc>) {
        let mut total_unrealized = Decimal::ZERO;
        for position in self.positions.values() {
            if position.status == crate::position::PositionStatus::Open {
                total_unrealized += position.unrealized_pnl;
            }
        }
        self.unrealized_pnl = total_unrealized;
        self.equity = self.cash + total_unrealized;
        self.updated_at = timestamp;
    }

    pub fn open_positions_count(&self) -> usize {
        self.positions
            .values()
            .filter(|p| p.status == crate::position::PositionStatus::Open)
            .count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSnapshot {
    pub id: Uuid,
    pub strategy_id: String,
    pub cash: Decimal,
    pub equity: Decimal,
    pub realized_pnl: Decimal,
    pub unrealized_pnl: Decimal,
    pub total_fees: Decimal,
    pub open_positions_count: usize,
    pub timestamp: DateTime<Utc>,
}

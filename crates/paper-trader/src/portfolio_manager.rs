use chrono::{DateTime, Utc};
use domain::{Portfolio, PortfolioSnapshot, PositionStatus, TokenAddress};
use rust_decimal::Decimal;
use std::collections::HashMap;
use uuid::Uuid;

pub struct PortfolioManager {
    pub portfolio: Portfolio,
    pub strategy_id: String,
}

impl PortfolioManager {
    pub fn new(
        strategy_id: impl Into<String>,
        initial_cash: Decimal,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            portfolio: Portfolio::new(initial_cash, timestamp),
            strategy_id: strategy_id.into(),
        }
    }

    /// Mark-to-market: updates current market prices for all open positions
    pub fn update_market_prices(
        &mut self,
        prices: &HashMap<TokenAddress, Decimal>,
        timestamp: DateTime<Utc>,
    ) {
        for pos in self.portfolio.positions.values_mut() {
            if pos.status == PositionStatus::Open {
                if let Some(&price) = prices.get(&pos.token_address) {
                    pos.update_price(price, timestamp);
                }
            }
        }
        self.portfolio.recompute_equity(timestamp);
    }

    /// Takes an immutable snapshot of current portfolio state
    pub fn take_snapshot(&self, timestamp: DateTime<Utc>) -> PortfolioSnapshot {
        PortfolioSnapshot {
            id: Uuid::new_v4(),
            strategy_id: self.strategy_id.clone(),
            cash: self.portfolio.cash,
            equity: self.portfolio.equity,
            realized_pnl: self.portfolio.realized_pnl,
            unrealized_pnl: self.portfolio.unrealized_pnl,
            total_fees: self.portfolio.total_fees_paid,
            open_positions_count: self.portfolio.open_positions_count(),
            timestamp,
        }
    }
}

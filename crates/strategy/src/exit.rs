use chrono::{DateTime, Utc};
use domain::{Position, Signal, SignalAction, SignalReason, Trade, TradeSide};
use rust_decimal::Decimal;
use uuid::Uuid;

pub trait ExitStrategy: Send + Sync {
    fn evaluate_exit(
        &self,
        strategy_id: &str,
        position: &Position,
        current_price: Decimal,
        current_time: DateTime<Utc>,
        wallet_trade: Option<&Trade>,
    ) -> Option<Signal>;
}

// 1. Fixed Take Profit
pub struct TakeProfitExit {
    pub target_profit_percent: Decimal, // e.g. 0.50 (+50%)
}

impl TakeProfitExit {
    pub fn new(target_profit_percent: Decimal) -> Self {
        Self {
            target_profit_percent,
        }
    }
}

impl ExitStrategy for TakeProfitExit {
    fn evaluate_exit(
        &self,
        strategy_id: &str,
        position: &Position,
        current_price: Decimal,
        current_time: DateTime<Utc>,
        _wallet_trade: Option<&Trade>,
    ) -> Option<Signal> {
        if position.entry_price <= Decimal::ZERO {
            return None;
        }

        let gain_pct = (current_price - position.entry_price) / position.entry_price;
        if gain_pct >= self.target_profit_percent {
            Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: strategy_id.to_string(),
                token_address: position.token_address.clone(),
                action: SignalAction::Sell,
                suggested_size_usd: position.current_value_usd,
                wallet_address: position.copied_wallet.clone(),
                wallet_score: None,
                token_risk_score: None,
                reason: SignalReason::TakeProfit,
                details: Some(format!(
                    "Take Profit triggered: gain of {:.2}% (target: {:.2}%)",
                    gain_pct * Decimal::from(100),
                    self.target_profit_percent * Decimal::from(100)
                )),
                timestamp: current_time,
            })
        } else {
            None
        }
    }
}

// 2. Stop Loss
pub struct StopLossExit {
    pub stop_loss_percent: Decimal, // e.g. 0.20 (-20%)
}

impl StopLossExit {
    pub fn new(stop_loss_percent: Decimal) -> Self {
        Self { stop_loss_percent }
    }
}

impl ExitStrategy for StopLossExit {
    fn evaluate_exit(
        &self,
        strategy_id: &str,
        position: &Position,
        current_price: Decimal,
        current_time: DateTime<Utc>,
        _wallet_trade: Option<&Trade>,
    ) -> Option<Signal> {
        if position.entry_price <= Decimal::ZERO {
            return None;
        }

        let loss_pct = (position.entry_price - current_price) / position.entry_price;
        if loss_pct >= self.stop_loss_percent {
            Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: strategy_id.to_string(),
                token_address: position.token_address.clone(),
                action: SignalAction::Sell,
                suggested_size_usd: position.current_value_usd,
                wallet_address: position.copied_wallet.clone(),
                wallet_score: None,
                token_risk_score: None,
                reason: SignalReason::StopLoss,
                details: Some(format!(
                    "Stop Loss triggered: loss of {:.2}% (threshold: {:.2}%)",
                    loss_pct * Decimal::from(100),
                    self.stop_loss_percent * Decimal::from(100)
                )),
                timestamp: current_time,
            })
        } else {
            None
        }
    }
}

// 3. Time Based Exit
pub struct TimeBasedExit {
    pub max_holding_seconds: i64, // e.g. 86400 (24h)
}

impl TimeBasedExit {
    pub fn new(max_holding_seconds: i64) -> Self {
        Self {
            max_holding_seconds,
        }
    }
}

impl ExitStrategy for TimeBasedExit {
    fn evaluate_exit(
        &self,
        strategy_id: &str,
        position: &Position,
        _current_price: Decimal,
        current_time: DateTime<Utc>,
        _wallet_trade: Option<&Trade>,
    ) -> Option<Signal> {
        let holding_time = (current_time - position.opened_at).num_seconds();
        if holding_time >= self.max_holding_seconds {
            Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: strategy_id.to_string(),
                token_address: position.token_address.clone(),
                action: SignalAction::Sell,
                suggested_size_usd: position.current_value_usd,
                wallet_address: position.copied_wallet.clone(),
                wallet_score: None,
                token_risk_score: None,
                reason: SignalReason::TimeExit,
                details: Some(format!(
                    "Time Exit triggered: held for {} seconds (limit: {})",
                    holding_time, self.max_holding_seconds
                )),
                timestamp: current_time,
            })
        } else {
            None
        }
    }
}

// 4. Copy-Wallet Exit
pub struct CopyWalletExit;

impl ExitStrategy for CopyWalletExit {
    fn evaluate_exit(
        &self,
        strategy_id: &str,
        position: &Position,
        _current_price: Decimal,
        current_time: DateTime<Utc>,
        wallet_trade: Option<&Trade>,
    ) -> Option<Signal> {
        if let (Some(ref copied_wallet), Some(trade)) = (&position.copied_wallet, wallet_trade) {
            if trade.wallet_address == *copied_wallet
                && trade.token_address == position.token_address
                && trade.side == TradeSide::Sell
            {
                return Some(Signal {
                    id: Uuid::new_v4(),
                    strategy_id: strategy_id.to_string(),
                    token_address: position.token_address.clone(),
                    action: SignalAction::Sell,
                    suggested_size_usd: position.current_value_usd,
                    wallet_address: Some(copied_wallet.clone()),
                    wallet_score: None,
                    token_risk_score: None,
                    reason: SignalReason::WalletExit,
                    details: Some(format!(
                        "Tracked wallet {} sold token {}",
                        copied_wallet, position.token_address
                    )),
                    timestamp: current_time,
                });
            }
        }
        None
    }
}

// Composite Exit Strategy
pub struct CompositeExitStrategy {
    strategies: Vec<Box<dyn ExitStrategy>>,
}

impl CompositeExitStrategy {
    pub fn default_rules(tp_pct: Decimal, sl_pct: Decimal, max_hold_secs: i64) -> Self {
        Self {
            strategies: vec![
                Box::new(StopLossExit::new(sl_pct)),
                Box::new(TakeProfitExit::new(tp_pct)),
                Box::new(CopyWalletExit),
                Box::new(TimeBasedExit::new(max_hold_secs)),
            ],
        }
    }
}

impl ExitStrategy for CompositeExitStrategy {
    fn evaluate_exit(
        &self,
        strategy_id: &str,
        position: &Position,
        current_price: Decimal,
        current_time: DateTime<Utc>,
        wallet_trade: Option<&Trade>,
    ) -> Option<Signal> {
        for strategy in &self.strategies {
            if let Some(signal) = strategy.evaluate_exit(
                strategy_id,
                position,
                current_price,
                current_time,
                wallet_trade,
            ) {
                return Some(signal);
            }
        }
        None
    }
}

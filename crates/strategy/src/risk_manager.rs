use domain::{Portfolio, PositionStatus, Signal, SignalAction};
use rust_decimal::Decimal;
use std::str::FromStr;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct RiskManagerConfig {
    pub max_open_positions: usize,
    pub max_portfolio_allocation_per_token: Decimal,
    pub min_cash_reserve: Decimal,
}

impl Default for RiskManagerConfig {
    fn default() -> Self {
        Self {
            max_open_positions: 5,
            max_portfolio_allocation_per_token: Decimal::from_str("0.20").unwrap(), // max 20%
            min_cash_reserve: Decimal::from(50),
        }
    }
}

pub struct RiskManager {
    config: RiskManagerConfig,
}

impl RiskManager {
    pub fn new(config: RiskManagerConfig) -> Self {
        Self { config }
    }

    /// Evaluates a proposed signal against strict portfolio risk limits.
    /// Returns Some(Signal) if approved (potentially adjusted), or None if rejected.
    pub fn evaluate_signal(&self, mut signal: Signal, portfolio: &Portfolio) -> Option<Signal> {
        match signal.action {
            SignalAction::Buy => {
                // Check 1: Max open positions limit
                let open_count = portfolio.open_positions_count();
                if open_count >= self.config.max_open_positions {
                    warn!(
                        open_count = open_count,
                        max = self.config.max_open_positions,
                        "RiskManager rejected signal: max open positions reached"
                    );
                    return None;
                }

                // Check 2: Prevent duplicate open positions for the same token
                for pos in portfolio.positions.values() {
                    if pos.status == PositionStatus::Open
                        && pos.token_address == signal.token_address
                    {
                        warn!(
                            token = %signal.token_address,
                            "RiskManager rejected signal: already holding open position for token"
                        );
                        return None;
                    }
                }

                // Check 3: Available cash limit
                let available_cash = portfolio.cash - self.config.min_cash_reserve;
                if available_cash <= Decimal::ZERO {
                    warn!(
                        cash = %portfolio.cash,
                        min_reserve = %self.config.min_cash_reserve,
                        "RiskManager rejected signal: insufficient cash after reserve"
                    );
                    return None;
                }

                // Check 4: Max allocation cap per token
                let max_allowed = portfolio.equity * self.config.max_portfolio_allocation_per_token;
                let adjusted_size = signal
                    .suggested_size_usd
                    .min(available_cash)
                    .min(max_allowed);

                if adjusted_size < Decimal::from(10) {
                    warn!("RiskManager rejected signal: adjusted position size too small (< $10)");
                    return None;
                }

                signal.suggested_size_usd = adjusted_size;
                Some(signal)
            }
            SignalAction::Sell => {
                // Sell / close signals are always approved to facilitate exits
                Some(signal)
            }
            SignalAction::Hold => None,
        }
    }
}

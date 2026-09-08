use crate::exit::{CompositeExitStrategy, ExitStrategy};
use crate::risk_manager::RiskManager;
use crate::sizing::PositionSizer;
use chrono::{DateTime, Utc};
use domain::{
    Portfolio, Position, Signal, SignalAction, SignalReason, TokenContext, Trade, TradeSide,
    WalletCategory,
};
use rust_decimal::Decimal;
use std::sync::Arc;
use token_risk::TokenRiskEngine;
use tracing::{debug, info, warn};
use uuid::Uuid;
use wallet_profiler::{calculate_wallet_score, WalletContext};

pub struct StrategyConfig {
    pub strategy_id: String,
    pub min_wallet_score: Decimal,
    pub min_wallet_trades: usize,
}

pub struct SmartWalletCopyStrategy {
    config: StrategyConfig,
    risk_engine: Arc<TokenRiskEngine>,
    sizer: PositionSizer,
    exit_strategy: CompositeExitStrategy,
    risk_manager: RiskManager,
}

impl SmartWalletCopyStrategy {
    pub fn new(
        config: StrategyConfig,
        risk_engine: Arc<TokenRiskEngine>,
        sizer: PositionSizer,
        exit_strategy: CompositeExitStrategy,
        risk_manager: RiskManager,
    ) -> Self {
        Self {
            config,
            risk_engine,
            sizer,
            exit_strategy,
            risk_manager,
        }
    }

    /// Processes an incoming wallet trade event and returns an approved Signal if generated
    pub fn on_trade_event(
        &self,
        trade: &Trade,
        wallet_historical_trades: &[Trade],
        token_context: &TokenContext,
        portfolio: &Portfolio,
    ) -> Option<Signal> {
        // 1. Anti-look-ahead check
        if trade.timestamp > token_context.current_timestamp {
            warn!("Rejected trade event: timestamp is ahead of evaluation timestamp");
            return None;
        }

        match trade.side {
            TradeSide::Buy => {
                // Step A: Score the wallet using ONLY historical trades prior to trade.timestamp
                let wallet_ctx = WalletContext {
                    wallet_address: trade.wallet_address.clone(),
                    trades: wallet_historical_trades.to_vec(),
                    eval_timestamp: trade.timestamp,
                    min_trades_threshold: self.config.min_wallet_trades,
                };

                let wallet_score = calculate_wallet_score(&wallet_ctx);

                if wallet_score.overall_score < self.config.min_wallet_score
                    || wallet_score.category == WalletCategory::Unknown
                {
                    debug!(
                        wallet = %trade.wallet_address,
                        score = %wallet_score.overall_score,
                        category = %wallet_score.category,
                        "Ignored trade: wallet does not meet smart wallet criteria"
                    );
                    return None;
                }

                info!(
                    wallet = %trade.wallet_address,
                    score = %wallet_score.overall_score,
                    category = %wallet_score.category,
                    token = %trade.token_address,
                    "Smart wallet BUY detected! Evaluating token risk"
                );

                // Step B: Calculate token risk
                let token_risk = self.risk_engine.calculate_token_risk(token_context);
                if !token_risk.accepted {
                    warn!(
                        token = %trade.token_address,
                        reasons = ?token_risk.reasons,
                        score = %token_risk.score,
                        "Token risk check rejected trade"
                    );
                    return None;
                }

                // Step C: Position sizing
                let suggested_size = self.sizer.calculate_size(
                    portfolio.equity,
                    &wallet_score,
                    token_context.pool_liquidity_usd,
                );

                if suggested_size <= Decimal::ZERO {
                    warn!("Calculated position size is zero, skipping signal");
                    return None;
                }

                // Step D: Generate Buy Signal
                let signal = Signal {
                    id: Uuid::new_v4(),
                    strategy_id: self.config.strategy_id.clone(),
                    token_address: trade.token_address.clone(),
                    action: SignalAction::Buy,
                    suggested_size_usd: suggested_size,
                    wallet_address: Some(trade.wallet_address.clone()),
                    wallet_score: Some(wallet_score.overall_score),
                    token_risk_score: Some(token_risk.score),
                    reason: SignalReason::SmartWalletFollow,
                    details: Some(format!(
                        "Copying smart wallet {} with score {}",
                        trade.wallet_address, wallet_score.overall_score
                    )),
                    timestamp: trade.timestamp,
                };

                // Step E: Pass through Risk Manager
                self.risk_manager.evaluate_signal(signal, portfolio)
            }
            TradeSide::Sell => {
                // If tracked wallet sells, check if we hold an open position
                for position in portfolio.positions.values() {
                    if position.status == domain::PositionStatus::Open
                        && position.token_address == trade.token_address
                    {
                        if let Some(signal) = self.exit_strategy.evaluate_exit(
                            &self.config.strategy_id,
                            position,
                            trade.price_usd,
                            trade.timestamp,
                            Some(trade),
                        ) {
                            return self.risk_manager.evaluate_signal(signal, portfolio);
                        }
                    }
                }
                None
            }
        }
    }

    /// Evaluates periodic exit conditions for open positions (TP/SL/Time)
    pub fn check_position_exits(
        &self,
        positions: &[Position],
        current_prices: &std::collections::HashMap<domain::TokenAddress, Decimal>,
        current_time: DateTime<Utc>,
        portfolio: &Portfolio,
    ) -> Vec<Signal> {
        let mut exit_signals = Vec::new();

        for position in positions {
            if position.status != domain::PositionStatus::Open {
                continue;
            }

            if let Some(&price) = current_prices.get(&position.token_address) {
                if let Some(signal) = self.exit_strategy.evaluate_exit(
                    &self.config.strategy_id,
                    position,
                    price,
                    current_time,
                    None,
                ) {
                    if let Some(approved_signal) =
                        self.risk_manager.evaluate_signal(signal, portfolio)
                    {
                        exit_signals.push(approved_signal);
                    }
                }
            }
        }

        exit_signals
    }
}

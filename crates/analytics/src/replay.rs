use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use domain::{
    DomainError, PerformanceSnapshot, Portfolio, PositionStatus, TokenAddress, TokenContext, Trade,
};
use paper_trader::{OrderExecutor, PaperExecutor};
use rust_decimal::Decimal;
use std::collections::HashMap;
use strategy::SmartWalletCopyStrategy;
use tracing::info;

pub struct ReplayEngine {
    strategy: SmartWalletCopyStrategy,
    executor: PaperExecutor,
    initial_balance: Decimal,
}

impl ReplayEngine {
    pub fn new(
        strategy: SmartWalletCopyStrategy,
        executor: PaperExecutor,
        initial_balance: Decimal,
    ) -> Self {
        Self {
            strategy,
            executor,
            initial_balance,
        }
    }

    /// Runs backtest over historical trade stream in strict chronological order
    pub async fn run_replay(
        &self,
        strategy_id: &str,
        mut trades: Vec<Trade>,
        token_contexts: &HashMap<TokenAddress, TokenContext>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<(Portfolio, PerformanceSnapshot)> {
        // Sort chronologically
        trades.sort_by_key(|t| t.timestamp);

        let mut portfolio = Portfolio::new(self.initial_balance, start_time);
        let mut equity_snapshots = vec![self.initial_balance];
        let mut historical_trades_buffer: Vec<Trade> = Vec::new();

        let mut latest_prices: HashMap<TokenAddress, Decimal> = HashMap::new();

        for trade in trades {
            if trade.timestamp < start_time || trade.timestamp > end_time {
                continue;
            }

            // --- STRICT ANTI LOOK-AHEAD CHECK ---
            for past_trade in &historical_trades_buffer {
                if past_trade.timestamp > trade.timestamp {
                    bail!(DomainError::AntiLookAheadViolation {
                        event_time: past_trade.timestamp,
                        eval_time: trade.timestamp,
                    });
                }
            }

            let decision_time = trade.timestamp;
            latest_prices.insert(trade.token_address.clone(), trade.price_usd);

            // Update mark-to-market prices on open positions
            for pos in portfolio.positions.values_mut() {
                if pos.status == PositionStatus::Open {
                    if let Some(&price) = latest_prices.get(&pos.token_address) {
                        pos.update_price(price, decision_time);
                    }
                }
            }
            portfolio.recompute_equity(decision_time);

            // 1. Evaluate open position exits (SL, TP, Time)
            let open_positions: Vec<domain::Position> = portfolio
                .positions
                .values()
                .filter(|p| p.status == PositionStatus::Open)
                .cloned()
                .collect();

            let exit_signals = self.strategy.check_position_exits(
                &open_positions,
                &latest_prices,
                decision_time,
                &portfolio,
            );

            for exit_signal in exit_signals {
                if let Some(&price) = latest_prices.get(&exit_signal.token_address) {
                    let dummy_liq = Decimal::from(100000);
                    let _ = self
                        .executor
                        .execute_order(&exit_signal, price, dummy_liq, &mut portfolio)
                        .await;
                }
            }

            // 2. Process incoming trade event
            if let Some(token_ctx) = token_contexts.get(&trade.token_address) {
                // Ensure token context timestamp respects anti look-ahead
                let mut valid_ctx = token_ctx.clone();
                valid_ctx.current_timestamp = decision_time;

                let signal = self.strategy.on_trade_event(
                    &trade,
                    &historical_trades_buffer,
                    &valid_ctx,
                    &portfolio,
                );

                if let Some(sig) = signal {
                    let _ = self
                        .executor
                        .execute_order(
                            &sig,
                            trade.price_usd,
                            token_ctx.pool_liquidity_usd,
                            &mut portfolio,
                        )
                        .await;
                }
            }

            historical_trades_buffer.push(trade);
            equity_snapshots.push(portfolio.equity);
        }

        // Close remaining open positions at end time with last known price
        for pos in portfolio.positions.values_mut() {
            if pos.status == PositionStatus::Open {
                let close_price = latest_prices
                    .get(&pos.token_address)
                    .cloned()
                    .unwrap_or(pos.entry_price);
                pos.close(close_price, Decimal::from(1), end_time);
                portfolio.realized_pnl += pos.realized_pnl;
            }
        }
        portfolio.recompute_equity(end_time);
        equity_snapshots.push(portfolio.equity);

        let performance = crate::metrics::PerformanceCalculator::compute_performance(
            strategy_id,
            self.initial_balance,
            &portfolio,
            start_time,
            end_time,
            &equity_snapshots,
        );

        info!(
            strategy = %strategy_id,
            roi = %performance.roi_percent,
            trades = %performance.total_trades,
            "Replay backtest completed"
        );

        Ok((portfolio, performance))
    }
}

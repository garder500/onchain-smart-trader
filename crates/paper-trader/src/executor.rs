use anyhow::{bail, Result};
use async_trait::async_trait;
use domain::{PaperOrder, Portfolio, Position, PositionStatus, Signal, SignalAction};
use rust_decimal::Decimal;
use std::str::FromStr;
use tracing::{info, warn};
use uuid::Uuid;

#[async_trait]
pub trait OrderExecutor: Send + Sync {
    async fn execute_order(
        &self,
        signal: &Signal,
        market_price: Decimal,
        pool_liquidity_usd: Decimal,
        portfolio: &mut Portfolio,
    ) -> Result<PaperOrder>;
}

#[derive(Debug, Clone)]
pub struct PaperExecutorConfig {
    pub base_slippage_bps: u32,
    pub trading_fee_bps: u32,
    pub max_slippage_bps: u32,
}

impl Default for PaperExecutorConfig {
    fn default() -> Self {
        Self {
            base_slippage_bps: 50, // 0.50%
            trading_fee_bps: 30,   // 0.30%
            max_slippage_bps: 300, // 3.00% max
        }
    }
}

pub struct PaperExecutor {
    config: PaperExecutorConfig,
}

impl PaperExecutor {
    pub fn new(config: PaperExecutorConfig) -> Self {
        Self { config }
    }

    /// Computes realistic slippage combining constant spread + dynamic automated market maker (AMM) price impact:
    /// Impact = order_size / (2 * pool_liquidity)
    pub fn calculate_slippage(
        &self,
        market_price: Decimal,
        order_volume_usd: Decimal,
        pool_liquidity_usd: Decimal,
    ) -> Decimal {
        let base_slip_rate = Decimal::from(self.config.base_slippage_bps) / Decimal::from(10000);

        let dynamic_impact = if pool_liquidity_usd > Decimal::ZERO {
            (order_volume_usd / (pool_liquidity_usd * Decimal::TWO))
                .min(Decimal::from(self.config.max_slippage_bps) / Decimal::from(10000))
        } else {
            Decimal::from_str("0.02").unwrap()
        };

        let total_slip_rate = (base_slip_rate + dynamic_impact)
            .min(Decimal::from(self.config.max_slippage_bps) / Decimal::from(10000));

        market_price * total_slip_rate
    }
}

#[async_trait]
impl OrderExecutor for PaperExecutor {
    async fn execute_order(
        &self,
        signal: &Signal,
        market_price: Decimal,
        pool_liquidity_usd: Decimal,
        portfolio: &mut Portfolio,
    ) -> Result<PaperOrder> {
        if market_price <= Decimal::ZERO {
            bail!("Invalid market price: must be positive");
        }

        let timestamp = signal.timestamp;

        match signal.action {
            SignalAction::Buy => {
                let requested_usd = signal.suggested_size_usd;
                if requested_usd <= Decimal::ZERO {
                    bail!("Invalid suggested order size: must be positive");
                }

                // Check cash limit
                if portfolio.cash < requested_usd {
                    bail!(
                        "Insufficient cash: requested {}, available {}",
                        requested_usd,
                        portfolio.cash
                    );
                }

                let requested_quantity = requested_usd / market_price;

                // Simulate partial fills if liquidity is constrained (< 2x order size)
                let executed_quantity = if pool_liquidity_usd > Decimal::ZERO
                    && requested_usd > (pool_liquidity_usd * Decimal::from_str("0.05").unwrap())
                {
                    // Partial fill to 80%
                    warn!(
                        order_volume = %requested_usd,
                        pool_liquidity = %pool_liquidity_usd,
                        "Order size exceeds 5% of pool liquidity: applying partial fill (80%)"
                    );
                    requested_quantity * Decimal::from_str("0.80").unwrap()
                } else {
                    requested_quantity
                };

                let order_volume = executed_quantity * market_price;

                let slippage_per_unit =
                    self.calculate_slippage(market_price, order_volume, pool_liquidity_usd);
                let execution_price = market_price + slippage_per_unit; // BUY pays above market price

                let actual_invested = executed_quantity * execution_price;
                let fee_rate = Decimal::from(self.config.trading_fee_bps) / Decimal::from(10000);
                let fees = actual_invested * fee_rate;

                let total_deduction = actual_invested + fees;
                if portfolio.cash < total_deduction {
                    bail!("Insufficient cash after slippage and fees");
                }

                // Update portfolio cash
                portfolio.cash -= total_deduction;
                portfolio.total_fees_paid += fees;
                portfolio.total_slippage_paid += slippage_per_unit * executed_quantity;

                // Add or update open position
                let position = Position::new(
                    signal.token_address.clone(),
                    executed_quantity,
                    execution_price,
                    actual_invested,
                    fees,
                    signal.wallet_address.clone(),
                    timestamp,
                );

                portfolio.positions.insert(position.id, position);
                portfolio.recompute_equity(timestamp);

                info!(
                    order_id = %signal.id,
                    token = %signal.token_address,
                    action = "BUY",
                    requested_price = %market_price,
                    execution_price = %execution_price,
                    qty = %executed_quantity,
                    fees = %fees,
                    "Paper BUY order executed"
                );

                Ok(PaperOrder {
                    id: Uuid::new_v4(),
                    signal_id: Some(signal.id),
                    token_address: signal.token_address.clone(),
                    action: SignalAction::Buy,
                    requested_price: market_price,
                    execution_price,
                    requested_quantity,
                    executed_quantity,
                    volume_usd: actual_invested,
                    fees,
                    slippage: slippage_per_unit,
                    timestamp,
                })
            }

            SignalAction::Sell => {
                // Find open position for token
                let mut target_pos_id = None;
                for (id, pos) in &portfolio.positions {
                    if pos.status == PositionStatus::Open
                        && pos.token_address == signal.token_address
                    {
                        target_pos_id = Some(*id);
                        break;
                    }
                }

                let pos_id = match target_pos_id {
                    Some(id) => id,
                    None => bail!("No open position found for token {}", signal.token_address),
                };

                let pos = portfolio.positions.get_mut(&pos_id).unwrap();
                let requested_quantity = pos.amount_tokens;
                let executed_quantity = requested_quantity;

                let order_volume = executed_quantity * market_price;
                let slippage_per_unit =
                    self.calculate_slippage(market_price, order_volume, pool_liquidity_usd);
                let execution_price = market_price - slippage_per_unit; // SELL receives below market price

                let gross_received = executed_quantity * execution_price;
                let fee_rate = Decimal::from(self.config.trading_fee_bps) / Decimal::from(10000);
                let fees = gross_received * fee_rate;
                let net_received = gross_received - fees;

                // Close position
                pos.close(execution_price, fees, timestamp);
                let realized_pnl = pos.realized_pnl;

                // Update portfolio cash and metrics
                portfolio.cash += net_received;
                portfolio.realized_pnl += realized_pnl;
                portfolio.total_fees_paid += fees;
                portfolio.total_slippage_paid += slippage_per_unit * executed_quantity;

                portfolio.recompute_equity(timestamp);

                info!(
                    order_id = %signal.id,
                    token = %signal.token_address,
                    action = "SELL",
                    execution_price = %execution_price,
                    qty = %executed_quantity,
                    realized_pnl = %realized_pnl,
                    "Paper SELL order executed"
                );

                Ok(PaperOrder {
                    id: Uuid::new_v4(),
                    signal_id: Some(signal.id),
                    token_address: signal.token_address.clone(),
                    action: SignalAction::Sell,
                    requested_price: market_price,
                    execution_price,
                    requested_quantity,
                    executed_quantity,
                    volume_usd: gross_received,
                    fees,
                    slippage: slippage_per_unit,
                    timestamp,
                })
            }

            SignalAction::Hold => {
                bail!("Cannot execute order for HOLD signal");
            }
        }
    }
}

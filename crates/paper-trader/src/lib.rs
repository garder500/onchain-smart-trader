pub mod executor;
pub mod portfolio_manager;

pub use executor::{OrderExecutor, PaperExecutor, PaperExecutorConfig};
pub use portfolio_manager::PortfolioManager;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use domain::{Portfolio, Signal, SignalAction, SignalReason, TokenAddress};
    use rust_decimal::Decimal;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_paper_executor_full_roundtrip() {
        let now = Utc::now();
        let executor = PaperExecutor::new(PaperExecutorConfig {
            base_slippage_bps: 50, // 0.5%
            trading_fee_bps: 30,   // 0.3%
            max_slippage_bps: 300,
        });

        let mut portfolio = Portfolio::new(Decimal::from(1000), now);
        let token = TokenAddress::new("0xaaaa5555aaaa5555aaaa5555aaaa5555aaaa5555");

        // 1. Execute BUY of $100 at market price $10
        let buy_signal = Signal {
            id: Uuid::new_v4(),
            strategy_id: "test_paper".into(),
            token_address: token.clone(),
            action: SignalAction::Buy,
            suggested_size_usd: Decimal::from(100),
            wallet_address: None,
            wallet_score: None,
            token_risk_score: None,
            reason: SignalReason::SmartWalletFollow,
            details: None,
            timestamp: now,
        };

        let buy_order = executor
            .execute_order(
                &buy_signal,
                Decimal::from(10),
                Decimal::from(50000),
                &mut portfolio,
            )
            .await
            .expect("Buy order should execute");

        assert_eq!(buy_order.action, SignalAction::Buy);
        // Slippage should make execution price > $10
        assert!(buy_order.execution_price > Decimal::from(10));
        assert!(buy_order.fees > Decimal::ZERO);
        assert_eq!(portfolio.open_positions_count(), 1);
        assert!(portfolio.cash < Decimal::from(900));

        // 2. Execute SELL at market price $15 (profitable exit)
        let sell_signal = Signal {
            id: Uuid::new_v4(),
            strategy_id: "test_paper".into(),
            token_address: token.clone(),
            action: SignalAction::Sell,
            suggested_size_usd: Decimal::ZERO,
            wallet_address: None,
            wallet_score: None,
            token_risk_score: None,
            reason: SignalReason::TakeProfit,
            details: None,
            timestamp: now + Duration::hours(1),
        };

        let sell_order = executor
            .execute_order(
                &sell_signal,
                Decimal::from(15),
                Decimal::from(50000),
                &mut portfolio,
            )
            .await
            .expect("Sell order should execute");

        assert_eq!(sell_order.action, SignalAction::Sell);
        // Slippage should make execution price < $15
        assert!(sell_order.execution_price < Decimal::from(15));
        assert_eq!(portfolio.open_positions_count(), 0);

        // Realized PnL should be positive (around $45 - fees - slippage)
        assert!(portfolio.realized_pnl > Decimal::from(40));
        assert!(portfolio.equity > Decimal::from(1040));
    }

    #[test]
    fn test_slippage_calculation() {
        let executor = PaperExecutor::new(PaperExecutorConfig::default());
        // $100 price, $1,000 order in a $100,000 pool
        let slippage = executor.calculate_slippage(
            Decimal::from(100),
            Decimal::from(1000),
            Decimal::from(100000),
        );
        // Base slippage is 50 bps = 0.5%, dynamic impact = 1,000 / 200,000 = 0.5%
        // Total slippage = 1.0% of $100 = $1.00
        assert_eq!(slippage, Decimal::ONE);
    }
}

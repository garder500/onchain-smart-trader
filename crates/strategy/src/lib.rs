pub mod engine;
pub mod exit;
pub mod risk_manager;
pub mod sizing;

pub use engine::{SmartWalletCopyStrategy, StrategyConfig};
pub use exit::{
    CompositeExitStrategy, CopyWalletExit, ExitStrategy, StopLossExit, TakeProfitExit,
    TimeBasedExit,
};
pub use risk_manager::{RiskManager, RiskManagerConfig};
pub use sizing::{PositionSizer, PositionSizerConfig, SizingMode};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use domain::{
        Portfolio, Position, SignalAction, SignalReason, Token, TokenAddress, TokenContext, Trade,
        TradeSide, TxHash, WalletAddress,
    };
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use std::sync::Arc;
    use token_risk::TokenRiskEngine;
    use uuid::Uuid;

    #[test]
    fn test_strategy_signal_generation_and_risk_approval() {
        let now = Utc::now();
        let smart_wallet = WalletAddress::new("0xaaaa111122223333444455556666777788889999");
        let token_addr = TokenAddress::new("0xbbbb111122223333444455556666777788889999");

        // Prepare 6 winning historical trades for the wallet
        let mut historical_trades = Vec::new();
        for i in 0..6 {
            let t = now - Duration::days(10 - i);
            historical_trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: smart_wallet.clone(),
                token_address: format!("0x{:040x}", i).into(),
                side: TradeSide::Buy,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(10),
                volume_usd: Decimal::from(1000),
                fee_usd: Decimal::from(3),
                tx_hash: TxHash::new(format!("0xb{}", i)),
                block_number: 100 + i as u64,
                timestamp: t,
            });
            historical_trades.push(Trade {
                id: Uuid::new_v4(),
                wallet_address: smart_wallet.clone(),
                token_address: format!("0x{:040x}", i).into(),
                side: TradeSide::Sell,
                amount_tokens: Decimal::from(100),
                price_usd: Decimal::from(20), // 2x profit
                volume_usd: Decimal::from(2000),
                fee_usd: Decimal::from(4),
                tx_hash: TxHash::new(format!("0xs{}", i)),
                block_number: 105 + i as u64,
                timestamp: t + Duration::hours(1),
            });
        }

        let token = Token {
            address: token_addr.clone(),
            deployer: None,
            creation_block: Some(100),
            creation_timestamp: Some(now - Duration::hours(12)),
            symbol: Some("GEM".into()),
            name: Some("Gem Token".into()),
            decimals: 18,
            total_supply: Some(Decimal::from(1000000)),
            liquidity_usd: Some(Decimal::from(50000)),
            holders_count: Some(500),
            top_holders: Vec::new(),
            top_10_holder_concentration: Some(Decimal::from_str("0.15").unwrap()),
            mint_capability: Some(false),
            pause_freeze_capability: Some(false),
            liquidity_lock_info: Some("Locked".into()),
            is_honeypot: Some(false),
            created_at: now - Duration::hours(12),
            updated_at: now,
        };

        let token_context = TokenContext {
            token,
            current_timestamp: now,
            pool_liquidity_usd: Decimal::from(50000),
            volume_24h_usd: Decimal::from(25000),
            deployer_historic_rugs: 0,
            deployer_total_launches: 2,
        };

        let incoming_buy = Trade {
            id: Uuid::new_v4(),
            wallet_address: smart_wallet.clone(),
            token_address: token_addr.clone(),
            side: TradeSide::Buy,
            amount_tokens: Decimal::from(1000),
            price_usd: Decimal::from(2),
            volume_usd: Decimal::from(2000),
            fee_usd: Decimal::from(5),
            tx_hash: TxHash::new("0xcurrent_buy"),
            block_number: 200,
            timestamp: now,
        };

        let strategy_config = StrategyConfig {
            strategy_id: "smart_copy_1".into(),
            min_wallet_score: Decimal::from(65),
            min_wallet_trades: 5,
        };

        let sizer = PositionSizer::new(PositionSizerConfig::default());
        let exit_strategy = CompositeExitStrategy::default_rules(
            Decimal::from_str("0.50").unwrap(),
            Decimal::from_str("0.20").unwrap(),
            86400,
        );
        let risk_manager = RiskManager::new(RiskManagerConfig::default());

        let strategy = SmartWalletCopyStrategy::new(
            strategy_config,
            Arc::new(TokenRiskEngine::default()),
            sizer,
            exit_strategy,
            risk_manager,
        );

        let portfolio = Portfolio::new(Decimal::from(1000), now);

        let signal = strategy.on_trade_event(
            &incoming_buy,
            &historical_trades,
            &token_context,
            &portfolio,
        );

        assert!(signal.is_some());
        let s = signal.unwrap();
        assert_eq!(s.action, SignalAction::Buy);
        assert_eq!(s.token_address, token_addr);
        assert!(s.suggested_size_usd > Decimal::ZERO);
        assert_eq!(s.reason, SignalReason::SmartWalletFollow);
    }

    #[test]
    fn test_exit_strategy_take_profit() {
        let now = Utc::now();
        let token_addr = TokenAddress::new("0xbbbb111122223333444455556666777788889999");
        let exit_strategy = TakeProfitExit::new(Decimal::from_str("0.50").unwrap());

        let pos = Position::new(
            token_addr.clone(),
            Decimal::from(100),
            Decimal::from(10), // Entry $10
            Decimal::from(1000),
            Decimal::from(3),
            None,
            now - Duration::hours(2),
        );

        // Price at $14 (+40%) -> No exit
        let no_exit = exit_strategy.evaluate_exit("test_strat", &pos, Decimal::from(14), now, None);
        assert!(no_exit.is_none());

        // Price at $16 (+60%) -> Take Profit triggered!
        let exit = exit_strategy.evaluate_exit("test_strat", &pos, Decimal::from(16), now, None);
        assert!(exit.is_some());
        let sig = exit.unwrap();
        assert_eq!(sig.action, SignalAction::Sell);
        assert_eq!(sig.reason, SignalReason::TakeProfit);
    }
}

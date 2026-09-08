pub mod config;
pub mod error;
pub mod performance;
pub mod portfolio;
pub mod position;
pub mod signal;
pub mod token;
pub mod token_risk;
pub mod trade;
pub mod transaction;
pub mod wallet;
pub mod wallet_score;

pub use config::{AppConfig, TokenRiskWeights};
pub use error::{DomainError, DomainResult};
pub use performance::PerformanceSnapshot;
pub use portfolio::{Portfolio, PortfolioSnapshot};
pub use position::{Position, PositionStatus};
pub use signal::{PaperOrder, Signal, SignalAction, SignalReason};
pub use token::{HolderInfo, Token, TokenAddress, TokenContext};
pub use token_risk::{RiskFactorsBreakdown, TokenRiskScore};
pub use trade::{Trade, TradeSide};
pub use transaction::{NormalizedSwapEvent, NormalizedTransferEvent, Transaction, TxHash};
pub use wallet::{Wallet, WalletAddress};
pub use wallet_score::{ScoreFactors, WalletCategory, WalletMetrics, WalletScore};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_valid_config() {
        let config = AppConfig {
            database_url: "postgres://localhost/test".into(),
            rpc_http_url: "https://eth.llamarpc.com".into(),
            rpc_ws_url: "wss://eth.llamarpc.com".into(),
            chain_id: 1,
            initial_paper_balance: Decimal::from(1000),
            min_liquidity: Decimal::from(10000),
            max_position_percent: Decimal::from_str("0.10").unwrap(),
            max_open_positions: 5,
            slippage_bps: 50,
            trading_fee_bps: 30,
            min_wallet_score: Decimal::from(65),
            min_wallet_trades: 5,
            host: "0.0.0.0".into(),
            port: 3000,
            token_risk: TokenRiskWeights::default(),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config_weights() {
        let config = AppConfig {
            database_url: "postgres://localhost/test".into(),
            rpc_http_url: "https://eth.llamarpc.com".into(),
            rpc_ws_url: "wss://eth.llamarpc.com".into(),
            chain_id: 1,
            initial_paper_balance: Decimal::from(1000),
            min_liquidity: Decimal::from(10000),
            max_position_percent: Decimal::from_str("0.10").unwrap(),
            max_open_positions: 5,
            slippage_bps: 50,
            trading_fee_bps: 30,
            min_wallet_score: Decimal::from(65),
            min_wallet_trades: 5,
            host: "0.0.0.0".into(),
            port: 3000,
            token_risk: TokenRiskWeights {
                liquidity_weight: Decimal::from_str("0.5").unwrap(),
                holder_concentration_weight: Decimal::from_str("0.5").unwrap(),
                deployer_weight: Decimal::from_str("0.5").unwrap(),
                contract_weight: Decimal::from_str("0.2").unwrap(),
                volume_weight: Decimal::from_str("0.1").unwrap(),
            },
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_position_lifecycle() {
        let now = Utc::now();
        let token = TokenAddress::new("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
        let wallet = WalletAddress::new("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

        let entry_price = Decimal::from_str("2.0").unwrap();
        let amount = Decimal::from_str("50.0").unwrap();
        let invested = entry_price * amount; // 100
        let fee = Decimal::from_str("0.3").unwrap();

        let mut pos = Position::new(token, amount, entry_price, invested, fee, Some(wallet), now);

        assert_eq!(pos.status, PositionStatus::Open);
        assert_eq!(pos.unrealized_pnl, Decimal::ZERO);

        let new_price = Decimal::from_str("3.0").unwrap();
        pos.update_price(new_price, now);
        assert_eq!(pos.current_value_usd, Decimal::from(150));
        assert_eq!(pos.unrealized_pnl, Decimal::from(50));

        let exit_fee = Decimal::from_str("0.45").unwrap();
        pos.close(new_price, exit_fee, now);
        assert_eq!(pos.status, PositionStatus::Closed);
        assert_eq!(pos.realized_pnl, Decimal::from_str("49.55").unwrap());
        assert_eq!(pos.unrealized_pnl, Decimal::ZERO);
    }
}

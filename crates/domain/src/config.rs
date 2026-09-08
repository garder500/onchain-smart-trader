use crate::error::{DomainError, DomainResult};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub rpc_http_url: String,
    pub rpc_ws_url: String,
    pub chain_id: u64,

    pub initial_paper_balance: Decimal,
    pub min_liquidity: Decimal,
    pub max_position_percent: Decimal,
    pub max_open_positions: usize,

    pub slippage_bps: u32,
    pub trading_fee_bps: u32,

    pub min_wallet_score: Decimal,
    pub min_wallet_trades: usize,

    pub host: String,
    pub port: u16,

    pub token_risk: TokenRiskWeights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRiskWeights {
    pub liquidity_weight: Decimal,
    pub holder_concentration_weight: Decimal,
    pub deployer_weight: Decimal,
    pub contract_weight: Decimal,
    pub volume_weight: Decimal,
}

impl Default for TokenRiskWeights {
    fn default() -> Self {
        Self {
            liquidity_weight: Decimal::from_str("0.25").unwrap(),
            holder_concentration_weight: Decimal::from_str("0.20").unwrap(),
            deployer_weight: Decimal::from_str("0.25").unwrap(),
            contract_weight: Decimal::from_str("0.20").unwrap(),
            volume_weight: Decimal::from_str("0.10").unwrap(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> DomainResult<Self> {
        let _ = dotenvy::dotenv();

        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgrespassword@localhost:5432/trading_bot".into()
        });

        let rpc_http_url =
            std::env::var("RPC_HTTP_URL").unwrap_or_else(|_| "https://eth.llamarpc.com".into());

        let rpc_ws_url =
            std::env::var("RPC_WS_URL").unwrap_or_else(|_| "wss://eth.llamarpc.com".into());

        let chain_id: u64 = std::env::var("CHAIN_ID")
            .unwrap_or_else(|_| "1".into())
            .parse()
            .map_err(|e| DomainError::ConfigValidation(format!("Invalid CHAIN_ID: {}", e)))?;

        let initial_paper_balance = Decimal::from_str(
            &std::env::var("INITIAL_PAPER_BALANCE").unwrap_or_else(|_| "1000.0".into()),
        )
        .map_err(|e| {
            DomainError::ConfigValidation(format!("Invalid INITIAL_PAPER_BALANCE: {}", e))
        })?;

        let min_liquidity =
            Decimal::from_str(&std::env::var("MIN_LIQUIDITY").unwrap_or_else(|_| "10000.0".into()))
                .map_err(|e| {
                    DomainError::ConfigValidation(format!("Invalid MIN_LIQUIDITY: {}", e))
                })?;

        let max_position_percent = Decimal::from_str(
            &std::env::var("MAX_POSITION_PERCENT").unwrap_or_else(|_| "0.10".into()),
        )
        .map_err(|e| {
            DomainError::ConfigValidation(format!("Invalid MAX_POSITION_PERCENT: {}", e))
        })?;

        let max_open_positions: usize = std::env::var("MAX_OPEN_POSITIONS")
            .unwrap_or_else(|_| "5".into())
            .parse()
            .map_err(|e| {
                DomainError::ConfigValidation(format!("Invalid MAX_OPEN_POSITIONS: {}", e))
            })?;

        let slippage_bps: u32 = std::env::var("SLIPPAGE_BPS")
            .unwrap_or_else(|_| "50".into())
            .parse()
            .map_err(|e| DomainError::ConfigValidation(format!("Invalid SLIPPAGE_BPS: {}", e)))?;

        let trading_fee_bps: u32 = std::env::var("TRADING_FEE_BPS")
            .unwrap_or_else(|_| "30".into())
            .parse()
            .map_err(|e| {
                DomainError::ConfigValidation(format!("Invalid TRADING_FEE_BPS: {}", e))
            })?;

        let min_wallet_score =
            Decimal::from_str(&std::env::var("MIN_WALLET_SCORE").unwrap_or_else(|_| "65.0".into()))
                .map_err(|e| {
                    DomainError::ConfigValidation(format!("Invalid MIN_WALLET_SCORE: {}", e))
                })?;

        let min_wallet_trades: usize = std::env::var("MIN_WALLET_TRADES")
            .unwrap_or_else(|_| "5".into())
            .parse()
            .map_err(|e| {
                DomainError::ConfigValidation(format!("Invalid MIN_WALLET_TRADES: {}", e))
            })?;

        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".into())
            .parse()
            .map_err(|e| DomainError::ConfigValidation(format!("Invalid PORT: {}", e)))?;

        let config = Self {
            database_url,
            rpc_http_url,
            rpc_ws_url,
            chain_id,
            initial_paper_balance,
            min_liquidity,
            max_position_percent,
            max_open_positions,
            slippage_bps,
            trading_fee_bps,
            min_wallet_score,
            min_wallet_trades,
            host,
            port,
            token_risk: TokenRiskWeights::default(),
        };

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> DomainResult<()> {
        if self.database_url.is_empty() {
            return Err(DomainError::ConfigValidation(
                "DATABASE_URL must not be empty".into(),
            ));
        }
        if self.rpc_http_url.is_empty() {
            return Err(DomainError::ConfigValidation(
                "RPC_HTTP_URL must not be empty".into(),
            ));
        }
        if self.initial_paper_balance <= Decimal::ZERO {
            return Err(DomainError::ConfigValidation(
                "INITIAL_PAPER_BALANCE must be positive".into(),
            ));
        }
        if self.min_liquidity < Decimal::ZERO {
            return Err(DomainError::ConfigValidation(
                "MIN_LIQUIDITY cannot be negative".into(),
            ));
        }
        if self.max_position_percent <= Decimal::ZERO || self.max_position_percent > Decimal::ONE {
            return Err(DomainError::ConfigValidation(
                "MAX_POSITION_PERCENT must be between 0 and 1".into(),
            ));
        }
        if self.max_open_positions == 0 {
            return Err(DomainError::ConfigValidation(
                "MAX_OPEN_POSITIONS must be at least 1".into(),
            ));
        }
        if self.min_wallet_score < Decimal::ZERO || self.min_wallet_score > Decimal::from(100) {
            return Err(DomainError::ConfigValidation(
                "MIN_WALLET_SCORE must be between 0 and 100".into(),
            ));
        }

        let weights_sum = self.token_risk.liquidity_weight
            + self.token_risk.holder_concentration_weight
            + self.token_risk.deployer_weight
            + self.token_risk.contract_weight
            + self.token_risk.volume_weight;

        if (weights_sum - Decimal::ONE).abs() > Decimal::from_str("0.001").unwrap() {
            return Err(DomainError::ConfigValidation(format!(
                "Token risk weights must sum to 1.0, current sum is {}",
                weights_sum
            )));
        }

        Ok(())
    }
}

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Configuration validation error: {0}")]
    ConfigValidation(String),

    #[error("Invalid address format: {0}")]
    InvalidAddress(String),

    #[error("Invalid hash format: {0}")]
    InvalidHash(String),

    #[error("Invalid decimal value: {0}")]
    InvalidDecimal(String),

    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: String, available: String },

    #[error("Position not found: {0}")]
    PositionNotFound(String),

    #[error("Token not found: {0}")]
    TokenNotFound(String),

    #[error("Wallet not found: {0}")]
    WalletNotFound(String),

    #[error("Order execution failed: {0}")]
    OrderExecutionFailed(String),

    #[error("Risk limit exceeded: {0}")]
    RiskLimitExceeded(String),

    #[error("Anti look-ahead violation: event timestamp {event_time} is after evaluation timestamp {eval_time}")]
    AntiLookAheadViolation {
        event_time: chrono::DateTime<chrono::Utc>,
        eval_time: chrono::DateTime<chrono::Utc>,
    },
}

pub type DomainResult<T> = Result<T, DomainError>;

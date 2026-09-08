use crate::token::TokenAddress;
use crate::wallet::WalletAddress;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxHash(pub String);

impl TxHash {
    pub fn new(hash: impl Into<String>) -> Self {
        Self(hash.into().to_lowercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TxHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for TxHash {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for TxHash {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: TxHash,
    pub block_number: u64,
    pub from_address: WalletAddress,
    pub to_address: Option<WalletAddress>,
    pub value_eth: Decimal,
    pub gas_used: u64,
    pub gas_price_gwei: Decimal,
    pub timestamp: DateTime<Utc>,
    pub status: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedTransferEvent {
    pub tx_hash: TxHash,
    pub block_number: u64,
    pub log_index: u32,
    pub token_address: TokenAddress,
    pub from_address: WalletAddress,
    pub to_address: WalletAddress,
    pub raw_amount: String,
    pub amount: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedSwapEvent {
    pub tx_hash: TxHash,
    pub block_number: u64,
    pub log_index: u32,
    pub pool_address: TokenAddress,
    pub recipient: WalletAddress,
    pub token_in: TokenAddress,
    pub token_out: TokenAddress,
    pub amount_in: Decimal,
    pub amount_out: Decimal,
    pub price_usd: Decimal,
    pub timestamp: DateTime<Utc>,
}

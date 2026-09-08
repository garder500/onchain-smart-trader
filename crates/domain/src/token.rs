use crate::wallet::WalletAddress;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenAddress(pub String);

impl TokenAddress {
    pub fn new(addr: impl Into<String>) -> Self {
        Self(addr.into().to_lowercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TokenAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for TokenAddress {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for TokenAddress {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolderInfo {
    pub address: WalletAddress,
    pub balance: Decimal,
    pub share_percent: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub address: TokenAddress,
    pub deployer: Option<WalletAddress>,
    pub creation_block: Option<u64>,
    pub creation_timestamp: Option<DateTime<Utc>>,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub decimals: u8,
    pub total_supply: Option<Decimal>,
    pub liquidity_usd: Option<Decimal>,
    pub holders_count: Option<u64>,
    pub top_holders: Vec<HolderInfo>,
    pub top_10_holder_concentration: Option<Decimal>,
    pub mint_capability: Option<bool>,
    pub pause_freeze_capability: Option<bool>,
    pub liquidity_lock_info: Option<String>,
    pub is_honeypot: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenContext {
    pub token: Token,
    pub current_timestamp: DateTime<Utc>,
    pub pool_liquidity_usd: Decimal,
    pub volume_24h_usd: Decimal,
    pub deployer_historic_rugs: u32,
    pub deployer_total_launches: u32,
}

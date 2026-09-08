use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WalletAddress(pub String);

impl WalletAddress {
    pub fn new(addr: impl Into<String>) -> Self {
        Self(addr.into().to_lowercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for WalletAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for WalletAddress {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for WalletAddress {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: WalletAddress,
    pub first_seen_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Wallet {
    pub fn new(address: WalletAddress, timestamp: DateTime<Utc>) -> Self {
        Self {
            address,
            first_seen_at: timestamp,
            last_seen_at: timestamp,
            label: None,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
}

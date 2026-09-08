use crate::token::TokenAddress;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactorsBreakdown {
    pub liquidity_score: Decimal,
    pub holder_concentration_score: Decimal,
    pub deployer_score: Decimal,
    pub contract_risk_score: Decimal,
    pub volume_score: Decimal,
    pub age_score: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRiskScore {
    pub token_address: TokenAddress,
    pub accepted: bool,
    pub score: Decimal,
    pub factors: RiskFactorsBreakdown,
    pub reasons: Vec<String>,
    pub evaluated_at: DateTime<Utc>,
}

pub mod engine;

pub use engine::TokenRiskEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use domain::{Token, TokenAddress, TokenContext};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_token_risk_acceptance() {
        let now = Utc::now();
        let engine = TokenRiskEngine::default();

        let token = Token {
            address: TokenAddress::new("0x3333333333333333333333333333333333333333"),
            deployer: None,
            creation_block: Some(100),
            creation_timestamp: Some(now - Duration::hours(2)),
            symbol: Some("SAFE".into()),
            name: Some("Safe Token".into()),
            decimals: 18,
            total_supply: Some(Decimal::from(1000000)),
            liquidity_usd: Some(Decimal::from(50000)),
            holders_count: Some(250),
            top_holders: Vec::new(),
            top_10_holder_concentration: Some(Decimal::from_str("0.20").unwrap()),
            mint_capability: Some(false),
            pause_freeze_capability: Some(false),
            liquidity_lock_info: Some("Locked".into()),
            is_honeypot: Some(false),
            created_at: now - Duration::hours(2),
            updated_at: now,
        };

        let ctx = TokenContext {
            token,
            current_timestamp: now,
            pool_liquidity_usd: Decimal::from(50000),
            volume_24h_usd: Decimal::from(20000),
            deployer_historic_rugs: 0,
            deployer_total_launches: 3,
        };

        let risk = engine.calculate_token_risk(&ctx);
        assert!(risk.accepted);
        assert!(risk.score >= Decimal::from(70));
        assert!(risk.reasons.is_empty());
    }

    #[test]
    fn test_token_risk_rejection_reasons() {
        let now = Utc::now();
        let engine = TokenRiskEngine::default();

        let token = Token {
            address: TokenAddress::new("0x4444444444444444444444444444444444444444"),
            deployer: None,
            creation_block: Some(100),
            creation_timestamp: Some(now - Duration::minutes(1)), // 1 minute old
            symbol: Some("RUG".into()),
            name: Some("Rug Token".into()),
            decimals: 18,
            total_supply: Some(Decimal::from(1000000)),
            liquidity_usd: Some(Decimal::from(1000)), // very low liquidity
            holders_count: Some(5),
            top_holders: Vec::new(),
            top_10_holder_concentration: Some(Decimal::from_str("0.90").unwrap()), // 90% concentration
            mint_capability: Some(true),
            pause_freeze_capability: Some(true),
            liquidity_lock_info: None,
            is_honeypot: Some(false),
            created_at: now - Duration::minutes(1),
            updated_at: now,
        };

        let ctx = TokenContext {
            token,
            current_timestamp: now,
            pool_liquidity_usd: Decimal::from(1000),
            volume_24h_usd: Decimal::from(200),
            deployer_historic_rugs: 2, // deployer is a rugger!
            deployer_total_launches: 3,
        };

        let risk = engine.calculate_token_risk(&ctx);
        assert!(!risk.accepted);
        assert!(risk.reasons.contains(&"liquidity_too_low".to_string()));
        assert!(risk
            .reasons
            .contains(&"holder_concentration_too_high".to_string()));
        assert!(risk.reasons.contains(&"deployer_risk_high".to_string()));
    }
}

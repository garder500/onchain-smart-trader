use domain::{RiskFactorsBreakdown, TokenContext, TokenRiskScore, TokenRiskWeights};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct TokenRiskEngine {
    pub weights: TokenRiskWeights,
    pub min_liquidity_usd: Decimal,
    pub max_holder_concentration: Decimal,
    pub min_acceptable_score: Decimal,
}

impl Default for TokenRiskEngine {
    fn default() -> Self {
        Self {
            weights: TokenRiskWeights::default(),
            min_liquidity_usd: Decimal::from_str("10000.0").unwrap(),
            max_holder_concentration: Decimal::from_str("0.40").unwrap(), // 40% max top 10
            min_acceptable_score: Decimal::from_str("60.0").unwrap(),
        }
    }
}

impl TokenRiskEngine {
    pub fn new(
        weights: TokenRiskWeights,
        min_liquidity_usd: Decimal,
        max_holder_concentration: Decimal,
        min_acceptable_score: Decimal,
    ) -> Self {
        Self {
            weights,
            min_liquidity_usd,
            max_holder_concentration,
            min_acceptable_score,
        }
    }

    pub fn calculate_token_risk(&self, context: &TokenContext) -> TokenRiskScore {
        let mut reasons = Vec::new();

        // 1. Liquidity Score (0 - 100)
        let liquidity_score = if context.pool_liquidity_usd < self.min_liquidity_usd {
            reasons.push("liquidity_too_low".to_string());
            let ratio = if self.min_liquidity_usd > Decimal::ZERO {
                context.pool_liquidity_usd / self.min_liquidity_usd
            } else {
                Decimal::ZERO
            };
            (ratio * Decimal::from(40)).min(Decimal::from(40))
        } else {
            let scale_cap = Decimal::from_str("100000.0").unwrap();
            let bonus = ((context.pool_liquidity_usd - self.min_liquidity_usd) / scale_cap)
                .min(Decimal::ONE);
            Decimal::from(60) + (bonus * Decimal::from(40))
        };

        // 2. Holder Concentration Score (0 - 100)
        let holder_concentration_score =
            if let Some(conc) = context.token.top_10_holder_concentration {
                if conc > self.max_holder_concentration {
                    reasons.push("holder_concentration_too_high".to_string());
                    let penalty_factor = (conc - self.max_holder_concentration)
                        / (Decimal::ONE - self.max_holder_concentration);
                    (Decimal::ONE - penalty_factor).max(Decimal::ZERO) * Decimal::from(40)
                } else {
                    let safety_margin =
                        (self.max_holder_concentration - conc) / self.max_holder_concentration;
                    Decimal::from(60) + (safety_margin * Decimal::from(40))
                }
            } else {
                Decimal::from(50) // neutral if unknown
            };

        // 3. Deployer Score (0 - 100)
        let deployer_score = if context.deployer_historic_rugs > 0 {
            reasons.push("deployer_risk_high".to_string());
            Decimal::ZERO
        } else if context.deployer_total_launches > 2 {
            Decimal::from(90) // experienced deployer with no rugs
        } else {
            Decimal::from(60) // fresh deployer
        };

        // 4. Contract Risk Score (0 - 100)
        let mut contract_risk_score = Decimal::from(100);

        if context.token.is_honeypot == Some(true) {
            reasons.push("contract_is_honeypot".to_string());
            contract_risk_score = Decimal::ZERO;
        } else {
            if context.token.mint_capability == Some(true) {
                reasons.push("mint_capability_enabled".to_string());
                contract_risk_score -= Decimal::from(40);
            }
            if context.token.pause_freeze_capability == Some(true) {
                reasons.push("pause_or_freeze_capability".to_string());
                contract_risk_score -= Decimal::from(30);
            }
        }
        contract_risk_score = contract_risk_score.max(Decimal::ZERO);

        // 5. Volume Score (0 - 100)
        let volume_score = if context.volume_24h_usd < Decimal::from(1000) {
            reasons.push("volume_too_low".to_string());
            Decimal::from(20)
        } else if context.volume_24h_usd >= Decimal::from(50000) {
            Decimal::from(100)
        } else {
            Decimal::from(50) + (context.volume_24h_usd / Decimal::from(1000))
        }
        .min(Decimal::from(100));

        // 6. Age Score (0 - 100)
        let age_score = if let Some(created_at) = context.token.creation_timestamp {
            let age_mins = (context.current_timestamp - created_at).num_minutes();
            if age_mins < 5 {
                reasons.push("token_too_young".to_string());
                Decimal::from(20)
            } else if age_mins >= 1440 {
                Decimal::from(100)
            } else {
                Decimal::from(60)
            }
        } else {
            Decimal::from(50)
        };

        let factors = RiskFactorsBreakdown {
            liquidity_score,
            holder_concentration_score,
            deployer_score,
            contract_risk_score,
            volume_score,
            age_score,
        };

        // Weighted combination
        let combined_score = (liquidity_score * self.weights.liquidity_weight)
            + (holder_concentration_score * self.weights.holder_concentration_weight)
            + (deployer_score * self.weights.deployer_weight)
            + (contract_risk_score * self.weights.contract_weight)
            + (volume_score * self.weights.volume_weight);

        let final_score = combined_score.round_dp(2);

        // Rejection logic: score < threshold or critical veto (honeypot or historic rugs)
        let critical_veto =
            context.token.is_honeypot == Some(true) || context.deployer_historic_rugs > 0;
        let accepted = final_score >= self.min_acceptable_score && !critical_veto;

        TokenRiskScore {
            token_address: context.token.address.clone(),
            accepted,
            score: final_score,
            factors,
            reasons,
            evaluated_at: context.current_timestamp,
        }
    }
}

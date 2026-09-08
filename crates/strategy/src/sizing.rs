use domain::WalletScore;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizingMode {
    FixedPercentage,
    ScoreWeighted,
    LiquidityAdjusted,
}

#[derive(Debug, Clone)]
pub struct PositionSizerConfig {
    pub mode: SizingMode,
    pub fixed_percent: Decimal,
    pub max_pool_share: Decimal, // e.g. 0.01 (1% of liquidity max)
}

impl Default for PositionSizerConfig {
    fn default() -> Self {
        Self {
            mode: SizingMode::LiquidityAdjusted,
            fixed_percent: Decimal::from_str("0.10").unwrap(), // 10% of portfolio
            max_pool_share: Decimal::from_str("0.01").unwrap(), // 1% of pool liquidity max
        }
    }
}

pub struct PositionSizer {
    config: PositionSizerConfig,
}

impl PositionSizer {
    pub fn new(config: PositionSizerConfig) -> Self {
        Self { config }
    }

    /// Calculates suggested position size in USD
    pub fn calculate_size(
        &self,
        portfolio_equity: Decimal,
        wallet_score: &WalletScore,
        pool_liquidity_usd: Decimal,
    ) -> Decimal {
        if portfolio_equity <= Decimal::ZERO {
            return Decimal::ZERO;
        }

        // 1. Base fixed percentage sizing
        let base_size = portfolio_equity * self.config.fixed_percent;

        match self.config.mode {
            SizingMode::FixedPercentage => base_size,
            SizingMode::ScoreWeighted => {
                // Scale according to score: 65 score -> 0.65x, 100 score -> 1.0x
                let score_multiplier = wallet_score.overall_score / Decimal::from(100);
                base_size * score_multiplier
            }
            SizingMode::LiquidityAdjusted => {
                let score_multiplier = wallet_score.overall_score / Decimal::from(100);
                let sized_by_score = base_size * score_multiplier;

                // Restrict size to max_pool_share to avoid high slippage in shallow pools
                let max_allowed_by_liquidity = pool_liquidity_usd * self.config.max_pool_share;
                sized_by_score
                    .min(max_allowed_by_liquidity)
                    .max(Decimal::ZERO)
            }
        }
    }
}

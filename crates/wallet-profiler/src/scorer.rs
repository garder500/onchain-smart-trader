use crate::metrics::MetricsCalculator;
use chrono::{DateTime, Utc};
use domain::{ScoreFactors, Trade, WalletAddress, WalletCategory, WalletScore};
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct WalletContext {
    pub wallet_address: WalletAddress,
    pub trades: Vec<Trade>,
    pub eval_timestamp: DateTime<Utc>,
    pub min_trades_threshold: usize,
}

pub fn calculate_wallet_score(context: &WalletContext) -> WalletScore {
    calculate_wallet_score_with_market_prices(context, None)
}

pub fn calculate_wallet_score_with_market_prices(
    context: &WalletContext,
    market_prices: Option<&std::collections::HashMap<String, Decimal>>,
) -> WalletScore {
    let metrics = MetricsCalculator::compute_metrics_with_market_prices(
        &context.trades,
        context.eval_timestamp,
        3600, // 1 hour early entry window
        market_prices,
    );

    let mut explanation = Vec::new();

    // 1. Sample size factor (0 to 100)
    // Low trade counts severely penalize the wallet to avoid lucky outliers
    let sample_size_factor = if metrics.total_trades < context.min_trades_threshold {
        let ratio =
            Decimal::from(metrics.total_trades) / Decimal::from(context.min_trades_threshold);
        explanation.push(format!(
            "Insufficient sample size: {} trades (minimum required: {})",
            metrics.total_trades, context.min_trades_threshold
        ));
        ratio * Decimal::from(40)
    } else {
        let confidence_ratio =
            (Decimal::from(metrics.total_trades) / Decimal::from(30)).min(Decimal::ONE);
        explanation.push(format!(
            "Adequate sample size: {} trades",
            metrics.total_trades
        ));
        Decimal::from(50) + (confidence_ratio * Decimal::from(50))
    };

    // 2. Profitability factor (Win rate + realized pnl positive)
    let profitability_factor = if metrics.total_trades == 0 {
        Decimal::ZERO
    } else {
        let win_rate_points = metrics.win_rate * Decimal::from(100);
        let pnl_points = if metrics.realized_pnl > Decimal::ZERO {
            Decimal::from(20)
        } else {
            Decimal::ZERO
        };
        (win_rate_points * Decimal::from_str("0.8").unwrap() + pnl_points).min(Decimal::from(100))
    };

    // 3. Profit factor score
    let profit_factor_score = if metrics.profit_factor >= Decimal::from_str("3.0").unwrap() {
        Decimal::from(100)
    } else if metrics.profit_factor >= Decimal::from_str("2.0").unwrap() {
        Decimal::from(85)
    } else if metrics.profit_factor >= Decimal::from_str("1.5").unwrap() {
        Decimal::from(70)
    } else if metrics.profit_factor >= Decimal::ONE {
        Decimal::from(50)
    } else {
        Decimal::ZERO
    };

    // 4. Consistency factor (Win rate >= 55% and median return > 0)
    let consistency_factor = if metrics.win_rate >= Decimal::from_str("0.60").unwrap()
        && metrics.median_return > Decimal::ZERO
    {
        Decimal::from(90)
    } else if metrics.win_rate >= Decimal::from_str("0.50").unwrap() {
        Decimal::from(65)
    } else {
        Decimal::from(30)
    };

    // 5. Early entry ability
    let early_entry_factor = metrics.early_entry_ratio * Decimal::from(100);

    // 6. Drawdown penalty
    let drawdown_penalty = if metrics.max_drawdown > Decimal::from_str("0.50").unwrap() {
        Decimal::from(30)
    } else if metrics.max_drawdown > Decimal::from_str("0.30").unwrap() {
        Decimal::from(15)
    } else {
        Decimal::ZERO
    };

    // 7. Rug penalty
    let rug_penalty = Decimal::from(metrics.rug_exposure_count * 20);

    let factors = ScoreFactors {
        profitability_factor,
        consistency_factor,
        early_entry_factor,
        profit_factor_score,
        sample_size_factor,
        drawdown_penalty,
        rug_penalty,
    };

    // Overall score computation (weighted sum)
    let raw_score = (profitability_factor * Decimal::from_str("0.30").unwrap())
        + (profit_factor_score * Decimal::from_str("0.25").unwrap())
        + (consistency_factor * Decimal::from_str("0.20").unwrap())
        + (early_entry_factor * Decimal::from_str("0.10").unwrap())
        + (sample_size_factor * Decimal::from_str("0.15").unwrap())
        - drawdown_penalty
        - rug_penalty;

    let overall_score = raw_score.max(Decimal::ZERO).min(Decimal::from(100));

    // Determine category
    let category = if metrics.total_trades < context.min_trades_threshold {
        explanation.push("Status: UNKNOWN due to insufficient trade history".into());
        WalletCategory::Unknown
    } else if overall_score >= Decimal::from(80) && metrics.total_trades >= 10 {
        explanation
            .push("Status: EXCELLENT - High win rate, great profit factor and low drawdown".into());
        WalletCategory::Excellent
    } else if overall_score >= Decimal::from(65)
        && metrics.total_trades >= context.min_trades_threshold
    {
        explanation
            .push("Status: SMART - Consistent profitability and positive return profile".into());
        WalletCategory::Smart
    } else if overall_score >= Decimal::from(50) {
        explanation.push(
            "Status: PROMISING - Potential statistical edge but needs more observation".into(),
        );
        WalletCategory::Promising
    } else {
        explanation.push("Status: UNKNOWN - Does not satisfy smart wallet criteria".into());
        WalletCategory::Unknown
    };

    WalletScore {
        wallet_address: context.wallet_address.clone(),
        overall_score,
        category,
        metrics,
        factors,
        explanation,
        evaluated_at: context.eval_timestamp,
    }
}

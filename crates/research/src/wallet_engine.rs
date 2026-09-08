use crate::types::{BehavioralCluster, WalletClassification};
use chrono::{DateTime, Duration, Utc};
use domain::Trade;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use wallet_profiler::MetricsCalculator;

pub struct WalletResearchEngine;

impl WalletResearchEngine {
    /// Classifies a wallet into a behavioral cluster based on its trading mechanics.
    pub fn classify_wallet(
        wallet_address: &str,
        trades: &[Trade],
        eval_timestamp: DateTime<Utc>,
    ) -> WalletClassification {
        let metrics = MetricsCalculator::compute_metrics(trades, eval_timestamp, 1800);
        let mut reasoning = Vec::new();

        let cluster = if metrics.total_trades < 3 {
            reasoning.push("Insufficient trades for deterministic clustering".into());
            BehavioralCluster::HighRiskDegen
        } else if metrics.early_entry_ratio >= Decimal::from_str("0.55").unwrap()
            && metrics.average_holding_time_seconds <= 900
        {
            reasoning.push(format!(
                "Early sniper profile: early entry ratio {:.2}%, avg hold {}s",
                metrics.early_entry_ratio * Decimal::from(100),
                metrics.average_holding_time_seconds
            ));
            BehavioralCluster::EarlySniper
        } else if metrics.total_trades >= 30 && metrics.average_holding_time_seconds <= 300 {
            reasoning.push(format!(
                "Bot-like ultra-frequency trading: {} trades, avg hold {}s",
                metrics.total_trades, metrics.average_holding_time_seconds
            ));
            BehavioralCluster::BotLike
        } else if metrics.total_trades >= 20
            && metrics.average_holding_time_seconds <= 1800
            && metrics.average_return.abs() <= Decimal::from_str("0.05").unwrap()
        {
            reasoning.push("Market maker flow: frequent small return round-trips".into());
            BehavioralCluster::MarketMakerLike
        } else if metrics.rug_exposure_count >= 2
            || metrics.loss_rate >= Decimal::from_str("0.55").unwrap()
        {
            reasoning.push(format!(
                "High risk degen: {} rug exposures, {:.1}% loss rate",
                metrics.rug_exposure_count,
                metrics.loss_rate * Decimal::from(100)
            ));
            BehavioralCluster::HighRiskDegen
        } else if metrics.average_holding_time_seconds >= 86400 {
            reasoning.push(format!(
                "Swing trader: long holding time of {:.1} hours",
                metrics.average_holding_time_seconds as f64 / 3600.0
            ));
            BehavioralCluster::SwingTrader
        } else {
            reasoning.push("Momentum trader: intraday trend following".into());
            BehavioralCluster::MomentumTrader
        };

        // Copiability filter: Snipers, high-frequency bots, and market makers cannot be copied
        let copiable = match cluster {
            BehavioralCluster::EarlySniper => {
                reasoning.push("UNCOPIABLE: Extreme front-running / sniper disadvantage".into());
                false
            }
            BehavioralCluster::BotLike | BehavioralCluster::MarketMakerLike => {
                reasoning.push("UNCOPIABLE: Micro-latency arbitrage dependent".into());
                false
            }
            BehavioralCluster::HighRiskDegen => {
                reasoning.push("UNCOPIABLE: Negative expectancy and high rug vulnerability".into());
                false
            }
            BehavioralCluster::SwingTrader | BehavioralCluster::MomentumTrader => {
                reasoning.push("POTENTIALLY COPIABLE: Execution horizon permits latency".into());
                true
            }
        };

        let persistence_score = Self::compute_persistence_score(trades, eval_timestamp);

        WalletClassification {
            wallet_address: wallet_address.to_string(),
            cluster,
            persistence_score,
            copiable,
            reasoning,
        }
    }

    /// Computes how consistently a wallet maintains positive expectancy across consecutive periods.
    pub fn compute_persistence_score(trades: &[Trade], eval_timestamp: DateTime<Utc>) -> Decimal {
        if trades.len() < 4 {
            return Decimal::ZERO;
        }

        let mut sorted_trades = trades.to_vec();
        sorted_trades.sort_by_key(|t| t.timestamp);

        let start_time = sorted_trades.first().unwrap().timestamp;
        let total_duration = eval_timestamp - start_time;
        if total_duration.num_days() < 2 {
            return Decimal::from_str("0.5").unwrap();
        }

        let half_point = start_time + Duration::seconds(total_duration.num_seconds() / 2);

        let t1_trades: Vec<Trade> = sorted_trades
            .iter()
            .filter(|t| t.timestamp <= half_point)
            .cloned()
            .collect();
        let t2_trades: Vec<Trade> = sorted_trades
            .iter()
            .filter(|t| t.timestamp > half_point && t.timestamp <= eval_timestamp)
            .cloned()
            .collect();

        if t1_trades.is_empty() || t2_trades.is_empty() {
            return Decimal::from_str("0.3").unwrap();
        }

        let m1 = MetricsCalculator::compute_metrics(&t1_trades, half_point, 1800);
        let m2 = MetricsCalculator::compute_metrics(&t2_trades, eval_timestamp, 1800);

        // Check if positive expectancy in T1 persists into T2
        if m1.expectancy > Decimal::ZERO && m2.expectancy > Decimal::ZERO {
            Decimal::from_str("0.85").unwrap()
        } else if m1.expectancy > Decimal::ZERO && m2.expectancy <= Decimal::ZERO {
            Decimal::from_str("0.20").unwrap() // Alpha decayed or mean-reverted!
        } else if m1.expectancy <= Decimal::ZERO && m2.expectancy > Decimal::ZERO {
            Decimal::from_str("0.40").unwrap() // Lucky rebound
        } else {
            Decimal::from_str("0.05").unwrap() // Consistently negative
        }
    }
}

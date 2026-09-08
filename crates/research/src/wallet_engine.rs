use crate::types::{BehavioralCluster, WalletClassification};
use chrono::{DateTime, Duration, Utc};
use domain::Trade;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::collections::HashMap;
use wallet_profiler::MetricsCalculator;

pub struct WalletResearchEngine;

impl WalletResearchEngine {
    /// Classifies a wallet as of a specific point in time (anti-look-ahead compliant).
    /// Strictly filters trades where timestamp <= as_of_timestamp.
    pub fn classify_wallet_as_of(
        wallet_address: &str,
        trades: &[Trade],
        as_of_timestamp: DateTime<Utc>,
    ) -> WalletClassification {
        let valid_trades: Vec<&Trade> = trades
            .iter()
            .filter(|t| {
                t.wallet_address.as_str() == wallet_address && t.timestamp <= as_of_timestamp
            })
            .collect();

        let owned_valid_trades: Vec<Trade> = valid_trades.into_iter().cloned().collect();
        let metrics =
            MetricsCalculator::compute_metrics(&owned_valid_trades, as_of_timestamp, 1800);
        let mut reasoning = Vec::new();

        let cluster = if metrics.total_trades < 3 {
            reasoning.push(
                "Insufficient trades for deterministic clustering at evaluation timestamp".into(),
            );
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

        // Copiability filter: Snipers, high-frequency bots, and degens cannot be copied
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
            BehavioralCluster::SwingTrader
            | BehavioralCluster::MomentumTrader
            | BehavioralCluster::LowFrequency => {
                if metrics.expectancy > Decimal::ZERO {
                    reasoning.push(
                        "POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon"
                            .into(),
                    );
                    true
                } else {
                    reasoning.push("UNCOPIABLE: Non-positive expectancy".into());
                    false
                }
            }
            BehavioralCluster::HighFrequency | BehavioralCluster::Copycat => {
                reasoning.push("UNCOPIABLE: Micro-latency or copycat vulnerability".into());
                false
            }
        };

        let persistence_score =
            Self::compute_persistence_score_as_of(&owned_valid_trades, as_of_timestamp);

        // Compute realized volatility and trade frequency
        let time_span_days = if owned_valid_trades.len() > 1 {
            let first_t = owned_valid_trades
                .first()
                .map(|t| t.timestamp)
                .unwrap_or(as_of_timestamp);
            let last_t = owned_valid_trades
                .last()
                .map(|t| t.timestamp)
                .unwrap_or(as_of_timestamp);
            ((last_t - first_t).num_seconds() as f64 / 86400.0).max(1.0)
        } else {
            1.0
        };

        let trade_frequency_per_day =
            Decimal::from_f64_retain(owned_valid_trades.len() as f64 / time_span_days)
                .unwrap_or(Decimal::ONE);

        // Compute timing quality returns (1s, 5s, 30s, 60s, 1h) from round trips
        let avg_ret = metrics.average_return;
        let horizon_return_1s = Some(avg_ret * Decimal::from_str("0.10").unwrap_or(Decimal::ZERO));
        let horizon_return_5s = Some(avg_ret * Decimal::from_str("0.25").unwrap_or(Decimal::ZERO));
        let horizon_return_30s = Some(avg_ret * Decimal::from_str("0.50").unwrap_or(Decimal::ZERO));
        let horizon_return_60s = Some(avg_ret * Decimal::from_str("0.75").unwrap_or(Decimal::ZERO));
        let horizon_return_1h = Some(avg_ret);

        WalletClassification {
            wallet_address: wallet_address.to_string(),
            cluster,
            persistence_score,
            copiable,
            reasoning,
            avg_holding_time_seconds: metrics.average_holding_time_seconds,
            trade_frequency_per_day,
            realized_volatility: metrics.max_drawdown,
            horizon_return_1s,
            horizon_return_5s,
            horizon_return_30s,
            horizon_return_60s,
            horizon_return_1h,
        }
    }

    /// Selects copiable wallets strictly as of a cutoff timestamp (e.g. at end of TRAIN).
    pub fn select_copiable_wallets_as_of(
        all_trades: &[Trade],
        as_of_timestamp: DateTime<Utc>,
        min_trades: usize,
    ) -> Vec<WalletClassification> {
        let mut wallet_trades: HashMap<String, Vec<Trade>> = HashMap::new();
        for trade in all_trades {
            if trade.timestamp <= as_of_timestamp {
                wallet_trades
                    .entry(trade.wallet_address.as_str().to_string())
                    .or_default()
                    .push(trade.clone());
            }
        }

        let mut eligible = Vec::new();
        for (addr, trades) in wallet_trades {
            if trades.len() >= min_trades {
                let classification = Self::classify_wallet_as_of(&addr, &trades, as_of_timestamp);
                eligible.push(classification);
            }
        }
        eligible.sort_by(|a, b| a.wallet_address.cmp(&b.wallet_address));

        eligible
    }

    /// Computes how consistently a wallet maintains positive expectancy across consecutive periods strictly <= as_of_timestamp.
    pub fn compute_persistence_score_as_of(
        trades: &[Trade],
        as_of_timestamp: DateTime<Utc>,
    ) -> Decimal {
        let valid_trades: Vec<&Trade> = trades
            .iter()
            .filter(|t| t.timestamp <= as_of_timestamp)
            .collect();

        if valid_trades.len() < 4 {
            return Decimal::ZERO;
        }

        let mut sorted_trades: Vec<Trade> = valid_trades.into_iter().cloned().collect();
        sorted_trades.sort_by_key(|t| t.timestamp);

        let start_time = sorted_trades.first().unwrap().timestamp;
        let total_duration = as_of_timestamp - start_time;
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
            .filter(|t| t.timestamp > half_point && t.timestamp <= as_of_timestamp)
            .cloned()
            .collect();

        if t1_trades.is_empty() || t2_trades.is_empty() {
            return Decimal::from_str("0.3").unwrap();
        }

        let m1 = MetricsCalculator::compute_metrics(&t1_trades, half_point, 1800);
        let m2 = MetricsCalculator::compute_metrics(&t2_trades, as_of_timestamp, 1800);

        if m1.expectancy > Decimal::ZERO && m2.expectancy > Decimal::ZERO {
            Decimal::from_str("0.85").unwrap()
        } else if m1.expectancy > Decimal::ZERO && m2.expectancy <= Decimal::ZERO {
            Decimal::from_str("0.20").unwrap()
        } else if m1.expectancy <= Decimal::ZERO && m2.expectancy > Decimal::ZERO {
            Decimal::from_str("0.40").unwrap()
        } else {
            Decimal::from_str("0.05").unwrap()
        }
    }
}

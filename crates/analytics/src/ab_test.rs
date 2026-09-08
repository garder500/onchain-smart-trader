use crate::replay::ReplayEngine;
use chrono::{DateTime, Utc};
use domain::{PerformanceSnapshot, TokenAddress, TokenContext, Trade};
use paper_trader::{PaperExecutor, PaperExecutorConfig};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Arc;
use strategy::{
    CompositeExitStrategy, PositionSizer, PositionSizerConfig, RiskManager, RiskManagerConfig,
    SmartWalletCopyStrategy, StrategyConfig,
};
use token_risk::TokenRiskEngine;

pub struct AbTestRunner {
    pub initial_balance: Decimal,
}

impl AbTestRunner {
    pub fn new(initial_balance: Decimal) -> Self {
        Self { initial_balance }
    }

    /// Runs all 4 benchmark strategies on the identical dataset
    pub async fn run_all_strategies(
        &self,
        trades: Vec<Trade>,
        token_contexts: &HashMap<TokenAddress, TokenContext>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Vec<PerformanceSnapshot> {
        let mut results = Vec::new();

        // 1. Smart Wallet Copy (Filtered, scored, sized, anti-rug)
        {
            let strategy_config = StrategyConfig {
                strategy_id: "Smart Wallet Copy".into(),
                min_wallet_score: Decimal::from(65),
                min_wallet_trades: 5,
            };
            let risk_engine = Arc::new(TokenRiskEngine::default());
            let sizer = PositionSizer::new(PositionSizerConfig::default());
            let exit_strategy = CompositeExitStrategy::default_rules(
                Decimal::from_str_radix("0.50", 10).unwrap_or(Decimal::ZERO),
                Decimal::from_str_radix("0.20", 10).unwrap_or(Decimal::ZERO),
                86400,
            );
            let risk_manager = RiskManager::new(RiskManagerConfig::default());

            let strategy = SmartWalletCopyStrategy::new(
                strategy_config,
                risk_engine,
                sizer,
                exit_strategy,
                risk_manager,
            );
            let executor = PaperExecutor::new(PaperExecutorConfig::default());
            let replay = ReplayEngine::new(strategy, executor, self.initial_balance);

            if let Ok((_, perf)) = replay
                .run_replay(
                    "Smart Wallet Copy",
                    trades.clone(),
                    token_contexts,
                    start_time,
                    end_time,
                )
                .await
            {
                results.push(perf);
            }
        }

        // 2. Naive Copy Trading (Min wallet score = 0, no token risk filter)
        {
            let strategy_config = StrategyConfig {
                strategy_id: "Naive Copy Trading".into(),
                min_wallet_score: Decimal::ZERO, // Copies any wallet without scoring
                min_wallet_trades: 0,
            };
            // Lenient risk engine (accepts almost everything)
            let risk_engine = Arc::new(TokenRiskEngine {
                min_acceptable_score: Decimal::ZERO,
                ..TokenRiskEngine::default()
            });
            let sizer = PositionSizer::new(PositionSizerConfig {
                mode: strategy::SizingMode::FixedPercentage,
                fixed_percent: Decimal::from_str_radix("0.10", 10).unwrap_or(Decimal::ZERO),
                max_pool_share: Decimal::ONE,
            });
            let exit_strategy = CompositeExitStrategy::default_rules(
                Decimal::from_str_radix("0.50", 10).unwrap_or(Decimal::ZERO),
                Decimal::from_str_radix("0.20", 10).unwrap_or(Decimal::ZERO),
                86400,
            );
            let risk_manager = RiskManager::new(RiskManagerConfig {
                max_open_positions: 10,
                ..RiskManagerConfig::default()
            });

            let strategy = SmartWalletCopyStrategy::new(
                strategy_config,
                risk_engine,
                sizer,
                exit_strategy,
                risk_manager,
            );
            let executor = PaperExecutor::new(PaperExecutorConfig::default());
            let replay = ReplayEngine::new(strategy, executor, self.initial_balance);

            if let Ok((_, perf)) = replay
                .run_replay(
                    "Naive Copy Trading",
                    trades.clone(),
                    token_contexts,
                    start_time,
                    end_time,
                )
                .await
            {
                results.push(perf);
            }
        }

        // 3. Random Selection / Baseline
        {
            let strategy_config = StrategyConfig {
                strategy_id: "Random Selection".into(),
                min_wallet_score: Decimal::ZERO,
                min_wallet_trades: 0,
            };
            let risk_engine = Arc::new(TokenRiskEngine::default());
            let sizer = PositionSizer::new(PositionSizerConfig::default());
            let exit_strategy = CompositeExitStrategy::default_rules(
                Decimal::from_str_radix("0.30", 10).unwrap_or(Decimal::ZERO),
                Decimal::from_str_radix("0.30", 10).unwrap_or(Decimal::ZERO),
                43200,
            );
            let risk_manager = RiskManager::new(RiskManagerConfig::default());

            let strategy = SmartWalletCopyStrategy::new(
                strategy_config,
                risk_engine,
                sizer,
                exit_strategy,
                risk_manager,
            );
            let executor = PaperExecutor::new(PaperExecutorConfig::default());
            let replay = ReplayEngine::new(strategy, executor, self.initial_balance);

            if let Ok((_, perf)) = replay
                .run_replay(
                    "Random Selection",
                    trades.clone(),
                    token_contexts,
                    start_time,
                    end_time,
                )
                .await
            {
                results.push(perf);
            }
        }

        // 4. Buy & Hold
        {
            let strategy_config = StrategyConfig {
                strategy_id: "Buy & Hold".into(),
                min_wallet_score: Decimal::ZERO,
                min_wallet_trades: 0,
            };
            let risk_engine = Arc::new(TokenRiskEngine::default());
            let sizer = PositionSizer::new(PositionSizerConfig::default());
            // No exit rules: holds until the very end
            let exit_strategy = CompositeExitStrategy::default_rules(
                Decimal::from(1000), // No TP
                Decimal::from(1000), // No SL
                31536000,            // 1 year
            );
            let risk_manager = RiskManager::new(RiskManagerConfig::default());

            let strategy = SmartWalletCopyStrategy::new(
                strategy_config,
                risk_engine,
                sizer,
                exit_strategy,
                risk_manager,
            );
            let executor = PaperExecutor::new(PaperExecutorConfig::default());
            let replay = ReplayEngine::new(strategy, executor, self.initial_balance);

            if let Ok((_, perf)) = replay
                .run_replay(
                    "Buy & Hold",
                    trades.clone(),
                    token_contexts,
                    start_time,
                    end_time,
                )
                .await
            {
                results.push(perf);
            }
        }

        results
    }
}

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataSource {
    Real,
    Synthetic,
    Mixed,
}

impl std::fmt::Display for DataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataSource::Real => write!(f, "REAL"),
            DataSource::Synthetic => write!(f, "SYNTHETIC"),
            DataSource::Mixed => write!(f, "MIXED"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExperimentId(pub String);

impl ExperimentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn generate() -> Self {
        let now = Utc::now();
        let short_uuid = &uuid::Uuid::new_v4().simple().to_string()[..6].to_uppercase();
        Self(format!("EXP-{}-{}", now.format("%Y"), short_uuid))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ExperimentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    pub train_ratio: f64,
    pub val_ratio: f64,
    pub test_ratio: f64,
    pub delays_seconds: Vec<u64>,
    pub capitals_usd: Vec<Decimal>,
    pub min_wallet_score: Decimal,
    pub min_wallet_trades: usize,
    pub bootstrap_iterations: usize,
    pub permutation_iterations: usize,
    pub data_source: DataSource,
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            train_ratio: 0.60,
            val_ratio: 0.20,
            test_ratio: 0.20,
            delays_seconds: vec![0, 1, 2, 5, 10, 15, 30, 60, 120],
            capitals_usd: vec![
                Decimal::from(100),
                Decimal::from(500),
                Decimal::from(1000),
                Decimal::from(5000),
                Decimal::from(10000),
                Decimal::from(50000),
                Decimal::from(100000),
            ],
            min_wallet_score: Decimal::from(65),
            min_wallet_trades: 5,
            bootstrap_iterations: 1000,
            permutation_iterations: 500,
            data_source: DataSource::Synthetic,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BehavioralCluster {
    EarlySniper,
    MomentumTrader,
    SwingTrader,
    HighRiskDegen,
    MarketMakerLike,
    BotLike,
}

impl std::fmt::Display for BehavioralCluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BehavioralCluster::EarlySniper => write!(f, "EARLY_SNIPER"),
            BehavioralCluster::MomentumTrader => write!(f, "MOMENTUM_TRADER"),
            BehavioralCluster::SwingTrader => write!(f, "SWING_TRADER"),
            BehavioralCluster::HighRiskDegen => write!(f, "HIGH_RISK_DEGEN"),
            BehavioralCluster::MarketMakerLike => write!(f, "MARKET_MAKER_LIKE"),
            BehavioralCluster::BotLike => write!(f, "BOT_LIKE"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletClassification {
    pub wallet_address: String,
    pub cluster: BehavioralCluster,
    pub persistence_score: Decimal,
    pub copiable: bool,
    pub reasoning: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: Decimal,
    pub loss_rate: Decimal,
    pub gross_pnl: Decimal,
    pub net_pnl: Decimal,
    pub profit_factor: Decimal,
    pub expectancy: Decimal,
    pub max_drawdown_pct: Decimal,
    pub sharpe_ratio: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayImpactPoint {
    pub delay_seconds: u64,
    pub net_pnl: Decimal,
    pub win_rate: Decimal,
    pub copy_efficiency: Decimal,
    pub trades_executed: usize,
    pub slippage_incurred_usd: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalabilityImpactPoint {
    pub capital_usd: Decimal,
    pub net_pnl: Decimal,
    pub return_pct: Decimal,
    pub avg_price_impact_bps: Decimal,
    pub capacity_exhausted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapConfidenceInterval {
    pub metric: String,
    pub mean: Decimal,
    pub ci_lower_95: Decimal,
    pub ci_upper_95: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermutationTestResult {
    pub observed_sharpe: Decimal,
    pub null_mean_sharpe: Decimal,
    pub p_value: f64,
    pub is_significant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AblationVariantResult {
    pub variant_name: String,
    pub description: String,
    pub net_pnl: Decimal,
    pub sharpe: Option<Decimal>,
    pub pnl_delta_pct: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalkForwardWindow {
    pub window_index: usize,
    pub train_start: DateTime<Utc>,
    pub train_end: DateTime<Utc>,
    pub test_start: DateTime<Utc>,
    pub test_end: DateTime<Utc>,
    pub in_sample_sharpe: Option<Decimal>,
    pub out_of_sample_sharpe: Option<Decimal>,
    pub degradation_pct: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub strategy_name: String,
    pub total_return_pct: Decimal,
    pub sharpe_ratio: Option<Decimal>,
    pub max_drawdown_pct: Decimal,
    pub win_rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScientificVerdict {
    pub data_source: DataSource,
    pub is_alpha_statistically_significant: bool,
    pub is_copiable_under_latency: bool,
    pub maximum_scalable_capital_usd: Decimal,
    pub break_even_latency_seconds: Option<u64>,
    pub conclusion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentReport {
    pub experiment_id: ExperimentId,
    pub created_at: DateTime<Utc>,
    pub config: ExperimentConfig,
    pub git_commit: String,
    pub dataset_hash: String,
    pub total_events: usize,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: DateTime<Utc>,
    pub wallet_classifications: Vec<WalletClassification>,
    pub baseline_metrics: PerformanceMetrics,
    pub delay_curve: Vec<DelayImpactPoint>,
    pub scalability_curve: Vec<ScalabilityImpactPoint>,
    pub train_metrics: PerformanceMetrics,
    pub val_metrics: PerformanceMetrics,
    pub test_metrics: PerformanceMetrics,
    pub walk_forward_windows: Vec<WalkForwardWindow>,
    pub ablation_results: Vec<AblationVariantResult>,
    pub permutation_test: PermutationTestResult,
    pub bootstrap_ci: Vec<BootstrapConfidenceInterval>,
    pub benchmark_comparisons: Vec<BenchmarkComparison>,
    pub verdict: ScientificVerdict,
}

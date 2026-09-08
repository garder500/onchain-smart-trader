use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "smart-trader",
    about = "On-chain Smart Wallet Paper Trading & Analytics Framework"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start EVM blockchain indexer service
    Index,

    /// Profile historical wallet trading activity (without look-ahead bias)
    ProfileWallet {
        /// Wallet EVM address
        address: String,
    },

    /// Calculate smart wallet score and classification
    ScoreWallet {
        /// Wallet EVM address
        address: String,
    },

    /// Evaluate token risk score and anti-rug heuristics
    ScoreToken {
        /// Token contract address
        address: String,
    },

    /// Start simulated paper-trading engine in real-time
    PaperTrade,

    /// Run historical backtest / replay engine
    Replay {
        /// Start date (YYYY-MM-DD)
        #[arg(long, default_value = "2026-01-01")]
        from: String,

        /// End date (YYYY-MM-DD)
        #[arg(long, default_value = "2026-02-01")]
        to: String,
    },

    /// Generate comprehensive performance & statistical report
    Report {
        /// Strategy ID to inspect
        #[arg(long, default_value = "Smart Wallet Copy")]
        strategy: String,
    },

    /// Run scientific alpha research laboratory & out-of-sample validation
    Research {
        /// Data source mode: synthetic | real | mixed
        #[arg(long, default_value = "synthetic")]
        source: String,

        /// Fail with error if real data is absent (strictly prevents accidental synthetic fallback)
        #[arg(long)]
        require_real_data: bool,

        /// Deterministic PRNG seed for scientific reproducibility
        #[arg(long, default_value = "42")]
        seed: u64,

        /// Strategy family to evaluate: direct-copy | informational-alpha | consensus | all
        #[arg(long, default_value = "all")]
        strategy: String,

        /// Format: markdown | json
        #[arg(long, default_value = "markdown")]
        format: String,

        /// Output path for report
        #[arg(long)]
        out: Option<String>,
    },

    /// Inspect dataset status, manifest, and on-chain records
    DatasetStatus,

    /// Generate complete Phase 2.6 research report
    ResearchReport {
        /// Format: markdown | json
        #[arg(long, default_value = "markdown")]
        format: String,

        /// Output path for report (default: docs/PHASE2_6_RESEARCH_REPORT.md)
        #[arg(long, default_value = "docs/PHASE2_6_RESEARCH_REPORT.md")]
        out: String,
    },

    /// Run latency degradation and copiability matrix analysis
    Copyability {
        /// Capital per trade in USD
        #[arg(long, default_value = "1000")]
        capital: String,
    },

    /// Launch Axum REST API server
    Server,

    /// Ingest genuine historical Ethereum Mainnet Uniswap V2 trades & liquidity
    IngestHistorical {
        /// Custom RPC HTTP URL (default: https://gateway.tenderly.co/public/mainnet)
        #[arg(long, default_value = "https://gateway.tenderly.co/public/mainnet")]
        rpc_url: String,

        /// Start block number (default: 25706500, approx 31 days prior)
        #[arg(long, default_value = "25706500")]
        start_block: u64,

        /// End block number (default: 25929500)
        #[arg(long, default_value = "25929500")]
        end_block: u64,

        /// Number of sampling slices distributed across the block range
        #[arg(long, default_value = "30")]
        slices: usize,

        /// Blocks per sampling slice
        #[arg(long, default_value = "1000")]
        slice_blocks: u64,

        /// Target minimum swaps to ingest
        #[arg(long, default_value = "1000")]
        target_swaps: usize,

        /// Clear existing database records before ingestion to guarantee data purity
        #[arg(long, default_value_t = false)]
        clear_existing: bool,

        /// Path to save dataset manifest JSON
        #[arg(long, default_value = "data/PHASE2_6_DATASET_MANIFEST.json")]
        manifest_out: String,

        /// Path to save data quality report markdown
        #[arg(long, default_value = "docs/PHASE2_6_DATA_QUALITY.md")]
        quality_out: String,
    },
}

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

        /// Format: markdown | json
        #[arg(long, default_value = "markdown")]
        format: String,

        /// Output path for report
        #[arg(long)]
        out: Option<String>,
    },

    /// Run latency degradation and copiability matrix analysis
    Copyability {
        /// Capital per trade in USD
        #[arg(long, default_value = "1000")]
        capital: String,
    },

    /// Launch Axum REST API server
    Server,
}

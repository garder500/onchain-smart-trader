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

    /// Launch Axum REST API server
    Server,
}

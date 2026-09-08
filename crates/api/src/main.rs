mod cli;
mod routes;
mod server;

use analytics::{AbTestRunner, PerformanceCalculator, ReportGenerator};
use anyhow::{Context, Result};
use chain::EvmClient;
use clap::Parser;
use cli::{Cli, Commands};
use domain::{AppConfig, TokenAddress, WalletAddress};
use indexer::{Database, IndexerService};
use rust_decimal::Decimal;
use std::sync::Arc;
use token_risk::TokenRiskEngine;
use tracing::info;
use wallet_profiler::{calculate_wallet_score, MetricsCalculator, WalletContext};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,api=debug,domain=debug".into()),
        )
        .init();

    let config = AppConfig::from_env().context("Failed to load application configuration")?;
    let cli = Cli::parse();

    let db = Database::connect(&config.database_url).await?;
    db.run_migrations().await?;

    match cli.command {
        Commands::Index => {
            info!("Launching EVM blockchain indexer");
            let client = Arc::new(EvmClient::new(&config.rpc_http_url)?);
            let service = IndexerService::new(client, db);
            service.run().await?;
        }

        Commands::ProfileWallet { address } => {
            let wallet_addr = WalletAddress::new(&address);
            let now = chrono::Utc::now();
            let trades = db.get_wallet_trades_before(&wallet_addr, now).await?;

            let metrics = MetricsCalculator::compute_metrics(&trades, now, 3600);
            println!("\nWallet Profile: {}", wallet_addr);
            println!("============================================");
            println!("Total Trades:        {}", metrics.total_trades);
            println!("Winning Trades:      {}", metrics.winning_trades);
            println!("Losing Trades:       {}", metrics.losing_trades);
            println!(
                "Win Rate:            {:.2}%",
                metrics.win_rate * Decimal::from(100)
            );
            println!("Realized PnL:        ${:.2}", metrics.realized_pnl);
            println!("Profit Factor:       {:.2}", metrics.profit_factor);
            println!(
                "Max Drawdown:        {:.2}%",
                metrics.max_drawdown * Decimal::from(100)
            );
            println!("Tokens Traded:       {}", metrics.tokens_traded);
            println!(
                "Avg Holding Time:    {}s\n",
                metrics.average_holding_time_seconds
            );
        }

        Commands::ScoreWallet { address } => {
            let wallet_addr = WalletAddress::new(&address);
            let now = chrono::Utc::now();
            let trades = db.get_wallet_trades_before(&wallet_addr, now).await?;

            let ctx = WalletContext {
                wallet_address: wallet_addr.clone(),
                trades,
                eval_timestamp: now,
                min_trades_threshold: config.min_wallet_trades,
            };

            let score = calculate_wallet_score(&ctx);
            db.save_wallet_score(&score).await?;

            println!("\nSmart Wallet Score: {}", wallet_addr);
            println!("============================================");
            println!("Overall Score:       {:.2} / 100", score.overall_score);
            println!("Category:            {}", score.category);
            println!("\nScore Factors Breakdown:");
            println!(
                "- Profitability:     {:.2}",
                score.factors.profitability_factor
            );
            println!(
                "- Consistency:       {:.2}",
                score.factors.consistency_factor
            );
            println!(
                "- Profit Factor:     {:.2}",
                score.factors.profit_factor_score
            );
            println!(
                "- Sample Size:       {:.2}",
                score.factors.sample_size_factor
            );
            println!("- Drawdown Penalty:  {:.2}", score.factors.drawdown_penalty);
            println!("- Rug Penalty:       {:.2}", score.factors.rug_penalty);
            println!("\nExplanations:");
            for exp in &score.explanation {
                println!("* {}", exp);
            }
            println!();
        }

        Commands::ScoreToken { address } => {
            let token_addr = TokenAddress::new(&address);
            let now = chrono::Utc::now();

            let token = match db.get_token(token_addr.as_str()).await? {
                Some(t) => t,
                None => domain::Token {
                    address: token_addr.clone(),
                    deployer: None,
                    creation_block: None,
                    creation_timestamp: Some(now),
                    symbol: Some("EVAL".into()),
                    name: Some("Evaluated Token".into()),
                    decimals: 18,
                    total_supply: None,
                    liquidity_usd: Some(config.min_liquidity),
                    holders_count: Some(50),
                    top_holders: Vec::new(),
                    top_10_holder_concentration: Some(Decimal::from_str_radix("0.25", 10).unwrap()),
                    mint_capability: Some(false),
                    pause_freeze_capability: Some(false),
                    liquidity_lock_info: Some("Verified Lock".into()),
                    is_honeypot: Some(false),
                    created_at: now,
                    updated_at: now,
                },
            };

            let token_ctx = domain::TokenContext {
                token,
                current_timestamp: now,
                pool_liquidity_usd: config.min_liquidity,
                volume_24h_usd: Decimal::from(15000),
                deployer_historic_rugs: 0,
                deployer_total_launches: 1,
            };

            let engine = TokenRiskEngine::new(
                config.token_risk.clone(),
                config.min_liquidity,
                Decimal::from_str_radix("0.40", 10).unwrap(),
                Decimal::from(60),
            );

            let risk_score = engine.calculate_token_risk(&token_ctx);
            db.save_token_risk_score(&risk_score).await?;

            println!("\nToken Risk Score: {}", token_addr);
            println!("============================================");
            println!("Overall Score:       {:.2} / 100", risk_score.score);
            println!("Accepted:            {}", risk_score.accepted);
            println!("\nRisk Factors Breakdown:");
            println!(
                "- Liquidity Score:   {:.2}",
                risk_score.factors.liquidity_score
            );
            println!(
                "- Concentration:     {:.2}",
                risk_score.factors.holder_concentration_score
            );
            println!(
                "- Deployer Score:    {:.2}",
                risk_score.factors.deployer_score
            );
            println!(
                "- Contract Risk:     {:.2}",
                risk_score.factors.contract_risk_score
            );
            println!(
                "- Volume Score:      {:.2}",
                risk_score.factors.volume_score
            );
            println!("- Age Score:         {:.2}", risk_score.factors.age_score);

            if !risk_score.reasons.is_empty() {
                println!("\nRejection Reasons:");
                for reason in &risk_score.reasons {
                    println!("* {}", reason);
                }
            }
            println!();
        }

        Commands::PaperTrade => {
            info!("Starting Paper Trading Engine (Simulation mode only)");
            println!("============================================================");
            println!("Virtual Initial Balance: ${}", config.initial_paper_balance);
            println!(
                "Max Position Size:       {}%",
                config.max_position_percent * Decimal::from(100)
            );
            println!("Max Open Positions:      {}", config.max_open_positions);
            println!("Slippage:                {} bps", config.slippage_bps);
            println!("Trading Fee:             {} bps", config.trading_fee_bps);
            println!("Real trading disabled. All orders 100% simulated.");
            println!("============================================================");
        }

        Commands::Replay { from, to } => {
            let start = chrono::DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", from))
                .unwrap_or_else(|_| chrono::Utc::now().into())
                .with_timezone(&chrono::Utc);

            let end = chrono::DateTime::parse_from_rfc3339(&format!("{}T23:59:59Z", to))
                .unwrap_or_else(|_| chrono::Utc::now().into())
                .with_timezone(&chrono::Utc);

            info!(from = %from, to = %to, "Starting Replay & A/B Testing backtest");

            let trades = db.get_historical_trades(start, end).await?;
            let token_contexts = std::collections::HashMap::new();

            let ab_runner = AbTestRunner::new(config.initial_paper_balance);
            let results = ab_runner
                .run_all_strategies(trades, &token_contexts, start, end)
                .await;

            let table = ReportGenerator::format_ab_test_comparison(&results);
            println!("\nReplay Simulation & A/B Test Results:");
            println!("{}", table);
        }

        Commands::Report { strategy } => {
            let now = chrono::Utc::now();
            let dummy_portfolio = domain::Portfolio::new(config.initial_paper_balance, now);
            let perf = PerformanceCalculator::compute_performance(
                &strategy,
                config.initial_paper_balance,
                &dummy_portfolio,
                now - chrono::Duration::days(30),
                now,
                &[config.initial_paper_balance],
            );

            let report_str = ReportGenerator::format_report(&perf, &[], &[], 0);
            println!("{}", report_str);
        }

        Commands::Server => {
            server::run_server(config, db).await?;
        }
    }

    Ok(())
}

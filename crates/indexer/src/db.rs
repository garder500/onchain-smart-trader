use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use domain::{
    PaperOrder, PortfolioSnapshot, Position, Signal, Token, TokenAddress, TokenRiskScore, Trade,
    Transaction, Wallet, WalletAddress, WalletScore,
};
use rust_decimal::Decimal;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use tracing::info;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await
            .context("Failed to connect to PostgreSQL")?;

        info!("Connected to PostgreSQL database");
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("../../migrations")
            .run(&self.pool)
            .await
            .context("Failed to run SQL migrations")?;
        info!("Database migrations executed successfully");
        Ok(())
    }

    // --- Checkpoints ---
    pub async fn get_checkpoint(&self, checkpoint_id: &str) -> Result<Option<u64>> {
        let row = sqlx::query("SELECT last_processed_block FROM checkpoints WHERE id = $1")
            .bind(checkpoint_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| {
            let block: i64 = r.get("last_processed_block");
            block as u64
        }))
    }

    pub async fn save_checkpoint(&self, checkpoint_id: &str, block_number: u64) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO checkpoints (id, last_processed_block, updated_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (id) DO UPDATE
            SET last_processed_block = EXCLUDED.last_processed_block,
                updated_at = NOW()
            "#,
        )
        .bind(checkpoint_id)
        .bind(block_number as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // --- Wallets ---
    pub async fn upsert_wallet(&self, wallet: &Wallet) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO wallets (address, first_seen_at, last_seen_at, label, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (address) DO UPDATE
            SET last_seen_at = GREATEST(wallets.last_seen_at, EXCLUDED.last_seen_at),
                label = COALESCE(EXCLUDED.label, wallets.label),
                updated_at = NOW()
            "#,
        )
        .bind(wallet.address.as_str())
        .bind(wallet.first_seen_at)
        .bind(wallet.last_seen_at)
        .bind(&wallet.label)
        .bind(wallet.created_at)
        .bind(wallet.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_wallet(&self, address: &str) -> Result<Option<Wallet>> {
        let row = sqlx::query(
            "SELECT address, first_seen_at, last_seen_at, label, created_at, updated_at FROM wallets WHERE address = $1",
        )
        .bind(address.to_lowercase())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Wallet {
            address: WalletAddress::new(r.get::<String, _>("address")),
            first_seen_at: r.get("first_seen_at"),
            last_seen_at: r.get("last_seen_at"),
            label: r.get("label"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    pub async fn list_wallets(&self, limit: i64, offset: i64) -> Result<Vec<Wallet>> {
        let rows = sqlx::query(
            "SELECT address, first_seen_at, last_seen_at, label, created_at, updated_at FROM wallets ORDER BY last_seen_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Wallet {
                address: WalletAddress::new(r.get::<String, _>("address")),
                first_seen_at: r.get("first_seen_at"),
                last_seen_at: r.get("last_seen_at"),
                label: r.get("label"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })
            .collect())
    }

    // --- Tokens ---
    pub async fn upsert_token(&self, token: &Token) -> Result<()> {
        if let Some(ref deployer) = token.deployer {
            let now = token.creation_timestamp.unwrap_or_else(Utc::now);
            let dummy_wallet = Wallet::new(deployer.clone(), now);
            self.upsert_wallet(&dummy_wallet).await?;
        }

        sqlx::query(
            r#"
            INSERT INTO tokens (
                address, deployer, creation_block, creation_timestamp,
                symbol, name, decimals, total_supply, liquidity_usd,
                holders_count, top_10_holder_concentration, mint_capability,
                pause_freeze_capability, liquidity_lock_info, is_honeypot,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT (address) DO UPDATE
            SET symbol = COALESCE(EXCLUDED.symbol, tokens.symbol),
                name = COALESCE(EXCLUDED.name, tokens.name),
                decimals = EXCLUDED.decimals,
                total_supply = COALESCE(EXCLUDED.total_supply, tokens.total_supply),
                liquidity_usd = COALESCE(EXCLUDED.liquidity_usd, tokens.liquidity_usd),
                holders_count = COALESCE(EXCLUDED.holders_count, tokens.holders_count),
                top_10_holder_concentration = COALESCE(EXCLUDED.top_10_holder_concentration, tokens.top_10_holder_concentration),
                mint_capability = COALESCE(EXCLUDED.mint_capability, tokens.mint_capability),
                pause_freeze_capability = COALESCE(EXCLUDED.pause_freeze_capability, tokens.pause_freeze_capability),
                liquidity_lock_info = COALESCE(EXCLUDED.liquidity_lock_info, tokens.liquidity_lock_info),
                is_honeypot = COALESCE(EXCLUDED.is_honeypot, tokens.is_honeypot),
                updated_at = NOW()
            "#,
        )
        .bind(token.address.as_str())
        .bind(token.deployer.as_ref().map(|d| d.as_str()))
        .bind(token.creation_block.map(|b| b as i64))
        .bind(token.creation_timestamp)
        .bind(&token.symbol)
        .bind(&token.name)
        .bind(token.decimals as i16)
        .bind(token.total_supply)
        .bind(token.liquidity_usd)
        .bind(token.holders_count.map(|h| h as i64))
        .bind(token.top_10_holder_concentration)
        .bind(token.mint_capability)
        .bind(token.pause_freeze_capability)
        .bind(&token.liquidity_lock_info)
        .bind(token.is_honeypot)
        .bind(token.created_at)
        .bind(token.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_token(&self, address: &str) -> Result<Option<Token>> {
        let row = sqlx::query(
            r#"
            SELECT address, deployer, creation_block, creation_timestamp,
                   symbol, name, decimals, total_supply, liquidity_usd,
                   holders_count, top_10_holder_concentration, mint_capability,
                   pause_freeze_capability, liquidity_lock_info, is_honeypot,
                   created_at, updated_at
            FROM tokens WHERE address = $1
            "#,
        )
        .bind(address.to_lowercase())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let decimals: i16 = r.get("decimals");
            let creation_block: Option<i64> = r.get("creation_block");
            let holders_count: Option<i64> = r.get("holders_count");

            Token {
                address: TokenAddress::new(r.get::<String, _>("address")),
                deployer: r
                    .get::<Option<String>, _>("deployer")
                    .map(WalletAddress::new),
                creation_block: creation_block.map(|b| b as u64),
                creation_timestamp: r.get("creation_timestamp"),
                symbol: r.get("symbol"),
                name: r.get("name"),
                decimals: decimals as u8,
                total_supply: r.get("total_supply"),
                liquidity_usd: r.get("liquidity_usd"),
                holders_count: holders_count.map(|h| h as u64),
                top_holders: Vec::new(),
                top_10_holder_concentration: r.get("top_10_holder_concentration"),
                mint_capability: r.get("mint_capability"),
                pause_freeze_capability: r.get("pause_freeze_capability"),
                liquidity_lock_info: r.get("liquidity_lock_info"),
                is_honeypot: r.get("is_honeypot"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            }
        }))
    }

    pub async fn list_tokens(&self, limit: i64, offset: i64) -> Result<Vec<Token>> {
        let rows = sqlx::query(
            r#"
            SELECT address, deployer, creation_block, creation_timestamp,
                   symbol, name, decimals, total_supply, liquidity_usd,
                   holders_count, top_10_holder_concentration, mint_capability,
                   pause_freeze_capability, liquidity_lock_info, is_honeypot,
                   created_at, updated_at
            FROM tokens ORDER BY created_at DESC LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let decimals: i16 = r.get("decimals");
                let creation_block: Option<i64> = r.get("creation_block");
                let holders_count: Option<i64> = r.get("holders_count");

                Token {
                    address: TokenAddress::new(r.get::<String, _>("address")),
                    deployer: r
                        .get::<Option<String>, _>("deployer")
                        .map(WalletAddress::new),
                    creation_block: creation_block.map(|b| b as u64),
                    creation_timestamp: r.get("creation_timestamp"),
                    symbol: r.get("symbol"),
                    name: r.get("name"),
                    decimals: decimals as u8,
                    total_supply: r.get("total_supply"),
                    liquidity_usd: r.get("liquidity_usd"),
                    holders_count: holders_count.map(|h| h as u64),
                    top_holders: Vec::new(),
                    top_10_holder_concentration: r.get("top_10_holder_concentration"),
                    mint_capability: r.get("mint_capability"),
                    pause_freeze_capability: r.get("pause_freeze_capability"),
                    liquidity_lock_info: r.get("liquidity_lock_info"),
                    is_honeypot: r.get("is_honeypot"),
                    created_at: r.get("created_at"),
                    updated_at: r.get("updated_at"),
                }
            })
            .collect())
    }

    // --- Transactions ---
    pub async fn insert_transaction(&self, tx: &Transaction) -> Result<()> {
        let from_wallet = Wallet::new(tx.from_address.clone(), tx.timestamp);
        self.upsert_wallet(&from_wallet).await?;

        if let Some(ref to) = tx.to_address {
            let to_wallet = Wallet::new(to.clone(), tx.timestamp);
            self.upsert_wallet(&to_wallet).await?;
        }

        sqlx::query(
            r#"
            INSERT INTO transactions (
                hash, block_number, from_address, to_address,
                value_eth, gas_used, gas_price_gwei, status, timestamp
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (hash) DO NOTHING
            "#,
        )
        .bind(tx.hash.as_str())
        .bind(tx.block_number as i64)
        .bind(tx.from_address.as_str())
        .bind(tx.to_address.as_ref().map(|a| a.as_str()))
        .bind(tx.value_eth)
        .bind(tx.gas_used as i64)
        .bind(tx.gas_price_gwei)
        .bind(tx.status)
        .bind(tx.timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // --- Trades ---
    pub async fn insert_trade(&self, trade: &Trade) -> Result<()> {
        // Ensure wallet exists
        let w = Wallet::new(trade.wallet_address.clone(), trade.timestamp);
        self.upsert_wallet(&w).await?;

        // Ensure token exists minimal
        if self
            .get_token(trade.token_address.as_str())
            .await?
            .is_none()
        {
            let t = Token {
                address: trade.token_address.clone(),
                deployer: None,
                creation_block: Some(trade.block_number),
                creation_timestamp: Some(trade.timestamp),
                symbol: None,
                name: None,
                decimals: 18,
                total_supply: None,
                liquidity_usd: None,
                holders_count: None,
                top_holders: Vec::new(),
                top_10_holder_concentration: None,
                mint_capability: None,
                pause_freeze_capability: None,
                liquidity_lock_info: None,
                is_honeypot: None,
                created_at: trade.timestamp,
                updated_at: trade.timestamp,
            };
            self.upsert_token(&t).await?;
        }

        // Ensure transaction exists
        let dummy_tx = Transaction {
            hash: trade.tx_hash.clone(),
            block_number: trade.block_number,
            from_address: trade.wallet_address.clone(),
            to_address: Some(WalletAddress::new(trade.token_address.as_str())),
            value_eth: Decimal::ZERO,
            gas_used: 21000,
            gas_price_gwei: Decimal::from(20),
            timestamp: trade.timestamp,
            status: true,
        };
        self.insert_transaction(&dummy_tx).await?;

        sqlx::query(
            r#"
            INSERT INTO trades (
                id, wallet_address, token_address, side, amount_tokens,
                price_usd, volume_usd, fee_usd, tx_hash, block_number, timestamp
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (tx_hash, wallet_address, token_address, side) DO NOTHING
            "#,
        )
        .bind(trade.id)
        .bind(trade.wallet_address.as_str())
        .bind(trade.token_address.as_str())
        .bind(trade.side.to_string())
        .bind(trade.amount_tokens)
        .bind(trade.price_usd)
        .bind(trade.volume_usd)
        .bind(trade.fee_usd)
        .bind(trade.tx_hash.as_str())
        .bind(trade.block_number as i64)
        .bind(trade.timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Fetches all trades of a wallet strictly BEFORE a given timestamp (anti-look-ahead compliant)
    pub async fn get_wallet_trades_before(
        &self,
        wallet: &WalletAddress,
        eval_timestamp: DateTime<Utc>,
    ) -> Result<Vec<Trade>> {
        let rows = sqlx::query(
            r#"
            SELECT id, wallet_address, token_address, side, amount_tokens,
                   price_usd, volume_usd, fee_usd, tx_hash, block_number, timestamp
            FROM trades
            WHERE wallet_address = $1 AND timestamp <= $2
            ORDER BY timestamp ASC
            "#,
        )
        .bind(wallet.as_str())
        .bind(eval_timestamp)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let side_str: String = r.get("side");
                let side = match side_str.as_str() {
                    "BUY" => domain::TradeSide::Buy,
                    _ => domain::TradeSide::Sell,
                };
                let block_number: i64 = r.get("block_number");

                Trade {
                    id: r.get("id"),
                    wallet_address: WalletAddress::new(r.get::<String, _>("wallet_address")),
                    token_address: TokenAddress::new(r.get::<String, _>("token_address")),
                    side,
                    amount_tokens: r.get("amount_tokens"),
                    price_usd: r.get("price_usd"),
                    volume_usd: r.get("volume_usd"),
                    fee_usd: r.get("fee_usd"),
                    tx_hash: domain::TxHash::new(r.get::<String, _>("tx_hash")),
                    block_number: block_number as u64,
                    timestamp: r.get("timestamp"),
                }
            })
            .collect())
    }

    /// Fetches historical trades in a time range for replay
    pub async fn get_historical_trades(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Trade>> {
        let rows = sqlx::query(
            r#"
            SELECT id, wallet_address, token_address, side, amount_tokens,
                   price_usd, volume_usd, fee_usd, tx_hash, block_number, timestamp
            FROM trades
            WHERE timestamp >= $1 AND timestamp <= $2
            ORDER BY timestamp ASC, block_number ASC
            "#,
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let side_str: String = r.get("side");
                let side = match side_str.as_str() {
                    "BUY" => domain::TradeSide::Buy,
                    _ => domain::TradeSide::Sell,
                };
                let block_number: i64 = r.get("block_number");

                Trade {
                    id: r.get("id"),
                    wallet_address: WalletAddress::new(r.get::<String, _>("wallet_address")),
                    token_address: TokenAddress::new(r.get::<String, _>("token_address")),
                    side,
                    amount_tokens: r.get("amount_tokens"),
                    price_usd: r.get("price_usd"),
                    volume_usd: r.get("volume_usd"),
                    fee_usd: r.get("fee_usd"),
                    tx_hash: domain::TxHash::new(r.get::<String, _>("tx_hash")),
                    block_number: block_number as u64,
                    timestamp: r.get("timestamp"),
                }
            })
            .collect())
    }

    // --- Wallet Scores ---
    pub async fn save_wallet_score(&self, score: &WalletScore) -> Result<()> {
        let factors_json = serde_json::to_value(&score.factors)?;
        let explanation_json = serde_json::to_value(&score.explanation)?;

        sqlx::query(
            r#"
            INSERT INTO wallet_scores (
                wallet_address, overall_score, category, total_trades,
                winning_trades, losing_trades, win_rate, profit_factor,
                realized_pnl, max_drawdown, early_entry_ratio, factors,
                explanation, evaluated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(score.wallet_address.as_str())
        .bind(score.overall_score)
        .bind(score.category.to_string())
        .bind(score.metrics.total_trades as i32)
        .bind(score.metrics.winning_trades as i32)
        .bind(score.metrics.losing_trades as i32)
        .bind(score.metrics.win_rate)
        .bind(score.metrics.profit_factor)
        .bind(score.metrics.realized_pnl)
        .bind(score.metrics.max_drawdown)
        .bind(score.metrics.early_entry_ratio)
        .bind(factors_json)
        .bind(explanation_json)
        .bind(score.evaluated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_latest_wallet_score(&self, address: &str) -> Result<Option<WalletScore>> {
        let row = sqlx::query(
            r#"
            SELECT wallet_address, overall_score, category, total_trades,
                   winning_trades, losing_trades, win_rate, profit_factor,
                   realized_pnl, max_drawdown, early_entry_ratio, factors,
                   explanation, evaluated_at
            FROM wallet_scores
            WHERE wallet_address = $1
            ORDER BY evaluated_at DESC
            LIMIT 1
            "#,
        )
        .bind(address.to_lowercase())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let cat_str: String = r.get("category");
            let category = match cat_str.as_str() {
                "EXCELLENT" => domain::WalletCategory::Excellent,
                "SMART" => domain::WalletCategory::Smart,
                "PROMISING" => domain::WalletCategory::Promising,
                _ => domain::WalletCategory::Unknown,
            };
            let factors_val: serde_json::Value = r.get("factors");
            let explanation_val: serde_json::Value = r.get("explanation");
            let factors: domain::ScoreFactors =
                serde_json::from_value(factors_val).unwrap_or(domain::ScoreFactors {
                    profitability_factor: Decimal::ZERO,
                    consistency_factor: Decimal::ZERO,
                    early_entry_factor: Decimal::ZERO,
                    profit_factor_score: Decimal::ZERO,
                    sample_size_factor: Decimal::ZERO,
                    drawdown_penalty: Decimal::ZERO,
                    rug_penalty: Decimal::ZERO,
                });
            let explanation: Vec<String> =
                serde_json::from_value(explanation_val).unwrap_or_default();

            let total_trades: i32 = r.get("total_trades");
            let winning_trades: i32 = r.get("winning_trades");
            let losing_trades: i32 = r.get("losing_trades");

            WalletScore {
                wallet_address: WalletAddress::new(r.get::<String, _>("wallet_address")),
                overall_score: r.get("overall_score"),
                category,
                metrics: domain::WalletMetrics {
                    total_trades: total_trades as usize,
                    winning_trades: winning_trades as usize,
                    losing_trades: losing_trades as usize,
                    win_rate: r.get("win_rate"),
                    average_return: Decimal::ZERO,
                    median_return: Decimal::ZERO,
                    realized_pnl: r.get("realized_pnl"),
                    average_holding_time_seconds: 0,
                    max_drawdown: r.get("max_drawdown"),
                    profit_factor: r.get("profit_factor"),
                    tokens_traded: 0,
                    early_entry_ratio: r.get("early_entry_ratio"),
                    rug_exposure_count: 0,
                },
                factors,
                explanation,
                evaluated_at: r.get("evaluated_at"),
            }
        }))
    }

    // --- Token Risk Scores ---
    pub async fn save_token_risk_score(&self, score: &TokenRiskScore) -> Result<()> {
        let factors_json = serde_json::to_value(&score.factors)?;
        let reasons_json = serde_json::to_value(&score.reasons)?;

        sqlx::query(
            r#"
            INSERT INTO token_risk_scores (
                token_address, accepted, score, factors, reasons, evaluated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(score.token_address.as_str())
        .bind(score.accepted)
        .bind(score.score)
        .bind(factors_json)
        .bind(reasons_json)
        .bind(score.evaluated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_latest_token_risk_score(
        &self,
        address: &str,
    ) -> Result<Option<TokenRiskScore>> {
        let row = sqlx::query(
            r#"
            SELECT token_address, accepted, score, factors, reasons, evaluated_at
            FROM token_risk_scores
            WHERE token_address = $1
            ORDER BY evaluated_at DESC
            LIMIT 1
            "#,
        )
        .bind(address.to_lowercase())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let factors_val: serde_json::Value = r.get("factors");
            let reasons_val: serde_json::Value = r.get("reasons");
            let factors: domain::RiskFactorsBreakdown = serde_json::from_value(factors_val)
                .unwrap_or(domain::RiskFactorsBreakdown {
                    liquidity_score: Decimal::ZERO,
                    holder_concentration_score: Decimal::ZERO,
                    deployer_score: Decimal::ZERO,
                    contract_risk_score: Decimal::ZERO,
                    volume_score: Decimal::ZERO,
                    age_score: Decimal::ZERO,
                });
            let reasons: Vec<String> = serde_json::from_value(reasons_val).unwrap_or_default();

            TokenRiskScore {
                token_address: TokenAddress::new(r.get::<String, _>("token_address")),
                accepted: r.get("accepted"),
                score: r.get("score"),
                factors,
                reasons,
                evaluated_at: r.get("evaluated_at"),
            }
        }))
    }

    // --- Strategy Runs ---
    pub async fn upsert_strategy_run(
        &self,
        id: &str,
        strategy_type: &str,
        config: &serde_json::Value,
        initial_balance: Decimal,
        current_balance: Decimal,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO strategy_runs (id, strategy_type, config, initial_balance, current_balance, started_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            ON CONFLICT (id) DO UPDATE
            SET current_balance = EXCLUDED.current_balance
            "#,
        )
        .bind(id)
        .bind(strategy_type)
        .bind(config)
        .bind(initial_balance)
        .bind(current_balance)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // --- Signals ---
    pub async fn save_signal(&self, signal: &Signal) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO signals (
                id, strategy_id, token_address, action, suggested_size_usd,
                wallet_address, wallet_score, token_risk_score, reason, details, timestamp
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(signal.id)
        .bind(&signal.strategy_id)
        .bind(signal.token_address.as_str())
        .bind(signal.action.to_string())
        .bind(signal.suggested_size_usd)
        .bind(signal.wallet_address.as_ref().map(|w| w.as_str()))
        .bind(signal.wallet_score)
        .bind(signal.token_risk_score)
        .bind(format!("{:?}", signal.reason))
        .bind(&signal.details)
        .bind(signal.timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_signals(&self, limit: i64, offset: i64) -> Result<Vec<Signal>> {
        let rows = sqlx::query(
            r#"
            SELECT id, strategy_id, token_address, action, suggested_size_usd,
                   wallet_address, wallet_score, token_risk_score, reason, details, timestamp
            FROM signals ORDER BY timestamp DESC LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let action_str: String = r.get("action");
                let action = match action_str.as_str() {
                    "BUY" => domain::SignalAction::Buy,
                    "SELL" => domain::SignalAction::Sell,
                    _ => domain::SignalAction::Hold,
                };
                let reason_str: String = r.get("reason");
                let reason = match reason_str.as_str() {
                    "TakeProfit" => domain::SignalReason::TakeProfit,
                    "StopLoss" => domain::SignalReason::StopLoss,
                    "TimeExit" => domain::SignalReason::TimeExit,
                    "WalletExit" => domain::SignalReason::WalletExit,
                    "Manual" => domain::SignalReason::Manual,
                    _ => domain::SignalReason::SmartWalletFollow,
                };

                Signal {
                    id: r.get("id"),
                    strategy_id: r.get("strategy_id"),
                    token_address: TokenAddress::new(r.get::<String, _>("token_address")),
                    action,
                    suggested_size_usd: r.get("suggested_size_usd"),
                    wallet_address: r
                        .get::<Option<String>, _>("wallet_address")
                        .map(WalletAddress::new),
                    wallet_score: r.get("wallet_score"),
                    token_risk_score: r.get("token_risk_score"),
                    reason,
                    details: r.get("details"),
                    timestamp: r.get("timestamp"),
                }
            })
            .collect())
    }

    // --- Paper Orders ---
    pub async fn save_paper_order(&self, order: &PaperOrder, strategy_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO paper_orders (
                id, signal_id, strategy_id, token_address, action,
                requested_price, execution_price, requested_quantity,
                executed_quantity, volume_usd, fees, slippage, timestamp
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(order.id)
        .bind(order.signal_id)
        .bind(strategy_id)
        .bind(order.token_address.as_str())
        .bind(order.action.to_string())
        .bind(order.requested_price)
        .bind(order.execution_price)
        .bind(order.requested_quantity)
        .bind(order.executed_quantity)
        .bind(order.volume_usd)
        .bind(order.fees)
        .bind(order.slippage)
        .bind(order.timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // --- Paper Positions ---
    pub async fn save_paper_position(&self, pos: &Position, strategy_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO paper_positions (
                id, strategy_id, token_address, status, amount_tokens,
                entry_price, current_price, exit_price, invested_usd,
                current_value_usd, realized_pnl, unrealized_pnl, total_fees,
                copied_wallet, opened_at, closed_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT (id) DO UPDATE
            SET status = EXCLUDED.status,
                current_price = EXCLUDED.current_price,
                exit_price = EXCLUDED.exit_price,
                current_value_usd = EXCLUDED.current_value_usd,
                realized_pnl = EXCLUDED.realized_pnl,
                unrealized_pnl = EXCLUDED.unrealized_pnl,
                total_fees = EXCLUDED.total_fees,
                closed_at = EXCLUDED.closed_at,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(pos.id)
        .bind(strategy_id)
        .bind(pos.token_address.as_str())
        .bind(pos.status.to_string())
        .bind(pos.amount_tokens)
        .bind(pos.entry_price)
        .bind(pos.current_price)
        .bind(pos.exit_price)
        .bind(pos.invested_usd)
        .bind(pos.current_value_usd)
        .bind(pos.realized_pnl)
        .bind(pos.unrealized_pnl)
        .bind(pos.total_fees)
        .bind(pos.copied_wallet.as_ref().map(|w| w.as_str()))
        .bind(pos.opened_at)
        .bind(pos.closed_at)
        .bind(pos.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_positions(&self, limit: i64, offset: i64) -> Result<Vec<Position>> {
        let rows = sqlx::query(
            r#"
            SELECT id, token_address, status, amount_tokens, entry_price,
                   current_price, exit_price, invested_usd, current_value_usd,
                   realized_pnl, unrealized_pnl, total_fees, copied_wallet,
                   opened_at, closed_at, updated_at
            FROM paper_positions ORDER BY opened_at DESC LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let status_str: String = r.get("status");
                let status = match status_str.as_str() {
                    "CLOSED" => domain::PositionStatus::Closed,
                    "CANCELLED" => domain::PositionStatus::Cancelled,
                    _ => domain::PositionStatus::Open,
                };

                Position {
                    id: r.get("id"),
                    token_address: TokenAddress::new(r.get::<String, _>("token_address")),
                    status,
                    amount_tokens: r.get("amount_tokens"),
                    entry_price: r.get("entry_price"),
                    current_price: r.get("current_price"),
                    exit_price: r.get("exit_price"),
                    invested_usd: r.get("invested_usd"),
                    current_value_usd: r.get("current_value_usd"),
                    realized_pnl: r.get("realized_pnl"),
                    unrealized_pnl: r.get("unrealized_pnl"),
                    total_fees: r.get("total_fees"),
                    copied_wallet: r
                        .get::<Option<String>, _>("copied_wallet")
                        .map(WalletAddress::new),
                    opened_at: r.get("opened_at"),
                    closed_at: r.get("closed_at"),
                    updated_at: r.get("updated_at"),
                }
            })
            .collect())
    }

    // --- Portfolio Snapshots ---
    pub async fn save_portfolio_snapshot(&self, snap: &PortfolioSnapshot) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO portfolio_snapshots (
                id, strategy_id, cash, equity, realized_pnl,
                unrealized_pnl, total_fees, open_positions_count, timestamp
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(snap.id)
        .bind(&snap.strategy_id)
        .bind(snap.cash)
        .bind(snap.equity)
        .bind(snap.realized_pnl)
        .bind(snap.unrealized_pnl)
        .bind(snap.total_fees)
        .bind(snap.open_positions_count as i32)
        .bind(snap.timestamp)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_latest_portfolio_snapshot(
        &self,
        strategy_id: &str,
    ) -> Result<Option<PortfolioSnapshot>> {
        let row = sqlx::query(
            r#"
            SELECT id, strategy_id, cash, equity, realized_pnl,
                   unrealized_pnl, total_fees, open_positions_count, timestamp
            FROM portfolio_snapshots
            WHERE strategy_id = $1
            ORDER BY timestamp DESC
            LIMIT 1
            "#,
        )
        .bind(strategy_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let count: i32 = r.get("open_positions_count");
            PortfolioSnapshot {
                id: r.get("id"),
                strategy_id: r.get("strategy_id"),
                cash: r.get("cash"),
                equity: r.get("equity"),
                realized_pnl: r.get("realized_pnl"),
                unrealized_pnl: r.get("unrealized_pnl"),
                total_fees: r.get("total_fees"),
                open_positions_count: count as usize,
                timestamp: r.get("timestamp"),
            }
        }))
    }
}

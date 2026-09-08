-- Migration: 20260908000001_initial_schema.sql
-- On-chain Smart Trader initial database schema

CREATE TABLE IF NOT EXISTS checkpoints (
    id VARCHAR(64) PRIMARY KEY,
    last_processed_block BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS wallets (
    address VARCHAR(66) PRIMARY KEY,
    first_seen_at TIMESTAMPTZ NOT NULL,
    last_seen_at TIMESTAMPTZ NOT NULL,
    label VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_wallets_first_seen ON wallets(first_seen_at);
CREATE INDEX IF NOT EXISTS idx_wallets_last_seen ON wallets(last_seen_at);

CREATE TABLE IF NOT EXISTS tokens (
    address VARCHAR(66) PRIMARY KEY,
    deployer VARCHAR(66) REFERENCES wallets(address) ON DELETE SET NULL,
    creation_block BIGINT,
    creation_timestamp TIMESTAMPTZ,
    symbol VARCHAR(64),
    name VARCHAR(255),
    decimals SMALLINT NOT NULL DEFAULT 18,
    total_supply NUMERIC(38, 18),
    liquidity_usd NUMERIC(38, 4),
    holders_count BIGINT,
    top_10_holder_concentration NUMERIC(10, 4),
    mint_capability BOOLEAN,
    pause_freeze_capability BOOLEAN,
    liquidity_lock_info TEXT,
    is_honeypot BOOLEAN,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_tokens_deployer ON tokens(deployer);
CREATE INDEX IF NOT EXISTS idx_tokens_creation_block ON tokens(creation_block);
CREATE INDEX IF NOT EXISTS idx_tokens_symbol ON tokens(symbol);

CREATE TABLE IF NOT EXISTS transactions (
    hash VARCHAR(66) PRIMARY KEY,
    block_number BIGINT NOT NULL,
    from_address VARCHAR(66) NOT NULL REFERENCES wallets(address),
    to_address VARCHAR(66) REFERENCES wallets(address),
    value_eth NUMERIC(38, 18) NOT NULL DEFAULT 0,
    gas_used BIGINT NOT NULL DEFAULT 0,
    gas_price_gwei NUMERIC(28, 9) NOT NULL DEFAULT 0,
    status BOOLEAN NOT NULL DEFAULT TRUE,
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_transactions_block_number ON transactions(block_number);
CREATE INDEX IF NOT EXISTS idx_transactions_from ON transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_transactions_to ON transactions(to_address);
CREATE INDEX IF NOT EXISTS idx_transactions_timestamp ON transactions(timestamp);

CREATE TABLE IF NOT EXISTS trades (
    id UUID PRIMARY KEY,
    wallet_address VARCHAR(66) NOT NULL REFERENCES wallets(address),
    token_address VARCHAR(66) NOT NULL REFERENCES tokens(address),
    side VARCHAR(10) NOT NULL,
    amount_tokens NUMERIC(38, 18) NOT NULL,
    price_usd NUMERIC(38, 8) NOT NULL,
    volume_usd NUMERIC(38, 4) NOT NULL,
    fee_usd NUMERIC(38, 4) NOT NULL DEFAULT 0,
    tx_hash VARCHAR(66) NOT NULL REFERENCES transactions(hash),
    block_number BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_trades_wallet ON trades(wallet_address);
CREATE INDEX IF NOT EXISTS idx_trades_token ON trades(token_address);
CREATE INDEX IF NOT EXISTS idx_trades_timestamp ON trades(timestamp);
CREATE INDEX IF NOT EXISTS idx_trades_block_number ON trades(block_number);
CREATE UNIQUE INDEX IF NOT EXISTS idx_trades_unique ON trades(tx_hash, wallet_address, token_address, side);

CREATE TABLE IF NOT EXISTS wallet_scores (
    id UUID PRIMARY KEY,
    wallet_address VARCHAR(66) NOT NULL REFERENCES wallets(address),
    overall_score NUMERIC(6, 2) NOT NULL,
    category VARCHAR(32) NOT NULL,
    total_trades INT NOT NULL,
    winning_trades INT NOT NULL,
    losing_trades INT NOT NULL,
    win_rate NUMERIC(6, 4) NOT NULL,
    profit_factor NUMERIC(10, 4) NOT NULL,
    realized_pnl NUMERIC(38, 4) NOT NULL,
    max_drawdown NUMERIC(6, 4) NOT NULL,
    early_entry_ratio NUMERIC(6, 4) NOT NULL,
    factors JSONB NOT NULL,
    explanation JSONB NOT NULL,
    evaluated_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_wallet_scores_wallet ON wallet_scores(wallet_address);
CREATE INDEX IF NOT EXISTS idx_wallet_scores_overall ON wallet_scores(overall_score);
CREATE INDEX IF NOT EXISTS idx_wallet_scores_category ON wallet_scores(category);
CREATE INDEX IF NOT EXISTS idx_wallet_scores_evaluated_at ON wallet_scores(evaluated_at);

CREATE TABLE IF NOT EXISTS token_risk_scores (
    id UUID PRIMARY KEY,
    token_address VARCHAR(66) NOT NULL REFERENCES tokens(address),
    accepted BOOLEAN NOT NULL,
    score NUMERIC(6, 2) NOT NULL,
    factors JSONB NOT NULL,
    reasons JSONB NOT NULL,
    evaluated_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_token_risk_token ON token_risk_scores(token_address);
CREATE INDEX IF NOT EXISTS idx_token_risk_accepted ON token_risk_scores(accepted);
CREATE INDEX IF NOT EXISTS idx_token_risk_score ON token_risk_scores(score);
CREATE INDEX IF NOT EXISTS idx_token_risk_evaluated_at ON token_risk_scores(evaluated_at);

CREATE TABLE IF NOT EXISTS strategy_runs (
    id VARCHAR(64) PRIMARY KEY,
    strategy_type VARCHAR(64) NOT NULL,
    config JSONB NOT NULL,
    initial_balance NUMERIC(38, 4) NOT NULL,
    current_balance NUMERIC(38, 4) NOT NULL,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS signals (
    id UUID PRIMARY KEY,
    strategy_id VARCHAR(64) NOT NULL REFERENCES strategy_runs(id),
    token_address VARCHAR(66) NOT NULL REFERENCES tokens(address),
    action VARCHAR(16) NOT NULL,
    suggested_size_usd NUMERIC(38, 4) NOT NULL,
    wallet_address VARCHAR(66) REFERENCES wallets(address),
    wallet_score NUMERIC(6, 2),
    token_risk_score NUMERIC(6, 2),
    reason VARCHAR(64) NOT NULL,
    details TEXT,
    timestamp TIMESTAMPTZ NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_signals_strategy ON signals(strategy_id);
CREATE INDEX IF NOT EXISTS idx_signals_token ON signals(token_address);
CREATE INDEX IF NOT EXISTS idx_signals_wallet ON signals(wallet_address);
CREATE INDEX IF NOT EXISTS idx_signals_timestamp ON signals(timestamp);

CREATE TABLE IF NOT EXISTS paper_orders (
    id UUID PRIMARY KEY,
    signal_id UUID REFERENCES signals(id),
    strategy_id VARCHAR(64) NOT NULL REFERENCES strategy_runs(id),
    token_address VARCHAR(66) NOT NULL REFERENCES tokens(address),
    action VARCHAR(16) NOT NULL,
    requested_price NUMERIC(38, 8) NOT NULL,
    execution_price NUMERIC(38, 8) NOT NULL,
    requested_quantity NUMERIC(38, 18) NOT NULL,
    executed_quantity NUMERIC(38, 18) NOT NULL,
    volume_usd NUMERIC(38, 4) NOT NULL,
    fees NUMERIC(38, 4) NOT NULL,
    slippage NUMERIC(38, 8) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_paper_orders_strategy ON paper_orders(strategy_id);
CREATE INDEX IF NOT EXISTS idx_paper_orders_token ON paper_orders(token_address);
CREATE INDEX IF NOT EXISTS idx_paper_orders_timestamp ON paper_orders(timestamp);

CREATE TABLE IF NOT EXISTS paper_positions (
    id UUID PRIMARY KEY,
    strategy_id VARCHAR(64) NOT NULL REFERENCES strategy_runs(id),
    token_address VARCHAR(66) NOT NULL REFERENCES tokens(address),
    status VARCHAR(16) NOT NULL,
    amount_tokens NUMERIC(38, 18) NOT NULL,
    entry_price NUMERIC(38, 8) NOT NULL,
    current_price NUMERIC(38, 8) NOT NULL,
    exit_price NUMERIC(38, 8),
    invested_usd NUMERIC(38, 4) NOT NULL,
    current_value_usd NUMERIC(38, 4) NOT NULL,
    realized_pnl NUMERIC(38, 4) NOT NULL DEFAULT 0,
    unrealized_pnl NUMERIC(38, 4) NOT NULL DEFAULT 0,
    total_fees NUMERIC(38, 4) NOT NULL DEFAULT 0,
    copied_wallet VARCHAR(66) REFERENCES wallets(address),
    opened_at TIMESTAMPTZ NOT NULL,
    closed_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_paper_positions_strategy ON paper_positions(strategy_id);
CREATE INDEX IF NOT EXISTS idx_paper_positions_token ON paper_positions(token_address);
CREATE INDEX IF NOT EXISTS idx_paper_positions_status ON paper_positions(status);
CREATE INDEX IF NOT EXISTS idx_paper_positions_copied_wallet ON paper_positions(copied_wallet);

CREATE TABLE IF NOT EXISTS portfolio_snapshots (
    id UUID PRIMARY KEY,
    strategy_id VARCHAR(64) NOT NULL REFERENCES strategy_runs(id),
    cash NUMERIC(38, 4) NOT NULL,
    equity NUMERIC(38, 4) NOT NULL,
    realized_pnl NUMERIC(38, 4) NOT NULL,
    unrealized_pnl NUMERIC(38, 4) NOT NULL,
    total_fees NUMERIC(38, 4) NOT NULL,
    open_positions_count INT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_portfolio_snapshots_strategy ON portfolio_snapshots(strategy_id);
CREATE INDEX IF NOT EXISTS idx_portfolio_snapshots_timestamp ON portfolio_snapshots(timestamp);

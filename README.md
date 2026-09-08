# On-Chain Smart Trader

> **High-Performance Rust Framework for On-Chain Research, Anti-Look-Ahead Replay & Paper Trading**

`onchain-smart-trader` is a robust, modular Rust research system designed to determine whether copy-trading historically top-performing "smart wallets" on EVM blockchains yields a statistically significant edge over random selection, naive copy trading, and buy-and-hold baselines.

---

## ⚠️ Important Disclaimer & Ethical Notice

* **100% Research & Paper Trading**: This software is exclusively built for mathematical modeling, simulation, and backtesting.
* **No Real Trading Execution**: No private keys are stored, manipulated, or accessible.
* **No Fund Transfers**: The codebase contains zero endpoints or capabilities to sign or broadcast real blockchain transactions.
* **Realistic Market Simulation**: Market orders, price impact, automated market maker (AMM) slippage, and transaction fees are simulated using high-precision financial arithmetic (`rust_decimal::Decimal`).

---

## System Architecture

The project is structured as a high-cohesion, low-coupling Cargo workspace:

```text
onchain-smart-trader/
├── Cargo.toml
├── crates/
│   ├── domain/           # Strongly typed financial models (Decimal precision, zero float for money)
│   ├── chain/            # Alloy EVM RPC client, bytecode inspection & event decoder (Transfer, Swap, Sync)
│   ├── indexer/          # Idempotent EVM block scanner, checkpointing & PostgreSQL persistence
│   ├── wallet-profiler/  # Look-ahead free wallet profiling, anti-survivorship rug marking & metrics
│   ├── token-risk/       # Configurable token risk scoring, honeypot detection & anti-rug rules
│   ├── strategy/         # Smart wallet copy-trading signals, position sizers & multi-exit rules
│   ├── paper-trader/     # Simulated order executor, realistic slippage/fees & virtual portfolio
│   ├── analytics/        # Replay engine, A/B testing suite (4 models) & performance metrics (Sharpe, Drawdown)
│   ├── research/         # Empirical alpha research lab, latency degradation, scalability & Monte Carlo tests
│   └── api/              # Axum REST API server & unified CLI binary (`smart-trader`)
├── migrations/           # PostgreSQL migration scripts (indexes, foreign keys, unique constraints)
├── config/               # Strategy and risk configuration TOML files
├── docs/                 # Research audit (PHASE2_AUDIT.md) & generated reports (RESEARCH_REPORT.md)
├── docker-compose.yml    # PostgreSQL container definition
├── .env.example          # Environment variables template
└── README.md
```

### Data Pipeline Flow

```text
EVM RPC Block
      │
      ▼
Chain Event Decoder (Transfer / Uniswap Swaps)
      │
      ▼
Indexer (Idempotent Checkpoint + PostgreSQL)
      │
      ▼
Wallet Profiler (Anti-Look-Ahead Filtering: T_data <= T_decision)
      │
      ▼
Token Risk Engine (Liquidity, Concentration, Mint/Pause, Deployer Rugs)
      │
      ▼
Strategy Engine (Smart Wallet Copy / Benchmark)
      │
      ▼
Risk Manager (Portfolio Limits, Cash Reserves, Max Positions)
      │
      ▼
Paper Executor (Slippage + AMM Impact + Trading Fees)
      │
      ▼
Virtual Portfolio & Performance Analytics (Sharpe, Max Drawdown, Win Rate, ROI)
```

---

## Prerequisites & Installation

### 1. Requirements

* **Rust**: `1.85+` (stable edition 2021)
* **PostgreSQL**: `15+` (or Docker)
* **Docker & Docker Compose** (optional for local database setup)

### 2. Environment Setup

Copy `.env.example` to `.env`:

```bash
cp .env.example .env
```

Review the typed configuration:

```env
DATABASE_URL=postgres://postgres:postgrespassword@localhost:5432/trading_bot
RPC_HTTP_URL=https://eth.llamarpc.com
RPC_WS_URL=wss://eth.llamarpc.com
CHAIN_ID=1

INITIAL_PAPER_BALANCE=1000.00
MIN_LIQUIDITY=10000.00
MAX_POSITION_PERCENT=0.10
MAX_OPEN_POSITIONS=5

SLIPPAGE_BPS=50
TRADING_FEE_BPS=30

MIN_WALLET_SCORE=65.00
MIN_WALLET_TRADES=5

HOST=0.0.0.0
PORT=3000
```

### 3. Start PostgreSQL

```bash
docker compose up -d
```

### 4. Build Workspace

```bash
cargo build --workspace
```

---

## Command Line Interface (CLI)

The unified `smart-trader` binary exposes subcommands for all subsystems:

### 1. Profiling a Wallet

Evaluates historical trades strictly before the current timestamp without look-ahead bias:

```bash
cargo run -p api -- profile-wallet 0xd8da6bf26964af9d7eed9e03e53415d37aa96045
```

### 2. Scoring a Wallet

Calculates the multi-factor smart wallet score and categorization (`UNKNOWN`, `PROMISING`, `SMART`, `EXCELLENT`):

```bash
cargo run -p api -- score-wallet 0xd8da6bf26964af9d7eed9e03e53415d37aa96045
```

### 3. Evaluating Token Risk

Evaluates liquidity, holder concentration, deployer track record, and bytecode safety flags (mint, freeze):

```bash
cargo run -p api -- score-token 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
```

### 4. Running Historical Replay & A/B Testing

Runs the backtesting engine over historical data comparing 4 strategies on the exact same dataset:

```bash
cargo run -p api -- replay --from 2026-01-01 --to 2026-02-01
```

Produces the comparative benchmark:

```text
+---------------------+-------------+-----------+--------+---------+-----------+----------+--------+
| Strategy            | Initial ($) | Final ($) | ROI %  | PnL ($) | MaxDD %   | WinRate% | Trades |
+---------------------+-------------+-----------+--------+---------+-----------+----------+--------+
| Smart Wallet Copy   |     1000.00 |   1185.40 |  18.54 |  185.40 |      8.20 |    65.00 |     20 |
| Naive Copy Trading  |     1000.00 |    840.10 | -15.99 | -159.90 |     32.50 |    41.00 |     45 |
| Random Selection    |     1000.00 |    910.00 |  -9.00 |  -90.00 |     25.40 |    45.00 |     22 |
| Buy & Hold          |     1000.00 |    950.20 |  -4.98 |  -49.80 |     21.10 |     0.00 |      5 |
+---------------------+-------------+-----------+--------+---------+-----------+----------+--------+
```

### 5. Generating Statistical Reports

```bash
cargo run -p api -- report --strategy "Smart Wallet Copy"
```

### 6. Running Empirical Alpha Research & Validation

Executes full scientific research pipeline including anti-survivorship unclosed rug evaluation, Train/Val/Test split, walk-forward validation, Monte Carlo permutation testing ($p$-value), and bootstrap 95% confidence intervals:

```bash
cargo run -p api -- research --source synthetic --out docs/RESEARCH_REPORT.md
```

### 7. Evaluating Copiability & Latency Degradation Matrix

Tests performance across delay bands ($0\text{s}, 1\text{s}, 2\text{s}, 5\text{s}, 10\text{s}, 15\text{s}, 30\text{s}, 60\text{s}, 120\text{s}$):

```bash
cargo run -p api -- copyability --capital 1000
```

### 8. Starting the REST API Server

```bash
cargo run -p api -- server
```

---

## REST API Endpoints

| Method | Path | Description |
| :--- | :--- | :--- |
| `GET` | `/health` | System health and simulation mode confirmation |
| `GET` | `/wallets` | Paginated list of indexed wallets (`?limit=50&offset=0`) |
| `GET` | `/wallets/{address}` | Wallet details, track record, and latest smart score |
| `GET` | `/tokens` | Paginated list of discovered tokens |
| `GET` | `/tokens/{address}` | Token details, liquidity, holder concentration, and risk breakdown |
| `GET` | `/signals` | Historical signals generated by strategies |
| `GET` | `/positions` | List of simulated open and closed paper positions |
| `GET` | `/portfolio` | Real-time virtual balance, cash, equity, and positions |
| `GET` | `/performance` | Strategy performance snapshot (Sharpe, ROI, PnL, Drawdown) |
| `GET` | `/strategies` | List of strategy runs |
| `GET` | `/strategies/{id}` | Detailed parameters and state of a strategy run |
| `GET` | `/api/v1/research/experiments` | Run research experiment and return JSON report |
| `GET` | `/api/v1/research/report` | Return full research report in Markdown format |
| `GET` | `/api/v1/research/copyability` | Evaluate latency degradation matrix across delays |
| `GET` | `/api/v1/research/scalability` | Evaluate capital scalability curve against AMM depth |
| `POST` | `/api/v1/research/run` | Execute custom research experiment from JSON config |

---

## Scientific Rigor & Anti Look-Ahead Policy

To ensure findings represent true statistical validity:

1. **Strict Timestamp Invariance**: Any evaluation performed at simulated timestamp $T$ satisfies:
   $$\forall \text{event} \in \text{Context},\quad \text{timestamp}(\text{event}) \le T$$
2. **Sample Size Penalty**: Wallets with fewer than `min_wallet_trades` (default 5) are automatically categorized as `UNKNOWN`, avoiding survivorship and luck bias.
3. **Execution Realism**: Price slippage scales with order size relative to automated market maker liquidity:
   $$\text{Slippage} = P_{\text{market}} \times \left( \text{Slippage}_{\text{base}} + \frac{V_{\text{order}}}{2 \times L_{\text{pool}}} \right)$$
4. **Out-of-Sample Validation**: Dataset separation guarantees parameter selection does not overfit test sets.

---

## Running Automated Tests

Run the test suite across all workspace crates:

```bash
export DATABASE_URL="postgres://postgres:postgrespassword@localhost:5432/trading_bot"
cargo test --workspace
```

Coverage includes:
* Unit tests: Wallet profiling, token risk scoring, position sizing, slippage & fee calculation, exit rules.
* Integration tests: Indexer idempotency with PostgreSQL, Axum REST API endpoints.
* End-to-end tests: `Indexer -> DB -> Strategy -> Signal -> Paper Order -> Portfolio`.
* Anti-look-ahead validation tests.

---

## License

MIT License. Developed for research and simulation by Jeremy (`garder500`).

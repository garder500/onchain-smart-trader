# onchain-smart-trader

> **Research & On-Chain Paper Trading System in Rust**

A modular, high-performance Rust framework designed to investigate whether copy-trading historically top-performing "smart wallets" provides a genuine statistical edge on EVM-compatible blockchains.

## Important Notice

This is an experimental **research and simulation framework only**.
* **Zero real trading**: No private keys are stored, handled, or required.
* **No fund transfers**: No transaction execution endpoints exist.
* **100% simulated**: Orders, fills, fees, and slippage are strictly executed against an in-memory or persisted paper trading model.

## Workspace Architecture

```text
onchain-smart-trader/
├── Cargo.toml
├── crates/
│   ├── domain/           # Strongly typed core models (Decimal precision, zero floats for money)
│   ├── chain/            # Alloy EVM connection, provider abstraction & event decoding
│   ├── indexer/          # Asynchronous idempotent block & event processor
│   ├── wallet-profiler/  # Historical wallet profiling (look-ahead free)
│   ├── token-risk/       # Token risk scoring & anti-rug rule engine
│   ├── strategy/         # Smart wallet copy-trading signals & risk manager
│   ├── paper-trader/     # Simulated order executor, portfolio tracking, fee & slippage modeling
│   ├── analytics/        # Performance snapshotting, metrics (Sharpe, drawdowns, win rate), A/B testing
│   └── api/              # Axum REST API and CLI entry points
├── migrations/           # PostgreSQL migrations
├── config/               # Configuration loading and validation
├── tests/                # Replay and end-to-end integration test suites
├── docker-compose.yml    # Local PostgreSQL & service stack
├── .env.example
├── .gitignore
└── README.md
```

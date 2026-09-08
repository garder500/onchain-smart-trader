# Real Historical Dataset Quality Report

## Dataset Identity & Provenance
* **Dataset Name**: Ethereum Mainnet Uniswap V2 Real Dataset
* **Blockchain**: Ethereum Mainnet (Chain ID: 1)
* **DEX**: Uniswap V2
* **Block Range**: 25706500 to 25929500 (223000 blocks)
* **Date Range (UTC)**: 2026-08-07 23:55:23 UTC to 2026-09-07 01:58:35 UTC
* **Timespan**: 30.09 days (Requirement: >= 30 days)
* **Generated At**: 2026-09-08 02:37:22.265388177 UTC
* **Canonical SHA-256 Hash**: `8742ae50ad53d1d145b63f1c9bbc5b65e25a276de8afef83108841fcdbb30fca`

---

## Statistical Summary
* **Total Trade Records**: 846
* **Unique Trade Records**: 846
* **Unique Trader Wallets (EOA Signers)**: 393
* **Unique Tokens**: 1
* **Total Ingested Volume (USD)**: $585461.64

---

## Data Quality Verification Suite
| Quality Metric | Expected Criteria | Measured Value | Validation Status |
| :--- | :--- | :--- | :--- |
| **Duplicate Trades** | 0 duplicates | 0 | PASSED |
| **Zero or Null Prices** | 0 invalid | 0 | PASSED |
| **Zero or Null Amounts** | 0 invalid | 0 | PASSED |
| **Negative Transaction Fees** | 0 negative | 0 | PASSED |
| **Invalid Wallet Addresses** | 0 malformed | 0 | PASSED |
| **Timestamp Monotonicity** | 0 chronological reversals | 0 | PASSED |
| **Minimum Timespan** | >= 30.0 days | 30.09 days | PASSED |
| **Minimum Trade Count** | >= 500 trades | 846 trades | PASSED |
| **Minimum Wallet Count** | >= 50 unique EOAs | 393 wallets | PASSED |

---

## Overall Quality Verdict
**Final Quality Status**: **ACCEPTED_FOR_RESEARCH**

All on-chain trades are 100% verified against Ethereum logs (Swap and Sync events). No synthetic, simulated, or randomized values exist in this dataset.

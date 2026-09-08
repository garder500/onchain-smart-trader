# Real Historical Dataset Quality Report

## Dataset Identity & Provenance
* **Dataset Name**: Ethereum Mainnet Uniswap V2 Real Dataset
* **Blockchain**: Ethereum Mainnet (Chain ID: 1)
* **DEX**: Uniswap V2
* **Block Range**: 24500000 to 25929500 (1429500 blocks)
* **Date Range (UTC)**: 2026-02-20 19:01:35 UTC to 2026-09-08 03:01:02.627342 UTC
* **Timespan**: 199.33 days (Requirement: >= 30 days)
* **Generated At**: 2026-09-08 03:00:27.210244841 UTC
* **Canonical SHA-256 Hash**: `27e3d93d3790b154b9f5d56e34f67081eb654be04a024eaa532e2bb836cb96f8`

---

## Statistical Summary
* **Total Trade Records**: 3788
* **Unique Trade Records**: 3788
* **Unique Trader Wallets (EOA Signers)**: 1450
* **Unique Tokens**: 14
* **Total Ingested Volume (USD)**: $4345656.76

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
| **Minimum Timespan** | >= 30.0 days | 199.33 days | PASSED |
| **Minimum Trade Count** | >= 500 trades | 3788 trades | PASSED |
| **Minimum Wallet Count** | >= 50 unique EOAs | 1450 wallets | PASSED |

---

## Overall Quality Verdict
**Final Quality Status**: **ACCEPTED_FOR_RESEARCH**

All on-chain trades are 100% verified against Ethereum logs (Swap and Sync events). No synthetic, simulated, or randomized values exist in this dataset.

# PHASE 2.1 — VALIDATION STATUS & PRODUCTION READINESS

**Repository**: `garder500/onchain-smart-trader`  
**Current Date**: September 8, 2026  
**Status**: **PHASE 2.1 COMPLETED — ENGINE SCIENTIFICALLY SOUND, AWAITING MASSIVE REAL DATA FOR ALPHA VERDICT**

---

## 1. What Has Been Scientifically Validated

1. **Anti-Look-Ahead Execution**:
   - The chronological splitting and as-of wallet selection mechanism has been mathematically formalized and empirically proven.
   - Wallets are strictly evaluated and filtered based only on trades occurring prior to the end of the Train period ($T \le T_{\text{train}}$).
   - Future trades ($T > T_{\text{train}}$) have zero impact on wallet classification, clustering, or copiability status.

2. **Seeded PRNG & Statistical Reproducibility**:
   - All Monte Carlo random draws (Permutation tests, Bootstrap confidence intervals, Random selection benchmarks) use explicitly seeded pseudo-random number generators (`rand::rngs::StdRng`).
   - Intra-wallet correlation structure is preserved by randomizing at the `WALLET` entity level rather than shuffling isolated trades.
   - Benchmark distribution calculations produce deterministic results across identical seeds.

3. **Capital Constraint Integrity**:
   - Execution engines reject trades when portfolio cash is insufficient or maximum open position limits are reached.
   - The engine no longer assumes infinite cash or infinite liquidity.

4. **Strict Empirical Segregation**:
   - The laboratory enforces a tripartite classification: `REAL`, `SYNTHETIC`, `MIXED`.
   - Results derived from synthetic datasets are strictly flagged with `VerdictStatus::NotValidated`.
   - The CLI flag `--require-real-data` protects research pipelines from silently falling back to synthetic data.

---

## 2. What Is Tested on Synthetic Data vs Real Data

| Dimension | Synthetic Data (`SYNTHETIC`) | Real On-Chain Data (`REAL`) | Status |
|---|---|---|---|
| **Engine Mechanics** | Fully tested (41 unit and integration tests passing) | Validated via indexer and DB ingestion schema | **VALIDATED** |
| **Statistical Alpha Claim** | Flagged `NOT_VALIDATED` | Evaluated against $p < 0.05$ permutation test | **READY FOR REAL DATA** |
| **Latency Curve** | Evaluated under synthetic stress testing | Requires real block inclusion delta measurements | **FRAMEWORK READY** |
| **Liquidity / Price Impact** | Modeled using constant-product AMM formulas ($x \cdot y = k$) | Requires real Uniswap pool liquidity depths | **FRAMEWORK READY** |

---

## 3. Clear Boundaries: What Is Required Before Phase 3

> [!IMPORTANT]
> **Phase 3 (Live VPS Paper Trading / Automated Execution) MUST NOT BE STARTED** until an empirical dataset of real historical on-chain trades has been indexed and confirmed to pass all scientific hurdles.

### Mandatory Criteria to Unlock Phase 3:
1. **Real Data Corpus**:
   - Minimum 500+ real Ethereum/Base/Arbitrum DEX swaps across 50+ independent wallets over at least 30 consecutive calendar days.
2. **Statistically Significant Alpha**:
   - Empirical $p$-value from wallet-level permutation testing must satisfy $p < 0.05$.
3. **Latency Resilience**:
   - Realized net PnL must remain positive at execution delay $d \ge 2\text{s}$ (the realistic time required to index, score, and submit an on-chain transaction).
4. **Scalability**:
   - Maximum scalable capital must be at least $\$1,000$ before price impact eliminates profitability.
5. **Verdict Status**:
   - The experiment report must issue `VerdictStatus::EmpiricallySupported`.

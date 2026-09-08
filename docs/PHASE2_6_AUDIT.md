# Phase 2.6 Audit: Scaling Evidence & Informational Alpha

> **Document ID**: `docs/PHASE2_6_AUDIT.md`  
> **Target**: Comprehensive assessment of methodology, dataset limitations, subtle look-ahead risks, execution realism, and research pipeline in `onchain-smart-trader`.  
> **Date**: September 2026  
> **Workspace**: `garder500/onchain-smart-trader`

---

## 1. Executive Summary & Context

Phase 2.5 established the first verified empirical dataset:
- **846 real on-chain swaps** across **393 unique wallets** on Ethereum Mainnet Uniswap V2 (`USDC/WETH` & `USDT/WETH`) over **30.09 days**.
- Zero synthetic data, zero timestamp inversions, zero price/volume nulls, verified canonical SHA-256 hash `8742ae50ad53d1d145b63f1c9bbc5b65e25a276de8afef83108841fcdbb30fca`.
- Empirical finding: Theoretical zero-latency alpha existed ($p = 0.0060$, trade Sharpe $1.35$), but was completely wiped out by $1\,\text{s}$ execution latency. At $\ge 2\,\text{s}$ latency, expectancy turned negative.
- Scientific verdict: **`EDGE_NOT_COPIABLE`**.

Phase 2.6 aims to investigate whether this conclusion holds when:
1. Scaling the empirical dataset to $\ge 6$ months, $\ge 10{,}000$ swaps, $\ge 1{,}000$ wallets, and across multiple pools/market regimes.
2. Exploring alternative alpha extraction paradigms (**Informational Alpha**, **Multi-Wallet Consensus**, **Confirmation Windows**, **Token Attention**) rather than direct, naive transaction-copying.
3. Incorporating empirical pool reserves (via decoded `Sync` logs) into the execution/impact model rather than heuristic formulas.
4. Implementing strict walk-forward cross-validation, multiple-testing corrections (Benjamini-Hochberg / FDR), cross-pool generalization, and unseen-wallet generalization.

---

## 2. Comprehensive Inventory: What is Truly Empirical vs. Assumptions

| Component / Subsystem | Truly Empirical (Verified from Blockchain) | Assumptions / Heuristic Fallbacks | Risk Level |
|---|---|---|---|
| **Trade Records (`Trade`)** | `tx_hash`, `block_number`, `timestamp`, `wallet_address` (EOA signer or recipient), `amount_tokens`, `price_usd`, `volume_usd`, `fee_usd` (gas used $\times$ gas price $\times$ ETH price). | When RPC transaction batch queries fail/rate-limit, `tx_hash` fallback uses `topics[2]` recipient and default gas estimate `0.0003 ETH`. | LOW / MEDIUM |
| **Pool Reserves & Liquidity** | Decoded `Sync(reserve0, reserve1)` logs capture exact reserves per transaction block. | When no `Sync` event is logged in the same block/tx, pool reserves fall back to constant `10,000,000 USDC / 4,000 WETH` ($20M default). In `ScalabilityEngine`, liquidity is often passed as a single fixed scalar ($50k - $100k). | **HIGH** |
| **Latency Model (`CopiabilityEngine`)** | None (delayed trade prices are simulated via mathematical price penalty). | Adverse slippage is simulated as $\Delta p = \min(25\%, 0.008\sqrt{d} + 0.001)$ where $d$ is delay in seconds. While monotonic and punitive, it is an analytical stress formula, NOT reconstructed historical order-flow prices. | **HIGH** |
| **Wallet Behavioral Clustering** | Categorization is derived from historical trade holding times, early-entry ratios, and loss rates. | Fixed heuristic thresholds (e.g. holding $> 24\,\text{h}$ = Swing, trades $< 3$ = Degen). Temporal timing quality ($1\text{s}, 5\text{s}, 1\text{m}, 1\text{h}$) is currently missing. | MEDIUM |
| **Walk-Forward Analysis** | Uses rolling chronological trade slices. Wallets selected in Train are evaluated in Test. | Currently evaluates only 3 rolling slices on a small 846-trade dataset. No fixed calendar windowing ($60\text{d} \to 14\text{d} \to 14\text{d}$). | MEDIUM |
| **Statistical Hypothesis Testing** | Permutation test with wallet-level randomization ($N=500$), Block Bootstrap CIs ($N=1000$). | No multiple-testing correction (e.g., Benjamini-Hochberg / Bonferroni) applied across parameter grids. Single-hypothesis $p$-value only. | **HIGH** |
| **Benchmark Suite** | Evaluates Buy & Hold from token price trajectories, Random Selection Monte Carlo, and Naive Copy. | Market Momentum and Volume Momentum baselines are not yet benchmarked against smart wallet strategies. | MEDIUM |

---

## 3. Dataset Limitations in Current Implementation

1. **Restricted Scope (2 Pools, 1 Pair Type)**:
   - Only `0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc` (`USDC/WETH`) and `0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852` (`USDT/WETH`).
   - Both are high-cap stablecoin/ETH pools on Uniswap V2. Highly correlated, meaning cross-pool validation between them is trivial and lacks market diversity.
2. **Sample Size**:
   - 846 swaps across 30.09 days is statistically sufficient to reject zero-latency direct copy, but insufficient to establish multi-wallet consensus clusters (e.g. 2 or 3 distinct smart wallets entering the same token within 30 seconds).
3. **Absence of Non-Correlated & Mid-Cap Assets**:
   - WBTC/ETH, DAI/USDC, PEPE/WETH, UNI/WETH, LINK/WETH are absent. We cannot currently test cross-pool generalization or low/high volatility regimes.
4. **Reserves Not Stored in Database**:
   - While `fetch_pool_swaps` decodes `Sync` logs into memory, the `trades` table in PostgreSQL does NOT store `reserves_before` or `reserves_after` for each trade. Price impact and execution modeling must re-estimate or use heuristic constants.

---

## 4. Leakage & Look-Ahead Risks Analysis

We audited all crates for implicit look-ahead leakage:

1. **Wallet Selection (`select_copiable_wallets_as_of`)**:
   - **Status: PASS**. Code strictly checks `trade.timestamp <= as_of_timestamp` before computing wallet metrics and classifications.
2. **Chronological Splitting (`chronological_split`)**:
   - **Status: PASS**. Trades are strictly sorted by timestamp, then tx_hash, then UUID, and sliced into Train, Validation, and Test.
3. **Dynamic Discovery**:
   - **Identified subtle leakage risk**: In `ResearchRunner`, `all_wallet_trades` is aggregated over the *entire dataset* to produce the summary taxonomy table in the report. While the actual strategy backtest only copies wallets selected strictly in `train_selected_wallets`, the reporting taxonomy table computes lifetime persistence up to `end_timestamp`. To avoid any ambiguity, the research report must explicitly separate `In-Sample Discovered Wallets` from `Post-Train / Unseen Wallets`.
4. **Token & Pool Metadata**:
   - Currently, tokens are statically registered in `get_supported_pools()`. To test true dynamic discovery without survivorship bias, the engine must discover tokens and pools dynamically as swaps occur at time $t$.

---

## 5. Execution Realism Gaps

1. **Single Delay Curve Formula**:
   - The current `CopiabilityEngine` applies $\Delta p = 0.008\sqrt{d} + 0.001$. While conservative, it does not distinguish between **detection latency**, **decision latency**, **mempool propagation**, and **next-block inclusion**.
2. **Finite Cash & Position Collisions**:
   - `CopiabilityEngine` tracks `buy_queue` with `max_open_positions = 5` and initial cash $10,000. This is good, but does not model partial fills, order rejection on pool liquidity depletion, or transaction gas priority bidding.
3. **Empirical Slippage vs Mathematical Stress**:
   - Because `Sync` reserves are not persisted with each trade, slippage on large orders ($100k) uses constant-product math against an assumed pool liquidity instead of the actual on-chain reserves at that exact block.

---

## 6. The 10 Primary Risks Identified

1. **RPC Rate Limiting & Gateway Throttling on 10,000+ Swaps**: Public RPC nodes (Tenderly, PublicNode, LlamaRPC) throttle batch queries (`eth_getTransactionByHash` and `eth_getLogs`) when scanning $> 100{,}000$ blocks. Ingestion must use resilient exponential backoff, chunking, and checkpointing.
2. **False Consensus on Correlated Stablecoin Pools**: If USDC/WETH and USDT/WETH are the only pools, a wallet arbitrageur hitting both pools simultaneously might be mistaken for "two smart wallets reaching consensus."
3. **Survivorship & Selection Bias in Pool Choice**: Manually choosing existing active pools could bias towards pools that survived the 6-month period. Pool selection methodology must be documented and include multiple liquidity tiers.
4. **Multiple Hypothesis Testing ($p$-Hacking Risk)**: Testing 5 strategies (Direct Copy, Confirmation, Consensus, Wallet Momentum, Token Attention) across 10 delay points and multiple parameter grids generates $> 100$ statistical tests. Without False Discovery Rate (FDR / Benjamini-Hochberg) control, false discoveries will occur.
5. **Lack of Event-Level Reserve Persistence**: Evaluating real price impact requires knowing pool reserves at trade execution time. Storing reserves in PostgreSQL or an event dataset is required.
6. **Label Contamination / Future Information Leakage**: Creating event-based snapshots (`price_change_1s`, `price_change_5s`, `price_change_60s`) must strictly ensure that future outcome columns are never fed into strategy signals or feature extractors at $t$.
7. **Overfitting on Small Wallet Clusters**: In a 6-month dataset, some wallets may achieve $100\%$ win rates across 5 trades purely by chance ($p \approx 0.03125$). Without sample-size penalties and FDR corrections, these wallets would produce fake OOS signals.
8. **Improper Bootstrap Clustering**: Running independent trade bootstrapping destroys the temporal auto-correlation of trades in the same market block or by the same wallet. Block/wallet-cluster bootstrapping is mandatory.
9. **Unseen Wallet Degradation**: Strategies often fail completely on wallets never observed in the training set. Failing to separate "Known Wallets", "Unseen Wallets", and "New Wallets" masks real-world decay.
10. **Database Volume & Query Performance**: Scaling from 846 to $\ge 10{,}000$ swaps and hundreds of thousands of raw logs will slow down unindexed queries. Indices on `(timestamp, token_address)`, `(wallet_address, timestamp)`, and `(block_number)` must be verified.

---

## 7. Affected Files & Crates

- **`crates/indexer/src/historical.rs`**: Supported pool definitions (add BTC/ETH, mid-caps, meme/degen pools), expanded block range (6+ months), reserve persistence, checkpointing.
- **`crates/indexer/src/db.rs`**: Database schema updates for pool reserves and event snapshots, optimized indices.
- **`crates/domain/src/trade.rs` & `transaction.rs`**: Additional fields for pool reserves, empirical slippage, and event snapshot structures.
- **`crates/research/src/types.rs`**: New data structures for `InformationalAlpha`, `ConsensusSignal`, `EventSnapshot`, `MultipleTestingReport`, `RegimeMetrics`, and `CrossPoolValidation`.
- **`crates/research/src/informational_alpha.rs` (NEW)**: Implementation of Confirmation windows, Consensus engine, Wallet Momentum, and Token Attention.
- **`crates/research/src/wallet_engine.rs`**: Advanced temporal timing features ($1\text{s}, 5\text{s}, 10\text{s}, 30\text{s}, 1\text{m}, 1\text{h}$) and refined behavioral clustering.
- **`crates/research/src/validation.rs`**: Benjamini-Hochberg FDR correction, calendar-based rolling walk-forward ($60\text{d} \to 14\text{d} \to 14\text{d}$), cross-pool split, unseen wallet split, event-cluster bootstrap.
- **`crates/research/src/runner.rs`**: Orchestrating all 5 strategy families, empirical reserve evaluation, and automated conservative verdict synthesis.
- **`crates/research/src/report.rs`**: Extended markdown generator with full Phase 2.6 sections.
- **`crates/api/src/cli.rs` & `main.rs`**: Extended CLI commands (`dataset-status`, `research --strategy <direct-copy|informational-alpha|consensus|all>`, `research-report`).

---

## 8. Implementation Plan & Staged Execution Strategy

### Step 1: Audit Complete (this document)
Deliver `docs/PHASE2_6_AUDIT.md` and present architectural audit.

### Step 2: Critical Methodology & Data Structures
- Implement `crates/research/src/types.rs` additions: `StrategyFamily` (`DirectCopy`, `Confirmation`, `Consensus`, `WalletMomentum`, `TokenAttention`), `EventSnapshot`, `MultipleTestingResult`, `RegimeType`.
- Implement Benjamini-Hochberg procedure in `crates/research/src/validation.rs`.

### Step 3: Dataset Ingestion & Expansion
- Add diverse pools to `historical.rs`:
  - `USDC/WETH` (Ultra-liquid stable/ETH)
  - `USDT/WETH` (Ultra-liquid stable/ETH)
  - `WBTC/WETH` (Major crypto/ETH cross, non-stable)
  - `DAI/WETH` (Decentralized stable/ETH)
  - `LINK/WETH` or `UNI/WETH` (Liquid DeFi token)
  - `PEPE/WETH` (High-volatility meme token)
- Expand time horizon across 6 months (spanning multiple market regimes).
- Ingest $\ge 10{,}000$ swaps and store reserves with each trade.
- Generate `data/PHASE2_6_DATASET_MANIFEST.json` with canonical SHA-256 hash.
- Produce `docs/PHASE2_6_DATA_QUALITY.md`.

### Step 4: Temporal Wallet Features & Specialization
- Implement temporal timing return horizons ($1\text{s}, 5\text{s}, 10\text{s}, 30\text{s}, 1\text{m}, 5\text{m}, 15\text{m}, 1\text{h}$) in `WalletResearchEngine`.
- Classify wallet specializations: `EARLY_ENTRY`, `MOMENTUM`, `SWING`, `HIGH_RISK`, `BOT_LIKE`, `MARKET_MAKER_LIKE`, `COPYCAT`.

### Step 5: Informational Alpha Engine
- Create `crates/research/src/informational_alpha.rs`:
  - **Strategy A (Direct Copy Baseline)**: 0ms to 60s delay.
  - **Strategy B (Confirmation)**: Wait $1\text{s}, 2\text{s}, 5\text{s}, 10\text{s}, 30\text{s}$ for directional price confirmation, volume spike, or liquidity stability before entering.
  - **Strategy C (Consensus)**: Multi-wallet concurrence ($K \ge 2$ smart wallets within $\Delta t \in \{1\text{s}, 5\text{s}, 10\text{s}, 30\text{s}, 60\text{s}\}$).
  - **Strategy D (Wallet Momentum)**: Entry velocity acceleration by high-scoring wallets.
  - **Strategy E (Token Attention)**: Smart-wallet inflow velocity and buy/sell volume imbalance.

### Step 6: Strict Walk-Forward, Cross-Pool & Unseen-Wallet Evaluation
- Rolling walk-forward windows ($60\text{d}$ train, $14\text{d}$ val, $14\text{d}$ test).
- Cross-pool evaluation: Train on Pool A (`USDC/WETH`, `WBTC/WETH`), evaluate blindly on Pool B (`LINK/WETH`, `PEPE/WETH`).
- Unseen wallet evaluation: Known wallets vs. wallets never seen in training set.

### Step 7: Statistical Rigor, Multiple-Testing & Ablations
- Apply Benjamini-Hochberg FDR at $\alpha = 0.05$ across all strategy variants.
- Conduct feature ablation: remove wallet score, liquidity filter, consensus, volume acceleration, timing.
- Conduct market regime analysis (High/Low Volatility, High/Low Liquidity, Bull/Bear).

### Step 8: Execution, Report & Automation
- Run all experiments.
- Generate `docs/PHASE2_6_RESEARCH_REPORT.md` with automatic conservative verdict:
  `NO_EDGE` | `EDGE_NOT_COPIABLE` | `EDGE_TOO_SMALL` | `EDGE_UNSCALABLE` | `PROMISING_BUT_UNPROVEN` | `EMPIRICALLY_SUPPORTED`.
- Verify workspace tests, formatting, clippy, and git commits.

---

## 9. Blockers & Technical Considerations

- **RPC Bandwidth / Rate Limits**: Ingesting $10{,}000+$ swaps from public RPC nodes requires distributed slice querying or multi-endpoint fallback. Tenderly + PublicNode endpoints will be throttled gracefully using our built-in exponential backoff.
- **Database Storage**: $\approx 15{,}000$ trades with metadata requires $< 50\,\text{MB}$ in PostgreSQL, easily handled by Docker `trading-bot-postgres-1`.
- **Zero Real Trading Invariant**: Private key generation and live tx submission remain strictly disabled. All evaluations are pure on-chain empirical backtests.

# Research Experiment Report: EXP-2026-C5E163

> [!IMPORTANT]
> **DATA SOURCE**: `SYNTHETIC` | **VERDICT STATUS**: `NOT_VALIDATED`
> **WARNING**: These results were generated using deterministic/synthetic blockchain test sequences. They do NOT constitute verified on-chain live alpha. Verdict is strictly `NOT_VALIDATED`.

---

## 1. Experiment Metadata & Provenance

- **Experiment ID**: `EXP-2026-C5E163`
- **Git Commit**: `0.1.0`
- **Dataset SHA-256**: `ba60524f5a4e45b9823508975a31ab83b5d9dd1802105d23ea77f56e64a4663a`
- **Evaluation Timestamp**: `2026-09-08 02:08:35.873924202 UTC`
- **Time Window**: `2026-08-09 02:08:35.873754264 UTC -> 2026-09-06 14:08:35.873754264 UTC`
- **Total Trades Evaluated**: `50`
- **Train-Selected Wallets Count**: `2`

## 2. Scientific Verdict & Executive Summary

> **Verdict Status**: `NOT_VALIDATED`
> **Conclusion**: NOT VALIDATED: Results are based on synthetic data and serve only to test engine functionality. No statistical alpha can be inferred.

| Hypothesis / Condition | Result | Assessment |
|---|---|---|
| Statistical Significance ($p < 0.05$) | `false` | FAIL (Luck / Noise) |
| Latency Copiability ($d \ge 2\text{s}$) | `true` | PASS (Robust) |
| Max Scalable Capital | `$5000` | Viable for Micro-Capital |
| Break-Even Latency | `N/A` | Profitable across tested latency bands |

## 3. Wallet Behavioral Taxonomy & Persistence

| Wallet Address | Cluster | Persistence Score | Copiable? | Selected in Train? | Notes |
|---|---|---|---|---|---|
| `0x2222222222222222222222222222222222222222` | `SWING_TRADER` | `0.85` | Yes | **YES** | Swing trader: long holding time of 48.0 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x1111111111111111111111111111111111111111` | `MOMENTUM_TRADER` | `0.85` | Yes | **YES** | Momentum trader: intraday trend following; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |

## 4. Latency Degradation Curve (Copiability Engine)

| Delay (s) | Mode | Realized Net PnL ($) | Win Rate | Copy Efficiency | Trades | Slippage USD |
|---|---|---|---|---|---|---|
| `0s` | `StressTest` | `$2518.35` | `75.0%` | `1.00x` | `4` | `$0.00` |
| `1s` | `StressTest` | `$2401.85` | `75.0%` | `0.95x` | `4` | `$35.67` |
| `2s` | `StressTest` | `$2359.48` | `75.0%` | `0.93x` | `4` | `$48.65` |
| `5s` | `StressTest` | `$2276.22` | `75.0%` | `0.90x` | `4` | `$74.15` |
| `10s` | `StressTest` | `$2183.67` | `75.0%` | `0.86x` | `4` | `$102.49` |
| `15s` | `StressTest` | `$2113.56` | `75.0%` | `0.83x` | `4` | `$123.97` |
| `30s` | `StressTest` | `$1958.10` | `75.0%` | `0.77x` | `4` | `$171.58` |
| `60s` | `StressTest` | `$1744.66` | `75.0%` | `0.69x` | `4` | `$236.95` |
| `120s` | `StressTest` | `$1454.96` | `75.0%` | `0.57x` | `4` | `$325.67` |

## 5. Capital Scalability Curve (Liquidity Impact)

| Capital ($) | Net PnL ($) | Return (%) | Price Impact (bps) | Real Liquidity? | Capacity Status |
|---|---|---|---|---|---|
| `$100` | `$250.53` | `62.63%` | `9.9 bps` | `false` | OK |
| `$500` | `$1226.84` | `61.34%` | `49.7 bps` | `false` | OK |
| `$1000` | `$2390.30` | `59.75%` | `99.0 bps` | `false` | OK |
| `$5000` | `$9623.40` | `48.11%` | `476.1 bps` | `false` | OK |
| `$10000` | `$-3798.75` | `-37.98%` | `909.0 bps` | `false` | EXHAUSTED |
| `$50000` | `$0.00` | `0.00%` | `3333.3 bps` | `false` | EXHAUSTED |
| `$100000` | `$0.00` | `0.00%` | `5000.0 bps` | `false` | EXHAUSTED |

## 6. Out-Of-Sample Validation (Anti-Look-Ahead Split)

| Data Partition | Trades | Gross PnL ($) | Fees ($) | Net PnL ($) | Win Rate | Expectancy ($) | Trade Sharpe |
|---|---|---|---|---|---|---|---|
| Train (60%) | `21` | `$6900.00` | `$105.00` | `$6795.00` | `71.4%` | `$323.57` | `0.81` |
| Validation (20%) | `8` | `$1300.00` | `$34.00` | `$1266.00` | `50.0%` | `$158.25` | `0.49` |
| Test / Blind OOS (20%) | `5` | `$1908.00` | `$36.00` | `$1872.00` | `80.0%` | `$374.40` | `0.82` |

## 7. Statistical Rigor: Permutation Testing & Bootstrap CIs

- **Unit of Randomization**: `WALLET`
- **Observed Trade Sharpe**: `0.82`
- **Null Mean Sharpe ($H_0$)**: `0.06`
- **Null Median Sharpe**: `0.17`
- **Empirical $p$-value**: `0.0600` (NOT Statistically Significant)

### Bootstrap Confidence Intervals (Resampled by Wallet)

| Metric | Unit | Mean | Median | 95% CI Lower | 95% CI Upper | 99% CI Lower | 99% CI Upper |
|---|---|---|---|---|---|---|---|
| Win Rate | % | `0.90` | `1.00` | `0.80` | `1.00` | `0.80` | `1.00` |
| Expectancy ($) | USD | `407.51` | `393.00` | `374.40` | `493.00` | `374.40` | `493.00` |
| Trade-Level Sharpe Ratio | ratio | `0.82` | `0.82` | `0.82` | `0.82` | `0.82` | `0.82` |

## 8. Benchmark Comparisons

| Strategy | Total Return (%) | Trade Sharpe | Max Drawdown (%) | Win Rate | Monte Carlo Detail |
|---|---|---|---|---|---|
| SmartWalletCopy (Filtered OOS) | `18.72%` | `0.82` | `13.6%` | `80.0%` | - |
| NaiveCopy (Unfiltered All Wallets) | `18.72%` | `0.82` | `13.6%` | `80.0%` | - |
| RandomSelection (Monte Carlo N=100) | `9.68%` | `N/A` | `0.0%` | `100.0%` | Mean: 9.68%, StdDev: 5.42%, 95% CI: [3.93%, 14.79%] |
| Buy & Hold (Empirical Market Basket) | `0.00%` | `N/A` | `0.0%` | `0.0%` | - |


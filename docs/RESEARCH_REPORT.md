# Research Experiment Report: EXP-2026-2AD7A5

> [!IMPORTANT]
> **DATA SOURCE**: `SYNTHETIC`
> **WARNING**: These results were generated using deterministic/synthetic blockchain test sequences. They do NOT constitute verified on-chain live alpha.

---

## 1. Experiment Metadata & Provenance

- **Experiment ID**: `EXP-2026-2AD7A5`
- **Git Commit**: `0.1.0`
- **Dataset Hash**: `66820faf6f9d0ecb`
- **Evaluation Timestamp**: `2026-09-08 01:54:35.413093490 UTC`
- **Time Window**: `2026-08-09 01:54:35.412988893 UTC -> 2026-09-06 13:54:35.412988893 UTC`
- **Total Trades Evaluated**: `50`

## 2. Scientific Verdict & Executive Summary

> **Conclusion**: Statistical test failed: the observed alpha is indistinguishable from random luck (p >= 0.05). Copy-trading this wallet set has NO statistically proven edge.

| Hypothesis / Condition | Result | Assessment |
|---|---|---|
| Statistical Significance ($p < 0.05$) | `false` | FAIL (Luck / Noise) |
| Latency Copiability ($d \ge 2\text{s}$) | `true` | PASS (Robust) |
| Max Scalable Capital | `$10000` | Viable for Micro-Capital |
| Break-Even Latency | `N/A` | Profitable across tested latency bands |

## 3. Wallet Behavioral Taxonomy & Persistence

| Wallet Address | Cluster | Persistence Score | Copiable? | Notes |
|---|---|---|---|---|
| `0x1111111111111111111111111111111111111111` | `MOMENTUM_TRADER` | `0.85` | Yes | Momentum trader: intraday trend following; POTENTIALLY COPIABLE: Execution horizon permits latency |
| `0x2222222222222222222222222222222222222222` | `SWING_TRADER` | `0.85` | Yes | Swing trader: long holding time of 48.0 hours; POTENTIALLY COPIABLE: Execution horizon permits latency |

## 4. Latency Degradation Curve (Copiability Engine)

| Delay (s) | Realized Net PnL ($) | Win Rate | Copy Efficiency | Trades | Slippage USD |
|---|---|---|---|---|---|
| `0s` | `$17796.00` | `60.0%` | `1.00x` | `25` | `$0.00` |
| `1s` | `$17031.20` | `60.0%` | `0.95x` | `25` | `$222.99` |
| `2s` | `$16753.04` | `60.0%` | `0.94x` | `25` | `$304.09` |
| `5s` | `$16206.48` | `60.0%` | `0.91x` | `25` | `$463.45` |
| `10s` | `$15598.91` | `60.0%` | `0.87x` | `25` | `$640.60` |
| `15s` | `$15138.63` | `60.0%` | `0.85x` | `25` | `$774.81` |
| `30s` | `$14118.06` | `60.0%` | `0.79x` | `25` | `$1072.38` |
| `60s` | `$12716.84` | `60.0%` | `0.71x` | `25` | `$1480.94` |
| `120s` | `$10814.97` | `60.0%` | `0.60x` | `25` | `$2035.47` |

## 5. Capital Scalability Curve (Liquidity Impact)

| Capital ($) | Net PnL ($) | Return (%) | Price Impact (bps) | Capacity Status |
|---|---|---|---|---|
| `$100` | `$1771.04` | `70.84%` | `9.9 bps` | OK |
| `$500` | `$8685.76` | `69.48%` | `49.7 bps` | OK |
| `$1000` | `$16955.39` | `67.82%` | `99.0 bps` | OK |
| `$5000` | `$69493.18` | `55.59%` | `476.1 bps` | OK |
| `$10000` | `$106508.33` | `42.60%` | `909.0 bps` | OK |
| `$50000` | `$-181974.99` | `-14.55%` | `3333.3 bps` | EXHAUSTED |
| `$100000` | `$-1078466.66` | `-43.13%` | `5000.0 bps` | EXHAUSTED |

## 6. Out-Of-Sample Validation (Anti-Look-Ahead)

| Data Partition | Trades | Win Rate | Net PnL ($) | Expectancy ($) | Sharpe Ratio |
|---|---|---|---|---|---|
| Train (60%) | `21` | `71.4%` | `$6795.00` | `$323.57` | `0.81` |
| Validation (20%) | `8` | `50.0%` | `$1266.00` | `$158.25` | `0.49` |
| Test / OOS (20%) | `5` | `80.0%` | `$1872.00` | `$374.40` | `0.82` |

## 7. Statistical Rigor: Permutation Testing & Bootstrap CIs

- **Observed Sharpe**: `0.82`
- **Null Hypothesis Mean Sharpe ($H_0$)**: `0.74`
- **Empirical $p$-value**: `0.1600` (NOT Statistically Significant)

### Bootstrap 95% Confidence Intervals (1,000 resamples)

| Metric | Sample Mean | 95% CI Lower | 95% CI Upper |
|---|---|---|---|
| Win Rate | `0.65` | `0.00` | `1.00` |
| Expectancy ($) | `318.40` | `-3.00` | `784.90` |
| Sharpe Ratio | `0.85` | `0.09` | `1.56` |

## 8. Benchmark Comparisons

| Strategy | Total Return (%) | Sharpe Ratio | Max Drawdown (%) | Win Rate |
|---|---|---|---|---|
| SmartWalletCopy (Filtered) | `113.25%` | `0.82` | `2.2%` | `71.4%` |
| NaiveCopy (Unfiltered All Wallets) | `113.25%` | `0.82` | `2.2%` | `71.4%` |
| RandomSelection (Random 33% Sample) | `30.45%` | `0.62` | `7.1%` | `58.3%` |
| Buy & Hold (Baseline Benchmark) | `5.00%` | `0.85` | `12.0%` | `50.0%` |


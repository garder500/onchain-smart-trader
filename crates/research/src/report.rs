use crate::types::ExperimentReport;

pub struct ReportGenerator;

impl ReportGenerator {
    /// Formats the experiment results as a scientific Markdown report.
    pub fn generate_markdown(report: &ExperimentReport) -> String {
        let mut md = String::new();

        md.push_str(&format!(
            "# Research Experiment Report: {}\n\n",
            report.experiment_id
        ));

        // Data Source Warning Banner & Verdict Status
        md.push_str("> [!IMPORTANT]\n");
        md.push_str(&format!(
            "> **DATA SOURCE**: `{}` | **VERDICT STATUS**: `{}`\n",
            report.verdict.data_source, report.verdict.status
        ));
        if report.verdict.data_source == crate::types::DataSource::Synthetic {
            md.push_str("> **WARNING**: These results were generated using deterministic/synthetic blockchain test sequences. They do NOT constitute verified on-chain live alpha. Verdict is strictly `NOT_VALIDATED`.\n");
        } else {
            md.push_str("> **NOTICE**: Empirical on-chain event evaluation.\n");
        }
        md.push_str("\n---\n\n");

        // Metadata
        md.push_str("## 1. Experiment Metadata & Provenance\n\n");
        md.push_str(&format!(
            "- **Experiment ID**: `{}`\n",
            report.experiment_id
        ));
        md.push_str(&format!("- **Git Commit**: `{}`\n", report.git_commit));
        md.push_str(&format!(
            "- **Dataset SHA-256**: `{}`\n",
            report.dataset_hash
        ));
        md.push_str(&format!(
            "- **Evaluation Timestamp**: `{}`\n",
            report.created_at
        ));
        md.push_str(&format!(
            "- **Time Window**: `{} -> {}`\n",
            report.start_timestamp, report.end_timestamp
        ));
        md.push_str(&format!(
            "- **Total Trades Evaluated**: `{}`\n",
            report.total_events
        ));
        md.push_str(&format!(
            "- **Train-Selected Wallets Count**: `{}`\n\n",
            report.train_selected_wallets.len()
        ));

        // Scientific Verdict
        md.push_str("## 2. Scientific Verdict & Executive Summary\n\n");
        md.push_str(&format!(
            "> **Verdict Status**: `{}`\n",
            report.verdict.status
        ));
        md.push_str(&format!(
            "> **Conclusion**: {}\n\n",
            report.verdict.conclusion
        ));
        md.push_str("| Hypothesis / Condition | Result | Assessment |\n");
        md.push_str("|---|---|---|\n");
        md.push_str(&format!(
            "| Statistical Significance ($p < 0.05$) | `{}` | {} |\n",
            report.verdict.is_alpha_statistically_significant,
            if report.verdict.is_alpha_statistically_significant {
                "PASS"
            } else {
                "FAIL (Luck / Noise)"
            }
        ));
        md.push_str(&format!(
            "| Latency Copiability ($d \\ge 2\\text{{s}}$) | `{}` | {} |\n",
            report.verdict.is_copiable_under_latency,
            if report.verdict.is_copiable_under_latency {
                "PASS (Robust)"
            } else {
                "DEGRADED / NEGATIVE"
            }
        ));
        md.push_str(&format!(
            "| Max Scalable Capital | `${}` | {} |\n",
            report.verdict.maximum_scalable_capital_usd,
            if report.verdict.maximum_scalable_capital_usd > rust_decimal::Decimal::ZERO {
                "Viable for Micro-Capital"
            } else {
                "Illiquid / Unscalable"
            }
        ));
        if let Some(be_delay) = report.verdict.break_even_latency_seconds {
            md.push_str(&format!(
                "| Break-Even Latency | `{}s` | Alpha drops to zero after {}s delay |\n\n",
                be_delay, be_delay
            ));
        } else {
            md.push_str(
                "| Break-Even Latency | `N/A` | Profitable across tested latency bands |\n\n",
            );
        }

        if let Some(ref sample) = report.sample_size_assessment {
            md.push_str(&format!("> **Sample Size Reliability**: `{}`\n\n", sample));
        }

        // Wallet Behavioral Classifications
        md.push_str("## 3. Wallet Behavioral Taxonomy & Persistence\n\n");
        md.push_str("| Wallet Address | Cluster | Persistence Score | Copiable? | Selected in Train? | Notes |\n");
        md.push_str("|---|---|---|---|---|---|\n");
        let train_set: std::collections::HashSet<&str> = report
            .train_selected_wallets
            .iter()
            .map(|s| s.as_str())
            .collect();
        for w in &report.wallet_classifications {
            md.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} | {} |\n",
                w.wallet_address,
                w.cluster,
                w.persistence_score,
                if w.copiable { "Yes" } else { "No" },
                if train_set.contains(w.wallet_address.as_str()) {
                    "**YES**"
                } else {
                    "No"
                },
                w.reasoning.join("; ")
            ));
        }
        md.push('\n');

        // Latency Impact Matrix (Copiability)
        md.push_str("## 4. Latency Degradation Curve (Copiability Engine)\n\n");
        md.push_str("| Delay (s) | Mode | Realized Net PnL ($) | Win Rate | Copy Efficiency | Trades | Slippage USD |\n");
        md.push_str("|---|---|---|---|---|---|---|\n");
        for d in &report.delay_curve {
            md.push_str(&format!(
                "| `{}s` | `{:?}` | `${:.2}` | `{:.1}%` | `{:.2}x` | `{}` | `${:.2}` |\n",
                d.delay_seconds,
                d.mode,
                d.net_pnl,
                d.win_rate * rust_decimal::Decimal::from(100),
                d.copy_efficiency,
                d.trades_executed,
                d.slippage_incurred_usd
            ));
        }
        md.push('\n');

        if !report.empirical_latencies.is_empty() {
            md.push_str("### Empirical Subsequent Price Observations (Real Trades)\n\n");
            md.push_str("| Target Delay | Actual Elapsed (s) | Observed Price ($) | Price Delta (bps) | Observations | Status |\n");
            md.push_str("|---|---|---|---|---|---|\n");
            for p in &report.empirical_latencies {
                md.push_str(&format!(
                    "| `{}s` | {} | {} | {} | `{}` | `{}` |\n",
                    p.target_delay_seconds,
                    p.actual_elapsed_seconds
                        .map(|e| format!("{:.1}s", e))
                        .unwrap_or_else(|| "-".into()),
                    p.observed_price
                        .map(|v| format!("${:.2}", v))
                        .unwrap_or_else(|| "-".into()),
                    p.price_delta_bps
                        .map(|d| format!("{:.1} bps", d))
                        .unwrap_or_else(|| "-".into()),
                    p.sample_size,
                    p.status
                ));
            }
            md.push('\n');
        }

        // Scalability Curve
        md.push_str("## 5. Capital Scalability Curve (Liquidity Impact)\n\n");
        md.push_str(
            "| Capital ($) | Net PnL ($) | Return (%) | Price Impact (bps) | Real Liquidity? | Capacity Status |\n",
        );
        md.push_str("|---|---|---|---|---|---|\n");
        for s in &report.scalability_curve {
            md.push_str(&format!(
                "| `${}` | `${:.2}` | `{:.2}%` | `{:.1} bps` | `{}` | {} |\n",
                s.capital_usd,
                s.net_pnl,
                s.return_pct * rust_decimal::Decimal::from(100),
                s.avg_price_impact_bps,
                if s.is_real_liquidity {
                    "REAL_ONCHAIN"
                } else {
                    "STRESS_TEST_ASSUMPTION"
                },
                if s.capacity_exhausted {
                    "EXHAUSTED"
                } else {
                    "OK"
                }
            ));
        }
        md.push('\n');

        // Anti-Look-Ahead Validation & Splits
        md.push_str("## 6. Out-Of-Sample Validation (Anti-Look-Ahead Split)\n\n");
        md.push_str("| Data Partition | Trades | Gross PnL ($) | Fees ($) | Net PnL ($) | Win Rate | Expectancy ($) | Trade Sharpe |\n");
        md.push_str("|---|---|---|---|---|---|---|---|\n");
        md.push_str(&format!(
            "| Train (60%) | `{}` | `${:.2}` | `${:.2}` | `${:.2}` | `{:.1}%` | `${:.2}` | `{}` |\n",
            report.train_metrics.total_trades,
            report.train_metrics.gross_pnl,
            report.train_metrics.trading_fees,
            report.train_metrics.net_pnl,
            report.train_metrics.win_rate * rust_decimal::Decimal::from(100),
            report.train_metrics.expectancy,
            report
                .train_metrics
                .trade_level_sharpe
                .map(|s| format!("{:.2}", s))
                .unwrap_or_else(|| "N/A".into())
        ));
        md.push_str(&format!(
            "| Validation (20%) | `{}` | `${:.2}` | `${:.2}` | `${:.2}` | `{:.1}%` | `${:.2}` | `{}` |\n",
            report.val_metrics.total_trades,
            report.val_metrics.gross_pnl,
            report.val_metrics.trading_fees,
            report.val_metrics.net_pnl,
            report.val_metrics.win_rate * rust_decimal::Decimal::from(100),
            report.val_metrics.expectancy,
            report
                .val_metrics
                .trade_level_sharpe
                .map(|s| format!("{:.2}", s))
                .unwrap_or_else(|| "N/A".into())
        ));
        md.push_str(&format!(
            "| Test / Blind OOS (20%) | `{}` | `${:.2}` | `${:.2}` | `${:.2}` | `{:.1}%` | `${:.2}` | `{}` |\n\n",
            report.test_metrics.total_trades,
            report.test_metrics.gross_pnl,
            report.test_metrics.trading_fees,
            report.test_metrics.net_pnl,
            report.test_metrics.win_rate * rust_decimal::Decimal::from(100),
            report.test_metrics.expectancy,
            report
                .test_metrics
                .trade_level_sharpe
                .map(|s| format!("{:.2}", s))
                .unwrap_or_else(|| "N/A".into())
        ));

        // PnL Decomposition Identity
        if let Some(ref decomp) = report.pnl_decomposition {
            md.push_str("### PnL Decomposition (Factor Attribution Identity)\n\n");
            md.push_str("| Component | Amount ($) | Description |\n");
            md.push_str("|---|---|---|\n");
            md.push_str(&format!(
                "| Gross Alpha | `${:.2}` | PnL if filled at signal time with 0 friction |\n",
                decomp.gross_alpha
            ));
            md.push_str(&format!(
                "| Latency Cost | `-${:.2}` | Adverse slippage from execution delay |\n",
                decomp.latency_cost
            ));
            md.push_str(&format!(
                "| Market Impact | `-${:.2}` | Constant-product price impact on pool reserves |\n",
                decomp.market_impact
            ));
            md.push_str(&format!(
                "| DEX Fees | `-${:.2}` | Uniswap V2 LP fee (30 bps per swap) |\n",
                decomp.dex_fees
            ));
            md.push_str(&format!(
                "| Gas Cost | `-${:.2}` | Transaction gas fees |\n",
                decomp.gas_cost
            ));
            md.push_str(&format!(
                "| **Net Alpha** | **`${:.2}`** | Realized net PnL after all frictions |\n\n",
                decomp.net_alpha
            ));
        }

        // Effect Size Report
        if let Some(ref eff) = report.effect_size {
            md.push_str("### Effect Size & Economic Significance\n\n");
            md.push_str("| Metric | Value | Interpretation |\n");
            md.push_str("|---|---|---|\n");
            md.push_str(&format!(
                "| Mean Excess Return | `${:.2}` | Strategy mean minus benchmark mean |\n",
                eff.mean_excess_return
            ));
            md.push_str(&format!(
                "| Median Excess Return | `${:.2}` | Robust central tendency excess |\n",
                eff.median_excess_return
            ));
            md.push_str(&format!(
                "| Cohen's d | `{}` | Standardized effect size |\n",
                eff.cohen_d
                    .map(|d| format!("{:.2}", d))
                    .unwrap_or_else(|| "N/A".into())
            ));
            md.push_str(&format!("| Win Rate Diff vs Benchmark | `{:.1}%` | Relative advantage in winning trade frequency |\n", eff.win_rate_diff_vs_benchmark * rust_decimal::Decimal::from(100)));
            md.push_str(&format!(
                "| Sharpe Diff vs Benchmark | `{}` | Risk-adjusted excess performance |\n\n",
                eff.sharpe_diff_vs_benchmark
                    .map(|s| format!("{:.2}", s))
                    .unwrap_or_else(|| "N/A".into())
            ));
        }

        // Permutation Testing & Bootstrap CI
        md.push_str("## 7. Statistical Rigor: Permutation Testing & Bootstrap CIs\n\n");
        md.push_str(&format!(
            "- **Unit of Randomization**: `{}`\n",
            report.permutation_test.unit_of_randomization
        ));
        md.push_str(&format!(
            "- **Observed Trade Sharpe**: `{:.2}`\n",
            report.permutation_test.observed_sharpe
        ));
        md.push_str(&format!(
            "- **Null Mean Sharpe ($H_0$)**: `{:.2}`\n",
            report.permutation_test.null_mean_sharpe
        ));
        md.push_str(&format!(
            "- **Null Median Sharpe**: `{:.2}`\n",
            report.permutation_test.null_median_sharpe
        ));
        md.push_str(&format!(
            "- **Empirical $p$-value**: `{:.4}` ({})\n\n",
            report.permutation_test.p_value,
            if report.permutation_test.is_significant {
                "Statistically Significant at alpha=0.05"
            } else {
                "NOT Statistically Significant"
            }
        ));

        md.push_str("### Bootstrap Confidence Intervals (Resampled by Wallet)\n\n");
        md.push_str("| Metric | Unit | Mean | Median | 95% CI Lower | 95% CI Upper | 99% CI Lower | 99% CI Upper |\n");
        md.push_str("|---|---|---|---|---|---|---|---|\n");
        for ci in &report.bootstrap_ci {
            md.push_str(&format!(
                "| {} | {} | `{:.2}` | `{:.2}` | `{:.2}` | `{:.2}` | `{:.2}` | `{:.2}` |\n",
                ci.metric,
                ci.unit,
                ci.mean,
                ci.median,
                ci.ci_lower_95,
                ci.ci_upper_95,
                ci.ci_lower_99,
                ci.ci_upper_99
            ));
        }
        md.push('\n');

        // Benchmark Comparisons
        md.push_str("## 8. Benchmark Comparisons\n\n");
        md.push_str(
            "| Strategy | Total Return (%) | Trade Sharpe | Max Drawdown (%) | Win Rate | Monte Carlo Detail |\n",
        );
        md.push_str("|---|---|---|---|---|---|\n");
        for b in &report.benchmark_comparisons {
            let mc_detail = if let Some(ref dist) = b.random_distribution {
                format!(
                    "Mean: {:.2}%, StdDev: {:.2}%, 95% CI: [{:.2}%, {:.2}%]",
                    dist.mean_return_pct, dist.std_dev, dist.ci_lower_95, dist.ci_upper_95
                )
            } else {
                "-".into()
            };
            md.push_str(&format!(
                "| {} | `{:.2}%` | `{}` | `{:.1}%` | `{:.1}%` | {} |\n",
                b.strategy_name,
                b.total_return_pct,
                b.trade_level_sharpe
                    .map(|s| format!("{:.2}", s))
                    .unwrap_or_else(|| "N/A".into()),
                b.max_drawdown_pct,
                b.win_rate * rust_decimal::Decimal::from(100),
                mc_detail
            ));
        }
        md.push('\n');

        // Phase 2.6: Informational Alpha & Alternative Strategy Families
        if !report.strategy_family_results.is_empty() {
            md.push_str("## 9. Informational Alpha & Strategy Family Comparisons\n\n");
            md.push_str("| Strategy Family | Variant | Latency / Window | OOS Net PnL ($) | Win Rate | OOS Sharpe | Drawdown | Trades | Raw p-val | Post-FDR Adj p-val | Significant? |\n");
            md.push_str("|---|---|---|---|---|---|---|---|---|---|---|\n");
            for s in &report.strategy_family_results {
                md.push_str(&format!(
                    "| `{}` | `{}` | `{}s` | `${:.2}` | `{:.1}%` | `{}` | `{:.1}%` | `{}` | `{:.4}` | `{:.4}` | {} |\n",
                    s.family,
                    s.name,
                    s.latency_seconds,
                    s.net_pnl,
                    s.win_rate * rust_decimal::Decimal::from(100),
                    s.out_of_sample_sharpe.map(|v| format!("{:.2}", v)).unwrap_or_else(|| "N/A".into()),
                    s.max_drawdown_pct,
                    s.trades_executed,
                    s.raw_p_value,
                    s.fdr_adjusted_p_value,
                    if s.is_significant_post_fdr { "**YES**" } else { "No" }
                ));
            }
            md.push('\n');
        }

        // Multiple Testing (FDR) Control
        if let Some(ref mt) = report.multiple_testing_report {
            md.push_str("## 10. Multiple Hypothesis Testing & FDR Control\n\n");
            md.push_str(&format!(
                "- **Correction Method**: `{}`\n",
                mt.discovery_method
            ));
            md.push_str(&format!(
                "- **Total Hypotheses Evaluated**: `{}`\n",
                mt.total_hypotheses_tested
            ));
            md.push_str(&format!(
                "- **Target False Discovery Rate (FDR)**: `{:.2}`\n",
                mt.target_fdr
            ));
            md.push_str(&format!(
                "- **Discoveries (Nulls Rejected)**: `{}`\n",
                mt.rejected_null_count
            ));
            md.push_str(&format!(
                "- **Minimum Raw p-value**: `{:.6}`\n",
                mt.lowest_raw_p_value
            ));
            md.push_str(&format!(
                "- **Minimum FDR-Adjusted q-value**: `{:.6}`\n\n",
                mt.lowest_adjusted_p_value
            ));
        }

        // 11. Ablation Study
        if !report.ablation_results.is_empty() {
            md.push_str("## 11. Ablation Study (Factor Attribution)\n\n");
            md.push_str(
                "| Ablation Variant | Description | Net PnL ($) | Trade Sharpe | PnL Delta (%) |\n",
            );
            md.push_str("|---|---|---|---|---|\n");
            for a in &report.ablation_results {
                md.push_str(&format!(
                    "| `{}` | {} | `${:.2}` | `{}` | `{:.2}%` |\n",
                    a.variant_name,
                    a.description,
                    a.net_pnl,
                    a.trade_level_sharpe
                        .map(|s| format!("{:.2}", s))
                        .unwrap_or_else(|| "N/A".into()),
                    a.pnl_delta_pct
                ));
            }
            md.push('\n');
        }

        // 12. Walk-Forward Stability Analysis
        if !report.walk_forward_windows.is_empty() {
            md.push_str("## 12. Walk-Forward Stability Windows\n\n");
            md.push_str("| Window | Train Window | Test Window | Selected Wallets | IS Sharpe | OOS Sharpe | Degradation (%) |\n");
            md.push_str("|---|---|---|---|---|---|---|\n");
            for w in &report.walk_forward_windows {
                md.push_str(&format!(
                    "| Window {} | {} -> {} | {} -> {} | `{}` | `{}` | `{}` | `{:.1}%` |\n",
                    w.window_index,
                    w.train_start.format("%Y-%m-%d"),
                    w.train_end.format("%Y-%m-%d"),
                    w.test_start.format("%Y-%m-%d"),
                    w.test_end.format("%Y-%m-%d"),
                    w.selected_wallets_count,
                    w.in_sample_sharpe
                        .map(|s| format!("{:.2}", s))
                        .unwrap_or_else(|| "N/A".into()),
                    w.out_of_sample_sharpe
                        .map(|s| format!("{:.2}", s))
                        .unwrap_or_else(|| "N/A".into()),
                    w.degradation_pct
                ));
            }
            md.push('\n');
        }

        // 13. Market Regime Breakdown
        if !report.regime_breakdown.is_empty() {
            md.push_str("## 13. Market Regime Breakdown\n\n");
            md.push_str("| Market Regime | Trades | Net PnL ($) | Win Rate | Trade Sharpe | Profit Factor |\n");
            md.push_str("|---|---|---|---|---|---|\n");
            for r in &report.regime_breakdown {
                md.push_str(&format!(
                    "| `{}` | `{}` | `${:.2}` | `{:.1}%` | `{}` | `{:.2}` |\n",
                    r.regime,
                    r.trade_count,
                    r.net_pnl,
                    r.win_rate * rust_decimal::Decimal::from(100),
                    r.trade_level_sharpe
                        .map(|s| format!("{:.2}", s))
                        .unwrap_or_else(|| "N/A".into()),
                    r.profit_factor
                ));
            }
            md.push('\n');
        }

        // 14. Cross-Pool Generalization
        if !report.cross_pool_results.is_empty() {
            md.push_str("## 14. Cross-Pool Generalization\n\n");
            md.push_str("| Train Pools | Test Pool | Pool Type | Trades | Net PnL ($) | Win Rate | OOS Sharpe | Generalization Ratio |\n");
            md.push_str("|---|---|---|---|---|---|---|---|\n");
            for cp in &report.cross_pool_results {
                md.push_str(&format!(
                    "| `{}` | `{}` | `{}` | `{}` | `${:.2}` | `{:.1}%` | `{}` | `{:.2}x` |\n",
                    cp.train_pools.join(", "),
                    cp.test_pool,
                    cp.pool_type,
                    cp.trade_count,
                    cp.net_pnl,
                    cp.win_rate * rust_decimal::Decimal::from(100),
                    cp.out_of_sample_sharpe
                        .map(|s| format!("{:.2}", s))
                        .unwrap_or_else(|| "N/A".into()),
                    cp.generalization_ratio
                ));
            }
            md.push('\n');
        }

        // 15. Unseen Wallet Generalization
        if let Some(ref uw) = report.unseen_wallet_results {
            md.push_str("## 15. Unseen Wallet Generalization Analysis\n\n");
            md.push_str("| Wallet Cohort | Evaluated Trades | Net PnL ($) | Trade Sharpe | Generalization Assessment |\n");
            md.push_str("|---|---|---|---|---|\n");
            md.push_str(&format!(
                "| Known Selected Wallets | `{}` | `${:.2}` | `{}` | In-Sample / Out-Of-Sample Baseline |\n",
                uw.known_wallets_trades,
                uw.known_wallets_pnl,
                uw.known_wallets_sharpe.map(|s| format!("{:.2}", s)).unwrap_or_else(|| "N/A".into())
            ));
            md.push_str(&format!(
                "| Unseen Pre-Existing Wallets | `{}` | `${:.2}` | `{}` | Filtered Out in Train Period |\n",
                uw.unseen_wallets_trades,
                uw.unseen_wallets_pnl,
                uw.unseen_wallets_sharpe.map(|s| format!("{:.2}", s)).unwrap_or_else(|| "N/A".into())
            ));
            md.push_str(&format!(
                "| New Post-Train Wallets | `{}` | `${:.2}` | `{}` | Novel Wallets First Seen in Test Period |\n\n",
                uw.new_post_train_wallets_trades,
                uw.new_post_train_wallets_pnl,
                uw.new_post_train_wallets_sharpe.map(|s| format!("{:.2}", s)).unwrap_or_else(|| "N/A".into())
            ));
        }

        // 16. Structural Failure Modes & Execution Realities
        md.push_str("## 16. Structural Failure Modes & Execution Realities\n\n");
        md.push_str("1. **Adverse Selection & Latency Tax**: MEV searchers and toxic flow dominate sub-second price moves. At zero latency on-chain copy trading assumes simultaneous block inclusion in the same transaction position, which is structurally impossible for public reactive copy-traders without builder private orderflow.\n");
        md.push_str("2. **Survivorship Bias Elimination**: Profiling requires full round-trip attribution including unclosed positions and rug-pull dead ends. High historical win rates frequently collapse when post-entry liquidity evaporation is accounted for.\n");
        md.push_str("3. **Slippage & Price Impact Asymmetry**: As order size approaches pool reserve thresholds, quadratic constant-product slippage compounds, converting paper trading gains into net losses.\n");
        md.push_str("4. **Multiple Hypothesis Overfitting**: Testing numerous latency and parameter permutations inflates naive discovery rates; Benjamini-Hochberg FDR adjustments ensure rigorous discipline against spurious noise.\n\n");

        // 17. Scientific Conclusion & Recommendation
        md.push_str("## 17. Final Scientific Recommendation & Next Steps\n\n");
        md.push_str(&format!(
            "- **Final Status**: `{}`\n",
            report.verdict.status
        ));
        md.push_str(&format!(
            "- **Conclusion**: {}\n",
            report.verdict.conclusion
        ));
        if report.verdict.status == crate::types::VerdictStatus::EmpiricallySupported {
            md.push_str("- **Next Step**: Proceed to execution architecture, builder integration, and private relay connections under strictly constrained sizing.\n");
        } else {
            md.push_str("- **Next Step**: Do NOT deploy real capital to naive direct copy trading. Explore informational confirmation, multi-wallet consensus, or private order-flow pre-confirmations.\n");
        }

        md
    }
}

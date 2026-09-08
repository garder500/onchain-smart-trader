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

        // Data Source Warning Banner
        md.push_str("> [!IMPORTANT]\n");
        md.push_str(&format!(
            "> **DATA SOURCE**: `{}`\n",
            report.verdict.data_source
        ));
        if report.verdict.data_source == crate::types::DataSource::Synthetic {
            md.push_str("> **WARNING**: These results were generated using deterministic/synthetic blockchain test sequences. They do NOT constitute verified on-chain live alpha.\n");
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
        md.push_str(&format!("- **Dataset Hash**: `{}`\n", report.dataset_hash));
        md.push_str(&format!(
            "- **Evaluation Timestamp**: `{}`\n",
            report.created_at
        ));
        md.push_str(&format!(
            "- **Time Window**: `{} -> {}`\n",
            report.start_timestamp, report.end_timestamp
        ));
        md.push_str(&format!(
            "- **Total Trades Evaluated**: `{}`\n\n",
            report.total_events
        ));

        // Scientific Verdict
        md.push_str("## 2. Scientific Verdict & Executive Summary\n\n");
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

        // Wallet Behavioral Classifications
        md.push_str("## 3. Wallet Behavioral Taxonomy & Persistence\n\n");
        md.push_str("| Wallet Address | Cluster | Persistence Score | Copiable? | Notes |\n");
        md.push_str("|---|---|---|---|---|\n");
        for w in &report.wallet_classifications {
            md.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} |\n",
                w.wallet_address,
                w.cluster,
                w.persistence_score,
                if w.copiable { "Yes" } else { "No" },
                w.reasoning.join("; ")
            ));
        }
        md.push('\n');

        // Latency Impact Matrix (Copiability)
        md.push_str("## 4. Latency Degradation Curve (Copiability Engine)\n\n");
        md.push_str("| Delay (s) | Realized Net PnL ($) | Win Rate | Copy Efficiency | Trades | Slippage USD |\n");
        md.push_str("|---|---|---|---|---|---|\n");
        for d in &report.delay_curve {
            md.push_str(&format!(
                "| `{}s` | `${:.2}` | `{:.1}%` | `{:.2}x` | `{}` | `${:.2}` |\n",
                d.delay_seconds,
                d.net_pnl,
                d.win_rate * rust_decimal::Decimal::from(100),
                d.copy_efficiency,
                d.trades_executed,
                d.slippage_incurred_usd
            ));
        }
        md.push('\n');

        // Scalability Curve
        md.push_str("## 5. Capital Scalability Curve (Liquidity Impact)\n\n");
        md.push_str(
            "| Capital ($) | Net PnL ($) | Return (%) | Price Impact (bps) | Capacity Status |\n",
        );
        md.push_str("|---|---|---|---|---|\n");
        for s in &report.scalability_curve {
            md.push_str(&format!(
                "| `${}` | `${:.2}` | `{:.2}%` | `{:.1} bps` | {} |\n",
                s.capital_usd,
                s.net_pnl,
                s.return_pct * rust_decimal::Decimal::from(100),
                s.avg_price_impact_bps,
                if s.capacity_exhausted {
                    "EXHAUSTED"
                } else {
                    "OK"
                }
            ));
        }
        md.push('\n');

        // Anti-Look-Ahead Validation & Splits
        md.push_str("## 6. Out-Of-Sample Validation (Anti-Look-Ahead)\n\n");
        md.push_str("| Data Partition | Trades | Win Rate | Net PnL ($) | Expectancy ($) | Sharpe Ratio |\n");
        md.push_str("|---|---|---|---|---|---|\n");
        md.push_str(&format!(
            "| Train (60%) | `{}` | `{:.1}%` | `${:.2}` | `${:.2}` | `{}` |\n",
            report.train_metrics.total_trades,
            report.train_metrics.win_rate * rust_decimal::Decimal::from(100),
            report.train_metrics.net_pnl,
            report.train_metrics.expectancy,
            report
                .train_metrics
                .sharpe_ratio
                .map(|s| format!("{:.2}", s))
                .unwrap_or_else(|| "N/A".into())
        ));
        md.push_str(&format!(
            "| Validation (20%) | `{}` | `{:.1}%` | `${:.2}` | `${:.2}` | `{}` |\n",
            report.val_metrics.total_trades,
            report.val_metrics.win_rate * rust_decimal::Decimal::from(100),
            report.val_metrics.net_pnl,
            report.val_metrics.expectancy,
            report
                .val_metrics
                .sharpe_ratio
                .map(|s| format!("{:.2}", s))
                .unwrap_or_else(|| "N/A".into())
        ));
        md.push_str(&format!(
            "| Test / OOS (20%) | `{}` | `{:.1}%` | `${:.2}` | `${:.2}` | `{}` |\n\n",
            report.test_metrics.total_trades,
            report.test_metrics.win_rate * rust_decimal::Decimal::from(100),
            report.test_metrics.net_pnl,
            report.test_metrics.expectancy,
            report
                .test_metrics
                .sharpe_ratio
                .map(|s| format!("{:.2}", s))
                .unwrap_or_else(|| "N/A".into())
        ));

        // Permutation Testing & Bootstrap CI
        md.push_str("## 7. Statistical Rigor: Permutation Testing & Bootstrap CIs\n\n");
        md.push_str(&format!(
            "- **Observed Sharpe**: `{:.2}`\n",
            report.permutation_test.observed_sharpe
        ));
        md.push_str(&format!(
            "- **Null Hypothesis Mean Sharpe ($H_0$)**: `{:.2}`\n",
            report.permutation_test.null_mean_sharpe
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

        md.push_str("### Bootstrap 95% Confidence Intervals (1,000 resamples)\n\n");
        md.push_str("| Metric | Sample Mean | 95% CI Lower | 95% CI Upper |\n");
        md.push_str("|---|---|---|---|\n");
        for ci in &report.bootstrap_ci {
            md.push_str(&format!(
                "| {} | `{:.2}` | `{:.2}` | `{:.2}` |\n",
                ci.metric, ci.mean, ci.ci_lower_95, ci.ci_upper_95
            ));
        }
        md.push('\n');

        // Benchmark Comparisons
        md.push_str("## 8. Benchmark Comparisons\n\n");
        md.push_str(
            "| Strategy | Total Return (%) | Sharpe Ratio | Max Drawdown (%) | Win Rate |\n",
        );
        md.push_str("|---|---|---|---|---|\n");
        for b in &report.benchmark_comparisons {
            md.push_str(&format!(
                "| {} | `{:.2}%` | `{}` | `{:.1}%` | `{:.1}%` |\n",
                b.strategy_name,
                b.total_return_pct,
                b.sharpe_ratio
                    .map(|s| format!("{:.2}", s))
                    .unwrap_or_else(|| "N/A".into()),
                b.max_drawdown_pct,
                b.win_rate * rust_decimal::Decimal::from(100)
            ));
        }
        md.push('\n');

        md
    }
}

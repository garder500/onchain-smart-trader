use domain::PerformanceSnapshot;
use std::fmt::Write;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn format_report(
        perf: &PerformanceSnapshot,
        top_wallets: &[String],
        top_tokens: &[String],
        rejected_count: usize,
    ) -> String {
        let mut out = String::new();

        let sharpe_display = match perf.sharpe_ratio {
            Some(s) => format!("{:.2}", s),
            None => "N/A (insufficient variance)".into(),
        };

        let _ = writeln!(out, "Smart Wallet Paper Trading Report");
        let _ = writeln!(out, "=================================");
        let _ = writeln!(out);
        let _ = writeln!(out, "Strategy:         {}", perf.strategy_id);
        let _ = writeln!(
            out,
            "Period:           {} -> {}",
            perf.period_start.format("%Y-%m-%d %H:%M:%S UTC"),
            perf.period_end.format("%Y-%m-%d %H:%M:%S UTC")
        );
        let _ = writeln!(out, "Initial balance:  ${:.2}", perf.initial_balance);
        let _ = writeln!(out, "Final balance:    ${:.2}", perf.final_balance);
        let _ = writeln!(out);
        let _ = writeln!(out, "ROI:              {:.2}%", perf.roi_percent);
        let _ = writeln!(out, "PnL:              ${:.2}", perf.net_pnl);
        let _ = writeln!(out, "Max drawdown:     {:.2}%", perf.max_drawdown_percent);
        let _ = writeln!(out, "Win rate:         {:.2}%", perf.win_rate);
        let _ = writeln!(out, "Profit factor:    {:.2}", perf.profit_factor);
        let _ = writeln!(out, "Sharpe:           {}", sharpe_display);
        let _ = writeln!(out);
        let _ = writeln!(out, "Trades:           {}", perf.total_trades);
        let _ = writeln!(out, "Winning:          {}", perf.winning_trades);
        let _ = writeln!(out, "Losing:           {}", perf.losing_trades);
        let _ = writeln!(out);
        let _ = writeln!(out, "Fees:             ${:.2}", perf.total_fees);
        let _ = writeln!(out, "Slippage:         ${:.2}", perf.total_slippage);
        let _ = writeln!(out);

        let _ = writeln!(out, "Top wallets:");
        if top_wallets.is_empty() {
            let _ = writeln!(out, "  (none)");
        } else {
            for w in top_wallets {
                let _ = writeln!(out, "  - {}", w);
            }
        }

        let _ = writeln!(out);
        let _ = writeln!(out, "Top tokens:");
        if top_tokens.is_empty() {
            let _ = writeln!(out, "  (none)");
        } else {
            for t in top_tokens {
                let _ = writeln!(out, "  - {}", t);
            }
        }

        let _ = writeln!(out);
        let _ = writeln!(out, "Rejected opportunities: {}", rejected_count);

        out
    }

    pub fn format_ab_test_comparison(reports: &[PerformanceSnapshot]) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "+---------------------+-------------+-----------+--------+---------+-----------+----------+--------+");
        let _ = writeln!(out, "| Strategy            | Initial ($) | Final ($) | ROI %  | PnL ($) | MaxDD %   | WinRate% | Trades |");
        let _ = writeln!(out, "+---------------------+-------------+-----------+--------+---------+-----------+----------+--------+");

        for r in reports {
            let _ = writeln!(
                out,
                "| {:<19} | {:>11.2} | {:>9.2} | {:>6.2} | {:>7.2} | {:>9.2} | {:>8.2} | {:>6} |",
                r.strategy_id,
                r.initial_balance,
                r.final_balance,
                r.roi_percent,
                r.net_pnl,
                r.max_drawdown_percent,
                r.win_rate,
                r.total_trades
            );
        }
        let _ = writeln!(out, "+---------------------+-------------+-----------+--------+---------+-----------+----------+--------+");

        out
    }
}

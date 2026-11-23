use std::fmt::Write;
use umuryango_i18n::{Locale, t};

use super::summary::MothersReport;
use crate::models::WalletChannel;

pub struct LocalizedExporter;

impl LocalizedExporter {
    /// Formats a clean, WhatsApp-ready bilingual text/Markdown financial summary (Zero emojis, Rule 4)
    pub fn format_mothers_report(report: &MothersReport, locale: Locale) -> String {
        let mut out = String::new();

        // 1. Header
        let title = t(locale, "report.mothers_report_title");
        let start_str = report.period_start.format("%Y-%m-%d").to_string();
        let end_str = report.period_end.format("%Y-%m-%d").to_string();

        let _ = writeln!(out, "*{}*", title);
        let _ = writeln!(out, "Period: {} - {}", start_str, end_str);
        let _ = writeln!(out, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // 2. High-Level Summary
        let opening_lbl = t(locale, "report.opening_balance");
        let income_lbl = t(locale, "report.total_income");
        let expenses_lbl = t(locale, "report.total_expenses");
        let closing_lbl = t(locale, "report.closing_balance");

        let _ = writeln!(out, "*{}*: {:.2} RWF", opening_lbl, report.opening_balance);
        let _ = writeln!(out, "*{}*: +{:.2} RWF", income_lbl, report.total_income);
        let _ = writeln!(out, "*{}*: -{:.2} RWF", expenses_lbl, report.total_expenses);
        let _ = writeln!(out, "*{}*: {:.2} RWF", closing_lbl, report.closing_balance);
        let _ = writeln!(out);

        // 3. Custody Breakdown (Where the cash is right now)
        let custody_lbl = t(locale, "report.closing_balance_by_person");
        let _ = writeln!(out, "*{}*:", custody_lbl);
        if report.custodian_balances.is_empty() {
            let _ = writeln!(out, "  • (N/A)");
        } else {
            for cb in &report.custodian_balances {
                let channel_label = match cb.channel {
                    WalletChannel::Bank => t(locale, "wallets.bank"),
                    WalletChannel::MobileMoney => t(locale, "wallets.mobile_money"),
                    WalletChannel::CashInHand => t(locale, "wallets.cash_in_hand"),
                };
                let _ = writeln!(
                    out,
                    "  • [{}] {}: {:.2} {}",
                    channel_label, cb.wallet_name, cb.balance, cb.currency
                );
            }
        }
        let _ = writeln!(out);

        // 4. Expenses Breakdown by Category
        if !report.expenses_by_category.is_empty() {
            let cat_lbl = t(locale, "report.expenses_by_category");
            let _ = writeln!(out, "*{}*:", cat_lbl);
            for (cat, amt) in &report.expenses_by_category {
                let _ = writeln!(out, "  • {}: {:.2} RWF", cat, amt);
            }
            let _ = writeln!(out);
        }

        // 5. Active Projects
        if !report.active_projects.is_empty() {
            let proj_lbl = t(locale, "report.active_projects");
            let _ = writeln!(out, "*{}*:", proj_lbl);
            for p in &report.active_projects {
                let _ = writeln!(
                    out,
                    "  • {}: {:.2} / {:.2} RWF",
                    p.name, p.allocated_amount, p.target_amount
                );
            }
            let _ = writeln!(out);
        }

        // 6. Loans
        if !report.open_loans.is_empty() {
            let loans_lbl = t(locale, "report.loans_summary");
            let _ = writeln!(out, "*{}*:", loans_lbl);
            for l in &report.open_loans {
                let _ = writeln!(
                    out,
                    "  • {}: {:.2} RWF",
                    l.counterparty_name, l.balance_remaining
                );
            }
            let _ = writeln!(out);
        }

        // 7. Future Plans
        if !report.future_plans.is_empty() {
            let plans_lbl = t(locale, "report.future_plans");
            let _ = writeln!(out, "*{}*:", plans_lbl);
            for fp in &report.future_plans {
                let _ = writeln!(out, "  • {}: {:.2} RWF", fp.title, fp.estimated_cost);
            }
            let _ = writeln!(out);
        }

        let _ = writeln!(out, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        let footer = t(locale, "report.footer_tagline");
        let _ = writeln!(out, "_{}_", footer);

        out
    }
}

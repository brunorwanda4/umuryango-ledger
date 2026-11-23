use std::fmt::Write;
use umuryango_i18n::{Locale, Translator, terms};

use super::summary::MothersReport;
use crate::models::WalletChannel;

pub struct LocalizedExporter;

impl LocalizedExporter {
    /// Formats a clean, WhatsApp-ready bilingual text/Markdown financial summary
    pub fn format_mothers_report(report: &MothersReport, locale: Locale) -> String {
        let mut out = String::new();

        // 1. Header
        let title = Translator::t(locale, terms::MOTHERS_REPORT_TITLE, None);
        let start_str = report.period_start.format("%Y-%m-%d").to_string();
        let end_str = report.period_end.format("%Y-%m-%d").to_string();

        let _ = writeln!(out, "🇷🇼 *{}*", title);
        let _ = writeln!(out, "📅 {} - {}", start_str, end_str);
        let _ = writeln!(out, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // 2. High-Level Summary
        let opening_lbl = Translator::t(locale, terms::OPENING_BALANCE, None);
        let income_lbl = Translator::t(locale, terms::TOTAL_INCOME, None);
        let expenses_lbl = Translator::t(locale, terms::TOTAL_EXPENSES, None);
        let closing_lbl = Translator::t(locale, terms::CLOSING_BALANCE, None);

        let _ = writeln!(
            out,
            "💰 *{}*: {:.2} RWF",
            opening_lbl, report.opening_balance
        );
        let _ = writeln!(out, "📈 *{}*: +{:.2} RWF", income_lbl, report.total_income);
        let _ = writeln!(
            out,
            "📉 *{}*: -{:.2} RWF",
            expenses_lbl, report.total_expenses
        );
        let _ = writeln!(
            out,
            "🏦 *{}*: {:.2} RWF",
            closing_lbl, report.closing_balance
        );
        let _ = writeln!(out);

        // 3. Custody Breakdown (Where the cash is right now)
        let custody_lbl = Translator::t(locale, terms::CASH_CUSTODY_BREAKDOWN, None);
        let _ = writeln!(out, "👤 *{}*:", custody_lbl);
        if report.custodian_balances.is_empty() {
            let _ = writeln!(out, "  • (N/A)");
        } else {
            for cb in &report.custodian_balances {
                let channel_icon = match cb.channel {
                    WalletChannel::Bank => "🏦",
                    WalletChannel::MobileMoney => "📱",
                    WalletChannel::CashInHand => "💵",
                };
                let _ = writeln!(
                    out,
                    "  {} {}: {:.2} {}",
                    channel_icon, cb.wallet_name, cb.balance, cb.currency
                );
            }
        }
        let _ = writeln!(out);

        // 4. Expenses Breakdown by Category
        if !report.expenses_by_category.is_empty() {
            let cat_lbl = if locale == Locale::Rw {
                "Ibyakoreshejwe ku Byiciro"
            } else {
                "Expenses by Category"
            };
            let _ = writeln!(out, "📊 *{}*:", cat_lbl);
            for (cat, amt) in &report.expenses_by_category {
                let _ = writeln!(out, "  • {}: {:.2} RWF", cat, amt);
            }
            let _ = writeln!(out);
        }

        // 5. Active Projects
        if !report.active_projects.is_empty() {
            let proj_lbl = Translator::t(locale, terms::ACTIVE_PROJECTS, None);
            let _ = writeln!(out, "🏗️ *{}*:", proj_lbl);
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
            let loans_lbl = Translator::t(locale, terms::LOANS_SUMMARY, None);
            let _ = writeln!(out, "🤝 *{}*:", loans_lbl);
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
            let plans_lbl = Translator::t(locale, terms::FUTURE_PLANS, None);
            let _ = writeln!(out, "🎯 *{}*:", plans_lbl);
            for fp in &report.future_plans {
                let _ = writeln!(out, "  • {}: {:.2} RWF", fp.title, fp.estimated_cost);
            }
            let _ = writeln!(out);
        }

        let _ = writeln!(out, "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        let _ = writeln!(out, "🇷🇼 _Umuryango Ledger - Igitabo cy'Umuryango_");

        out
    }
}

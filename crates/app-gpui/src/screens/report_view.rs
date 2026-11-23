use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_core_domain::reporting::{generate_mothers_report, LocalizedExporter};
use umuryango_i18n::{terms, Locale, Translator};

use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct ReportViewScreen;

impl ReportViewScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let now = chrono::Utc::now();
        let period_start = now - chrono::Duration::days(30);

        // Compute Mother's Report using pure domain financial brain
        let report = generate_mothers_report(
            state.active_scope,
            &state.wallets,
            &state.transactions,
            &state.projects,
            &state.loans,
            &state.future_plans,
            period_start,
            now,
        );

        let whatsapp_text = LocalizedExporter::format_mothers_report(&report, state.locale);
        let title = Translator::t(state.locale, terms::MOTHERS_REPORT_TITLE, None);

        div()
            .id(id)
            .flex_1()
            .h_full()
            .overflow_y_scroll()
            .p(px(24.0))
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            // Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .mb(px(24.0))
                    .child(
                        div()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(title),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(if state.locale == Locale::Rw {
                                        "Raporo y'umuryango yoroshye gusobanukira no gusangiza kuri WhatsApp"
                                    } else {
                                        "Family financial summary formatted for WhatsApp & zero-login sharing"
                                    }),
                            ),
                    )
                    // Export Actions: WhatsApp Share & Magic Link
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(10.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .py(px(8.0))
                                    .px(px(14.0))
                                    .rounded(px(8.0))
                                    .bg(theme.income_color)
                                    .text_color(theme.bg_canvas)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .cursor_pointer()
                                    .child("💬 WhatsApp Share"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .py(px(8.0))
                                    .px(px(14.0))
                                    .rounded(px(8.0))
                                    .bg(theme.accent_primary)
                                    .text_color(theme.bg_canvas)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .cursor_pointer()
                                    .child("🔗 Magic Link"),
                            ),
                    ),
            )
            // KPI Summary Cards
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(16.0))
                    .mb(px(24.0))
                    // Total Income Card
                    .child(
                        div()
                            .flex_1()
                            .p(px(18.0))
                            .rounded(px(12.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(Translator::t(state.locale, terms::TOTAL_INCOME, None)),
                            )
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.income_color)
                                    .child(format!("{} RWF", report.total_income)),
                            ),
                    )
                    // Total Expenses Card
                    .child(
                        div()
                            .flex_1()
                            .p(px(18.0))
                            .rounded(px(12.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(Translator::t(state.locale, terms::TOTAL_EXPENSES, None)),
                            )
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.expense_color)
                                    .child(format!("{} RWF", report.total_expenses)),
                            ),
                    )
                    // Closing Net Balance
                    .child(
                        div()
                            .flex_1()
                            .p(px(18.0))
                            .rounded(px(12.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(Translator::t(state.locale, terms::CLOSING_BALANCE, None)),
                            )
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.accent_primary)
                                    .child(format!("{} RWF", report.closing_balance)),
                            ),
                    ),
            )
            // Section: Cash Breakdown Held by Each Custodian (Person)
            .child(
                div()
                    .p(px(20.0))
                    .rounded(px(12.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .mb(px(20.0))
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .mb(px(12.0))
                            .child(Translator::t(state.locale, terms::CASH_CUSTODY_BREAKDOWN, None)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .children(report.custodian_balances.iter().map(|c| {
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .items_center()
                                    .py(px(8.0))
                                    .px(px(12.0))
                                    .rounded(px(6.0))
                                    .bg(theme.bg_canvas)
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(div().text_sm().child("👤"))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                    .text_color(theme.text_primary)
                                                    .child(c.wallet_name.clone()),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                            .text_color(theme.income_color)
                                            .child(format!("{} RWF", c.balance)),
                                    )
                            })),
                    ),
            )
            // Section: WhatsApp Formatted Preview Box
            .child(
                div()
                    .p(px(20.0))
                    .rounded(px(12.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .mb(px(12.0))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(if state.locale == Locale::Rw {
                                        "Ibyoherezwa kuri WhatsApp (Formatted)"
                                    } else {
                                        "WhatsApp Message Preview"
                                    }),
                            )
                            .child(
                                div()
                                    .py(px(4.0))
                                    .px(px(10.0))
                                    .rounded(px(6.0))
                                    .bg(theme.bg_elevated)
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .cursor_pointer()
                                    .child("📋 Copy Text"),
                            ),
                    )
                    .child(
                        div()
                            .p(px(14.0))
                            .rounded(px(8.0))
                            .bg(theme.bg_canvas)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_primary)
                                    .child(whatsapp_text),
                            ),
                    ),
            )
    }
}

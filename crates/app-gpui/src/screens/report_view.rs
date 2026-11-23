use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::reporting::{LocalizedExporter, generate_mothers_report};
use umuryango_i18n::{Translator, terms};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct ReportViewScreen;

impl ReportViewScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
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

        let share_text = whatsapp_text.clone();
        let share_success = umuryango_i18n::t(state.locale, "report.share_copied");

        let magic_link = format!(
            "https://ledger.umuryango.rw/reports/magic/{}",
            state.active_scope.family_id().unwrap_or(state.user_id)
        );
        let link_success = umuryango_i18n::t(state.locale, "report.link_copied");

        let copy_preview_text = whatsapp_text.clone();
        let copy_preview_success = share_success.clone();

        div()
            .id(id)
            .flex_1()
            .h_full()
            .overflow_y_scroll()
            .pt(px(24.0))
            .pb(px(24.0) + state.safe_area.total_bottom_px())
            .pl(px(24.0) + state.safe_area.left_px())
            .pr(px(24.0) + state.safe_area.right_px())
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
                                    .child(umuryango_i18n::t(state.locale, "report.report_subtitle")),
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
                                    .id("report-whatsapp-share-btn")
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
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.copy_to_clipboard(share_text.clone(), share_success.clone(), cx);
                                    }))
                                    .child(app_icon(
                                        AppIcon::Share,
                                        IconSize::Xs,
                                        IconTone::OnPrimaryButton,
                                        theme,
                                    ))
                                    .child("WhatsApp Share"),
                            )
                            .child(
                                div()
                                    .id("report-magic-link-btn")
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
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.copy_to_clipboard(magic_link.clone(), link_success.clone(), cx);
                                    }))
                                    .child(app_icon(
                                        AppIcon::Language,
                                        IconSize::Xs,
                                        IconTone::OnPrimaryButton,
                                        theme,
                                    ))
                                    .child("Magic Link"),
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
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(Translator::t(
                                                state.locale,
                                                terms::TOTAL_INCOME,
                                                None,
                                            )),
                                    )
                                    .child(app_icon(
                                        AppIcon::Income,
                                        IconSize::Sm,
                                        IconTone::Income,
                                        theme,
                                    )),
                            )
                            .child(
                                div()
                                    .mt(px(4.0))
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
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(Translator::t(
                                                state.locale,
                                                terms::TOTAL_EXPENSES,
                                                None,
                                            )),
                                    )
                                    .child(app_icon(
                                        AppIcon::Expense,
                                        IconSize::Sm,
                                        IconTone::Expense,
                                        theme,
                                    )),
                            )
                            .child(
                                div()
                                    .mt(px(4.0))
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
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(Translator::t(
                                                state.locale,
                                                terms::CLOSING_BALANCE,
                                                None,
                                            )),
                                    )
                                    .child(app_icon(
                                        AppIcon::Reports,
                                        IconSize::Sm,
                                        IconTone::AccentPrimary,
                                        theme,
                                    )),
                            )
                            .child(
                                div()
                                    .mt(px(4.0))
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
                            .child(Translator::t(
                                state.locale,
                                terms::CASH_CUSTODY_BREAKDOWN,
                                None,
                            )),
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
                                            .child(app_icon(
                                                AppIcon::Members,
                                                IconSize::Xs,
                                                IconTone::Muted,
                                                theme,
                                            ))
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
                                    .child(umuryango_i18n::t(state.locale, "report.whatsapp_preview")),
                            )
                            .child(
                                div()
                                    .id("report-copy-preview-btn")
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .py(px(4.0))
                                    .px(px(10.0))
                                    .rounded(px(6.0))
                                    .bg(theme.bg_elevated)
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.copy_to_clipboard(copy_preview_text.clone(), copy_preview_success.clone(), cx);
                                    }))
                                    .child(app_icon(
                                        AppIcon::Copy,
                                        IconSize::Xs,
                                        IconTone::Muted,
                                        theme,
                                    ))
                                    .child("Copy Text"),
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

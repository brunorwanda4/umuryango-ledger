use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px, relative};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use umuryango_i18n::{Translator, terms};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::components::tx_row::TxRow;
use crate::components::wallet_card::WalletCard;
use crate::state::{ActiveModal, AppState};
use crate::theme::SemanticTheme;

pub struct DashboardScreen;

impl DashboardScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let total_balance_str = format!("{} RWF", state.total_balance());
        let income_str = format!("+{} RWF", state.total_income());
        let expense_str = format!("-{} RWF", state.total_expenses());
        let projects_budget_str = format!("{} RWF", state.active_projects_budget());

        let total_balance_label = Translator::t(state.locale, terms::CLOSING_BALANCE, None);
        let income_label = Translator::t(state.locale, terms::TOTAL_INCOME, None);
        let expense_label = Translator::t(state.locale, terms::TOTAL_EXPENSES, None);
        let projects_label = Translator::t(state.locale, terms::ACTIVE_PROJECTS, None);
        let category_breakdown = state.expense_category_breakdown();
        let category_colors = [
            theme.expense_color,
            theme.accent_primary,
            theme.income_color,
            theme.accent_secondary,
            theme.text_muted,
            theme.handover_color,
        ];

        div()
            .id(id)
            .flex_1()
            .h_full()
            .pt(px(24.0))
            .pb(px(24.0) + state.safe_area.total_bottom_px())
            .pl(px(24.0) + state.safe_area.left_px())
            .pr(px(24.0) + state.safe_area.right_px())
            .overflow_y_scroll()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            // Header: Screen Title + Refresh & Quick Log Buttons
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .mb(px(20.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(if state.active_scope.is_family() {
                                        state.active_family_name.clone().unwrap_or_else(|| {
                                            Translator::t(state.locale, terms::FAMILY_SCOPE, None)
                                        })
                                    } else {
                                        Translator::t(state.locale, terms::PERSONAL_SCOPE, None)
                                    }),
                            )
                            .child(div().text_xs().text_color(theme.text_muted).child(
                                umuryango_i18n::t(state.locale, "dashboard.overview_subtitle"),
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("dashboard-refresh-btn")
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(theme.bg_surface)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_color(theme.text_primary)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.refresh_active_scope(cx);
                                    }))
                                    .child(app_icon(
                                        AppIcon::SyncPending,
                                        IconSize::Xs,
                                        IconTone::Muted,
                                        theme,
                                    ))
                                    .child(umuryango_i18n::t(state.locale, "common.refresh")),
                            )
                            .child(
                                div()
                                    .id("dashboard-quick-entry-btn")
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(14.0))
                                    .py(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(theme.accent_primary)
                                    .text_color(theme.bg_canvas)
                                    .text_sm()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_quick_entry(window, cx);
                                    }))
                                    .child(app_icon(
                                        AppIcon::Add,
                                        IconSize::Sm,
                                        IconTone::OnPrimaryButton,
                                        theme,
                                    ))
                                    .child(umuryango_i18n::t(
                                        state.locale,
                                        "dashboard.quick_log_btn",
                                    )),
                            ),
                    ),
            )
            // Row 1: KPI Cards Grid
            .child(
                div()
                    .grid()
                    .grid_cols(4)
                    .gap(px(14.0))
                    .mb(px(24.0))
                    .child(Self::render_kpi_card(
                        "kpi-total-balance",
                        AppIcon::Dashboard,
                        IconTone::Income,
                        &total_balance_label,
                        &total_balance_str,
                        theme.income_color,
                        theme,
                    ))
                    .child(Self::render_kpi_card(
                        "kpi-total-income",
                        AppIcon::Income,
                        IconTone::Income,
                        &income_label,
                        &income_str,
                        theme.income_color,
                        theme,
                    ))
                    .child(Self::render_kpi_card(
                        "kpi-total-expenses",
                        AppIcon::Expense,
                        IconTone::Expense,
                        &expense_label,
                        &expense_str,
                        theme.expense_color,
                        theme,
                    ))
                    .child(Self::render_kpi_card(
                        "kpi-projects-budget",
                        AppIcon::Projects,
                        IconTone::AccentPrimary,
                        &projects_label,
                        &projects_budget_str,
                        theme.accent_primary,
                        theme,
                    )),
            )
            // Row 2: Custody Wallets Grid + Add Wallet Button
            .child(
                div()
                    .mb(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .mb(px(12.0))
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(Translator::t(
                                        state.locale,
                                        terms::CASH_CUSTODY_BREAKDOWN,
                                        None,
                                    )),
                            )
                            .child(
                                div()
                                    .id("dashboard-add-wallet-btn")
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(10.0))
                                    .py(px(5.0))
                                    .rounded(px(6.0))
                                    .bg(theme.bg_surface)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(theme.accent_primary)
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_modal(ActiveModal::AddWallet, window, cx);
                                    }))
                                    .child(app_icon(
                                        AppIcon::Add,
                                        IconSize::Xs,
                                        IconTone::AccentPrimary,
                                        theme,
                                    ))
                                    .child(umuryango_i18n::t(state.locale, "wallets.add_wallet")),
                            ),
                    )
                    .child(if state.wallets.is_empty() {
                        div()
                            .p(px(20.0))
                            .rounded(px(10.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(umuryango_i18n::t(state.locale, "wallets.no_wallets"))
                            .into_any_element()
                    } else {
                        div()
                            .grid()
                            .grid_cols(3)
                            .gap(px(14.0))
                            .children(state.wallets.iter().enumerate().map(|(idx, w)| {
                                let custodian = state.custodian_name_for_wallet(w);
                                WalletCard::render(
                                    format!("dash-wallet-{}", idx),
                                    w,
                                    custodian.as_deref(),
                                    state.locale,
                                    theme,
                                )
                            }))
                            .into_any_element()
                    }),
            )
            // Row 3: Charts Section (Dynamic Inflow/Outflow Bar & Dynamic Expense Breakdown)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(16.0))
                    .mb(px(24.0))
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
                                    .text_sm()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .mb(px(12.0))
                                    .child(umuryango_i18n::t(
                                        state.locale,
                                        "dashboard.inflow_outflow_chart",
                                    )),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(Self::render_chart_bar(
                                        &Translator::t(state.locale, terms::INCOME, None),
                                        state.total_income(),
                                        state.total_income() + state.total_expenses(),
                                        theme.income_color,
                                        theme,
                                    ))
                                    .child(Self::render_chart_bar(
                                        &Translator::t(state.locale, terms::EXPENSE, None),
                                        state.total_expenses(),
                                        state.total_income() + state.total_expenses(),
                                        theme.expense_color,
                                        theme,
                                    )),
                            ),
                    )
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
                                    .text_sm()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .mb(px(12.0))
                                    .child(umuryango_i18n::t(
                                        state.locale,
                                        "dashboard.categories_chart",
                                    )),
                            )
                            .child(if category_breakdown.is_empty() {
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(umuryango_i18n::t(
                                        state.locale,
                                        "dashboard.no_expenses_yet",
                                    ))
                                    .into_any_element()
                            } else {
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .children(category_breakdown.into_iter().enumerate().map(
                                        |(idx, (cat, amt, pct))| {
                                            let color =
                                                category_colors[idx % category_colors.len()];
                                            Self::render_donut_row(
                                                &cat,
                                                &format!("{}% ({} RWF)", pct, amt),
                                                color,
                                                theme,
                                            )
                                        },
                                    ))
                                    .into_any_element()
                            }),
                    ),
            )
            // Row 4: Recent Activity List
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(10.0))
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(umuryango_i18n::t(state.locale, "dashboard.recent_activity")),
                    )
                    .child(div().flex().flex_col().gap(px(8.0)).children(
                        if state.transactions.is_empty() {
                            vec![
                                div()
                                    .p(px(16.0))
                                    .rounded(px(8.0))
                                    .bg(theme.bg_surface)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(umuryango_i18n::t(
                                        state.locale,
                                        "dashboard.no_recent_activity",
                                    ))
                                    .into_any_element(),
                            ]
                        } else {
                            state
                                .transactions
                                .iter()
                                .take(6)
                                .enumerate()
                                .map(|(idx, tx)| {
                                    let actor = state.actor_name(tx.authorized_by);
                                    TxRow::render(
                                        format!("dash-tx-{}", idx),
                                        tx,
                                        actor.as_deref(),
                                        state.locale,
                                        theme,
                                    )
                                    .into_any_element()
                                })
                                .collect()
                        },
                    )),
            )
    }

    fn render_kpi_card(
        id: impl Into<ElementId>,
        icon: AppIcon,
        icon_tone: IconTone,
        label: &str,
        value: &str,
        value_color: gpui_kit::Hsla,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        div()
            .id(id)
            .p(px(16.0))
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
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.text_muted)
                            .child(label.to_string()),
                    )
                    .child(app_icon(icon, IconSize::Sm, icon_tone, theme)),
            )
            .child(
                div()
                    .mt(px(6.0))
                    .text_xl()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .text_color(value_color)
                    .child(value.to_string()),
            )
    }

    fn render_chart_bar(
        label: &str,
        amount: Decimal,
        total: Decimal,
        bar_color: gpui_kit::Hsla,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let ratio = if total > Decimal::ZERO {
            (amount / total).to_f32().unwrap_or(0.0).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let pct = (ratio * 100.0).round() as i32;

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .text_color(theme.text_primary)
                    .child(format!("{} ({} RWF)", label, amount))
                    .child(format!("{}%", pct)),
            )
            .child(
                div()
                    .w_full()
                    .h(px(12.0))
                    .rounded(px(6.0))
                    .bg(theme.bg_elevated)
                    .child(
                        div()
                            .h_full()
                            .rounded(px(6.0))
                            .bg(bar_color)
                            .w(relative(ratio)),
                    ),
            )
    }

    fn render_donut_row(
        category: &str,
        percent: &str,
        color: gpui_kit::Hsla,
        theme: &SemanticTheme,
    ) -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(div().w(px(10.0)).h(px(10.0)).rounded(px(3.0)).bg(color))
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_primary)
                            .child(category.to_string()),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .text_color(theme.text_muted)
                    .child(percent.to_string()),
            )
    }
}

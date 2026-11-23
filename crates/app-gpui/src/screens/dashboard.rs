use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use rust_decimal::Decimal;
use umuryango_i18n::{terms, Locale, Translator};

use crate::components::tx_row::TxRow;
use crate::components::wallet_card::WalletCard;
use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct DashboardScreen;

impl DashboardScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let total_balance_str = format!("{} RWF", state.total_balance());
        let income_str = format!("+{} RWF", state.total_income());
        let expense_str = format!("-{} RWF", state.total_expenses());
        let projects_budget_str = format!("{} RWF", state.active_projects_budget());

        let total_balance_label = Translator::t(state.locale, terms::CLOSING_BALANCE, None);
        let income_label = Translator::t(state.locale, terms::TOTAL_INCOME, None);
        let expense_label = Translator::t(state.locale, terms::TOTAL_EXPENSES, None);
        let projects_label = Translator::t(state.locale, terms::ACTIVE_PROJECTS, None);

        div()
            .id(id)
            .flex_1()
            .h_full()
            .p(px(24.0))
            .overflow_y_scroll()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            // Header: Screen Title + Action Button
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
                                        state
                                            .active_family_name
                                            .clone()
                                            .unwrap_or_else(|| "Family Ledger".to_string())
                                    } else {
                                        Translator::t(state.locale, terms::PERSONAL_SCOPE, None)
                                    }),
                            )
                            .child(div().text_xs().text_color(theme.text_muted).child(
                                if state.locale == Locale::Rw {
                                    "Incamake y'umutungo n'ibikorwa biheruka"
                                } else {
                                    "Financial overview & recent movements"
                                },
                            )),
                    )
                    .child(
                        div()
                            .px(px(14.0))
                            .py(px(8.0))
                            .rounded(px(8.0))
                            .bg(theme.accent_primary)
                            .text_color(theme.bg_canvas)
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .cursor_pointer()
                            .child(if state.locale == Locale::Rw {
                                "+ Andika Igikorwa"
                            } else {
                                "+ Log Transaction"
                            }),
                    ),
            )
            // Row 1: KPI Cards Grid
            .child(
                div()
                    .grid()
                    .grid_cols(4)
                    .gap(px(14.0))
                    .mb(px(24.0))
                    // Total Balance Card
                    .child(Self::render_kpi_card(
                        "kpi-total-balance",
                        &total_balance_label,
                        &total_balance_str,
                        theme.income_color,
                        theme,
                    ))
                    // Income Card
                    .child(Self::render_kpi_card(
                        "kpi-total-income",
                        &income_label,
                        &income_str,
                        theme.income_color,
                        theme,
                    ))
                    // Expenses Card
                    .child(Self::render_kpi_card(
                        "kpi-total-expenses",
                        &expense_label,
                        &expense_str,
                        theme.expense_color,
                        theme,
                    ))
                    // Projects Card
                    .child(Self::render_kpi_card(
                        "kpi-projects-budget",
                        &projects_label,
                        &projects_budget_str,
                        theme.accent_primary,
                        theme,
                    )),
            )
            // Row 2: Custody Wallets Grid
            .child(
                div()
                    .mb(px(24.0))
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
                    .child(div().grid().grid_cols(3).gap(px(14.0)).children(
                        state.wallets.iter().enumerate().map(|(idx, w)| {
                            WalletCard::render(
                                format!("dash-wallet-{}", idx),
                                w,
                                Some(&state.user_name),
                                state.locale,
                                theme,
                            )
                        }),
                    )),
            )
            // Row 3: Charts Section (Income vs Expenses BarChart & Category DonutChart)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(16.0))
                    .mb(px(24.0))
                    // BarChart: Monthly Income vs Expenses
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
                                    .child(if state.locale == Locale::Rw {
                                        "Ibyinjijwe n'Ibyasohotse (Bar Chart)"
                                    } else {
                                        "Income vs Expenses (Bar Chart)"
                                    }),
                            )
                            // Visual Bar Chart representation
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(12.0))
                                    .child(Self::render_chart_bar(
                                        "Income",
                                        state.total_income(),
                                        state.total_income() + state.total_expenses(),
                                        theme.income_color,
                                        theme,
                                    ))
                                    .child(Self::render_chart_bar(
                                        "Expenses",
                                        state.total_expenses(),
                                        state.total_income() + state.total_expenses(),
                                        theme.expense_color,
                                        theme,
                                    )),
                            ),
                    )
                    // DonutChart: Spending by Category
                    .child(
                        div()
                            .w(px(320.0))
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
                                    .child(if state.locale == Locale::Rw {
                                        "Ibyiciro by'Amakoreshereze (Donut)"
                                    } else {
                                        "Spending by Category (Donut)"
                                    }),
                            )
                            // Visual Donut Chart Breakdown
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(Self::render_donut_row(
                                        "🥗 Food / Ibiribwa",
                                        "45%",
                                        theme.expense_color,
                                        theme,
                                    ))
                                    .child(Self::render_donut_row(
                                        "💡 Utilities / Imirasire",
                                        "25%",
                                        theme.accent_primary,
                                        theme,
                                    ))
                                    .child(Self::render_donut_row(
                                        "🏥 Health / Ubuzima",
                                        "15%",
                                        theme.income_color,
                                        theme,
                                    ))
                                    .child(Self::render_donut_row(
                                        "📦 Others / Ibindi",
                                        "15%",
                                        theme.text_muted,
                                        theme,
                                    )),
                            ),
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
                            .child(if state.locale == Locale::Rw {
                                "Ibyakozwe vuba"
                            } else {
                                "Recent Activity"
                            }),
                    )
                    .child(
                        div().flex().flex_col().gap(px(8.0)).children(
                            state
                                .transactions
                                .iter()
                                .take(6)
                                .enumerate()
                                .map(|(idx, tx)| {
                                    TxRow::render(
                                        format!("dash-tx-{}", idx),
                                        tx,
                                        Some(&state.user_name),
                                        state.locale,
                                        theme,
                                    )
                                }),
                        ),
                    ),
            )
    }

    fn render_kpi_card(
        id: impl Into<ElementId>,
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
                    .text_xs()
                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                    .text_color(theme.text_muted)
                    .child(label.to_string()),
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
        let pct = if total > Decimal::ZERO {
            ((amount / total) * Decimal::new(100, 0)).to_string()
        } else {
            "0".to_string()
        };

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
                    .child(div().h_full().rounded(px(6.0)).bg(bar_color).w(px(200.0))),
            )
    }

    fn render_donut_row(
        category: &str,
        percent: &str,
        color: gpui_kit::Hsla,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
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

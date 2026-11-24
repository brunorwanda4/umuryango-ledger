use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use rust_decimal::Decimal;
use umuryango_core_domain::models::{Loan, LoanDirection};
use umuryango_i18n::{Locale, Translator, terms};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{ActiveModal, AppState};
use crate::theme::SemanticTheme;

pub struct LoansScreen;

impl LoansScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let title = Translator::t(state.locale, terms::LOANS_SUMMARY, None);

        let lent_loans: Vec<&Loan> = state
            .loans
            .iter()
            .filter(|l| l.direction == LoanDirection::Lent)
            .collect();

        let borrowed_loans: Vec<&Loan> = state
            .loans
            .iter()
            .filter(|l| l.direction == LoanDirection::Borrowed)
            .collect();

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
            // Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .mb(px(20.0))
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(title),
                    )
                    .child(
                        div()
                            .id("loans-add-btn")
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(theme.accent_primary)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.open_modal(ActiveModal::AddLoan, window, cx);
                            }))
                            .child(app_icon(
                                AppIcon::Add,
                                IconSize::Xs,
                                IconTone::OnPrimaryButton,
                                theme,
                            ))
                            .child(umuryango_i18n::t(state.locale, "loans.record_loan_btn")),
                    ),
            )
            // Section 1: Lent Out
            .child(
                div()
                    .mb(px(24.0))
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .mb(px(12.0))
                            .child(umuryango_i18n::t(state.locale, "loans.lent_out_title")),
                    )
                    .child(div().flex().flex_col().gap(px(10.0)).children(
                        if lent_loans.is_empty() {
                            vec![
                                div()
                                    .p(px(16.0))
                                    .rounded(px(8.0))
                                    .bg(theme.bg_surface)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(umuryango_i18n::t(state.locale, "loans.no_lent_out"))
                                    .into_any_element(),
                            ]
                        } else {
                            lent_loans
                                .into_iter()
                                .map(|l| {
                                    Self::render_loan_row(l, true, state.locale, theme, cx)
                                        .into_any_element()
                                })
                                .collect()
                        },
                    )),
            )
            // Section 2: Borrowed
            .child(
                div()
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .mb(px(12.0))
                            .child(umuryango_i18n::t(state.locale, "loans.borrowed_title")),
                    )
                    .child(div().flex().flex_col().gap(px(10.0)).children(
                        if borrowed_loans.is_empty() {
                            vec![
                                div()
                                    .p(px(16.0))
                                    .rounded(px(8.0))
                                    .bg(theme.bg_surface)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(umuryango_i18n::t(state.locale, "loans.no_borrowed"))
                                    .into_any_element(),
                            ]
                        } else {
                            borrowed_loans
                                .into_iter()
                                .map(|l| {
                                    Self::render_loan_row(l, false, state.locale, theme, cx)
                                        .into_any_element()
                                })
                                .collect()
                        },
                    )),
            )
    }

    fn render_loan_row(
        loan: &Loan,
        is_lent: bool,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let color = if is_lent {
            theme.income_color
        } else {
            theme.expense_color
        };
        let tone = if is_lent {
            IconTone::Income
        } else {
            IconTone::Expense
        };
        let is_repaid = loan.balance_remaining <= Decimal::ZERO;
        let loan_id = loan.id;
        let counterparty = loan.counterparty_name.clone();
        let remaining = loan.balance_remaining;

        div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .p(px(14.0))
            .rounded(px(10.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(app_icon(AppIcon::Loans, IconSize::Sm, tone, theme))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(loan.counterparty_name.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(loan.notes.clone().unwrap_or_default()),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(14.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_end()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(color)
                                    .child(format!("{} RWF", loan.balance_remaining)),
                            )
                            .child(div().text_xs().text_color(theme.text_muted).child(format!(
                                "{}: {} RWF",
                                umuryango_i18n::t(locale, "loans.principal"),
                                loan.principal_amount
                            ))),
                    )
                    .child(if is_repaid {
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(theme.income_color.opacity(0.15))
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.income_color)
                            .child(umuryango_i18n::t(locale, "loans.status_repaid"))
                            .into_any_element()
                    } else {
                        div()
                            .id(format!("loan-repay-btn-{}", loan_id))
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_elevated)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.text_primary)
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.open_modal(
                                    ActiveModal::RepayLoan {
                                        loan_id,
                                        counterparty: counterparty.clone(),
                                        remaining,
                                    },
                                    window,
                                    cx,
                                );
                            }))
                            .child(umuryango_i18n::t(locale, "loans.repay_btn"))
                            .into_any_element()
                    }),
            )
    }
}

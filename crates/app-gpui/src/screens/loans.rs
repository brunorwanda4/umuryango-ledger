use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_core_domain::models::{Loan, LoanDirection};
use umuryango_i18n::{terms, Locale, Translator};

use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct LoansScreen;

impl LoansScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
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
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(theme.accent_primary)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .child(if state.locale == Locale::Rw {
                                "+ Andika Umwenda"
                            } else {
                                "+ Record Loan"
                            }),
                    ),
            )
            // Section 1: Lent Out (Ayo twagurije)
            .child(
                div()
                    .mb(px(24.0))
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .mb(px(12.0))
                            .child(if state.locale == Locale::Rw {
                                "Ayo Twagurije Abandi (Lent Out)"
                            } else {
                                "Money Lent Out"
                            }),
                    )
                    .child(
                        div().flex().flex_col().gap(px(10.0)).children(
                            lent_loans
                                .into_iter()
                                .map(|l| Self::render_loan_row(l, true, state.locale, theme)),
                        ),
                    ),
            )
            // Section 2: Borrowed (Ayo twagujije)
            .child(
                div()
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .mb(px(12.0))
                            .child(if state.locale == Locale::Rw {
                                "Ayo Twagujije (Money Borrowed)"
                            } else {
                                "Money Borrowed"
                            }),
                    )
                    .child(
                        div().flex().flex_col().gap(px(10.0)).children(
                            borrowed_loans
                                .into_iter()
                                .map(|l| Self::render_loan_row(l, false, state.locale, theme)),
                        ),
                    ),
            )
    }

    fn render_loan_row(
        loan: &Loan,
        is_lent: bool,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let color = if is_lent {
            theme.income_color
        } else {
            theme.expense_color
        };

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
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(format!("Principal: {} RWF", loan.principal_amount)),
                            ),
                    )
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_elevated)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.text_primary)
                            .cursor_pointer()
                            .child(if locale == Locale::Rw {
                                "Kwishyura"
                            } else {
                                "Repay"
                            }),
                    ),
            )
    }
}

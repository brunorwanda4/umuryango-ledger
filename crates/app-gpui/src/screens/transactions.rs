use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_i18n::{terms, Locale, Translator};

use crate::components::tx_row::TxRow;
use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct TransactionsScreen;

impl TransactionsScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let title = Translator::t(state.locale, terms::NAV_TRANSACTIONS, None);

        div()
            .id(id)
            .flex_1()
            .h_full()
            .p(px(24.0))
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
                            .px(px(14.0))
                            .py(px(8.0))
                            .rounded(px(8.0))
                            .bg(theme.accent_primary)
                            .text_color(theme.bg_canvas)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .cursor_pointer()
                            .child(if state.locale == Locale::Rw {
                                "+ Andika Igikorwa"
                            } else {
                                "+ Log Transaction"
                            }),
                    ),
            )
            // Transactions full list
            .child(div().flex().flex_col().gap(px(8.0)).children(
                state.transactions.iter().enumerate().map(|(idx, tx)| {
                    TxRow::render(
                        format!("txs-row-{}", idx),
                        tx,
                        Some(&state.user_name),
                        state.locale,
                        theme,
                    )
                }),
            ))
    }
}

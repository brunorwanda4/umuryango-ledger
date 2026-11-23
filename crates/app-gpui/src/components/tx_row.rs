use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_core_domain::models::{Transaction, TransactionType};
use umuryango_i18n::{terms, Locale, Translator};

use crate::theme::SemanticTheme;

pub struct TxRow;

impl TxRow {
    pub fn amount_display(tx: &Transaction) -> String {
        let prefix = match tx.tx_type {
            TransactionType::Income => "+",
            TransactionType::Expense => "-",
            TransactionType::Handover => "~",
        };
        format!("{} {} RWF", prefix, tx.amount)
    }

    pub fn type_label(tx_type: TransactionType, locale: Locale) -> String {
        match tx_type {
            TransactionType::Income => Translator::t(locale, terms::INCOME, None),
            TransactionType::Expense => Translator::t(locale, terms::EXPENSE, None),
            TransactionType::Handover => Translator::t(locale, terms::HANDOVER, None),
        }
    }

    pub fn render(
        id: impl Into<ElementId>,
        tx: &Transaction,
        authorizer_name: Option<&str>,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let type_str = Self::type_label(tx.tx_type, locale);
        let amount_str = Self::amount_display(tx);
        let amount_color = match tx.tx_type {
            TransactionType::Income => theme.income_color,
            TransactionType::Expense => theme.expense_color,
            TransactionType::Handover => theme.handover_color,
        };

        let type_bg = match tx.tx_type {
            TransactionType::Income => theme.income_color.opacity(0.15),
            TransactionType::Expense => theme.expense_color.opacity(0.15),
            TransactionType::Handover => theme.accent_primary.opacity(0.15),
        };

        div()
            .id(id)
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .py(px(10.0))
            .px(px(14.0))
            .rounded(px(8.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            // Left: Type badge + category + description + date
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(3.0))
                            .rounded(px(6.0))
                            .bg(type_bg)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .text_color(amount_color)
                            .child(type_str),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .text_color(theme.text_primary)
                                    .child(tx.category.clone()),
                            )
                            .child(
                                div().text_xs().text_color(theme.text_muted).child(
                                    tx.description.clone().unwrap_or_else(|| {
                                        tx.tx_date.format("%Y-%m-%d").to_string()
                                    }),
                                ),
                            ),
                    ),
            )
            // Right: Amount + Authorized By
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_end()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(amount_color)
                            .child(amount_str),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(format!("✓ {}", authorizer_name.unwrap_or("Authorized"))),
                    ),
            )
    }
}

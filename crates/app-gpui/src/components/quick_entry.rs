use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use rust_decimal::Decimal;
use std::str::FromStr;
use umuryango_core_domain::models::{Transaction, TransactionType, Wallet};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{terms, Locale, Translator};
use uuid::Uuid;

use crate::theme::SemanticTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickEntryMode {
    Income,
    Expense,
    Handover,
}

#[derive(Debug, Clone)]
pub struct QuickEntryForm {
    pub mode: QuickEntryMode,
    pub source_wallet_id: Option<Uuid>,
    pub destination_wallet_id: Option<Uuid>,
    pub amount_input: String,
    pub category: String,
    pub description: String,
    pub authorized_by: Option<Uuid>,
    pub paid_to: String,
    pub receipt_url: String,
}

impl Default for QuickEntryForm {
    fn default() -> Self {
        Self {
            mode: QuickEntryMode::Expense,
            source_wallet_id: None,
            destination_wallet_id: None,
            amount_input: String::new(),
            category: "Food".to_string(),
            description: String::new(),
            authorized_by: None,
            paid_to: String::new(),
            receipt_url: String::new(),
        }
    }
}

pub struct QuickEntryDialog;

impl QuickEntryDialog {
    pub fn render(
        id: impl Into<ElementId>,
        form: &QuickEntryForm,
        wallets: &[Wallet],
        _is_family: bool,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let income_label = Translator::t(locale, terms::INCOME, None);
        let expense_label = Translator::t(locale, terms::EXPENSE, None);
        let handover_label = Translator::t(locale, terms::HANDOVER, None);

        let selected_source = wallets
            .iter()
            .find(|w| Some(w.id) == form.source_wallet_id)
            .map(|w| format!("{} ({} RWF)", w.name, w.balance))
            .unwrap_or_else(|| {
                if locale == Locale::Rw {
                    "Hitamo Konti".to_string()
                } else {
                    "Select Wallet".to_string()
                }
            });

        div()
            .id(id)
            .w(px(480.0))
            .p(px(24.0))
            .rounded(px(14.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            // Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .mb(px(16.0))
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(if locale == Locale::Rw {
                                "Andika Igikorwa Gishya"
                            } else {
                                "Log New Transaction"
                            }),
                    ),
            )
            // Mode selector tabs
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .mb(px(16.0))
                    // Expense tab
                    .child(
                        div()
                            .flex_1()
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .text_center()
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .bg(if form.mode == QuickEntryMode::Expense {
                                theme.expense_color
                            } else {
                                theme.bg_elevated
                            })
                            .text_color(if form.mode == QuickEntryMode::Expense {
                                theme.bg_canvas
                            } else {
                                theme.text_primary
                            })
                            .cursor_pointer()
                            .child(expense_label),
                    )
                    // Income tab
                    .child(
                        div()
                            .flex_1()
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .text_center()
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .bg(if form.mode == QuickEntryMode::Income {
                                theme.income_color
                            } else {
                                theme.bg_elevated
                            })
                            .text_color(if form.mode == QuickEntryMode::Income {
                                theme.bg_canvas
                            } else {
                                theme.text_primary
                            })
                            .cursor_pointer()
                            .child(income_label),
                    )
                    // Handover tab (only in Family mode)
                    .child(
                        div()
                            .flex_1()
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .text_center()
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .bg(if form.mode == QuickEntryMode::Handover {
                                theme.accent_primary
                            } else {
                                theme.bg_elevated
                            })
                            .text_color(if form.mode == QuickEntryMode::Handover {
                                theme.bg_canvas
                            } else {
                                theme.text_primary
                            })
                            .cursor_pointer()
                            .child(handover_label),
                    ),
            )
            // Form body
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    // Source wallet selector
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(div().text_xs().text_color(theme.text_muted).child(
                                if form.mode == QuickEntryMode::Handover {
                                    "From Custody Wallet / Konti Ivamo"
                                } else {
                                    "Source Wallet / Konti Ikoreshwa"
                                },
                            ))
                            .child(
                                div()
                                    .p(px(10.0))
                                    .rounded(px(8.0))
                                    .bg(theme.bg_canvas)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_sm()
                                    .text_color(theme.text_primary)
                                    .child(selected_source),
                            ),
                    )
                    // Amount input
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("Amount (RWF) / Umubare"),
                            )
                            .child(
                                div()
                                    .p(px(10.0))
                                    .rounded(px(8.0))
                                    .bg(theme.bg_canvas)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_base()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(if form.amount_input.is_empty() {
                                        "0 RWF".to_string()
                                    } else {
                                        form.amount_input.clone()
                                    }),
                            ),
                    )
                    // Category & description row
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap(px(4.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("Category / Icyiciro"),
                                    )
                                    .child(
                                        div()
                                            .p(px(8.0))
                                            .rounded(px(6.0))
                                            .bg(theme.bg_canvas)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .text_sm()
                                            .text_color(theme.text_primary)
                                            .child(form.category.clone()),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap(px(4.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("Paid To / Wishyuwe"),
                                    )
                                    .child(
                                        div()
                                            .p(px(8.0))
                                            .rounded(px(6.0))
                                            .bg(theme.bg_canvas)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .text_sm()
                                            .text_color(theme.text_primary)
                                            .child(if form.paid_to.is_empty() {
                                                "-".to_string()
                                            } else {
                                                form.paid_to.clone()
                                            }),
                                    ),
                            ),
                    ),
            )
            // Action button
            .child(
                div()
                    .mt(px(20.0))
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(theme.accent_primary)
                    .text_center()
                    .text_sm()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .text_color(theme.bg_canvas)
                    .cursor_pointer()
                    .child(if locale == Locale::Rw {
                        "✓ Emeza Igikorwa"
                    } else {
                        "✓ Confirm Transaction"
                    }),
            )
    }

    /// Build domain transaction from form state
    pub fn build_transaction(
        form: &QuickEntryForm,
        scope: &LedgerScope,
        actor_id: Uuid,
    ) -> Result<Transaction, String> {
        let amount = Decimal::from_str(&form.amount_input)
            .map_err(|_| "Invalid amount format".to_string())?;

        let source_id = form
            .source_wallet_id
            .ok_or_else(|| "Source wallet is required".to_string())?;

        let tx_type = match form.mode {
            QuickEntryMode::Income => TransactionType::Income,
            QuickEntryMode::Expense => TransactionType::Expense,
            QuickEntryMode::Handover => TransactionType::Handover,
        };

        let paid_to = if form.paid_to.trim().is_empty() {
            None
        } else {
            Some(form.paid_to.trim().to_string())
        };

        let now = chrono::Utc::now();
        Ok(Transaction {
            id: Uuid::new_v4(),
            scope: scope.clone(),
            source_wallet_id: source_id,
            destination_wallet_id: form.destination_wallet_id,
            tx_type,
            amount,
            currency: "RWF".to_string(),
            category: form.category.clone(),
            description: if form.description.trim().is_empty() {
                None
            } else {
                Some(form.description.trim().to_string())
            },
            authorized_by: form.authorized_by.unwrap_or(actor_id),
            paid_to,
            tx_date: now,
            created_at: now,
        })
    }
}

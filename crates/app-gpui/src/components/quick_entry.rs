use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use rust_decimal::Decimal;
use std::str::FromStr;
use umuryango_core_domain::models::{Transaction, TransactionType, Wallet};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{Locale, Translator, terms};
use uuid::Uuid;

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
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
            amount_input: "5000".to_string(),
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
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let income_label = Translator::t(locale, terms::INCOME, None);
        let expense_label = Translator::t(locale, terms::EXPENSE, None);
        let handover_label = Translator::t(locale, terms::HANDOVER, None);

        let selected_source = wallets
            .iter()
            .find(|w| Some(w.id) == form.source_wallet_id)
            .map(|w| format!("{} ({} RWF)", w.name, w.balance))
            .unwrap_or_else(|| umuryango_i18n::t(locale, "wallets.select_wallet_placeholder"));

        div()
            .id(id)
            .w(px(500.0))
            .max_h(px(640.0))
            .overflow_y_scroll()
            .p(px(24.0))
            .rounded(px(14.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            // Header with vector icon & Close button
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .mb(px(16.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(app_icon(
                                AppIcon::Add,
                                IconSize::Md,
                                IconTone::AccentPrimary,
                                theme,
                            ))
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(umuryango_i18n::t(
                                        locale,
                                        "transactions.log_new_transaction",
                                    )),
                            ),
                    )
                    .child(
                        div()
                            .id("quick-entry-header-close")
                            .p(px(4.0))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close_quick_entry(cx);
                            }))
                            .child(app_icon(
                                AppIcon::Close,
                                IconSize::Sm,
                                IconTone::Muted,
                                theme,
                            )),
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
                            .id("quick-entry-tab-expense")
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(6.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
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
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.set_quick_entry_mode(QuickEntryMode::Expense, cx);
                            }))
                            .child(app_icon(
                                AppIcon::Expense,
                                IconSize::Xs,
                                if form.mode == QuickEntryMode::Expense {
                                    IconTone::OnPrimaryButton
                                } else {
                                    IconTone::Expense
                                },
                                theme,
                            ))
                            .child(expense_label),
                    )
                    // Income tab
                    .child(
                        div()
                            .id("quick-entry-tab-income")
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(6.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
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
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.set_quick_entry_mode(QuickEntryMode::Income, cx);
                            }))
                            .child(app_icon(
                                AppIcon::Income,
                                IconSize::Xs,
                                if form.mode == QuickEntryMode::Income {
                                    IconTone::OnPrimaryButton
                                } else {
                                    IconTone::Income
                                },
                                theme,
                            ))
                            .child(income_label),
                    )
                    // Handover tab
                    .child(
                        div()
                            .id("quick-entry-tab-handover")
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(6.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
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
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.set_quick_entry_mode(QuickEntryMode::Handover, cx);
                            }))
                            .child(app_icon(
                                AppIcon::Handover,
                                IconSize::Xs,
                                if form.mode == QuickEntryMode::Handover {
                                    IconTone::OnPrimaryButton
                                } else {
                                    IconTone::AccentSecondary
                                },
                                theme,
                            ))
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
                                    .p(px(8.0))
                                    .rounded(px(8.0))
                                    .bg(theme.bg_canvas)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_sm()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .text_color(theme.text_primary)
                                    .child(selected_source),
                            )
                            // Wallets quick picker list
                            .child(div().flex().flex_col().gap(px(4.0)).children(
                                wallets.iter().map(|w| {
                                    let is_sel = Some(w.id) == form.source_wallet_id;
                                    let wid = w.id;
                                    div()
                                        .id(format!("wallet-pick-{}", w.id))
                                        .p(px(6.0))
                                        .rounded(px(6.0))
                                        .bg(if is_sel {
                                            theme.accent_primary.opacity(0.12)
                                        } else {
                                            theme.bg_canvas
                                        })
                                        .border_1()
                                        .border_color(if is_sel {
                                            theme.accent_primary
                                        } else {
                                            theme.border_subtle
                                        })
                                        .cursor_pointer()
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.quick_entry_form.source_wallet_id = Some(wid);
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .justify_between()
                                                .items_center()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                        .text_color(theme.text_primary)
                                                        .child(w.name.clone()),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(gpui_kit::FontWeight::BOLD)
                                                        .text_color(theme.income_color)
                                                        .child(format!("{} RWF", w.balance)),
                                                ),
                                        )
                                }),
                            )),
                    )
                    // Amount input & quick presets
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
                                        format!("{} RWF", form.amount_input)
                                    }),
                            )
                            // Amount quick chips
                            .child(div().flex().flex_row().gap(px(6.0)).children(
                                ["1000", "5000", "10000", "25000", "50000"].into_iter().map(
                                    |amt| {
                                        let is_sel = form.amount_input == amt;
                                        let amt_val = amt.to_string();
                                        div()
                                            .id(format!("amt-preset-{}", amt))
                                            .px(px(8.0))
                                            .py(px(4.0))
                                            .rounded(px(4.0))
                                            .bg(if is_sel {
                                                theme.accent_primary
                                            } else {
                                                theme.bg_elevated
                                            })
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .text_xs()
                                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                                            .text_color(if is_sel {
                                                theme.bg_canvas
                                            } else {
                                                theme.text_primary
                                            })
                                            .cursor_pointer()
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.quick_entry_form.amount_input =
                                                    amt_val.clone();
                                                cx.notify();
                                            }))
                                            .child(format!("{} RWF", amt))
                                    },
                                ),
                            )),
                    )
                    // Category & quick chips
                    .child(
                        div()
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
                                div().flex().flex_row().gap(px(6.0)).children(
                                    ["Food", "Utilities", "Health", "Education", "Transport"]
                                        .into_iter()
                                        .map(|cat| {
                                            let is_sel = form.category == cat;
                                            let cat_val = cat.to_string();
                                            div()
                                                .id(format!("cat-preset-{}", cat))
                                                .px(px(8.0))
                                                .py(px(4.0))
                                                .rounded(px(4.0))
                                                .bg(if is_sel {
                                                    theme.accent_primary
                                                } else {
                                                    theme.bg_elevated
                                                })
                                                .border_1()
                                                .border_color(theme.border_subtle)
                                                .text_xs()
                                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                .text_color(if is_sel {
                                                    theme.bg_canvas
                                                } else {
                                                    theme.text_primary
                                                })
                                                .cursor_pointer()
                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                    this.quick_entry_form.category =
                                                        cat_val.clone();
                                                    cx.notify();
                                                }))
                                                .child(cat)
                                        }),
                                ),
                            ),
                    ),
            )
            // Action buttons row: Confirm + Cancel
            .child(
                div()
                    .mt(px(20.0))
                    .flex()
                    .flex_row()
                    .gap(px(12.0))
                    .child(
                        div()
                            .id("quick-entry-cancel-btn")
                            .flex_1()
                            .py(px(10.0))
                            .rounded(px(8.0))
                            .bg(theme.bg_elevated)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.text_primary)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close_quick_entry(cx);
                            }))
                            .child(umuryango_i18n::t(locale, "common.cancel")),
                    )
                    .child(
                        div()
                            .id("quick-entry-confirm-btn")
                            .flex_1()
                            .py(px(10.0))
                            .rounded(px(8.0))
                            .bg(theme.accent_primary)
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(8.0))
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.submit_quick_entry(cx);
                            }))
                            .child(app_icon(
                                AppIcon::Check,
                                IconSize::Sm,
                                IconTone::OnPrimaryButton,
                                theme,
                            ))
                            .child(umuryango_i18n::t(locale, "common.confirm")),
                    ),
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
            scope: *scope,
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

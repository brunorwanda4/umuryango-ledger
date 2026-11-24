use std::str::FromStr;

use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use rust_decimal::Decimal;
use umuryango_core_domain::models::{Transaction, TransactionType, Wallet};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{Locale, Translator, terms};
use uuid::Uuid;

use crate::RootView;
use crate::common::input::text_field;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::components::quick_entry_inputs::QuickEntryInputs;
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
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        id: impl Into<ElementId>,
        form: &QuickEntryForm,
        inputs: &QuickEntryInputs,
        wallets: &[Wallet],
        _is_family: bool,
        is_busy: bool,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let income_label = Translator::t(locale, terms::INCOME, None);
        let expense_label = Translator::t(locale, terms::EXPENSE, None);
        let handover_label = Translator::t(locale, terms::HANDOVER, None);

        div()
            .id(id)
            .w(px(520.0))
            .max_h(px(680.0))
            .overflow_y_scroll()
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
                    .child(Self::render_mode_tab(
                        "quick-entry-tab-expense",
                        QuickEntryMode::Expense,
                        form.mode == QuickEntryMode::Expense,
                        AppIcon::Expense,
                        IconTone::Expense,
                        theme.expense_color,
                        expense_label,
                        theme,
                        cx,
                    ))
                    .child(Self::render_mode_tab(
                        "quick-entry-tab-income",
                        QuickEntryMode::Income,
                        form.mode == QuickEntryMode::Income,
                        AppIcon::Income,
                        IconTone::Income,
                        theme.income_color,
                        income_label,
                        theme,
                        cx,
                    ))
                    .child(Self::render_mode_tab(
                        "quick-entry-tab-handover",
                        QuickEntryMode::Handover,
                        form.mode == QuickEntryMode::Handover,
                        AppIcon::Handover,
                        IconTone::AccentSecondary,
                        theme.accent_primary,
                        handover_label,
                        theme,
                        cx,
                    )),
            )
            // Form body
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    // Source wallet selector
                    .child(Self::render_wallet_picker(
                        "source",
                        if form.mode == QuickEntryMode::Handover {
                            umuryango_i18n::t(locale, "transactions.source_wallet")
                        } else {
                            umuryango_i18n::t(locale, "wallets.select_wallet")
                        },
                        form.source_wallet_id,
                        wallets,
                        locale,
                        theme,
                        cx,
                        false,
                    ))
                    // Destination wallet selector (Handover only)
                    .children(if form.mode == QuickEntryMode::Handover {
                        Some(Self::render_wallet_picker(
                            "dest",
                            umuryango_i18n::t(locale, "transactions.destination_wallet"),
                            form.destination_wallet_id,
                            wallets,
                            locale,
                            theme,
                            cx,
                            true,
                        ))
                    } else {
                        None
                    })
                    // Editable Amount input & quick chips
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(text_field(
                                "quick-entry-amount-input",
                                format!(
                                    "{} (RWF)",
                                    umuryango_i18n::t(locale, "transactions.amount")
                                ),
                                &inputs.amount,
                                theme,
                            ))
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
                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                this.set_quick_entry_amount_preset(
                                                    amt_val.clone(),
                                                    window,
                                                    cx,
                                                );
                                            }))
                                            .child(format!("{} RWF", amt))
                                    },
                                ),
                            )),
                    )
                    // Category quick chips
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .child(umuryango_i18n::t(locale, "transactions.category")),
                            )
                            .child(
                                div().flex().flex_wrap().gap(px(6.0)).children(
                                    [
                                        "Food",
                                        "Utilities",
                                        "Health",
                                        "Education",
                                        "Transport",
                                        "Salary",
                                        "Other",
                                    ]
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
                                                this.quick_entry_form.category = cat_val.clone();
                                                cx.notify();
                                            }))
                                            .child(cat)
                                    }),
                                ),
                            ),
                    )
                    // Editable Description & Paid To
                    .child(text_field(
                        "quick-entry-description-input",
                        format!(
                            "{} ({})",
                            umuryango_i18n::t(locale, "transactions.description"),
                            umuryango_i18n::t(locale, "common.optional")
                        ),
                        &inputs.description,
                        theme,
                    ))
                    .children(if form.mode == QuickEntryMode::Expense {
                        Some(text_field(
                            "quick-entry-paid-to-input",
                            format!(
                                "{} ({})",
                                umuryango_i18n::t(locale, "transactions.paid_to"),
                                umuryango_i18n::t(locale, "common.optional")
                            ),
                            &inputs.paid_to,
                            theme,
                        ))
                    } else {
                        None
                    }),
            )
            // Action buttons row
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
                            .bg(if is_busy {
                                theme.accent_primary.opacity(0.6)
                            } else {
                                theme.accent_primary
                            })
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
                            .child(if is_busy {
                                umuryango_i18n::t(locale, "common.loading")
                            } else {
                                umuryango_i18n::t(locale, "common.confirm")
                            }),
                    ),
            )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_mode_tab(
        id: &'static str,
        mode: QuickEntryMode,
        is_active: bool,
        icon: AppIcon,
        inactive_tone: IconTone,
        active_bg: gpui_kit::Hsla,
        label: String,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        div()
            .id(id)
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .gap(px(6.0))
            .py(px(6.0))
            .rounded(px(6.0))
            .text_xs()
            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
            .bg(if is_active {
                active_bg
            } else {
                theme.bg_elevated
            })
            .text_color(if is_active {
                theme.bg_canvas
            } else {
                theme.text_primary
            })
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| {
                this.set_quick_entry_mode(mode, cx);
            }))
            .child(app_icon(
                icon,
                IconSize::Xs,
                if is_active {
                    IconTone::OnPrimaryButton
                } else {
                    inactive_tone
                },
                theme,
            ))
            .child(label)
    }

    #[allow(clippy::too_many_arguments)]
    fn render_wallet_picker(
        prefix: &'static str,
        label: String,
        selected_id: Option<Uuid>,
        wallets: &[Wallet],
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
        is_destination: bool,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                    .text_color(theme.text_muted)
                    .child(label),
            )
            .child(if wallets.is_empty() {
                div()
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .bg(theme.bg_canvas)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(umuryango_i18n::t(locale, "wallets.no_wallets"))
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(wallets.iter().map(|w| {
                        let is_sel = Some(w.id) == selected_id;
                        let wid = w.id;
                        div()
                            .id(format!("wallet-pick-{}-{}", prefix, w.id))
                            .p(px(7.0))
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
                                if is_destination {
                                    this.quick_entry_form.destination_wallet_id = Some(wid);
                                } else {
                                    this.quick_entry_form.source_wallet_id = Some(wid);
                                }
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
                                            .child(format!("{} {}", w.balance, w.currency)),
                                    ),
                            )
                    }))
                    .into_any_element()
            })
    }

    /// Build domain transaction from form state with localized validation errors.
    pub fn build_transaction(
        form: &QuickEntryForm,
        scope: &LedgerScope,
        actor_id: Uuid,
        locale: Locale,
    ) -> Result<Transaction, String> {
        let cleaned = form.amount_input.trim().replace([',', ' '], "");
        let amount = Decimal::from_str(&cleaned)
            .map_err(|_| umuryango_i18n::t(locale, "errors.invalid_amount"))?;
        if amount <= Decimal::ZERO {
            return Err(umuryango_i18n::t(locale, "errors.invalid_amount"));
        }

        let source_id = form
            .source_wallet_id
            .ok_or_else(|| umuryango_i18n::t(locale, "wallets.select_wallet_placeholder"))?;

        let tx_type = match form.mode {
            QuickEntryMode::Income => TransactionType::Income,
            QuickEntryMode::Expense => TransactionType::Expense,
            QuickEntryMode::Handover => TransactionType::Handover,
        };

        let destination_wallet_id = if tx_type == TransactionType::Handover {
            let dest = form.destination_wallet_id.ok_or_else(|| {
                umuryango_i18n::t(locale, "transactions.select_destination_wallet")
            })?;
            if dest == source_id {
                return Err(umuryango_i18n::t(
                    locale,
                    "transactions.select_destination_wallet",
                ));
            }
            Some(dest)
        } else {
            None
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
            destination_wallet_id,
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

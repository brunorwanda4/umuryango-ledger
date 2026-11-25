use std::str::FromStr;

use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use rust_decimal::Decimal;
use umuryango_core_domain::models::{Transaction, TransactionType, Wallet};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{Locale, Translator, terms};
use uuid::Uuid;

use crate::RootView;
use crate::common::{
    Button, ButtonGroup, ButtonSize, ButtonVariant, DialogContent, DialogFooter, DialogHeader,
    DialogOverlay, DialogTitle, Empty, EmptyDescription, EmptyHeader, Field, Item, ItemActions,
    ItemContent, ItemSize, Label, text_field,
};
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
        let view_for_close = cx.entity().clone();

        DialogOverlay::new("quick-entry-overlay")
            .on_click(cx.listener(|this, _, _, cx| {
                this.close_quick_entry(cx);
            }))
            .child(
                DialogContent::new(id)
                    .show_close_button(true)
                    .on_close(move |_window, cx| {
                        view_for_close.update(cx, |this, cx| {
                            this.close_quick_entry(cx);
                        });
                    })
                    .w(px(520.0))
                    .max_h(px(680.0))
                    .gap(px(16.0))
                    .theme(theme)
                    // Header
                    .child(
                        DialogHeader::new().pr(px(40.0)).child(
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
                                    DialogTitle::new(umuryango_i18n::t(
                                        locale,
                                        "transactions.log_new_transaction",
                                    ))
                                    .theme(theme),
                                ),
                        ),
                    )
                    // Mode selector tabs
                    .child(
                        ButtonGroup::horizontal()
                            .w_full()
                            .child(Self::render_mode_tab(
                                "quick-entry-tab-expense",
                                QuickEntryMode::Expense,
                                form.mode == QuickEntryMode::Expense,
                                AppIcon::Expense,
                                expense_label,
                                theme,
                                cx,
                            ))
                            .child(Self::render_mode_tab(
                                "quick-entry-tab-income",
                                QuickEntryMode::Income,
                                form.mode == QuickEntryMode::Income,
                                AppIcon::Income,
                                income_label,
                                theme,
                                cx,
                            ))
                            .child(Self::render_mode_tab(
                                "quick-entry-tab-handover",
                                QuickEntryMode::Handover,
                                form.mode == QuickEntryMode::Handover,
                                AppIcon::Handover,
                                handover_label,
                                theme,
                                cx,
                            )),
                    )
                    // Scrollable form body
                    .child(
                        div()
                            .id("quick-entry-body-scroll")
                            .w_full()
                            .flex_1()
                            .min_h(px(0.0))
                            .overflow_y_scroll()
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
                                    .gap(px(8.0))
                                    .child(text_field(
                                        "quick-entry-amount-input",
                                        format!(
                                            "{} (RWF)",
                                            umuryango_i18n::t(locale, "transactions.amount")
                                        ),
                                        &inputs.amount,
                                        theme,
                                    ))
                                    .child(
                                        div().flex().flex_row().flex_wrap().gap(px(6.0)).children(
                                            ["1000", "5000", "10000", "25000", "50000"]
                                                .into_iter()
                                                .map(|amt| {
                                                    let is_sel = form.amount_input == amt;
                                                    let amt_val = amt.to_string();
                                                    Button::new(format!("amt-preset-{}", amt))
                                                        .variant(if is_sel {
                                                            ButtonVariant::Default
                                                        } else {
                                                            ButtonVariant::Outline
                                                        })
                                                        .size(ButtonSize::Xs)
                                                        .label(format!("{} RWF", amt))
                                                        .theme(theme)
                                                        .on_click(cx.listener(
                                                            move |this, _, window, cx| {
                                                                this.set_quick_entry_amount_preset(
                                                                    amt_val.clone(),
                                                                    window,
                                                                    cx,
                                                                );
                                                            },
                                                        ))
                                                }),
                                        ),
                                    ),
                            )
                            // Category quick chips
                            .child(
                                Field::new()
                                    .gap(px(6.0))
                                    .theme(theme)
                                    .child(
                                        Label::new(umuryango_i18n::t(
                                            locale,
                                            "transactions.category",
                                        ))
                                        .theme(theme),
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
                                            .map(
                                                |cat| {
                                                    let is_sel = form.category == cat;
                                                    let cat_val = cat.to_string();
                                                    Button::new(format!("cat-preset-{}", cat))
                                                        .variant(if is_sel {
                                                            ButtonVariant::Default
                                                        } else {
                                                            ButtonVariant::Outline
                                                        })
                                                        .size(ButtonSize::Xs)
                                                        .label(cat)
                                                        .theme(theme)
                                                        .on_click(cx.listener(
                                                            move |this, _, _, cx| {
                                                                this.quick_entry_form.category =
                                                                    cat_val.clone();
                                                                cx.notify();
                                                            },
                                                        ))
                                                },
                                            ),
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
                        DialogFooter::new()
                            .mt(px(4.0))
                            .theme(theme)
                            .child(
                                Button::new("quick-entry-cancel-btn")
                                    .outline()
                                    .label(umuryango_i18n::t(locale, "common.cancel"))
                                    .flex_1()
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.close_quick_entry(cx);
                                    })),
                            )
                            .child(
                                Button::new("quick-entry-confirm-btn")
                                    .icon_start(AppIcon::Check)
                                    .loading(is_busy)
                                    .label(if is_busy {
                                        umuryango_i18n::t(locale, "common.loading")
                                    } else {
                                        umuryango_i18n::t(locale, "common.confirm")
                                    })
                                    .flex_1()
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.submit_quick_entry(cx);
                                    })),
                            ),
                    ),
            )
    }

    fn render_mode_tab(
        id: &'static str,
        mode: QuickEntryMode,
        is_active: bool,
        icon: AppIcon,
        label: String,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        Button::new(id)
            .variant(if is_active {
                ButtonVariant::Default
            } else {
                ButtonVariant::Outline
            })
            .size(ButtonSize::Sm)
            .icon_start(icon)
            .label(label)
            .flex_1()
            .theme(theme)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.set_quick_entry_mode(mode, cx);
            }))
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
        Field::new()
            .gap(px(6.0))
            .theme(theme)
            .child(Label::new(label).theme(theme))
            .child(if wallets.is_empty() {
                Empty::new()
                    .bordered(true)
                    .p(px(14.0))
                    .theme(theme)
                    .child(
                        EmptyHeader::new().child(
                            EmptyDescription::new(umuryango_i18n::t(locale, "wallets.no_wallets"))
                                .theme(theme),
                        ),
                    )
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .children(wallets.iter().map(|w| {
                        let is_sel = Some(w.id) == selected_id;
                        let wid = w.id;
                        Item::new()
                            .id(format!("wallet-pick-{}-{}", prefix, w.id))
                            .outline()
                            .size(ItemSize::Xs)
                            .bg(if is_sel {
                                theme.accent_primary.opacity(0.12)
                            } else {
                                theme.bg_canvas
                            })
                            .border_color(if is_sel {
                                theme.accent_primary
                            } else {
                                theme.border_subtle
                            })
                            .theme(theme)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if is_destination {
                                    this.quick_entry_form.destination_wallet_id = Some(wid);
                                } else {
                                    this.quick_entry_form.source_wallet_id = Some(wid);
                                }
                                cx.notify();
                            }))
                            .child(
                                ItemContent::new().child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_primary)
                                        .child(w.name.clone()),
                                ),
                            )
                            .child(
                                ItemActions::new().child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
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

use std::str::FromStr;

use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use rust_decimal::Decimal;
use umuryango_core_domain::models::{CategoryKind, Transaction, TransactionType, Wallet};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{Locale, Translator, terms};
use uuid::Uuid;

use crate::RootView;
use crate::common::{
    Button, ButtonGroup, ButtonSize, ButtonVariant, DialogContent, DialogOverlay, DialogPortal,
    NativeSelect, NativeSelectOption, input_primitive, to_preset_uppercase,
};
use crate::components::icon::AppIcon;
use crate::components::money::MoneyCategoryInputs;
use crate::components::quick_entry_inputs::QuickEntryInputs;
use crate::state::{ActiveModal, AppState};
use crate::theme::SemanticTheme;

pub const DEFAULT_INCOME_CATEGORIES: &[&str] = &[
    "Monthly Contribution",
    "Rental Income",
    "Business Support",
    "Salary & Other",
];

pub const DEFAULT_EXPENSE_CATEGORIES: &[&str] = &[
    "Food & Groceries",
    "School Fees",
    "House Maintenance",
    "Utilities & Bills",
    "Medical & Care",
    "Transport & Other",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickEntryMode {
    Income,
    Expense,
    Handover,
}

#[derive(Debug, Clone)]
pub struct QuickEntryForm {
    pub mode: QuickEntryMode,
    pub locked_mode: bool,
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
            locked_mode: false,
            source_wallet_id: None,
            destination_wallet_id: None,
            amount_input: String::new(),
            category: DEFAULT_EXPENSE_CATEGORIES[0].to_string(),
            description: String::new(),
            authorized_by: None,
            paid_to: String::new(),
            receipt_url: String::new(),
        }
    }
}

pub struct QuickEntryDialog;

impl QuickEntryDialog {
    pub fn default_categories(kind: CategoryKind) -> &'static [&'static str] {
        match kind {
            CategoryKind::Income => DEFAULT_INCOME_CATEGORIES,
            CategoryKind::Expense => DEFAULT_EXPENSE_CATEGORIES,
        }
    }

    pub fn categories_for_kind(state: &AppState, kind: CategoryKind) -> Vec<String> {
        let mut list: Vec<String> = Self::default_categories(kind)
            .iter()
            .map(|s| (*s).to_string())
            .collect();

        for saved in &state.categories {
            if saved.kind == kind
                && !list
                    .iter()
                    .any(|existing| existing.eq_ignore_ascii_case(&saved.name))
            {
                list.push(saved.name.clone());
            }
        }
        list
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        form: &QuickEntryForm,
        inputs: &QuickEntryInputs,
        _category_inputs: &MoneyCategoryInputs,
        wallets: &[Wallet],
        is_family: bool,
        is_busy: bool,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let income_label = Translator::t(locale, terms::INCOME, None);
        let expense_label = Translator::t(locale, terms::EXPENSE, None);
        let handover_label = Translator::t(locale, terms::HANDOVER, None);
        let view_for_close = cx.entity().clone();

        let header_title = match form.mode {
            QuickEntryMode::Expense => {
                if is_family {
                    umuryango_i18n::t(locale, "money_page.modal_log_family_expense")
                } else {
                    umuryango_i18n::t(locale, "money_page.modal_log_personal_expense")
                }
            }
            QuickEntryMode::Income => {
                if is_family {
                    umuryango_i18n::t(locale, "money_page.modal_add_family_income")
                } else {
                    umuryango_i18n::t(locale, "money_page.modal_add_personal_income")
                }
            }
            QuickEntryMode::Handover => to_preset_uppercase(&handover_label),
        };

        let save_label = if is_busy {
            umuryango_i18n::t(locale, "common.loading")
        } else {
            match form.mode {
                QuickEntryMode::Expense => {
                    umuryango_i18n::t(locale, "money_page.modal_save_expense")
                }
                QuickEntryMode::Income => umuryango_i18n::t(locale, "money_page.modal_save_income"),
                QuickEntryMode::Handover => umuryango_i18n::t(locale, "common.confirm"),
            }
        };

        let body = if form.mode == QuickEntryMode::Handover {
            Self::render_handover_body(form, inputs, wallets, locale, theme, cx).into_any_element()
        } else {
            Self::render_money_entry_body(
                state, form, inputs, wallets, is_family, locale, theme, cx,
            )
            .into_any_element()
        };

        DialogPortal::new().child(
            DialogOverlay::new("quick-entry-overlay")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.close_quick_entry(cx);
                }))
                .child(
                    DialogContent::new(id)
                        .show_close_button(false)
                        .on_close(move |_window, cx| {
                            view_for_close.update(cx, |this, cx| {
                                this.close_quick_entry(cx);
                            });
                        })
                        .p(px(0.0))
                        .gap(px(0.0))
                        .w_full()
                        .max_w(px(520.0))
                        .bg(theme.bg_surface)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .theme(theme)
                        // Top red primary accent stripe
                        .child(div().w_full().h(px(3.0)).bg(theme.primary))
                        // Modal header with full-width bottom divider & square X close button
                        .child(
                            div()
                                .w_full()
                                .px(px(24.0))
                                .py(px(18.0))
                                .border_b_1()
                                .border_color(theme.border_subtle)
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .gap(px(12.0))
                                .child(
                                    div()
                                        .text_base()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(header_title),
                                )
                                .child(
                                    Button::new("quick-entry-close-x")
                                        .outline()
                                        .size(ButtonSize::IconSm)
                                        .icon(AppIcon::Close)
                                        .theme(theme)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.close_quick_entry(cx);
                                        })),
                                ),
                        )
                        // Optional mode switcher (only when opened in generic unlocked mode)
                        .children((!form.locked_mode).then(|| {
                            div().w_full().px(px(24.0)).pt(px(16.0)).child(
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
                                    .children(is_family.then(|| {
                                        Self::render_mode_tab(
                                            "quick-entry-tab-handover",
                                            QuickEntryMode::Handover,
                                            form.mode == QuickEntryMode::Handover,
                                            AppIcon::Handover,
                                            handover_label,
                                            theme,
                                            cx,
                                        )
                                    })),
                            )
                        }))
                        // Form body
                        .child(body)
                        // Modal footer
                        .child(
                            div()
                                .w_full()
                                .px(px(24.0))
                                .py(px(16.0))
                                .border_t_1()
                                .border_color(theme.border_subtle)
                                .flex()
                                .flex_row()
                                .justify_end()
                                .items_center()
                                .gap(px(10.0))
                                .child(
                                    Button::new("quick-entry-cancel-btn")
                                        .outline()
                                        .label(umuryango_i18n::t(locale, "common.cancel"))
                                        .theme(theme)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.close_quick_entry(cx);
                                        })),
                                )
                                .child(
                                    Button::new("quick-entry-confirm-btn")
                                        .loading(is_busy)
                                        .label(save_label)
                                        .theme(theme)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.submit_quick_entry(cx);
                                        })),
                                ),
                        ),
                ),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_money_entry_body(
        state: &AppState,
        form: &QuickEntryForm,
        inputs: &QuickEntryInputs,
        wallets: &[Wallet],
        is_family: bool,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let active_kind = match form.mode {
            QuickEntryMode::Income => CategoryKind::Income,
            QuickEntryMode::Expense | QuickEntryMode::Handover => CategoryKind::Expense,
        };
        let mut category_options = Self::categories_for_kind(state, active_kind);
        if !form.category.trim().is_empty()
            && !category_options
                .iter()
                .any(|c| c.eq_ignore_ascii_case(&form.category))
        {
            category_options.insert(0, form.category.clone());
        }
        let selected_category = category_options
            .iter()
            .find(|c| c.eq_ignore_ascii_case(&form.category))
            .cloned()
            .or_else(|| category_options.first().cloned())
            .unwrap_or_default();

        let view_for_cat = cx.entity().clone();
        let cat_select = NativeSelect::new("quick-entry-category-select")
            .boxed(true)
            .value(selected_category)
            .options(
                category_options
                    .into_iter()
                    .map(|cat| NativeSelectOption::new(cat.clone(), cat)),
            )
            .theme(theme)
            .on_change(move |val, _window, cx| {
                let next_cat = val.to_string();
                view_for_cat.update(cx, |this, cx| {
                    this.quick_entry_form.category = next_cat;
                    cx.notify();
                });
            });

        let amount_label = format!(
            "{} (RWF)",
            to_preset_uppercase(&umuryango_i18n::t(locale, "transactions.amount"))
        );
        let category_label = umuryango_i18n::t(locale, "money_page.col_category");
        let item_label = umuryango_i18n::t(locale, "money_page.modal_item_label");
        let wallet_label = umuryango_i18n::t(locale, "money_page.modal_wallet_label");

        div()
            .w_full()
            .px(px(24.0))
            .py(px(20.0))
            .flex()
            .flex_col()
            .gap(px(18.0))
            // Row 1: AMOUNT (RWF) + CATEGORY
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(16.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(190.0))
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(Self::render_field_label(amount_label, theme))
                            .child(Self::render_boxed_input(
                                "quick-entry-amount-input",
                                &inputs.amount,
                                theme,
                            )),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(190.0))
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(Self::render_field_label(category_label, theme))
                            .child(cat_select),
                    ),
            )
            // Row 2: ITEM / DESCRIPTION
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(Self::render_field_label(item_label, theme))
                    .child(Self::render_boxed_input(
                        "quick-entry-description-input",
                        &inputs.description,
                        theme,
                    )),
            )
            // Row 3: [AUTHORIZED BY / ADDED BY in Family scope] + WALLET
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(16.0))
                    .children(is_family.then(|| {
                        let actor_label = if form.mode == QuickEntryMode::Expense {
                            umuryango_i18n::t(locale, "money_page.modal_authorized_by_label")
                        } else {
                            umuryango_i18n::t(locale, "money_page.modal_added_by_label")
                        };
                        Self::render_actor_select_col(state, form, actor_label, locale, theme, cx)
                    }))
                    .child(Self::render_wallet_select_col(
                        "quick-entry-wallet-select",
                        wallet_label,
                        form.source_wallet_id,
                        wallets,
                        false,
                        locale,
                        theme,
                        cx,
                    )),
            )
    }

    fn render_handover_body(
        form: &QuickEntryForm,
        inputs: &QuickEntryInputs,
        wallets: &[Wallet],
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let src_label =
            to_preset_uppercase(&umuryango_i18n::t(locale, "transactions.source_wallet"));
        let dst_label = to_preset_uppercase(&umuryango_i18n::t(
            locale,
            "transactions.destination_wallet",
        ));
        let amount_label = format!(
            "{} (RWF)",
            to_preset_uppercase(&umuryango_i18n::t(locale, "transactions.amount"))
        );
        let desc_label = umuryango_i18n::t(locale, "money_page.modal_item_label");

        div()
            .w_full()
            .px(px(24.0))
            .py(px(20.0))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(16.0))
                    .child(Self::render_wallet_select_col(
                        "quick-entry-handover-src",
                        src_label,
                        form.source_wallet_id,
                        wallets,
                        false,
                        locale,
                        theme,
                        cx,
                    ))
                    .child(Self::render_wallet_select_col(
                        "quick-entry-handover-dst",
                        dst_label,
                        form.destination_wallet_id,
                        wallets,
                        true,
                        locale,
                        theme,
                        cx,
                    )),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(Self::render_field_label(amount_label, theme))
                    .child(Self::render_boxed_input(
                        "quick-entry-handover-amount",
                        &inputs.amount,
                        theme,
                    )),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(Self::render_field_label(desc_label, theme))
                    .child(Self::render_boxed_input(
                        "quick-entry-handover-desc",
                        &inputs.description,
                        theme,
                    )),
            )
    }

    fn render_actor_select_col(
        state: &AppState,
        form: &QuickEntryForm,
        label: String,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let mut actors: Vec<(Uuid, String)> = state
            .family_members
            .iter()
            .map(|m| {
                let name = m
                    .display_name
                    .clone()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| m.full_name.clone());
                (m.user_id, name)
            })
            .collect();

        if !state.user_id.is_nil() && !actors.iter().any(|(id, _)| *id == state.user_id) {
            let fallback = if state.user_name.trim().is_empty() {
                umuryango_i18n::t(locale, "money_page.self_actor")
            } else {
                state.user_name.clone()
            };
            actors.insert(0, (state.user_id, fallback));
        }

        let selected_actor = form
            .authorized_by
            .or_else(|| actors.first().map(|(id, _)| *id))
            .unwrap_or(state.user_id);

        let view_for_actor = cx.entity().clone();
        let select = NativeSelect::new("quick-entry-actor-select")
            .boxed(true)
            .value(selected_actor.to_string())
            .options(
                actors
                    .into_iter()
                    .map(|(uid, name)| NativeSelectOption::new(uid.to_string(), name)),
            )
            .theme(theme)
            .on_change(move |val, _window, cx| {
                if let Ok(parsed) = Uuid::parse_str(val.as_ref()) {
                    view_for_actor.update(cx, |this, cx| {
                        this.quick_entry_form.authorized_by = Some(parsed);
                        cx.notify();
                    });
                }
            });

        div()
            .flex_1()
            .min_w(px(190.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Self::render_field_label(label, theme))
            .child(select)
    }

    #[allow(clippy::too_many_arguments)]
    fn render_wallet_select_col(
        select_id: &'static str,
        label: String,
        selected_id: Option<Uuid>,
        wallets: &[Wallet],
        is_destination: bool,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let control = if wallets.is_empty() {
            div()
                .h(px(40.0))
                .w_full()
                .px(px(12.0))
                .flex()
                .flex_row()
                .justify_between()
                .items_center()
                .gap(px(8.0))
                .bg(theme.bg_canvas)
                .border_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .truncate()
                        .child(umuryango_i18n::t(locale, "wallets.no_wallets")),
                )
                .child(
                    Button::new(format!("{select_id}-create-wallet"))
                        .outline()
                        .size(ButtonSize::Xs)
                        .icon_start(AppIcon::Add)
                        .label(umuryango_i18n::t(locale, "wallets.create_wallet"))
                        .theme(theme)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.close_quick_entry(cx);
                            this.open_modal(ActiveModal::AddWallet, window, cx);
                        })),
                )
                .into_any_element()
        } else {
            let active_id = selected_id.or_else(|| wallets.first().map(|w| w.id));
            let view_for_wallet = cx.entity().clone();
            let mut select = NativeSelect::new(select_id)
                .boxed(true)
                .placeholder(umuryango_i18n::t(
                    locale,
                    "wallets.select_wallet_placeholder",
                ))
                .options(
                    wallets
                        .iter()
                        .map(|w| NativeSelectOption::new(w.id.to_string(), w.name.clone())),
                )
                .theme(theme)
                .on_change(move |val, _window, cx| {
                    if let Ok(wid) = Uuid::parse_str(val.as_ref()) {
                        view_for_wallet.update(cx, |this, cx| {
                            if is_destination {
                                this.quick_entry_form.destination_wallet_id = Some(wid);
                            } else {
                                this.quick_entry_form.source_wallet_id = Some(wid);
                            }
                            cx.notify();
                        });
                    }
                });
            if let Some(wid) = active_id {
                select = select.value(wid.to_string());
            }
            select.into_any_element()
        };

        div()
            .flex_1()
            .min_w(px(190.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Self::render_field_label(label, theme))
            .child(control)
    }

    fn render_field_label(label: String, theme: &SemanticTheme) -> impl IntoElement {
        div()
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(theme.text_muted)
            .child(label)
    }

    fn render_boxed_input(
        id: &'static str,
        input_state: &gpui_kit::Entity<crate::common::InputState>,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        div()
            .h(px(40.0))
            .w_full()
            .px(px(12.0))
            .flex()
            .items_center()
            .bg(theme.bg_canvas)
            .border_1()
            .border_color(theme.border_subtle)
            .child(input_primitive(id, input_state, theme).borderless(true))
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
            .on_click(cx.listener(move |this, _, window, cx| {
                this.set_quick_entry_mode_with_window(mode, window, cx);
            }))
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

        let category = if tx_type == TransactionType::Handover {
            "Handover".to_string()
        } else {
            form.category.clone()
        };

        let effective_actor = if scope.is_family() {
            form.authorized_by.unwrap_or(actor_id)
        } else {
            actor_id
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
            category,
            description: if form.description.trim().is_empty() {
                None
            } else {
                Some(form.description.trim().to_string())
            },
            authorized_by: effective_actor,
            paid_to,
            tx_date: now,
            created_at: now,
        })
    }
}

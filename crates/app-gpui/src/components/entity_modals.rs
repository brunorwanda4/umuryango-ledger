//! Shared modal overlay for creating/updating wallets, projects, loans,
//! future plans, and family invitations backed by PostgreSQL API endpoints.

use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::{FamilyRole, LoanDirection, WalletChannel};
use umuryango_i18n::{Locale, Translator, terms};

use crate::RootView;
use crate::common::{
    Alert, AlertDescription, Button, ButtonGroup, ButtonSize, ButtonVariant, DialogContent,
    DialogFooter, DialogHeader, DialogOverlay, DialogTitle, Field, Label, text_field,
};
use crate::components::entity_modal_inputs::EntityModalInputs;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{ActiveModal, AppState};
use crate::theme::SemanticTheme;

pub struct EntityModalDialog;

impl EntityModalDialog {
    pub fn render(
        id: impl Into<ElementId>,
        modal: &ActiveModal,
        state: &AppState,
        inputs: &EntityModalInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let locale = state.locale;
        let (title, icon) = match modal {
            ActiveModal::AddWallet => (
                umuryango_i18n::t(locale, "wallets.add_wallet"),
                AppIcon::PersonalWallet,
            ),
            ActiveModal::AddProject => (
                umuryango_i18n::t(locale, "projects.new_project"),
                AppIcon::Projects,
            ),
            ActiveModal::AddLoan => (
                umuryango_i18n::t(locale, "loans.record_loan_btn"),
                AppIcon::Loans,
            ),
            ActiveModal::RepayLoan { counterparty, .. } => (
                format!(
                    "{}: {}",
                    umuryango_i18n::t(locale, "loans.repay_btn"),
                    counterparty
                ),
                AppIcon::Loans,
            ),
            ActiveModal::AddPlan => (
                umuryango_i18n::t(locale, "future_plans.new_plan"),
                AppIcon::FuturePlans,
            ),
            ActiveModal::AllocatePlan { title, .. } => (
                format!(
                    "{}: {}",
                    umuryango_i18n::t(locale, "future_plans.allocate_funds"),
                    title
                ),
                AppIcon::FuturePlans,
            ),
            ActiveModal::InviteMember => (
                umuryango_i18n::t(locale, "members.invite_member"),
                AppIcon::Members,
            ),
        };

        let view_for_close = cx.entity().clone();

        DialogOverlay::new("entity-modal-overlay")
            .on_click(cx.listener(|this, _, _, cx| {
                this.close_modal(cx);
            }))
            .child(
                DialogContent::new(id)
                    .show_close_button(true)
                    .on_close(move |_window, cx| {
                        view_for_close.update(cx, |this, cx| {
                            this.close_modal(cx);
                        });
                    })
                    .w(px(460.0))
                    .max_h(px(620.0))
                    .gap(px(16.0))
                    .theme(theme)
                    // Header
                    .child(
                        DialogHeader::new().pr(px(40.0)).child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(app_icon(icon, IconSize::Md, IconTone::AccentPrimary, theme))
                                .child(DialogTitle::new(title).text_base().theme(theme)),
                        ),
                    )
                    // Scrollable Body
                    .child(
                        div()
                            .id("entity-modal-body-scroll")
                            .w_full()
                            .flex_1()
                            .min_h(px(0.0))
                            .overflow_y_scroll()
                            .child(Self::render_body(modal, state, inputs, locale, theme, cx)),
                    )
                    // Footer buttons
                    .child(
                        DialogFooter::new()
                            .mt(px(4.0))
                            .theme(theme)
                            .child(
                                Button::new("entity-modal-cancel-btn")
                                    .outline()
                                    .label(umuryango_i18n::t(locale, "common.cancel"))
                                    .flex_1()
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.close_modal(cx);
                                    })),
                            )
                            .child(
                                Button::new("entity-modal-submit-btn")
                                    .icon_start(AppIcon::Check)
                                    .loading(state.is_busy)
                                    .label(if state.is_busy {
                                        umuryango_i18n::t(locale, "common.loading")
                                    } else {
                                        umuryango_i18n::t(locale, "common.save")
                                    })
                                    .flex_1()
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.submit_entity_modal(cx);
                                    })),
                            ),
                    ),
            )
    }

    fn render_body(
        modal: &ActiveModal,
        state: &AppState,
        inputs: &EntityModalInputs,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let mut container = div().w_full().flex().flex_col().gap(px(12.0));

        match modal {
            ActiveModal::AddWallet => {
                container = container
                    .child(text_field(
                        "modal-wallet-name",
                        umuryango_i18n::t(locale, "wallets.wallet_name"),
                        &inputs.primary,
                        theme,
                    ))
                    .child(Self::render_wallet_channel_picker(
                        state.entity_form.wallet_channel,
                        locale,
                        theme,
                        cx,
                    ))
                    .child(text_field(
                        "modal-wallet-account",
                        format!(
                            "{} ({})",
                            umuryango_i18n::t(locale, "wallets.account_number"),
                            umuryango_i18n::t(locale, "common.optional")
                        ),
                        &inputs.secondary,
                        theme,
                    ));
            }
            ActiveModal::AddProject => {
                container = container
                    .child(text_field(
                        "modal-project-name",
                        umuryango_i18n::t(locale, "projects.project_name"),
                        &inputs.primary,
                        theme,
                    ))
                    .child(text_field(
                        "modal-project-target",
                        format!(
                            "{} (RWF)",
                            umuryango_i18n::t(locale, "projects.target_amount")
                        ),
                        &inputs.amount,
                        theme,
                    ))
                    .child(text_field(
                        "modal-project-desc",
                        format!(
                            "{} ({})",
                            umuryango_i18n::t(locale, "projects.project_description"),
                            umuryango_i18n::t(locale, "common.optional")
                        ),
                        &inputs.secondary,
                        theme,
                    ));
            }
            ActiveModal::AddLoan => {
                container = container
                    .child(Self::render_loan_direction_picker(
                        state.entity_form.loan_direction,
                        locale,
                        theme,
                        cx,
                    ))
                    .child(text_field(
                        "modal-loan-counterparty",
                        umuryango_i18n::t(locale, "loans.counterparty"),
                        &inputs.primary,
                        theme,
                    ))
                    .child(text_field(
                        "modal-loan-amount",
                        format!("{} (RWF)", umuryango_i18n::t(locale, "loans.principal")),
                        &inputs.amount,
                        theme,
                    ))
                    .child(text_field(
                        "modal-loan-notes",
                        format!(
                            "{} ({})",
                            umuryango_i18n::t(locale, "common.notes"),
                            umuryango_i18n::t(locale, "common.optional")
                        ),
                        &inputs.secondary,
                        theme,
                    ));
            }
            ActiveModal::RepayLoan { remaining, .. } => {
                container = container
                    .child(
                        Alert::new().icon(AppIcon::Loans).theme(theme).child(
                            AlertDescription::new(format!(
                                "{}: {} RWF",
                                umuryango_i18n::t(locale, "loans.remaining"),
                                remaining
                            ))
                            .theme(theme),
                        ),
                    )
                    .child(text_field(
                        "modal-repay-amount",
                        format!("{} (RWF)", umuryango_i18n::t(locale, "loans.repay_amount")),
                        &inputs.amount,
                        theme,
                    ));
            }
            ActiveModal::AddPlan => {
                container = container
                    .child(text_field(
                        "modal-plan-title",
                        umuryango_i18n::t(locale, "future_plans.plan_title"),
                        &inputs.primary,
                        theme,
                    ))
                    .child(text_field(
                        "modal-plan-cost",
                        format!(
                            "{} (RWF)",
                            umuryango_i18n::t(locale, "future_plans.estimated_cost")
                        ),
                        &inputs.amount,
                        theme,
                    ))
                    .child(text_field(
                        "modal-plan-desc",
                        format!(
                            "{} ({})",
                            umuryango_i18n::t(locale, "projects.project_description"),
                            umuryango_i18n::t(locale, "common.optional")
                        ),
                        &inputs.secondary,
                        theme,
                    ));
            }
            ActiveModal::AllocatePlan { .. } => {
                container = container.child(text_field(
                    "modal-plan-allocate-amount",
                    format!(
                        "{} (RWF)",
                        umuryango_i18n::t(locale, "future_plans.allocate_amount")
                    ),
                    &inputs.amount,
                    theme,
                ));
            }
            ActiveModal::InviteMember => {
                container = container
                    .child(text_field(
                        "modal-invite-phone",
                        Translator::t(locale, terms::PHONE_NUMBER, None),
                        &inputs.primary,
                        theme,
                    ))
                    .child(Self::render_invite_role_picker(
                        state.entity_form.invite_role,
                        locale,
                        theme,
                        cx,
                    ));
            }
        }

        container
    }

    fn render_wallet_channel_picker(
        selected: WalletChannel,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let channels = [
            (
                "momo",
                WalletChannel::MobileMoney,
                umuryango_i18n::t(locale, "wallets.channel_momo"),
            ),
            (
                "bank",
                WalletChannel::Bank,
                umuryango_i18n::t(locale, "wallets.channel_bank"),
            ),
            (
                "cash",
                WalletChannel::CashInHand,
                umuryango_i18n::t(locale, "wallets.channel_cash"),
            ),
        ];

        Field::new()
            .gap(px(6.0))
            .theme(theme)
            .child(Label::new(umuryango_i18n::t(locale, "wallets.channel")).theme(theme))
            .child(
                ButtonGroup::horizontal()
                    .w_full()
                    .children(channels.into_iter().map(|(slug, ch, label)| {
                        let is_sel = selected == ch;
                        Button::new(format!("modal-wallet-ch-{}", slug))
                            .variant(if is_sel {
                                ButtonVariant::Default
                            } else {
                                ButtonVariant::Outline
                            })
                            .size(ButtonSize::Sm)
                            .label(label)
                            .flex_1()
                            .theme(theme)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.state.entity_form.wallet_channel = ch;
                                cx.notify();
                            }))
                    })),
            )
    }

    fn render_loan_direction_picker(
        selected: LoanDirection,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let options = [
            (
                "lent",
                LoanDirection::Lent,
                umuryango_i18n::t(locale, "loans.lent_out_title"),
            ),
            (
                "borrowed",
                LoanDirection::Borrowed,
                umuryango_i18n::t(locale, "loans.borrowed_title"),
            ),
        ];

        Field::new()
            .gap(px(6.0))
            .theme(theme)
            .child(Label::new(umuryango_i18n::t(locale, "loans.direction")).theme(theme))
            .child(
                ButtonGroup::horizontal()
                    .w_full()
                    .children(options.into_iter().map(|(slug, dir, label)| {
                        let is_sel = selected == dir;
                        Button::new(format!("modal-loan-dir-{}", slug))
                            .variant(if is_sel {
                                ButtonVariant::Default
                            } else {
                                ButtonVariant::Outline
                            })
                            .size(ButtonSize::Sm)
                            .label(label)
                            .flex_1()
                            .theme(theme)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.state.entity_form.loan_direction = dir;
                                cx.notify();
                            }))
                    })),
            )
    }

    fn render_invite_role_picker(
        selected: FamilyRole,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let roles = [
            (
                "member",
                FamilyRole::Member,
                umuryango_i18n::t(locale, "members.role_member"),
            ),
            (
                "custodian",
                FamilyRole::Custodian,
                umuryango_i18n::t(locale, "members.role_custodian"),
            ),
            (
                "head",
                FamilyRole::Head,
                umuryango_i18n::t(locale, "members.role_head"),
            ),
        ];

        Field::new()
            .gap(px(6.0))
            .theme(theme)
            .child(
                ButtonGroup::horizontal()
                    .w_full()
                    .children(roles.into_iter().map(|(slug, role, label)| {
                        let is_sel = selected == role;
                        Button::new(format!("modal-invite-role-{}", slug))
                            .variant(if is_sel {
                                ButtonVariant::Default
                            } else {
                                ButtonVariant::Outline
                            })
                            .size(ButtonSize::Sm)
                            .label(label)
                            .flex_1()
                            .theme(theme)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.state.entity_form.invite_role = role;
                                cx.notify();
                            }))
                    })),
            )
    }
}

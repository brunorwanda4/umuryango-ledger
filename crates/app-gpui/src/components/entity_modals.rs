//! Shared modal overlay for creating/updating wallets, projects, loans,
//! future plans, and family invitations backed by PostgreSQL API endpoints.

use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::{FamilyRole, LoanDirection, WalletChannel};
use umuryango_i18n::{Locale, Translator, terms};

use crate::RootView;
use crate::common::input::text_field;
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

        div()
            .id(id)
            .w(px(460.0))
            .max_h(px(620.0))
            .overflow_y_scroll()
            .p(px(24.0))
            .rounded(px(14.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            .flex()
            .flex_col()
            .gap(px(14.0))
            // Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(app_icon(icon, IconSize::Md, IconTone::AccentPrimary, theme))
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(title),
                            ),
                    )
                    .child(
                        div()
                            .id("entity-modal-close-btn")
                            .p(px(4.0))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close_modal(cx);
                            }))
                            .child(app_icon(
                                AppIcon::Close,
                                IconSize::Sm,
                                IconTone::Muted,
                                theme,
                            )),
                    ),
            )
            // Body
            .child(Self::render_body(modal, state, inputs, locale, theme, cx))
            // Footer buttons
            .child(
                div()
                    .mt(px(6.0))
                    .flex()
                    .flex_row()
                    .gap(px(10.0))
                    .child(
                        div()
                            .id("entity-modal-cancel-btn")
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
                                this.close_modal(cx);
                            }))
                            .child(umuryango_i18n::t(locale, "common.cancel")),
                    )
                    .child(
                        div()
                            .id("entity-modal-submit-btn")
                            .flex_1()
                            .py(px(10.0))
                            .rounded(px(8.0))
                            .bg(theme.accent_primary)
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(6.0))
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.submit_entity_modal(cx);
                            }))
                            .child(app_icon(
                                AppIcon::Check,
                                IconSize::Sm,
                                IconTone::OnPrimaryButton,
                                theme,
                            ))
                            .child(if state.is_busy {
                                umuryango_i18n::t(locale, "common.loading")
                            } else {
                                umuryango_i18n::t(locale, "common.save")
                            }),
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
                        div()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .bg(theme.bg_canvas)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(format!(
                                "{}: {} RWF",
                                umuryango_i18n::t(locale, "loans.remaining"),
                                remaining
                            )),
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

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                    .text_color(theme.text_muted)
                    .child(umuryango_i18n::t(locale, "wallets.channel")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(6.0))
                    .children(channels.into_iter().map(|(slug, ch, label)| {
                        let is_sel = selected == ch;
                        div()
                            .id(format!("modal-wallet-ch-{}", slug))
                            .flex_1()
                            .py(px(7.0))
                            .px(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_sel {
                                theme.accent_primary
                            } else {
                                theme.bg_elevated
                            })
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .text_center()
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .text_color(if is_sel {
                                theme.bg_canvas
                            } else {
                                theme.text_primary
                            })
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.state.entity_form.wallet_channel = ch;
                                cx.notify();
                            }))
                            .child(label)
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

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                    .text_color(theme.text_muted)
                    .child(umuryango_i18n::t(locale, "loans.direction")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(6.0))
                    .children(options.into_iter().map(|(slug, dir, label)| {
                        let is_sel = selected == dir;
                        div()
                            .id(format!("modal-loan-dir-{}", slug))
                            .flex_1()
                            .py(px(7.0))
                            .px(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_sel {
                                theme.accent_primary
                            } else {
                                theme.bg_elevated
                            })
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .text_center()
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .text_color(if is_sel {
                                theme.bg_canvas
                            } else {
                                theme.text_primary
                            })
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.state.entity_form.loan_direction = dir;
                                cx.notify();
                            }))
                            .child(label)
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

        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(6.0))
                    .children(roles.into_iter().map(|(slug, role, label)| {
                        let is_sel = selected == role;
                        div()
                            .id(format!("modal-invite-role-{}", slug))
                            .flex_1()
                            .py(px(7.0))
                            .px(px(8.0))
                            .rounded(px(6.0))
                            .bg(if is_sel {
                                theme.accent_primary
                            } else {
                                theme.bg_elevated
                            })
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .text_center()
                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                            .text_color(if is_sel {
                                theme.bg_canvas
                            } else {
                                theme.text_primary
                            })
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.state.entity_form.invite_role = role;
                                cx.notify();
                            }))
                            .child(label)
                    })),
            )
    }
}

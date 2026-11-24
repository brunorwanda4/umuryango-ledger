use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::{FamilyMemberView, FamilyRole};
use umuryango_i18n::{Locale, Translator, terms};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{ActiveModal, AppState, Screen};
use crate::theme::SemanticTheme;

pub struct MembersScreen;

impl MembersScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let title = Translator::t(state.locale, terms::NAV_MEMBERS, None);
        let copy_code_msg = umuryango_i18n::t(state.locale, "members.code_copied");
        let is_family_scope = state.active_scope.is_family();
        let is_head = state.active_role == FamilyRole::Head;
        let invite_code = state.active_family_invite_code.clone();

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
            // Header + Real Invite Code & Invite Action
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .mb(px(20.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(title),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(umuryango_i18n::t(state.locale, "members.subtitle")),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            .children(invite_code.map(|code| {
                                let code_for_copy = code.clone();
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .bg(theme.bg_surface)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .child(div().text_xs().text_color(theme.text_muted).child(
                                        format!(
                                            "{}:",
                                            umuryango_i18n::t(state.locale, "members.invite_code")
                                        ),
                                    ))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                            .text_color(theme.accent_primary)
                                            .child(code),
                                    )
                                    .child(
                                        div()
                                            .id("members-copy-code-btn")
                                            .flex()
                                            .items_center()
                                            .gap(px(4.0))
                                            .px(px(8.0))
                                            .py(px(3.0))
                                            .bg(theme.accent_primary)
                                            .text_xs()
                                            .text_color(theme.bg_canvas)
                                            .cursor_pointer()
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.copy_to_clipboard(
                                                    code_for_copy.clone(),
                                                    copy_code_msg.clone(),
                                                    cx,
                                                );
                                            }))
                                            .child(app_icon(
                                                AppIcon::Copy,
                                                IconSize::Xs,
                                                IconTone::OnPrimaryButton,
                                                theme,
                                            ))
                                            .child(umuryango_i18n::t(state.locale, "common.copy")),
                                    )
                            }))
                            .children(if is_family_scope && is_head {
                                Some(
                                    div()
                                        .id("members-invite-btn")
                                        .flex()
                                        .items_center()
                                        .gap(px(6.0))
                                        .px(px(12.0))
                                        .py(px(6.0))
                                        .bg(theme.accent_primary)
                                        .text_xs()
                                        .font_weight(gpui_kit::FontWeight::BOLD)
                                        .text_color(theme.bg_canvas)
                                        .cursor_pointer()
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.open_modal(ActiveModal::InviteMember, window, cx);
                                        }))
                                        .child(app_icon(
                                            AppIcon::Add,
                                            IconSize::Xs,
                                            IconTone::OnPrimaryButton,
                                            theme,
                                        ))
                                        .child(umuryango_i18n::t(
                                            state.locale,
                                            "members.invite_member",
                                        )),
                                )
                            } else {
                                None
                            }),
                    ),
            )
            // Body: No Family Prompt or Real Members List from PostgreSQL
            .child(if !is_family_scope {
                div()
                    .p(px(24.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_muted)
                            .child(umuryango_i18n::t(
                                state.locale,
                                "members.no_family_selected",
                            )),
                    )
                    .child(
                        div()
                            .id("members-setup-family-btn")
                            .px(px(14.0))
                            .py(px(8.0))
                            .bg(theme.accent_primary)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.select_screen(Screen::Onboarding, cx);
                            }))
                            .child(umuryango_i18n::t(state.locale, "members.setup_family_btn")),
                    )
                    .into_any_element()
            } else if state.family_members.is_empty() {
                div()
                    .p(px(24.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .text_sm()
                    .text_center()
                    .text_color(theme.text_muted)
                    .child(umuryango_i18n::t(state.locale, "members.no_members"))
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .gap(px(10.0))
                    .children(state.family_members.iter().map(|m| {
                        let is_you = m.user_id == state.user_id;
                        Self::render_member_row(m, is_you, is_head, state.locale, theme, cx)
                    }))
                    .into_any_element()
            })
    }

    fn render_member_row(
        member: &FamilyMemberView,
        is_you: bool,
        viewer_is_head: bool,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let name = member
            .display_name
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(&member.full_name);
        let detail = member.phone.as_str();

        let (role_label, role_color) = match member.role {
            FamilyRole::Head => (
                umuryango_i18n::t(locale, "members.role_head"),
                theme.accent_primary,
            ),
            FamilyRole::Custodian => (
                umuryango_i18n::t(locale, "members.role_custodian"),
                theme.income_color,
            ),
            FamilyRole::Member => (
                umuryango_i18n::t(locale, "members.role_member"),
                theme.text_muted,
            ),
        };

        let initial = name.chars().next().unwrap_or('?').to_string();
        let next_role = match member.role {
            FamilyRole::Member => FamilyRole::Custodian,
            FamilyRole::Custodian => FamilyRole::Head,
            FamilyRole::Head => FamilyRole::Member,
        };
        let member_id = member.id;
        let can_change_role = viewer_is_head && !is_you;

        let role_badge = div()
            .id(format!("member-role-{}", member_id))
            .flex()
            .items_center()
            .gap(px(6.0))
            .px(px(10.0))
            .py(px(4.0))
            .bg(role_color.opacity(0.15))
            .text_xs()
            .font_weight(gpui_kit::FontWeight::BOLD)
            .text_color(role_color)
            .child(app_icon(
                AppIcon::Members,
                IconSize::Xs,
                match member.role {
                    FamilyRole::Head => IconTone::AccentPrimary,
                    FamilyRole::Custodian => IconTone::Income,
                    FamilyRole::Member => IconTone::Muted,
                },
                theme,
            ))
            .child(role_label);

        let role_control = if can_change_role {
            role_badge
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.handle_update_member_role(member_id, next_role, cx);
                }))
                .into_any_element()
        } else {
            role_badge.into_any_element()
        };

        div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .p(px(14.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(36.0))
                            .bg(theme.bg_elevated)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.accent_primary)
                            .child(initial),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(name.to_string()),
                                    )
                                    .children(if is_you {
                                        Some(
                                            div()
                                                .px(px(6.0))
                                                .py(px(1.0))
                                                .bg(theme.bg_elevated)
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(umuryango_i18n::t(
                                                    locale,
                                                    "common.you_badge",
                                                )),
                                        )
                                    } else {
                                        None
                                    }),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(detail.to_string()),
                            ),
                    ),
            )
            .child(role_control)
    }
}

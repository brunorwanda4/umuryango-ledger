use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::{FamilyMemberView, FamilyRole};
use umuryango_i18n::{Locale, Translator, terms};

use crate::RootView;
use crate::common::{
    Avatar, AvatarSize, Badge, Button, ButtonGroup, ButtonGroupText, ButtonSize, Empty,
    EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, Item, ItemActions, ItemContent,
    ItemMedia, ItemSize,
};
use crate::components::icon::AppIcon;
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
                                    .font_weight(FontWeight::BOLD)
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
                                ButtonGroup::horizontal()
                                    .child(
                                        ButtonGroupText::new(format!(
                                            "{}: {}",
                                            umuryango_i18n::t(state.locale, "members.invite_code"),
                                            code
                                        ))
                                        .outlined(true)
                                        .bg(theme.bg_surface)
                                        .text_color(theme.accent_primary)
                                        .theme(theme),
                                    )
                                    .child(
                                        Button::new("members-copy-code-btn")
                                            .size(ButtonSize::Sm)
                                            .icon_start(AppIcon::Copy)
                                            .label(umuryango_i18n::t(state.locale, "common.copy"))
                                            .theme(theme)
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.copy_to_clipboard(
                                                    code_for_copy.clone(),
                                                    copy_code_msg.clone(),
                                                    cx,
                                                );
                                            })),
                                    )
                            }))
                            .children(if is_family_scope && is_head {
                                Some(
                                    Button::new("members-invite-btn")
                                        .size(ButtonSize::Sm)
                                        .icon_start(AppIcon::Add)
                                        .label(umuryango_i18n::t(
                                            state.locale,
                                            "members.invite_member",
                                        ))
                                        .theme(theme)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.open_modal(ActiveModal::InviteMember, window, cx);
                                        })),
                                )
                            } else {
                                None
                            }),
                    ),
            )
            // Body: No Family Prompt or Real Members List from PostgreSQL
            .child(if !is_family_scope {
                Empty::new()
                    .bordered(true)
                    .p(px(32.0))
                    .bg(theme.bg_surface)
                    .theme(theme)
                    .child(
                        EmptyHeader::new()
                            .child(EmptyMedia::from_icon(AppIcon::FamilyLedger).theme(theme))
                            .child(
                                EmptyDescription::new(umuryango_i18n::t(
                                    state.locale,
                                    "members.no_family_selected",
                                ))
                                .theme(theme),
                            ),
                    )
                    .child(
                        EmptyContent::new().child(
                            Button::new("members-setup-family-btn")
                                .size(ButtonSize::Sm)
                                .label(umuryango_i18n::t(state.locale, "members.setup_family_btn"))
                                .theme(theme)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.select_screen(Screen::Onboarding, cx);
                                })),
                        ),
                    )
                    .into_any_element()
            } else if state.family_members.is_empty() {
                Empty::new()
                    .bordered(true)
                    .p(px(32.0))
                    .bg(theme.bg_surface)
                    .theme(theme)
                    .child(
                        EmptyHeader::new()
                            .child(EmptyMedia::from_icon(AppIcon::Members).theme(theme))
                            .child(
                                EmptyDescription::new(umuryango_i18n::t(
                                    state.locale,
                                    "members.no_members",
                                ))
                                .theme(theme),
                            ),
                    )
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

        let next_role = match member.role {
            FamilyRole::Member => FamilyRole::Custodian,
            FamilyRole::Custodian => FamilyRole::Head,
            FamilyRole::Head => FamilyRole::Member,
        };
        let member_id = member.id;
        let can_change_role = viewer_is_head && !is_you;

        let mut role_badge = Badge::new(role_label)
            .id(format!("member-role-{}", member_id))
            .outline()
            .icon_start(AppIcon::Members)
            .px(px(10.0))
            .py(px(4.0))
            .bg(role_color.opacity(0.15))
            .text_color(role_color)
            .theme(theme);

        if can_change_role {
            role_badge = role_badge.on_click(cx.listener(move |this, _, _, cx| {
                this.handle_update_member_role(member_id, next_role, cx);
            }));
        }

        Item::new()
            .outline()
            .size(ItemSize::Sm)
            .bg(theme.bg_surface)
            .justify_between()
            .theme(theme)
            .child(
                ItemMedia::new()
                    .theme(theme)
                    .child(Avatar::from_name(name, theme).size(AvatarSize::Lg)),
            )
            .child(
                ItemContent::new()
                    .gap(px(2.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(name.to_string()),
                            )
                            .children(if is_you {
                                Some(
                                    Badge::new(umuryango_i18n::t(locale, "common.you_badge"))
                                        .secondary()
                                        .px(px(6.0))
                                        .py(px(1.0))
                                        .theme(theme),
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
            )
            .child(ItemActions::new().child(role_badge))
    }
}

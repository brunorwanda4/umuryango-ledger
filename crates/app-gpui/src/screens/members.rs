use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::FamilyRole;
use umuryango_i18n::{Locale, Translator, terms};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::AppState;
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

        let copy_code_msg = if state.locale == Locale::Rw {
            "Kode y'ubutumire yakoporowe!".to_string()
        } else {
            "Invite code copied to clipboard!".to_string()
        };

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
            // Header + Invite Code
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
                            .child(div().text_xs().text_color(theme.text_muted).child(
                                if state.locale == Locale::Rw {
                                    "Abagize umuryango n'inshingano zabo"
                                } else {
                                    "Family members and custody authorization roles"
                                },
                            )),
                    )
                    // Invite code badge & copy button
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("Invite Code:"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.accent_primary)
                                    .child("UMUR7890"),
                            )
                            .child(
                                div()
                                    .id("members-copy-code-btn")
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .px(px(8.0))
                                    .py(px(3.0))
                                    .rounded(px(4.0))
                                    .bg(theme.accent_primary)
                                    .text_xs()
                                    .text_color(theme.bg_canvas)
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.copy_to_clipboard(
                                            "UMUR7890".to_string(),
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
                                    .child("Copy"),
                            ),
                    ),
            )
            // Members List
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(10.0))
                    .child(Self::render_member_row(
                        &state.user_name,
                        "Family Head",
                        FamilyRole::Head,
                        true,
                        state.locale,
                        theme,
                    ))
                    .child(Self::render_member_row(
                        "Mama Hapiman",
                        "Co-Custodian (Main MoMo)",
                        FamilyRole::Custodian,
                        false,
                        state.locale,
                        theme,
                    ))
                    .child(Self::render_member_row(
                        "Uncle Jean",
                        "Audit & Transparency",
                        FamilyRole::Member,
                        false,
                        state.locale,
                        theme,
                    ))
                    .child(Self::render_member_row(
                        "Grace Ingabire",
                        "Family Elder Advisory",
                        FamilyRole::Member,
                        false,
                        state.locale,
                        theme,
                    )),
            )
    }

    fn render_member_row(
        name: &str,
        detail: &str,
        role: FamilyRole,
        is_you: bool,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let (role_label, role_color) = match role {
            FamilyRole::Head => (
                if locale == Locale::Rw {
                    "Umukuru w'Umuryango"
                } else {
                    "Family Head"
                },
                theme.accent_primary,
            ),
            FamilyRole::Custodian => (
                if locale == Locale::Rw {
                    "Umubikizi w'Imari"
                } else {
                    "Custodian"
                },
                theme.income_color,
            ),
            FamilyRole::Member => (
                if locale == Locale::Rw {
                    "Umunyamuryango"
                } else {
                    "Member"
                },
                theme.text_muted,
            ),
        };

        let initial = name.chars().next().unwrap_or('?').to_string();

        div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .p(px(14.0))
            .rounded(px(10.0))
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
                            .rounded(px(18.0))
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
                                    .child(if is_you {
                                        div()
                                            .px(px(6.0))
                                            .py(px(1.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_elevated)
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("(You)")
                                    } else {
                                        div()
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
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .px(px(10.0))
                    .py(px(4.0))
                    .rounded(px(6.0))
                    .bg(role_color.opacity(0.15))
                    .text_xs()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .text_color(role_color)
                    .child(app_icon(
                        AppIcon::Members,
                        IconSize::Xs,
                        match role {
                            FamilyRole::Head => IconTone::AccentPrimary,
                            FamilyRole::Custodian => IconTone::Income,
                            FamilyRole::Member => IconTone::Muted,
                        },
                        theme,
                    ))
                    .child(role_label),
            )
    }
}

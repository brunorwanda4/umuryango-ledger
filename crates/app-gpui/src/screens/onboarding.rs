use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Translator, terms};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::components::lang_toggle::LangToggle;
use crate::state::AppState;
use crate::theme::{SemanticTheme, ThemeMode};

pub struct OnboardingScreen;

impl OnboardingScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_dark = state.theme_mode == ThemeMode::Dark;

        div()
            .id(id)
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            .pt(px(16.0) + state.safe_area.top_px())
            .pb(px(16.0) + state.safe_area.total_bottom_px())
            .pl(px(16.0) + state.safe_area.left_px())
            .pr(px(16.0) + state.safe_area.right_px())
            .overflow_y_scroll()
            .relative()
            // Top Bar: Language & Theme Toggles
            .child(
                div()
                    .absolute()
                    .top(px(16.0) + state.safe_area.top_px())
                    .right(px(24.0) + state.safe_area.right_px())
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .child(LangToggle::render(
                        "onboarding-lang-toggle",
                        state.locale,
                        theme,
                        cx,
                    ))
                    .child(
                        div()
                            .id("onboarding-theme-toggle")
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(32.0))
                            .h(px(32.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_elevated)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_theme(cx);
                            }))
                            .child(app_icon(
                                if is_dark { AppIcon::Sun } else { AppIcon::Moon },
                                IconSize::Sm,
                                IconTone::Default,
                                theme,
                            )),
                    ),
            )
            // Main Onboarding Container
            .child(
                div()
                    .w(px(520.0))
                    .max_w(px(520.0))
                    .flex()
                    .flex_col()
                    .gap(px(16.0))
                    // Header Card
                    .child(
                        div()
                            .p(px(20.0))
                            .rounded(px(14.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(10.0))
                            .child(
                                gpui_kit::img("crates/app-gpui/assets/icons/icon-256.png")
                                    .w(px(52.0))
                                    .h(px(52.0))
                                    .rounded(px(12.0)),
                            )
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(Translator::t(
                                        state.locale,
                                        terms::ONBOARDING_TITLE,
                                        None,
                                    )),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_center()
                                    .text_color(theme.text_muted)
                                    .child(Translator::t(state.locale, terms::TAGLINE, None)),
                            ),
                    )
                    // Pending Invites Banner (if non-empty)
                    .children(if !state.pending_invites.is_empty() {
                        Some(Self::render_pending_invites(state, theme, cx))
                    } else {
                        None
                    })
                    // Option 1: Create Family Card
                    .child(Self::render_create_family_card(state, theme, cx))
                    // Option 2: Join Family Card
                    .child(Self::render_join_family_card(state, theme, cx))
                    // Option 3: Skip for now Action
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .py(px(8.0))
                            .child(
                                div()
                                    .id("onboarding-skip-btn")
                                    .cursor_pointer()
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .text_color(theme.text_muted)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.handle_skip_onboarding(cx);
                                    }))
                                    .child(Translator::t(state.locale, terms::SKIP_FOR_NOW, None)),
                            ),
                    ),
            )
    }

    fn render_pending_invites(
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        div()
            .w_full()
            .p(px(16.0))
            .rounded(px(12.0))
            .bg(theme.accent_primary.opacity(0.12))
            .border_1()
            .border_color(theme.accent_primary.opacity(0.4))
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(app_icon(
                        AppIcon::FamilyLedger,
                        IconSize::Sm,
                        IconTone::AccentPrimary,
                        theme,
                    ))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.accent_primary)
                            .child(Translator::t(
                                state.locale,
                                terms::PENDING_FAMILY_INVITES,
                                None,
                            )),
                    ),
            )
            .children(
                state
                    .pending_invites
                    .iter()
                    .enumerate()
                    .map(|(idx, invite)| {
                        let invite_id = invite.id;
                        let family_name = invite.family_name.clone();
                        let invited_by = invite.invited_by_name.clone();

                        div()
                            .id(format!("invite-{}", idx))
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(format!(
                                                "{}: {}",
                                                Translator::t(
                                                    state.locale,
                                                    terms::YOU_WERE_ADDED_TO,
                                                    None
                                                ),
                                                family_name
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(format!("By: {}", invited_by)),
                                    ),
                            )
                            .child(
                                div()
                                    .id(format!("accept-invite-btn-{}", idx))
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(theme.income_color)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.bg_canvas)
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.handle_accept_invite(
                                            invite_id,
                                            family_name.clone(),
                                            cx,
                                        );
                                    }))
                                    .child(Translator::t(state.locale, terms::JOIN_BTN, None)),
                            )
                    }),
            )
    }

    fn render_create_family_card(
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        div()
            .w_full()
            .p(px(18.0))
            .rounded(px(14.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(app_icon(
                        AppIcon::Add,
                        IconSize::Sm,
                        IconTone::AccentPrimary,
                        theme,
                    ))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(Translator::t(state.locale, terms::CREATE_FAMILY, None)),
                    ),
            )
            // Family Name Input
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
                            .child(Translator::t(state.locale, terms::FAMILY_NAME, None)),
                    )
                    .child(
                        div()
                            .w_full()
                            .px(px(12.0))
                            .py(px(8.0))
                            .rounded(px(8.0))
                            .bg(theme.bg_canvas)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_sm()
                            .text_color(theme.text_primary)
                            .child(if state.onboarding_family_name.is_empty() {
                                "Umuryango w'Abahire".to_string()
                            } else {
                                state.onboarding_family_name.clone()
                            }),
                    ),
            )
            // Repeatable Phone Number Inputs
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(theme.text_muted)
                                    .child(Translator::t(
                                        state.locale,
                                        terms::ADD_MEMBER_PHONE,
                                        None,
                                    )),
                            )
                            .child(
                                div()
                                    .id("add-member-phone-btn")
                                    .cursor_pointer()
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.accent_primary)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.state.onboarding_member_phones.push(String::new());
                                        cx.notify();
                                    }))
                                    .child(format!(
                                        "+ {}",
                                        Translator::t(state.locale, terms::ADD_PHONE_BTN, None)
                                    )),
                            ),
                    )
                    .children(state.onboarding_member_phones.iter().enumerate().map(
                        |(idx, phone)| {
                            div()
                                .id(format!("phone-{}", idx))
                                .w_full()
                                .px(px(12.0))
                                .py(px(7.0))
                                .rounded(px(8.0))
                                .bg(theme.bg_canvas)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .text_color(theme.text_primary)
                                .child(if phone.is_empty() {
                                    "+250 788 000 000".to_string()
                                } else {
                                    phone.clone()
                                })
                        },
                    )),
            )
            // Create Family Submit Button
            .child(
                div()
                    .id("create-family-submit-btn")
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(8.0))
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(theme.accent_primary)
                    .text_sm()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .text_color(theme.bg_canvas)
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.handle_create_family_submit(cx);
                    }))
                    .child(app_icon(
                        AppIcon::Check,
                        IconSize::Sm,
                        IconTone::OnPrimaryButton,
                        theme,
                    ))
                    .child(Translator::t(
                        state.locale,
                        terms::CONTINUE_TO_DASHBOARD,
                        None,
                    )),
            )
    }

    fn render_join_family_card(
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        div()
            .w_full()
            .p(px(18.0))
            .rounded(px(14.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(app_icon(
                        AppIcon::FamilyLedger,
                        IconSize::Sm,
                        IconTone::AccentPrimary,
                        theme,
                    ))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(Translator::t(state.locale, terms::JOIN_FAMILY, None)),
                    ),
            )
            // 4-Character Code Input
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
                            .child(Translator::t(state.locale, terms::ENTER_4_CHAR_CODE, None)),
                    )
                    .child(
                        div()
                            .w_full()
                            .px(px(16.0))
                            .py(px(10.0))
                            .rounded(px(8.0))
                            .bg(theme.bg_canvas)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_base()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .text_center()
                            .child(if state.onboarding_join_code.is_empty() {
                                "HK0N".to_string()
                            } else {
                                state.onboarding_join_code.clone()
                            }),
                    ),
            )
            // Join Family Submit Button
            .child(
                div()
                    .id("join-family-submit-btn")
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap(px(8.0))
                    .py(px(10.0))
                    .rounded(px(8.0))
                    .bg(theme.accent_primary)
                    .text_sm()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .text_color(theme.bg_canvas)
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.handle_join_family_submit(cx);
                    }))
                    .child(app_icon(
                        AppIcon::ChevronRight,
                        IconSize::Sm,
                        IconTone::OnPrimaryButton,
                        theme,
                    ))
                    .child(Translator::t(state.locale, terms::JOIN_BTN, None)),
            )
    }
}

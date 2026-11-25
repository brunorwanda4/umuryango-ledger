use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Translator, terms};

use super::onboarding_inputs::OnboardingInputs;
use crate::RootView;
use crate::common::input::text_field;
use crate::components::icon::{AppIcon, BrandMark, IconSize, IconTone, app_icon, brand_mark};
use crate::components::lang_toggle::LangToggle;
use crate::state::AppState;
use crate::theme::{SemanticTheme, ThemeMode};

pub struct OnboardingScreen;

impl OnboardingScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        inputs: &OnboardingInputs,
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
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(10.0))
                            .child(brand_mark(BrandMark::Logo, 56.0))
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
                    // Pending Invites Banner (from real PostgreSQL)
                    .children(if !state.pending_invites.is_empty() {
                        Some(Self::render_pending_invites(state, theme, cx))
                    } else {
                        None
                    })
                    // Option 1: Create Family Card
                    .child(Self::render_create_family_card(state, inputs, theme, cx))
                    // Option 2: Join Family Card
                    .child(Self::render_join_family_card(state, inputs, theme, cx))
                    // Option 3: Skip for now -> Personal Ledger
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
                        let invited_by = umuryango_i18n::t_args(
                            state.locale,
                            "onboarding.invited_by",
                            &[("name", &invite.invited_by_name)],
                        );

                        div()
                            .id(format!("invite-{}", idx))
                            .p(px(12.0))
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
                                            .child(invited_by),
                                    ),
                            )
                            .child(
                                div()
                                    .id(format!("accept-invite-btn-{}", idx))
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .bg(theme.income_color)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.bg_canvas)
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.handle_accept_invite(invite_id, cx);
                                    }))
                                    .child(Translator::t(state.locale, terms::JOIN_BTN, None)),
                            )
                    }),
            )
    }

    fn render_create_family_card(
        state: &AppState,
        inputs: &OnboardingInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let submit_label = if state.is_busy {
            umuryango_i18n::t(state.locale, "onboarding.creating_family")
        } else {
            Translator::t(state.locale, terms::CONTINUE_TO_DASHBOARD, None)
        };

        div()
            .w_full()
            .p(px(18.0))
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
            // Editable Family Name Input
            .child(text_field(
                "onboarding-family-name-input",
                Translator::t(state.locale, terms::FAMILY_NAME, None),
                &inputs.family_name,
                theme,
            ))
            // Editable Member Phone Input + Add Button + Chips
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_end()
                            .gap(px(8.0))
                            .child(div().flex_1().child(text_field(
                                "onboarding-member-phone-input",
                                Translator::t(state.locale, terms::ADD_MEMBER_PHONE, None),
                                &inputs.member_phone,
                                theme,
                            )))
                            .child(
                                div()
                                    .id("add-member-phone-btn")
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .bg(theme.bg_elevated)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .cursor_pointer()
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.accent_primary)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.add_onboarding_phone_with_window(window, cx);
                                    }))
                                    .child(Translator::t(state.locale, terms::ADD_PHONE_BTN, None)),
                            ),
                    )
                    .children(if !state.onboarding_member_phones.is_empty() {
                        Some(div().flex().flex_wrap().gap(px(6.0)).children(
                            state.onboarding_member_phones.iter().enumerate().map(
                                |(idx, phone)| {
                                    div()
                                        .id(format!("phone-chip-{}", idx))
                                        .flex()
                                        .items_center()
                                        .gap(px(6.0))
                                        .px(px(10.0))
                                        .py(px(4.0))
                                        .bg(theme.bg_canvas)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .child(phone.clone())
                                        .child(
                                            div()
                                                .id(format!("phone-chip-rm-{}", idx))
                                                .cursor_pointer()
                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                    this.remove_onboarding_phone(idx, cx);
                                                }))
                                                .child(app_icon(
                                                    AppIcon::Close,
                                                    IconSize::Xs,
                                                    IconTone::Muted,
                                                    theme,
                                                )),
                                        )
                                },
                            ),
                        ))
                    } else {
                        None
                    }),
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
                    .bg(if state.is_busy {
                        theme.accent_primary.opacity(0.6)
                    } else {
                        theme.accent_primary
                    })
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
                    .child(submit_label),
            )
    }

    fn render_join_family_card(
        state: &AppState,
        inputs: &OnboardingInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let submit_label = if state.is_busy {
            umuryango_i18n::t(state.locale, "onboarding.joining_family")
        } else {
            Translator::t(state.locale, terms::JOIN_BTN, None)
        };

        div()
            .w_full()
            .p(px(18.0))
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
            // Editable 4-Character Code Input
            .child(text_field(
                "onboarding-join-code-input",
                Translator::t(state.locale, terms::ENTER_4_CHAR_CODE, None),
                &inputs.join_code,
                theme,
            ))
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
                    .bg(if state.is_busy {
                        theme.accent_primary.opacity(0.6)
                    } else {
                        theme.accent_primary
                    })
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
                    .child(submit_label),
            )
    }
}

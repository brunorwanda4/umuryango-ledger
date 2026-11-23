use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Translator, terms};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::components::lang_toggle::LangToggle;
use crate::state::AppState;
use crate::theme::{SemanticTheme, ThemeMode};

pub struct AuthScreen;

impl AuthScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_register = state.auth_mode_register;
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
                        "auth-lang-toggle",
                        state.locale,
                        theme,
                        cx,
                    ))
                    .child(
                        div()
                            .id("auth-theme-toggle")
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
            // Auth Modal Card
            .child(
                div()
                    .w(px(460.0))
                    .p(px(28.0))
                    .rounded(px(16.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(16.0))
                    // Official Application Emblem
                    .child(
                        gpui_kit::img("crates/app-gpui/assets/icons/icon-256.png")
                            .w(px(56.0))
                            .h(px(56.0))
                            .rounded(px(12.0)),
                    )
                    // Card Title & Tagline
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(if is_register {
                                Translator::t(state.locale, terms::REGISTER_TITLE, None)
                            } else {
                                Translator::t(state.locale, terms::LOGIN_TITLE, None)
                            }),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_center()
                            .text_color(theme.text_muted)
                            .child(Translator::t(state.locale, terms::TAGLINE, None)),
                    )
                    // Form Fields
                    .child(if is_register {
                        Self::render_register_form(state, theme, cx).into_any_element()
                    } else {
                        Self::render_login_form(state, theme, cx).into_any_element()
                    })
                    // Toggle Between Login and Register
                    .child(
                        div()
                            .id("auth-switch-mode-btn")
                            .cursor_pointer()
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.accent_primary)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.state.auth_mode_register = !this.state.auth_mode_register;
                                cx.notify();
                            }))
                            .child(if is_register {
                                Translator::t(state.locale, terms::ALREADY_HAVE_ACCOUNT, None)
                            } else {
                                Translator::t(state.locale, terms::CREATE_ACCOUNT, None)
                            }),
                    ),
            )
    }

    fn render_login_form(
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(12.0))
            // Phone input
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
                            .child(Translator::t(state.locale, terms::PHONE_NUMBER, None)),
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
                            .child(if state.auth_phone.is_empty() {
                                "+250 788 000 000".to_string()
                            } else {
                                state.auth_phone.clone()
                            }),
                    ),
            )
            // Password input
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
                            .child(Translator::t(state.locale, terms::PASSWORD, None)),
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
                            .child(if state.auth_password.is_empty() {
                                "••••••••".to_string()
                            } else {
                                "••••••••".to_string()
                            }),
                    ),
            )
            // Login Button
            .child(
                div()
                    .id("auth-login-submit-btn")
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
                        this.handle_login_submit(cx);
                    }))
                    .child(app_icon(
                        AppIcon::Check,
                        IconSize::Sm,
                        IconTone::OnPrimaryButton,
                        theme,
                    ))
                    .child(Translator::t(state.locale, terms::LOGIN_TITLE, None)),
            )
    }

    fn render_register_form(
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_male = state.auth_gender == "MALE";

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(10.0))
            // Full Name input
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
                            .child(Translator::t(state.locale, terms::FULL_NAME, None)),
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
                            .child(if state.auth_full_name.is_empty() {
                                "Kwizera Aimable".to_string()
                            } else {
                                state.auth_full_name.clone()
                            }),
                    ),
            )
            // Phone input
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
                            .child(Translator::t(state.locale, terms::PHONE_NUMBER, None)),
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
                            .child(if state.auth_phone.is_empty() {
                                "+250 788 123 456".to_string()
                            } else {
                                state.auth_phone.clone()
                            }),
                    ),
            )
            // Gender Selector: Male / Female
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
                            .child(Translator::t(state.locale, terms::GENDER, None)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("gender-male-btn")
                                    .flex_1()
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(if is_male {
                                        theme.accent_primary
                                    } else {
                                        theme.bg_canvas
                                    })
                                    .border_1()
                                    .border_color(if is_male {
                                        theme.accent_primary
                                    } else {
                                        theme.border_subtle
                                    })
                                    .text_center()
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(if is_male {
                                        theme.bg_canvas
                                    } else {
                                        theme.text_primary
                                    })
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.state.auth_gender = "MALE".to_string();
                                        cx.notify();
                                    }))
                                    .child(Translator::t(state.locale, terms::GENDER_MALE, None)),
                            )
                            .child(
                                div()
                                    .id("gender-female-btn")
                                    .flex_1()
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(if !is_male {
                                        theme.accent_primary
                                    } else {
                                        theme.bg_canvas
                                    })
                                    .border_1()
                                    .border_color(if !is_male {
                                        theme.accent_primary
                                    } else {
                                        theme.border_subtle
                                    })
                                    .text_center()
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(if !is_male {
                                        theme.bg_canvas
                                    } else {
                                        theme.text_primary
                                    })
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.state.auth_gender = "FEMALE".to_string();
                                        cx.notify();
                                    }))
                                    .child(Translator::t(state.locale, terms::GENDER_FEMALE, None)),
                            ),
                    ),
            )
            // Password input
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
                            .child(Translator::t(state.locale, terms::PASSWORD, None)),
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
                            .child("••••••••".to_string()),
                    ),
            )
            // Password Again (Confirm Password) input
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
                            .child(Translator::t(state.locale, terms::CONFIRM_PASSWORD, None)),
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
                            .child("••••••••".to_string()),
                    ),
            )
            // Create Account Submit Button
            .child(
                div()
                    .id("auth-register-submit-btn")
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
                        this.handle_register_submit(cx);
                    }))
                    .child(app_icon(
                        AppIcon::Check,
                        IconSize::Sm,
                        IconTone::OnPrimaryButton,
                        theme,
                    ))
                    .child(Translator::t(state.locale, terms::CREATE_ACCOUNT, None)),
            )
    }
}

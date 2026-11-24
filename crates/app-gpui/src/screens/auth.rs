use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Translator, terms};

use super::auth_inputs::AuthInputs;
use crate::RootView;
use crate::common::input::{password_field, text_field};
use crate::common::radio::{radio_field, radio_option};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::components::lang_toggle::LangToggle;
use crate::state::AppState;
use crate::theme::{SemanticTheme, ThemeMode};

pub struct AuthScreen;

impl AuthScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        inputs: &AuthInputs,
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
                        Self::render_register_form(state, inputs, theme, cx).into_any_element()
                    } else {
                        Self::render_login_form(state, inputs, theme, cx).into_any_element()
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
        inputs: &AuthInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let t = |key| Translator::t(state.locale, key, None);

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(text_field(
                "auth-login-phone",
                t(terms::PHONE_NUMBER),
                &inputs.phone,
                theme,
            ))
            .child(password_field(
                "auth-login-password",
                t(terms::PASSWORD),
                &inputs.password,
                theme,
                cx,
            ))
            .child(Self::submit_button(
                "auth-login-submit-btn",
                t(terms::LOGIN_TITLE),
                theme,
                cx,
                |this, cx| this.handle_login_submit(cx),
            ))
    }

    fn render_register_form(
        state: &AppState,
        inputs: &AuthInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let t = |key| Translator::t(state.locale, key, None);

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(text_field(
                "auth-register-name",
                t(terms::FULL_NAME),
                &inputs.full_name,
                theme,
            ))
            .child(text_field(
                "auth-register-phone",
                t(terms::PHONE_NUMBER),
                &inputs.phone,
                theme,
            ))
            .child(Self::render_gender_selector(state, theme, cx))
            .child(password_field(
                "auth-register-password",
                t(terms::PASSWORD),
                &inputs.password,
                theme,
                cx,
            ))
            .child(password_field(
                "auth-register-confirm-password",
                t(terms::CONFIRM_PASSWORD),
                &inputs.confirm_password,
                theme,
                cx,
            ))
            .child(Self::submit_button(
                "auth-register-submit-btn",
                t(terms::CREATE_ACCOUNT),
                theme,
                cx,
                |this, cx| this.handle_register_submit(cx),
            ))
    }

    fn render_gender_selector(
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let selected = match state.auth_gender.as_str() {
            "MALE" => Some(0),
            "FEMALE" => Some(1),
            _ => None,
        };

        radio_field(
            "auth-register-gender",
            Translator::t(state.locale, terms::GENDER, None),
            [
                radio_option(
                    "gender-male",
                    Translator::t(state.locale, terms::GENDER_MALE, None),
                ),
                radio_option(
                    "gender-female",
                    Translator::t(state.locale, terms::GENDER_FEMALE, None),
                ),
            ],
            selected,
            cx.listener(|this, index: &usize, _, cx| {
                this.state.auth_gender = match *index {
                    0 => "MALE".to_string(),
                    _ => "FEMALE".to_string(),
                };
                cx.notify();
            }),
            theme,
        )
    }

    fn submit_button(
        id: &'static str,
        label: String,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
        on_submit: fn(&mut RootView, &mut Context<RootView>),
    ) -> impl IntoElement {
        div()
            .id(id)
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
            .on_click(cx.listener(move |this, _, _, cx| on_submit(this, cx)))
            .child(app_icon(
                AppIcon::Check,
                IconSize::Sm,
                IconTone::OnPrimaryButton,
                theme,
            ))
            .child(label)
    }
}

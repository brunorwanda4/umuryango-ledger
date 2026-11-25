use gpui_kit::{
    Context, ElementId, InteractiveElement, IntoElement, ParentElement, Styled, div, px,
};
use umuryango_i18n::{Translator, terms};

use super::auth_inputs::AuthInputs;
use crate::RootView;
use crate::common::{
    Button, ButtonSize, Card, CardContent, CardFooter, CardHeader, CardSize, CardTitle, Field,
    Label, RadioGroupContainer, RadioGroupItem, password_field, text_field,
};
use crate::components::icon::{AppIcon, BrandMark, brand_mark};
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
                        Button::new("auth-theme-toggle")
                            .outline()
                            .size(ButtonSize::IconSm)
                            .icon(if is_dark { AppIcon::Sun } else { AppIcon::Moon })
                            .theme(theme)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_theme(cx);
                            })),
                    ),
            )
            // Auth Modal Card
            .child(
                Card::new()
                    .size(CardSize::Sm)
                    .w(px(460.0))
                    .gap(px(18.0))
                    .py(px(24.0))
                    .theme(theme)
                    .child(
                        CardHeader::new()
                            .size(CardSize::Sm)
                            .items_center()
                            .theme(theme)
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(brand_mark(BrandMark::Logo, 64.0))
                                    .child(
                                        CardTitle::new(if is_register {
                                            Translator::t(state.locale, terms::REGISTER_TITLE, None)
                                        } else {
                                            Translator::t(state.locale, terms::LOGIN_TITLE, None)
                                        })
                                        .text_center()
                                        .theme(theme),
                                    ),
                            ),
                    )
                    .child(CardContent::new().size(CardSize::Sm).child(if is_register {
                        Self::render_register_form(state, inputs, theme, cx).into_any_element()
                    } else {
                        Self::render_login_form(state, inputs, theme, cx).into_any_element()
                    }))
                    .child(
                        CardFooter::new()
                            .size(CardSize::Sm)
                            .justify_center()
                            .theme(theme)
                            .child(
                                Button::new("auth-switch-mode-btn")
                                    .link()
                                    .size(ButtonSize::Sm)
                                    .disabled(state.is_busy)
                                    .label(if is_register {
                                        Translator::t(
                                            state.locale,
                                            terms::ALREADY_HAVE_ACCOUNT,
                                            None,
                                        )
                                    } else {
                                        Translator::t(state.locale, terms::CREATE_ACCOUNT, None)
                                    })
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.toggle_auth_mode(cx);
                                    })),
                            ),
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
        let submit_label = if state.is_busy {
            umuryango_i18n::t(state.locale, "auth.logging_in")
        } else {
            t(terms::LOGIN_TITLE)
        };

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(14.0))
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
                submit_label,
                state.is_busy,
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
        let submit_label = if state.is_busy {
            umuryango_i18n::t(state.locale, "auth.creating_account")
        } else {
            t(terms::CREATE_ACCOUNT)
        };

        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(12.0))
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
                submit_label,
                state.is_busy,
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

        Field::new()
            .gap(px(8.0))
            .theme(theme)
            .child(Label::new(Translator::t(state.locale, terms::GENDER, None)).theme(theme))
            .child(
                RadioGroupContainer::horizontal("auth-register-gender")
                    .selected_index(selected)
                    .item(
                        RadioGroupItem::new("gender-male")
                            .label(Translator::t(state.locale, terms::GENDER_MALE, None))
                            .primary_accent(true),
                    )
                    .item(
                        RadioGroupItem::new("gender-female")
                            .label(Translator::t(state.locale, terms::GENDER_FEMALE, None))
                            .primary_accent(true),
                    )
                    .theme(theme)
                    .on_change(cx.listener(|this, index: &usize, _, cx| {
                        this.state.auth_gender = match *index {
                            0 => "MALE".to_string(),
                            _ => "FEMALE".to_string(),
                        };
                        cx.notify();
                    })),
            )
    }

    fn submit_button(
        id: &'static str,
        label: String,
        is_busy: bool,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
        on_submit: fn(&mut RootView, &mut Context<RootView>),
    ) -> impl IntoElement {
        Button::new(id)
            .label(label)
            .icon_start(AppIcon::Check)
            .loading(is_busy)
            .w_full()
            .theme(theme)
            .on_click(cx.listener(move |this, _, _, cx| on_submit(this, cx)))
    }
}

use gpui_kit::prelude::*;
use gpui_kit::{
    Context, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement, Styled, Window,
    div, px,
};
use umuryango_i18n::{Translator, terms};

use super::auth_inputs::AuthInputs;
use crate::RootView;
use crate::common::utils::StyleComposeExt as _;
use crate::common::{
    Button, ButtonSize, Card, CardContent, CardFooter, CardHeader, CardSize, CardTitle, Field,
    Label, RadioGroupContainer, RadioGroupItem, password_field, text_field,
};
use crate::components::icon::{AppIcon, BrandMark, brand_mark};
use crate::components::lang_toggle::LangToggle;
use crate::hooks::use_mobile::{MOBILE_BREAKPOINT, viewport_height, viewport_width};
use crate::safe_area::SafeAreaInsets;
use crate::state::AppState;
use crate::theme::{SemanticTheme, ThemeMode};

const AUTH_CARD_MAX_WIDTH_PX: f32 = 460.0;
const NARROW_SCREEN_WIDTH_PX: f32 = 420.0;
const ULTRA_NARROW_WIDTH_PX: f32 = 360.0;

pub const AUTHOR_NAME: &str = "Rwanda Bruno";
pub const AUTHOR_GITHUB_URL: &str = "https://github.com/brunorwanda4";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AuthResponsiveLayout {
    pub is_mobile: bool,
    pub is_narrow: bool,
    pub center_vertically: bool,
    pub outer_px: f32,
    pub top_bar_pt: f32,
    pub body_pt: f32,
    pub body_pb: f32,
    pub card_px: f32,
    pub card_py: f32,
    pub card_gap: f32,
    pub form_gap: f32,
    pub logo_size: f32,
}

impl AuthResponsiveLayout {
    pub fn compute(raw_vw: f32, raw_vh: f32, safe_area: SafeAreaInsets, is_register: bool) -> Self {
        let (vw, vh) = if raw_vw > 0.0 && raw_vh > 0.0 {
            (raw_vw, raw_vh)
        } else if gpui_kit::is_mobile() {
            (375.0, 667.0)
        } else {
            (1280.0, 800.0)
        };

        let usable_w = (vw - safe_area.left - safe_area.right).max(260.0);
        let usable_h = (vh - safe_area.top - safe_area.total_bottom()).max(300.0);

        let is_mobile = vw < MOBILE_BREAKPOINT;
        let is_narrow = usable_w < NARROW_SCREEN_WIDTH_PX;
        let is_ultra_narrow = usable_w < ULTRA_NARROW_WIDTH_PX;

        let min_center_height = if is_register { 690.0 } else { 500.0 };
        let center_vertically = usable_h >= min_center_height;
        let compact_vertical = is_mobile || !center_vertically;

        let outer_px = if is_ultra_narrow {
            12.0
        } else if is_mobile {
            16.0
        } else {
            24.0
        };

        let top_bar_pt = if is_mobile { 12.0 } else { 16.0 };
        let body_pt = if center_vertically { 8.0 } else { 4.0 };
        let body_pb = if center_vertically { 8.0 } else { 16.0 };

        let card_px = if is_narrow { 16.0 } else { 20.0 };
        let card_py = if compact_vertical { 18.0 } else { 24.0 };
        let card_gap = if compact_vertical { 14.0 } else { 18.0 };
        let form_gap = if is_register {
            if compact_vertical { 10.0 } else { 12.0 }
        } else if compact_vertical {
            12.0
        } else {
            14.0
        };
        let logo_size = if compact_vertical { 48.0 } else { 64.0 };

        Self {
            is_mobile,
            is_narrow,
            center_vertically,
            outer_px,
            top_bar_pt,
            body_pt,
            body_pb,
            card_px,
            card_py,
            card_gap,
            form_gap,
            logo_size,
        }
    }
}

pub struct AuthScreen;

impl AuthScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        inputs: &AuthInputs,
        theme: &SemanticTheme,
        window: &Window,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_register = state.auth_mode_register;
        let is_dark = state.theme_mode == ThemeMode::Dark;
        let layout = AuthResponsiveLayout::compute(
            viewport_width(window),
            viewport_height(window),
            state.safe_area,
            is_register,
        );

        div()
            .id(id)
            .flex_1()
            .w_full()
            .h_full()
            .min_w_0()
            .min_h_0()
            .flex()
            .flex_col()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            // Top Utility Bar: Language & Theme Toggles (in-flow so it never overlaps the card)
            .child(
                div()
                    .w_full()
                    .flex_shrink_0()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_end()
                    .gap(px(8.0))
                    .pt(px(layout.top_bar_pt) + state.safe_area.top_px())
                    .pb(px(8.0))
                    .pl(px(layout.outer_px) + state.safe_area.left_px())
                    .pr(px(layout.outer_px) + state.safe_area.right_px())
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
            // Scrollable Body Viewport: centers card when space permits, scrolls cleanly on small screens
            .child(
                div()
                    .id("auth-scroll-body")
                    .flex_1()
                    .w_full()
                    .min_w_0()
                    .min_h_0()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .items_center()
                    .cx_when(layout.center_vertically, |d| d.justify_center())
                    .cx_when(!layout.center_vertically, |d| d.justify_start())
                    .pt(px(layout.body_pt))
                    .pb(if layout.center_vertically {
                        px(layout.body_pb)
                    } else {
                        px(layout.body_pb) + state.safe_area.total_bottom_px()
                    })
                    .pl(px(layout.outer_px) + state.safe_area.left_px())
                    .pr(px(layout.outer_px) + state.safe_area.right_px())
                    .child(
                        Card::new()
                            .size(CardSize::Sm)
                            .w_full()
                            .max_w(px(AUTH_CARD_MAX_WIDTH_PX))
                            .min_w_0()
                            .flex_shrink_0()
                            .gap(px(layout.card_gap))
                            .py(px(layout.card_py))
                            .theme(theme)
                            .child(
                                CardHeader::new()
                                    .size(CardSize::Sm)
                                    .px(px(layout.card_px))
                                    .items_center()
                                    .theme(theme)
                                    .child(
                                        div()
                                            .w_full()
                                            .min_w_0()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(brand_mark(BrandMark::Logo, layout.logo_size))
                                            .child(
                                                CardTitle::new(if is_register {
                                                    Translator::t(
                                                        state.locale,
                                                        terms::REGISTER_TITLE,
                                                        None,
                                                    )
                                                } else {
                                                    Translator::t(
                                                        state.locale,
                                                        terms::LOGIN_TITLE,
                                                        None,
                                                    )
                                                })
                                                .w_full()
                                                .text_center()
                                                .cx_when(layout.is_narrow, |t| t.text_base())
                                                .theme(theme),
                                            ),
                                    ),
                            )
                            .child(
                                CardContent::new()
                                    .size(CardSize::Sm)
                                    .w_full()
                                    .min_w_0()
                                    .px(px(layout.card_px))
                                    .child(if is_register {
                                        Self::render_register_form(
                                            state,
                                            inputs,
                                            layout.form_gap,
                                            theme,
                                            cx,
                                        )
                                        .into_any_element()
                                    } else {
                                        Self::render_login_form(
                                            state,
                                            inputs,
                                            layout.form_gap,
                                            theme,
                                            cx,
                                        )
                                        .into_any_element()
                                    }),
                            )
                            .child(
                                CardFooter::new()
                                    .size(CardSize::Sm)
                                    .w_full()
                                    .min_w_0()
                                    .px(px(layout.card_px))
                                    .justify_center()
                                    .theme(theme)
                                    .child(
                                        Button::new("auth-switch-mode-btn")
                                            .link()
                                            .size(ButtonSize::Sm)
                                            .disabled(state.is_busy)
                                            .max_w_full()
                                            .flex_shrink(1.0)
                                            .whitespace_normal()
                                            .h_auto()
                                            .min_h(px(32.0))
                                            .py(px(4.0))
                                            .px(px(8.0))
                                            .text_center()
                                            .label(if is_register {
                                                Translator::t(
                                                    state.locale,
                                                    terms::ALREADY_HAVE_ACCOUNT,
                                                    None,
                                                )
                                            } else {
                                                Translator::t(
                                                    state.locale,
                                                    terms::CREATE_ACCOUNT,
                                                    None,
                                                )
                                            })
                                            .theme(theme)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.toggle_auth_mode(cx);
                                            })),
                                    ),
                            ),
                    )
                    // On short screens, flow footer directly under the card inside the scroll container
                    .children((!layout.center_vertically).then(|| {
                        div()
                            .w_full()
                            .mt(px(16.0))
                            .child(Self::render_about_footer(state, theme))
                    })),
            )
            // On taller screens, anchor footer cleanly at the bottom of the viewport
            .children(layout.center_vertically.then(|| {
                div()
                    .w_full()
                    .flex_shrink_0()
                    .pt(px(6.0))
                    .pb(px(14.0) + state.safe_area.total_bottom_px())
                    .pl(px(layout.outer_px) + state.safe_area.left_px())
                    .pr(px(layout.outer_px) + state.safe_area.right_px())
                    .child(Self::render_about_footer(state, theme))
            }))
    }

    fn render_about_footer(state: &AppState, theme: &SemanticTheme) -> impl IntoElement {
        let about_text = umuryango_i18n::t(state.locale, "auth.about_footer");
        let built_by_text = umuryango_i18n::t(state.locale, "auth.built_by");
        let link_hover = theme.primary_hover;

        div()
            .id("auth-app-footer")
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(2.0))
            .text_xs()
            .text_color(theme.text_muted)
            .child(
                div()
                    .w_full()
                    .max_w(px(AUTH_CARD_MAX_WIDTH_PX))
                    .text_center()
                    .child(about_text),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .justify_center()
                    .gap(px(4.0))
                    .child(built_by_text)
                    .child(
                        div()
                            .id("auth-author-github-link")
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.accent_primary)
                            .underline()
                            .cursor_pointer()
                            .hover(move |s| s.text_color(link_hover))
                            .on_click(|_, _, cx| {
                                cx.open_url(AUTHOR_GITHUB_URL);
                            })
                            .child(AUTHOR_NAME),
                    ),
            )
    }

    fn render_login_form(
        state: &AppState,
        inputs: &AuthInputs,
        form_gap: f32,
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
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(form_gap))
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
        form_gap: f32,
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
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(form_gap))
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
            .loading(is_busy)
            .w_full()
            .theme(theme)
            .on_click(cx.listener(move |this, _, _, cx| on_submit(this, cx)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use umuryango_i18n::Locale;

    #[test]
    fn small_phone_layout_uses_compact_padding_and_mobile_logo() {
        let layout = AuthResponsiveLayout::compute(375.0, 667.0, SafeAreaInsets::zero(), false);
        assert!(layout.is_mobile);
        assert!(layout.is_narrow);
        assert!(layout.center_vertically);
        assert_eq!(layout.outer_px, 16.0);
        assert_eq!(layout.card_px, 16.0);
        assert_eq!(layout.logo_size, 48.0);
    }

    #[test]
    fn ultra_narrow_phone_reduces_outer_gutter() {
        let layout = AuthResponsiveLayout::compute(320.0, 568.0, SafeAreaInsets::zero(), true);
        assert!(layout.is_mobile);
        assert!(layout.is_narrow);
        assert!(!layout.center_vertically);
        assert_eq!(layout.outer_px, 12.0);
        assert_eq!(layout.card_px, 16.0);
    }

    #[test]
    fn short_viewport_or_open_ime_disables_vertical_centering_to_prevent_top_clipping() {
        let mut insets = SafeAreaInsets::zero();
        insets.ime_height = 280.0;
        let layout = AuthResponsiveLayout::compute(390.0, 844.0, insets, true);
        assert!(!layout.center_vertically);
    }

    #[test]
    fn desktop_layout_preserves_generous_spacing_and_full_logo() {
        let layout = AuthResponsiveLayout::compute(1280.0, 800.0, SafeAreaInsets::zero(), false);
        assert!(!layout.is_mobile);
        assert!(!layout.is_narrow);
        assert!(layout.center_vertically);
        assert_eq!(layout.outer_px, 24.0);
        assert_eq!(layout.card_px, 20.0);
        assert_eq!(layout.card_py, 24.0);
        assert_eq!(layout.logo_size, 64.0);
    }

    #[test]
    fn footer_localization_and_author_metadata_present_in_both_locales() {
        assert_eq!(AUTHOR_NAME, "Rwanda Bruno");
        assert_eq!(AUTHOR_GITHUB_URL, "https://github.com/brunorwanda4");
        assert!(umuryango_i18n::t(Locale::En, "auth.about_footer").contains("Umuryango Ledger"));
        assert!(umuryango_i18n::t(Locale::Rw, "auth.about_footer").contains("Umuryango Ledger"));
        assert_eq!(umuryango_i18n::t(Locale::En, "auth.built_by"), "Built by");
        assert_eq!(umuryango_i18n::t(Locale::Rw, "auth.built_by"), "Yakozwe na");
    }
}

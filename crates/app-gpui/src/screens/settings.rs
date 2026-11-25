use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};

use crate::RootView;
use crate::components::icon::{AppIcon, BrandMark, IconSize, IconTone, app_icon, brand_mark};
use crate::state::AppState;
use crate::theme::{SemanticTheme, hex_to_hsla};

pub struct SettingsScreen;

impl SettingsScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let locale = state.locale;
        let active_family = state
            .active_family_name
            .clone()
            .unwrap_or_else(|| umuryango_i18n::t(locale, "settings.none_family"));

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
            // Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .mb(px(20.0))
                    .child(app_icon(
                        AppIcon::Settings,
                        IconSize::Lg,
                        IconTone::AccentPrimary,
                        theme,
                    ))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(umuryango_i18n::t(locale, "settings.theme_appearance")),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(umuryango_i18n::t(locale, "settings.theme_subtitle")),
                            ),
                    ),
            )
            // Section 0: Account & Session (real authenticated user + Logout)
            .child(
                div()
                    .p(px(20.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .mb(px(20.0))
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(umuryango_i18n::t(locale, "settings.account_section")),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_secondary)
                                    .child(format!(
                                        "{}: {} ({})",
                                        umuryango_i18n::t(locale, "settings.signed_in_as"),
                                        state.user_name,
                                        state.user_phone
                                    )),
                            )
                            .child(div().text_xs().text_color(theme.text_muted).child(format!(
                                "{}: {}",
                                umuryango_i18n::t(locale, "settings.active_family_label"),
                                active_family
                            )))
                            .child(div().text_xs().text_color(theme.text_muted).child(format!(
                                "{}: {}",
                                umuryango_i18n::t(locale, "settings.api_server_label"),
                                state.api_base_url
                            ))),
                    )
                    .child(
                        div()
                            .id("settings-logout-btn")
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(14.0))
                            .py(px(8.0))
                            .bg(theme.expense_color)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.handle_logout(window, cx);
                            }))
                            .child(app_icon(
                                AppIcon::Close,
                                IconSize::Xs,
                                IconTone::OnPrimaryButton,
                                theme,
                            ))
                            .child(umuryango_i18n::t(locale, "auth.logout")),
                    ),
            )
            // Section 1: Mode Switcher (Light / Dark)
            .child(
                div()
                    .p(px(20.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .mb(px(20.0))
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .mb(px(8.0))
                            .child(umuryango_i18n::t(locale, "settings.appearance_mode")),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .id("settings-light-mode-card")
                                    .flex_1()
                                    .p(px(14.0))
                                    .bg(if !state.theme_mode.is_dark() {
                                        theme.accent_primary
                                    } else {
                                        theme.bg_canvas
                                    })
                                    .text_color(if !state.theme_mode.is_dark() {
                                        theme.bg_canvas
                                    } else {
                                        theme.text_primary
                                    })
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.set_theme_mode(false, cx);
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(app_icon(
                                                AppIcon::Sun,
                                                IconSize::Sm,
                                                if !state.theme_mode.is_dark() {
                                                    IconTone::OnPrimaryButton
                                                } else {
                                                    IconTone::Default
                                                },
                                                theme,
                                            ))
                                            .child(
                                                div()
                                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                                    .text_sm()
                                                    .child(umuryango_i18n::t(
                                                        locale,
                                                        "settings.light_mode",
                                                    )),
                                            ),
                                    )
                                    .child(div().text_xs().mt(px(4.0)).child(umuryango_i18n::t(
                                        locale,
                                        "settings.light_theme_desc",
                                    ))),
                            )
                            .child(
                                div()
                                    .id("settings-dark-mode-card")
                                    .flex_1()
                                    .p(px(14.0))
                                    .bg(if state.theme_mode.is_dark() {
                                        theme.accent_primary
                                    } else {
                                        theme.bg_canvas
                                    })
                                    .text_color(if state.theme_mode.is_dark() {
                                        theme.bg_canvas
                                    } else {
                                        theme.text_primary
                                    })
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.set_theme_mode(true, cx);
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(8.0))
                                            .child(app_icon(
                                                AppIcon::Moon,
                                                IconSize::Sm,
                                                if state.theme_mode.is_dark() {
                                                    IconTone::OnPrimaryButton
                                                } else {
                                                    IconTone::Default
                                                },
                                                theme,
                                            ))
                                            .child(
                                                div()
                                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                                    .text_sm()
                                                    .child(umuryango_i18n::t(
                                                        locale,
                                                        "settings.dark_mode",
                                                    )),
                                            ),
                                    )
                                    .child(div().text_xs().mt(px(4.0)).child(umuryango_i18n::t(
                                        locale,
                                        "settings.dark_theme_desc",
                                    ))),
                            ),
                    ),
            )
            // Section 2: Brand Palette
            .child(
                div()
                    .p(px(20.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .mb(px(20.0))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .mb(px(12.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .text_base()
                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(umuryango_i18n::t(
                                                locale,
                                                "settings.brand_palette",
                                            )),
                                    )
                                    .child(
                                        div()
                                            .px(px(10.0))
                                            .py(px(4.0))
                                            .bg(theme.bg_elevated)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .text_xs()
                                            .text_color(theme.accent_secondary)
                                            .child(umuryango_i18n::t(
                                                locale,
                                                "settings.persisted_sqlite",
                                            )),
                                    ),
                            )
                            .child(
                                div()
                                    .id("settings-reset-palette-btn")
                                    .cursor_pointer()
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .bg(theme.accent_primary)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.bg_canvas)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.reset_theme_palette(cx);
                                    }))
                                    .child(umuryango_i18n::t(locale, "settings.reset_palette")),
                            ),
                    )
                    .child(
                        div()
                            .grid()
                            .grid_cols(5)
                            .gap(px(12.0))
                            .child(Self::render_color_token_card(
                                "primary",
                                &state.theme_config.primary,
                                "Main Brand Primary",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "primary_hover",
                                &state.theme_config.primary_hover,
                                "Hover / Active",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "primary_pressed",
                                &state.theme_config.primary_pressed,
                                "Pressed / Badges",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "primary_deep",
                                &state.theme_config.primary_deep,
                                "Deep Crimson",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "dark_canvas",
                                &state.theme_config.dark_canvas,
                                "Dark Canvas",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "dark_surface",
                                &state.theme_config.dark_surface,
                                "Dark Surface",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "text_muted",
                                &state.theme_config.text_muted,
                                "Text Muted",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "light_border",
                                &state.theme_config.light_border,
                                "Light Border",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "light_canvas",
                                &state.theme_config.light_canvas,
                                "Light Canvas",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "light_surface",
                                &state.theme_config.light_surface,
                                "Light Surface",
                                theme,
                            )),
                    ),
            )
            // Section 3: About
            .child(
                div()
                    .p(px(20.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.0))
                    .child(brand_mark(BrandMark::AppIcon, 54.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(umuryango_i18n::t(locale, "settings.about_app")),
                            )
                            .child(div().text_xs().text_color(theme.text_muted).child(format!(
                                "v{} • PostgreSQL + SQLite WAL",
                                env!("CARGO_PKG_VERSION")
                            ))),
                    ),
            )
    }

    fn render_color_token_card(
        token_name: &str,
        hex_val: &str,
        role: &str,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let color = hex_to_hsla(hex_val);

        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .p(px(10.0))
            .bg(theme.bg_canvas)
            .border_1()
            .border_color(theme.border_subtle)
            .child(div().w_full().h(px(40.0)).bg(color))
            .child(
                div()
                    .text_xs()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .text_color(theme.text_primary)
                    .child(token_name.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .font_family(theme.font_family)
                    .text_color(theme.accent_primary)
                    .child(hex_val.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(role.to_string()),
            )
    }
}

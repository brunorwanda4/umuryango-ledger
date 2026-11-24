use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
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
                            .child(div().text_xs().text_color(theme.text_muted).child(
                                umuryango_i18n::t(locale, "settings.theme_subtitle"),
                            )),
                    ),
            )
            // Section 1: Mode Switcher (Light / Dark)
            .child(
                div()
                    .p(px(20.0))
                    .rounded(px(12.0))
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
                            // Light mode card
                            .child(
                                div()
                                    .id("settings-light-mode-card")
                                    .flex_1()
                                    .p(px(14.0))
                                    .rounded(px(8.0))
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
                                                    .child("Light Mode (Umucyo)"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .mt(px(4.0))
                                            .child(umuryango_i18n::t(locale, "settings.light_theme_desc")),
                                    ),
                            )
                            // Dark mode card
                            .child(
                                div()
                                    .id("settings-dark-mode-card")
                                    .flex_1()
                                    .p(px(14.0))
                                    .rounded(px(8.0))
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
                                                    .child("Dark Mode (Umwijima)"),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .mt(px(4.0))
                                            .child(umuryango_i18n::t(locale, "settings.dark_theme_desc")),
                                    ),
                            ),
                    ),
            )
            // Section 2: Customizable Brand Palette
            .child(
                div()
                    .p(px(20.0))
                    .rounded(px(12.0))
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
                                            .child(umuryango_i18n::t(locale, "settings.brand_palette")),
                                    )
                                    .child(
                                        div()
                                            .px(px(10.0))
                                            .py(px(4.0))
                                            .rounded(px(6.0))
                                            .bg(theme.bg_elevated)
                                            .border_1()
                                            .border_color(theme.border_subtle)
                                            .text_xs()
                                            .text_color(theme.accent_secondary)
                                            .child(umuryango_i18n::t(locale, "settings.persisted_sqlite")),
                                    ),
                            )
                            .child(
                                div()
                                    .id("settings-reset-palette-btn")
                                    .cursor_pointer()
                                    .px(px(12.0))
                                    .py(px(6.0))
                                    .rounded(px(6.0))
                                    .bg(theme.accent_primary)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(crate::theme::hex_to_hsla("#ffffff"))
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
                                "Main Brand Primary (#ba181b)",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "primary_hover",
                                &state.theme_config.primary_hover,
                                "Hover / Active (#e5383b)",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "primary_pressed",
                                &state.theme_config.primary_pressed,
                                "Pressed / Badges (#a4161a)",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "primary_deep",
                                &state.theme_config.primary_deep,
                                "Deep Crimson / Elevated (#660708)",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "dark_canvas",
                                &state.theme_config.dark_canvas,
                                "Dark Canvas BG (#0b090a)",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "dark_surface",
                                &state.theme_config.dark_surface,
                                "Dark Surface / Cards (#161a1d)",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "text_muted",
                                &state.theme_config.text_muted,
                                "Text Muted / Subtle (#b1a7a6)",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "light_border",
                                &state.theme_config.light_border,
                                "Light Border / Headers (#d3d3d3)",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "light_canvas",
                                &state.theme_config.light_canvas,
                                "Light Canvas BG (#f5f3f4)",
                                theme,
                            ))
                            .child(Self::render_color_token_card(
                                "light_surface",
                                &state.theme_config.light_surface,
                                "Light Surface / Cards (#ffffff)",
                                theme,
                            )),
                    ),
            )
            // Section 3: Typography & Offline Storage Status
            .child(
                div()
                    .p(px(20.0))
                    .rounded(px(12.0))
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
                            .child(umuryango_i18n::t(locale, "settings.typography_sqlite")),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("• Font Family: Archivo (Regular, Medium, SemiBold, Bold bundled & registered via text_system)")
                            .child("• Local Storage: umuryango_offline.db (WAL mode enabled, full offline caching of wallets & sync queue)")
                            .child("• Idempotency: All offline mutations assigned unique UUIDs to guarantee zero duplication on sync"),
                    ),
            )
            // Section 4: About Umuryango Ledger
            .child(
                div()
                    .p(px(20.0))
                    .rounded(px(12.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.0))
                    .child(
                        gpui_kit::img("crates/app-gpui/assets/icons/icon-256.png")
                            .w(px(54.0))
                            .h(px(54.0))
                            .rounded(px(12.0))
                            .border_1()
                            .border_color(theme.border_subtle),
                    )
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
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("Version 0.1.0 • Multi-tenant Family & Personal Accountability"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.accent_secondary)
                                    .child("Built with Rust, GPUI, Actix-web, SQLite & Next.js"),
                            ),
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
            .rounded(px(8.0))
            .bg(theme.bg_canvas)
            .border_1()
            .border_color(theme.border_subtle)
            // Swatch
            .child(div().w_full().h(px(40.0)).rounded(px(6.0)).bg(color))
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

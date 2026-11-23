use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_i18n::Locale;

use crate::state::AppState;
use crate::theme::{hex_to_hsla, SemanticTheme};

pub struct SettingsScreen;

impl SettingsScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let is_rw = state.locale == Locale::Rw;

        div()
            .id(id)
            .flex_1()
            .h_full()
            .p(px(24.0))
            .overflow_y_scroll()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            // Header
            .child(
                div()
                    .flex()
                    .flex_col()
                    .mb(px(20.0))
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(if is_rw { "Igenamiterere ry'Insanganyamatsiko" } else { "Theme & Appearance Settings" }),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(if is_rw {
                                "Hindura amabara, umwijima/umucyo n'inyandiko ya Archivo"
                            } else {
                                "Customize runtime palette, Light/Dark modes, and Archivo typography"
                            }),
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
                            .child(if is_rw { "Umucyo n'Umwijima (Appearance Mode)" } else { "Appearance Mode" }),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(12.0))
                            // Light mode card
                            .child(
                                div()
                                    .flex_1()
                                    .p(px(14.0))
                                    .rounded(px(8.0))
                                    .bg(if !state.theme_mode.is_dark() { theme.accent_primary } else { theme.bg_canvas })
                                    .text_color(if !state.theme_mode.is_dark() { theme.bg_canvas } else { theme.text_primary })
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .cursor_pointer()
                                    .child(
                                        div()
                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                            .text_sm()
                                            .child("☀️ Light Mode (Umucyo)"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .mt(px(4.0))
                                            .child("#fdf0d5 warm cream canvas with crisp contrast"),
                                    ),
                            )
                            // Dark mode card
                            .child(
                                div()
                                    .flex_1()
                                    .p(px(14.0))
                                    .rounded(px(8.0))
                                    .bg(if state.theme_mode.is_dark() { theme.accent_primary } else { theme.bg_canvas })
                                    .text_color(if state.theme_mode.is_dark() { theme.bg_canvas } else { theme.text_primary })
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .cursor_pointer()
                                    .child(
                                        div()
                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                            .text_sm()
                                            .child("🌙 Dark Mode (Umwijima)"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .mt(px(4.0))
                                            .child("#003049 prussian navy surface with warm accents"),
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
                                    .text_base()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(if is_rw { "Amabara y'Insanganyamatsiko (Palette Tokens)" } else { "Brand Palette Tokens" }),
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
                                    .child(if is_rw { "Byasubijwe mu mwimerere" } else { "Persisted to SQLite" }),
                            ),
                    )
                    .child(
                        div()
                            .grid()
                            .grid_cols(5)
                            .gap(px(12.0))
                            .child(Self::render_color_token_card("crimson_deep", &state.theme_config.crimson_deep, "Primary Brand / Expense", theme))
                            .child(Self::render_color_token_card("crimson_bright", &state.theme_config.crimson_bright, "Badges / Alert Accent", theme))
                            .child(Self::render_color_token_card("cream_warm", &state.theme_config.cream_warm, "Light Canvas / Dark Text", theme))
                            .child(Self::render_color_token_card("prussian_navy", &state.theme_config.prussian_navy, "Dark Canvas / Light Text", theme))
                            .child(Self::render_color_token_card("steel_blue", &state.theme_config.steel_blue, "Income / Custody Balance", theme)),
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
                            .child(if is_rw { "Inyandiko n'Ububiko bwa SQLite" } else { "Typography & Offline SQLite Engine" }),
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
                                    .child(if is_rw { "Ibyerekeye Umuryango Ledger" } else { "About Umuryango Ledger" }),
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
                                    .child("🇷🇼 Built with Rust, GPUI, Actix-web, SQLite & Next.js"),
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

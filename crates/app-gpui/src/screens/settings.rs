use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};

use crate::RootView;
use crate::common::{
    Badge, Button, ButtonSize, Card, CardAction, CardContent, CardHeader, CardSize, CardTitle,
    Item, ItemContent, ItemSize,
};
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
                                    .font_weight(FontWeight::BOLD)
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
                Card::new()
                    .size(CardSize::Sm)
                    .gap(px(10.0))
                    .py(px(18.0))
                    .mb(px(20.0))
                    .border_1()
                    .border_color(theme.border_subtle)
                    .theme(theme)
                    .child(
                        CardHeader::new()
                            .size(CardSize::Sm)
                            .action(
                                CardAction::new().child(
                                    Button::new("settings-logout-btn")
                                        .destructive()
                                        .size(ButtonSize::Sm)
                                        .icon_start(AppIcon::Close)
                                        .label(umuryango_i18n::t(locale, "auth.logout"))
                                        .theme(theme)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.handle_logout(window, cx);
                                        })),
                                ),
                            )
                            .theme(theme)
                            .child(
                                CardTitle::new(umuryango_i18n::t(
                                    locale,
                                    "settings.account_section",
                                ))
                                .text_base()
                                .theme(theme),
                            ),
                    )
                    .child(
                        CardContent::new()
                            .size(CardSize::Sm)
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
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
                    ),
            )
            // Section 1: Mode Switcher (Light / Dark)
            .child(
                Card::new()
                    .size(CardSize::Sm)
                    .gap(px(12.0))
                    .py(px(18.0))
                    .mb(px(20.0))
                    .border_1()
                    .border_color(theme.border_subtle)
                    .theme(theme)
                    .child(
                        CardHeader::new().size(CardSize::Sm).theme(theme).child(
                            CardTitle::new(umuryango_i18n::t(locale, "settings.appearance_mode"))
                                .text_base()
                                .theme(theme),
                        ),
                    )
                    .child(
                        CardContent::new().size(CardSize::Sm).child(
                            div()
                                .flex()
                                .gap(px(12.0))
                                .child(
                                    Item::new()
                                        .id("settings-light-mode-card")
                                        .outline()
                                        .size(ItemSize::Sm)
                                        .flex_1()
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
                                        .theme(theme)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.set_theme_mode(false, cx);
                                        }))
                                        .child(
                                            ItemContent::new()
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
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_sm()
                                                                .child(umuryango_i18n::t(
                                                                    locale,
                                                                    "settings.light_mode",
                                                                )),
                                                        ),
                                                )
                                                .child(div().text_xs().child(umuryango_i18n::t(
                                                    locale,
                                                    "settings.light_theme_desc",
                                                ))),
                                        ),
                                )
                                .child(
                                    Item::new()
                                        .id("settings-dark-mode-card")
                                        .outline()
                                        .size(ItemSize::Sm)
                                        .flex_1()
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
                                        .theme(theme)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.set_theme_mode(true, cx);
                                        }))
                                        .child(
                                            ItemContent::new()
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
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_sm()
                                                                .child(umuryango_i18n::t(
                                                                    locale,
                                                                    "settings.dark_mode",
                                                                )),
                                                        ),
                                                )
                                                .child(div().text_xs().child(umuryango_i18n::t(
                                                    locale,
                                                    "settings.dark_theme_desc",
                                                ))),
                                        ),
                                ),
                        ),
                    ),
            )
            // Section 2: Brand Palette
            .child(
                Card::new()
                    .size(CardSize::Sm)
                    .gap(px(14.0))
                    .py(px(18.0))
                    .mb(px(20.0))
                    .border_1()
                    .border_color(theme.border_subtle)
                    .theme(theme)
                    .child(
                        CardHeader::new()
                            .size(CardSize::Sm)
                            .action(
                                CardAction::new().child(
                                    Button::new("settings-reset-palette-btn")
                                        .size(ButtonSize::Xs)
                                        .label(umuryango_i18n::t(locale, "settings.reset_palette"))
                                        .theme(theme)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.reset_theme_palette(cx);
                                        })),
                                ),
                            )
                            .theme(theme)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(
                                        CardTitle::new(umuryango_i18n::t(
                                            locale,
                                            "settings.brand_palette",
                                        ))
                                        .text_base()
                                        .theme(theme),
                                    )
                                    .child(
                                        Badge::new(umuryango_i18n::t(
                                            locale,
                                            "settings.persisted_sqlite",
                                        ))
                                        .secondary()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .theme(theme),
                                    ),
                            ),
                    )
                    .child(
                        CardContent::new().size(CardSize::Sm).child(
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
                    ),
            )
            // Section 3: About
            .child(
                Card::new()
                    .size(CardSize::Sm)
                    .py(px(18.0))
                    .border_1()
                    .border_color(theme.border_subtle)
                    .theme(theme)
                    .child(
                        CardContent::new()
                            .size(CardSize::Sm)
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
                                        CardTitle::new(umuryango_i18n::t(
                                            locale,
                                            "settings.about_app",
                                        ))
                                        .text_base()
                                        .theme(theme),
                                    )
                                    .child(div().text_xs().text_color(theme.text_muted).child(
                                        format!(
                                            "v{} • PostgreSQL + SQLite WAL",
                                            env!("CARGO_PKG_VERSION")
                                        ),
                                    )),
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
            .bg(theme.bg_canvas)
            .border_1()
            .border_color(theme.border_subtle)
            .child(div().w_full().h(px(40.0)).bg(color))
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
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

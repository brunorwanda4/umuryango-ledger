use gpui_kit::prelude::*;
use gpui_kit::{ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::Locale;
use umuryango_storage_local::SyncStatus;

use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::components::lang_toggle::LangToggle;
use crate::components::scope_switcher::ScopeSwitcher;
use crate::state::{AppState, Screen};
use crate::theme::SemanticTheme;

pub struct Sidebar;

impl Sidebar {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let screens = [
            (Screen::Dashboard, AppIcon::Dashboard, state.locale),
            (Screen::Transactions, AppIcon::Transactions, state.locale),
            (Screen::Reports, AppIcon::Reports, state.locale),
            (Screen::Projects, AppIcon::Projects, state.locale),
            (Screen::Loans, AppIcon::Loans, state.locale),
            (Screen::Plans, AppIcon::FuturePlans, state.locale),
            (Screen::Members, AppIcon::Members, state.locale),
            (Screen::Settings, AppIcon::Settings, state.locale),
        ];

        let (sync_icon, sync_label, sync_tone, sync_color) = match state.sync_status {
            SyncStatus::Synced => {
                let lbl = if state.locale == Locale::Rw {
                    "Bihujwe"
                } else {
                    "Synced"
                };
                (
                    AppIcon::SyncOnline,
                    lbl.to_string(),
                    IconTone::Success,
                    theme.income_color,
                )
            }
            SyncStatus::PendingSync(n) => {
                let text = match state.locale {
                    Locale::Rw => format!("Biri ku murongo ({})", n),
                    Locale::En => format!("Pending ({})", n),
                };
                (
                    AppIcon::SyncPending,
                    text,
                    IconTone::Warning,
                    theme.accent_secondary,
                )
            }
            SyncStatus::Offline => {
                let lbl = if state.locale == Locale::Rw {
                    "Bidakorera ku murongo"
                } else {
                    "Offline"
                };
                (
                    AppIcon::SyncOffline,
                    lbl.to_string(),
                    IconTone::Error,
                    theme.expense_color,
                )
            }
        };

        div()
            .id(id)
            .w(px(240.0))
            .h_full()
            .p(px(16.0))
            .flex()
            .flex_col()
            .justify_between()
            .bg(theme.bg_canvas)
            .border_r_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            // Top Section: Brand + Scope Switcher + Sync Badge
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    // Brand title with custom emblem & Notification Bell
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(
                                        gpui_kit::img("crates/app-gpui/assets/icons/icon-256.png")
                                            .w(px(30.0))
                                            .h(px(30.0))
                                            .rounded(px(7.0)),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(1.0))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                                    .text_color(theme.text_primary)
                                                    .child("Umuryango"),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_muted)
                                                    .child("Finance Ledger"),
                                            ),
                                    ),
                            )
                            // Notification Bell Icon with dynamic Badge
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .p(px(6.0))
                                    .rounded(px(8.0))
                                    .bg(if state.unread_notifications_count > 0 {
                                        theme.bg_surface
                                    } else {
                                        theme.bg_canvas
                                    })
                                    .border_1()
                                    .border_color(if state.unread_notifications_count > 0 {
                                        theme.accent_primary
                                    } else {
                                        theme.border_subtle
                                    })
                                    .cursor_pointer()
                                    .child(app_icon(
                                        AppIcon::Bell,
                                        IconSize::Sm,
                                        if state.unread_notifications_count > 0 {
                                            IconTone::AccentPrimary
                                        } else {
                                            IconTone::Muted
                                        },
                                        theme,
                                    ))
                                    .children(if state.unread_notifications_count > 0 {
                                        Some(
                                            div()
                                                .px(px(6.0))
                                                .py(px(2.0))
                                                .rounded(px(10.0))
                                                .bg(theme.accent_primary)
                                                .text_xs()
                                                .font_weight(gpui_kit::FontWeight::BOLD)
                                                .text_color(theme.bg_canvas)
                                                .child(
                                                    state.unread_notifications_count.to_string(),
                                                ),
                                        )
                                    } else {
                                        None
                                    }),
                            ),
                    )
                    // Scope switcher
                    .child(ScopeSwitcher::render(
                        "sidebar-scope-switcher",
                        &state.active_scope,
                        state.locale,
                        theme,
                    ))
                    // Sync status badge
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(sync_color)
                            .child(app_icon(sync_icon, IconSize::Xs, sync_tone, theme))
                            .child(sync_label),
                    ),
            )
            // Center Section: Nav links
            .child(div().my(px(16.0)).flex().flex_col().gap(px(4.0)).children(
                screens.into_iter().map(|(screen, icon, loc)| {
                    let is_active = state.current_screen == screen;
                    let label = screen.label(loc);
                    let bg = if is_active {
                        theme.accent_primary
                    } else {
                        theme.bg_canvas
                    };
                    let text_color = if is_active {
                        theme.bg_canvas
                    } else {
                        theme.text_primary
                    };
                    let icon_tone = if is_active {
                        IconTone::OnPrimaryButton
                    } else {
                        IconTone::Default
                    };

                    div()
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .py(px(8.0))
                        .px(px(12.0))
                        .rounded(px(8.0))
                        .bg(bg)
                        .cursor_pointer()
                        .child(app_icon(icon, IconSize::Md, icon_tone, theme))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(if is_active {
                                    gpui_kit::FontWeight::SEMIBOLD
                                } else {
                                    gpui_kit::FontWeight::NORMAL
                                })
                                .text_color(text_color)
                                .child(label),
                        )
                }),
            ))
            // Bottom Section: Language & Light/Dark Theme Toggles
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .pt(px(12.0))
                    .border_t_1()
                    .border_color(theme.border_subtle)
                    // Language toggle
                    .child(LangToggle::render(
                        "sidebar-lang-toggle",
                        state.locale,
                        theme,
                    ))
                    // Mode toggle
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_elevated)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.text_primary)
                            .cursor_pointer()
                            .child(app_icon(
                                if state.theme_mode.is_dark() {
                                    AppIcon::Sun
                                } else {
                                    AppIcon::Moon
                                },
                                IconSize::Xs,
                                IconTone::Default,
                                theme,
                            ))
                            .child(if state.theme_mode.is_dark() {
                                "Light"
                            } else {
                                "Dark"
                            }),
                    ),
            )
    }

    /// Renders bottom navigation bar for mobile viewports
    pub fn render_mobile_bottom_bar(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let mobile_screens = [
            (Screen::Dashboard, AppIcon::Dashboard),
            (Screen::Transactions, AppIcon::Transactions),
            (Screen::Reports, AppIcon::Reports),
            (Screen::Projects, AppIcon::Projects),
            (Screen::Settings, AppIcon::Settings),
        ];

        div()
            .id(id)
            .w_full()
            .py(px(8.0))
            .px(px(12.0))
            .flex()
            .flex_row()
            .justify_around()
            .items_center()
            .bg(theme.bg_surface)
            .border_t_1()
            .border_color(theme.border_subtle)
            .children(mobile_screens.into_iter().map(|(screen, icon)| {
                let is_active = state.current_screen == screen;
                let tone = if is_active {
                    IconTone::AccentPrimary
                } else {
                    IconTone::Muted
                };
                let color = if is_active {
                    theme.accent_primary
                } else {
                    theme.text_muted
                };
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(2.0))
                    .cursor_pointer()
                    .child(app_icon(icon, IconSize::Lg, tone, theme))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(if is_active {
                                gpui_kit::FontWeight::BOLD
                            } else {
                                gpui_kit::FontWeight::NORMAL
                            })
                            .text_color(color)
                            .child(screen.label(state.locale)),
                    )
            }))
    }
}

use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::SyncStatus;

use crate::RootView;
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
        cx: &Context<RootView>,
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
            SyncStatus::Synced => (
                AppIcon::SyncOnline,
                umuryango_i18n::t(state.locale, "common.synced"),
                IconTone::Success,
                theme.income_color,
            ),
            SyncStatus::PendingSync(n) => (
                AppIcon::SyncPending,
                umuryango_i18n::t_args(
                    state.locale,
                    "common.sync_pending",
                    &[("count", &n.to_string())],
                ),
                IconTone::Warning,
                theme.accent_secondary,
            ),
            SyncStatus::Offline => (
                AppIcon::SyncOffline,
                umuryango_i18n::t(state.locale, "common.offline"),
                IconTone::Error,
                theme.expense_color,
            ),
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
            // Top Section: Brand + Scope Switcher + Clickable Sync Badge
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
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
                                            .h(px(30.0)),
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
                                                    .child(state.user_name.clone()),
                                            ),
                                    ),
                            )
                            // Notification Bell Icon with dynamic Badge
                            .child(
                                div()
                                    .id("sidebar-bell-btn")
                                    .flex()
                                    .items_center()
                                    .gap(px(4.0))
                                    .p(px(6.0))
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
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.toggle_notifications(cx);
                                    }))
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
                        cx,
                    ))
                    // Clickable sync status badge (triggers real PostgreSQL refresh)
                    .child(
                        div()
                            .id("sidebar-sync-refresh-btn")
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(8.0))
                            .py(px(4.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(sync_color)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.refresh_active_scope(cx);
                            }))
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

                    let target_screen = screen;
                    div()
                        .id(format!("sidebar-nav-{:?}", screen))
                        .flex()
                        .items_center()
                        .gap(px(10.0))
                        .py(px(8.0))
                        .px(px(12.0))
                        .bg(bg)
                        .cursor_pointer()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.select_screen(target_screen, cx);
                        }))
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
            // Bottom Section: Language, Theme, and Logout
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .pt(px(12.0))
                    .border_t_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .child(LangToggle::render(
                                "sidebar-lang-toggle",
                                state.locale,
                                theme,
                                cx,
                            ))
                            .child(
                                div()
                                    .id("sidebar-theme-toggle")
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(10.0))
                                    .py(px(4.0))
                                    .bg(theme.bg_elevated)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .text_color(theme.text_primary)
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.toggle_theme(cx);
                                    }))
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
                                        umuryango_i18n::t(state.locale, "settings.light_mode")
                                    } else {
                                        umuryango_i18n::t(state.locale, "settings.dark_mode")
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .id("sidebar-logout-btn")
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap(px(6.0))
                            .py(px(6.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.expense_color)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.handle_logout(window, cx);
                            }))
                            .child(app_icon(
                                AppIcon::Close,
                                IconSize::Xs,
                                IconTone::Expense,
                                theme,
                            ))
                            .child(umuryango_i18n::t(state.locale, "auth.logout")),
                    ),
            )
    }

    /// Renders bottom navigation bar for mobile viewports
    pub fn render_mobile_bottom_bar(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
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
                let target_screen = screen;
                div()
                    .id(format!("mobile-nav-{:?}", screen))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(2.0))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.select_screen(target_screen, cx);
                    }))
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

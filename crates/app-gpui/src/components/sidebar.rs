use gpui_kit::{
    Context, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement, Styled, div, px,
};
use umuryango_core_domain::SyncStatus;

use crate::RootView;
use crate::common::{
    Badge, Button, ButtonSize, ButtonVariant, SidebarContent, SidebarFooter, SidebarHeader,
    SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem, SidebarSeparator,
};
use crate::components::icon::{AppIcon, BrandMark, IconSize, IconTone, app_icon, brand_mark};
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

        let (sync_icon, sync_label, sync_color) = match state.sync_status {
            SyncStatus::Synced => (
                AppIcon::SyncOnline,
                umuryango_i18n::t(state.locale, "common.synced"),
                theme.income_color,
            ),
            SyncStatus::PendingSync(n) => (
                AppIcon::SyncPending,
                umuryango_i18n::t_args(
                    state.locale,
                    "common.sync_pending",
                    &[("count", &n.to_string())],
                ),
                theme.accent_secondary,
            ),
            SyncStatus::Offline => (
                AppIcon::SyncOffline,
                umuryango_i18n::t(state.locale, "common.offline"),
                theme.expense_color,
            ),
        };

        div()
            .id(id)
            .w(px(248.0))
            .h_full()
            .p(px(12.0))
            .flex()
            .flex_col()
            .justify_between()
            .bg(theme.bg_canvas)
            .border_r_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            // Top Section: Brand + Scope Switcher + Clickable Sync Status
            .child(
                SidebarHeader::new()
                    .gap(px(12.0))
                    .p(px(4.0))
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
                                    .child(brand_mark(BrandMark::Logo, 32.0))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(1.0))
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::BOLD)
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
                            // Notification Bell Button with Unread Count Badge
                            .child(
                                Button::new("sidebar-bell-btn")
                                    .variant(if state.unread_notifications_count > 0 {
                                        ButtonVariant::Secondary
                                    } else {
                                        ButtonVariant::Outline
                                    })
                                    .size(if state.unread_notifications_count > 0 {
                                        ButtonSize::Xs
                                    } else {
                                        ButtonSize::IconXs
                                    })
                                    .icon_start(AppIcon::Bell)
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.toggle_notifications(cx);
                                    }))
                                    .children(if state.unread_notifications_count > 0 {
                                        Some(
                                            Badge::new(
                                                state.unread_notifications_count.to_string(),
                                            )
                                            .px(px(5.0))
                                            .py(px(1.0))
                                            .bg(theme.accent_primary)
                                            .text_color(theme.bg_canvas)
                                            .theme(theme),
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
                        Button::new("sidebar-sync-refresh-btn")
                            .outline()
                            .size(ButtonSize::Xs)
                            .icon_start(sync_icon)
                            .label(sync_label)
                            .w_full()
                            .justify_start()
                            .text_color(sync_color)
                            .theme(theme)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.refresh_active_scope(cx);
                            })),
                    ),
            )
            // Center Section: Nav links
            .child(
                SidebarContent::new("sidebar-nav-scroll")
                    .my(px(12.0))
                    .child(
                        SidebarMenu::new()
                            .gap(px(4.0))
                            .children(screens.into_iter().map(|(screen, icon, loc)| {
                                let is_active = state.current_screen == screen;
                                let label = screen.label(loc);
                                let target_screen = screen;

                                SidebarMenuItem::new().child(
                                    SidebarMenuButton::new(format!("sidebar-nav-{:?}", screen))
                                        .icon(icon)
                                        .label(label)
                                        .size(SidebarMenuButtonSize::Default)
                                        .active(is_active)
                                        .theme(theme)
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.select_screen(target_screen, cx);
                                        })),
                                )
                            })),
                    ),
            )
            // Bottom Section: Separator + Language, Theme, and Logout
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(SidebarSeparator::new().mx(px(0.0)).theme(theme))
                    .child(
                        SidebarFooter::new()
                            .p(px(4.0))
                            .gap(px(8.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .gap(px(8.0))
                                    .child(LangToggle::render(
                                        "sidebar-lang-toggle",
                                        state.locale,
                                        theme,
                                        cx,
                                    ))
                                    .child(
                                        Button::new("sidebar-theme-toggle")
                                            .outline()
                                            .size(ButtonSize::Sm)
                                            .icon_start(if state.theme_mode.is_dark() {
                                                AppIcon::Sun
                                            } else {
                                                AppIcon::Moon
                                            })
                                            .label(if state.theme_mode.is_dark() {
                                                umuryango_i18n::t(
                                                    state.locale,
                                                    "settings.light_mode",
                                                )
                                            } else {
                                                umuryango_i18n::t(
                                                    state.locale,
                                                    "settings.dark_mode",
                                                )
                                            })
                                            .theme(theme)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.toggle_theme(cx);
                                            })),
                                    ),
                            )
                            .child(
                                Button::new("sidebar-logout-btn")
                                    .destructive()
                                    .size(ButtonSize::Sm)
                                    .icon_start(AppIcon::Close)
                                    .label(umuryango_i18n::t(state.locale, "auth.logout"))
                                    .w_full()
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.handle_logout(window, cx);
                                    })),
                            ),
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
                Button::new(format!("mobile-nav-{:?}", screen))
                    .ghost()
                    .size(ButtonSize::Xs)
                    .h_auto()
                    .px(px(6.0))
                    .py(px(4.0))
                    .theme(theme)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.select_screen(target_screen, cx);
                    }))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(2.0))
                            .child(app_icon(icon, IconSize::Lg, tone, theme))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(if is_active {
                                        FontWeight::BOLD
                                    } else {
                                        FontWeight::NORMAL
                                    })
                                    .text_color(color)
                                    .child(screen.label(state.locale)),
                            ),
                    )
            }))
    }
}

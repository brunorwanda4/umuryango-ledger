use gpui_kit::{
    Context, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, div, px,
};
use umuryango_i18n::t;

use crate::RootView;
use crate::common::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuSeparator,
    DropdownMenuTrigger,
};
use crate::common::{Button, ButtonSize, PopoverAlign, PopoverSide};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{AppState, Screen};
use crate::theme::{SemanticTheme, ThemeMode};

pub struct Sidebar;

/// What a nav row does when clicked.
#[derive(Clone, Copy)]
enum NavAction {
    Go(Screen),
    Notifications,
    ToggleMoney,
}

impl Sidebar {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let loc = state.locale;
        let current = state.current_screen;

        let money_children = [
            (
                Screen::Transactions,
                AppIcon::Transactions,
                "nav.transactions",
            ),
            (Screen::Loans, AppIcon::Loans, "nav.loans"),
            (Screen::Plans, AppIcon::FuturePlans, "nav.plans"),
        ];
        let money_active = money_children.iter().any(|(s, _, _)| *s == current);
        let money_open = state.money_expanded;

        let mut money = div().flex().flex_col().gap(px(2.0)).child(nav_row(
            "sidebar-nav-money".to_string(),
            AppIcon::Transactions,
            t(loc, "nav.money"),
            NavAction::ToggleMoney,
            money_active,
            Some(if money_open {
                AppIcon::ChevronDown
            } else {
                AppIcon::ChevronRight
            }),
            theme,
            cx,
        ));

        if money_open {
            for (screen, icon, key) in money_children {
                money = money.child(div().pl(px(14.0)).child(go_row(
                    icon,
                    t(loc, key),
                    screen,
                    current,
                    theme,
                    cx,
                )));
            }
        }

        let family = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(go_row(
                AppIcon::Dashboard,
                t(loc, "nav.dashboard"),
                Screen::Dashboard,
                current,
                theme,
                cx,
            ))
            .child(money)
            .child(go_row(
                AppIcon::Projects,
                t(loc, "nav.projects"),
                Screen::Projects,
                current,
                theme,
                cx,
            ))
            .child(go_row(
                AppIcon::Reports,
                t(loc, "nav.reports"),
                Screen::Reports,
                current,
                theme,
                cx,
            ))
            .child(nav_row(
                "sidebar-nav-notifications".to_string(),
                AppIcon::Bell,
                t(loc, "nav.notifications"),
                NavAction::Notifications,
                false,
                None,
                theme,
                cx,
            ));

        let management = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(go_row(
                AppIcon::Members,
                t(loc, "nav.members"),
                Screen::Members,
                current,
                theme,
                cx,
            ))
            .child(go_row(
                AppIcon::Settings,
                t(loc, "nav.settings"),
                Screen::Settings,
                current,
                theme,
                cx,
            ));

        div()
            .id(id)
            .w(px(240.0))
            .h_full()
            .flex()
            .flex_col()
            .bg(theme.bg_canvas)
            .border_r_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            .child(
                div()
                    .id("sidebar-nav-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .px(px(12.0))
                    .pt(px(16.0))
                    .pb(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(24.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(section_label(t(loc, "nav.group_family"), theme))
                            .child(family),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(section_label(t(loc, "nav.group_management"), theme))
                            .child(management),
                    ),
            )
            .child(user_footer(state, theme, cx))
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

fn section_label(text: String, theme: &SemanticTheme) -> impl IntoElement {
    div()
        .px(px(10.0))
        .pb(px(6.0))
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(theme.text_muted)
        .child(text)
}

fn go_row(
    icon: AppIcon,
    label: String,
    screen: Screen,
    current: Screen,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    nav_row(
        format!("sidebar-nav-{:?}", screen),
        icon,
        label,
        NavAction::Go(screen),
        current == screen,
        None,
        theme,
        cx,
    )
}

fn nav_row(
    id: String,
    icon: AppIcon,
    label: String,
    action: NavAction,
    active: bool,
    trailing: Option<AppIcon>,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let tone = if active {
        IconTone::AccentPrimary
    } else {
        IconTone::Muted
    };
    let text = if active {
        theme.text_primary
    } else {
        theme.text_muted
    };
    let bg = if active {
        theme.bg_surface
    } else {
        theme.bg_canvas
    };

    div()
        .id(id)
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .h(px(36.0))
        .px(px(10.0))
        .bg(bg)
        .text_color(text)
        .text_sm()
        .font_weight(if active {
            FontWeight::MEDIUM
        } else {
            FontWeight::NORMAL
        })
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_surface))
        .on_click(cx.listener(move |this, _, _, cx| match action {
            NavAction::Go(screen) => this.select_screen(screen, cx),
            NavAction::Notifications => this.toggle_notifications(cx),
            NavAction::ToggleMoney => this.toggle_money_group(cx),
        }))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(10.0))
                .child(app_icon(icon, IconSize::Md, tone, theme))
                .child(label),
        )
        .children(trailing.map(|chev| app_icon(chev, IconSize::Sm, IconTone::Muted, theme)))
}

fn user_footer(
    state: &AppState,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let initial = state
        .user_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());
    let loc = state.locale;
    let menu_open = state.user_menu_open;
    let is_dark = state.theme_mode == ThemeMode::Dark;
    let hover_bg = theme.bg_surface;

    let trigger_row = div()
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(10.0))
        .p(px(8.0))
        .bg(if menu_open {
            theme.bg_surface
        } else {
            theme.bg_canvas
        })
        .hover(move |s| s.bg(hover_bg))
        .child(
            div()
                .size(px(32.0))
                .flex()
                .items_center()
                .justify_center()
                .bg(theme.accent_primary)
                .text_color(theme.bg_canvas)
                .text_sm()
                .font_weight(FontWeight::BOLD)
                .child(initial),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text_primary)
                        .truncate()
                        .child(state.user_name.clone()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .truncate()
                        .child(state.user_phone.clone()),
                ),
        )
        .child(app_icon(
            if menu_open {
                AppIcon::ChevronDown
            } else {
                AppIcon::ChevronUp
            },
            IconSize::Md,
            IconTone::Muted,
            theme,
        ));

    let trigger = DropdownMenuTrigger::new("sidebar-user-menu-trigger")
        .w_full()
        .child(trigger_row);

    let content = DropdownMenuContent::new()
        .side(PopoverSide::Top)
        .side_offset(8.0)
        .align(PopoverAlign::Start)
        .w(px(216.0))
        .theme(theme)
        .child(
            DropdownMenuGroup::new().child(
                DropdownMenuItem::new(
                    "sidebar-user-theme",
                    t(
                        loc,
                        if is_dark {
                            "common.light_mode"
                        } else {
                            "common.dark_mode"
                        },
                    ),
                )
                .icon(if is_dark { AppIcon::Sun } else { AppIcon::Moon })
                .theme(theme)
                .on_click(cx.listener(|this, _, _, cx| this.toggle_theme(cx))),
            ),
        )
        .child(DropdownMenuSeparator::new().theme(theme))
        .child(
            DropdownMenuItem::new("sidebar-user-logout", t(loc, "auth.logout"))
                .icon(AppIcon::ArrowRight)
                .destructive()
                .theme(theme)
                .on_click(cx.listener(|this, _, window, cx| this.handle_logout(window, cx))),
        );

    div()
        .border_t_1()
        .border_color(theme.border_subtle)
        .p(px(12.0))
        .child(
            DropdownMenu::new("sidebar-user-menu")
                .w_full()
                .open(menu_open)
                .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                    this.set_user_menu_open(*open, cx);
                }))
                .trigger(trigger)
                .content(content),
        )
}

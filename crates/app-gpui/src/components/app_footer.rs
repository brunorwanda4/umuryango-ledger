//! Mobile / Android bottom navigation footer (`AppFooter`).
//!
//! Provides one-handed thumb navigation on mobile viewports (`< 768px`):
//! 1. **Home** (`Screen::Dashboard`)
//! 2. **Money** (`Screen::Transactions` | `Loans` | `Plans`, with a contextual
//!    sub-switcher + `+` Quick Entry action bar when inside the Money group)
//! 3. **Projects** (`Screen::Projects` in Family scope; `Screen::Plans` in Personal scope)
//! 4. **Reports** (`Screen::Reports`)
//! 5. **More** (toggles the slide-over `Sidebar` drawer and surfaces unread alerts)

use gpui_kit::{
    Context, ElementId, FontWeight, InteractiveElement, IntoElement, MouseButton, ParentElement,
    StatefulInteractiveElement, Styled, div, px,
};
use umuryango_i18n::t;

use crate::RootView;
use crate::common::PresetRadius;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{AppState, Screen};
use crate::theme::SemanticTheme;

/// Action triggered when tapping a primary footer tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FooterTabAction {
    Navigate(Screen),
    OpenMoneyGroup,
    ToggleSidebar,
}

/// Resolved descriptor for one of the 5 primary mobile footer tabs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FooterTabSpec {
    pub id: &'static str,
    pub label: String,
    pub icon: AppIcon,
    pub is_active: bool,
    pub badge_count: usize,
    pub action: FooterTabAction,
}

/// Pure view-model for [`AppFooter`] enabling deterministic layout & state tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppFooterModel {
    pub tabs: [FooterTabSpec; 5],
    pub show_money_subbar: bool,
    pub is_family: bool,
}

impl AppFooterModel {
    pub fn compute(state: &AppState) -> Self {
        let loc = state.locale;
        let current = state.current_screen;
        let is_family = state.active_scope.is_family();
        let sidebar_open = state.sidebar_open;

        let in_money_group = if is_family {
            matches!(
                current,
                Screen::Transactions | Screen::Loans | Screen::Plans
            )
        } else {
            matches!(current, Screen::Transactions | Screen::Loans)
        };

        let (third_id, third_label, third_icon, third_screen) = if is_family {
            (
                "footer-tab-projects",
                t(loc, "nav.projects"),
                AppIcon::Projects,
                Screen::Projects,
            )
        } else {
            (
                "footer-tab-plans",
                t(loc, "nav.plans"),
                AppIcon::FuturePlans,
                Screen::Plans,
            )
        };

        let in_more_screens = matches!(current, Screen::Members | Screen::Settings);

        let tabs = [
            FooterTabSpec {
                id: "footer-tab-home",
                label: t(loc, "nav.home"),
                icon: AppIcon::Dashboard,
                is_active: !sidebar_open && current == Screen::Dashboard,
                badge_count: 0,
                action: FooterTabAction::Navigate(Screen::Dashboard),
            },
            FooterTabSpec {
                id: "footer-tab-money",
                label: t(loc, "nav.money"),
                icon: AppIcon::PersonalWallet,
                is_active: !sidebar_open && in_money_group,
                badge_count: 0,
                action: FooterTabAction::OpenMoneyGroup,
            },
            FooterTabSpec {
                id: third_id,
                label: third_label,
                icon: third_icon,
                is_active: !sidebar_open && current == third_screen,
                badge_count: 0,
                action: FooterTabAction::Navigate(third_screen),
            },
            FooterTabSpec {
                id: "footer-tab-reports",
                label: t(loc, "nav.reports"),
                icon: AppIcon::Reports,
                is_active: !sidebar_open && current == Screen::Reports,
                badge_count: 0,
                action: FooterTabAction::Navigate(Screen::Reports),
            },
            FooterTabSpec {
                id: "footer-tab-more",
                label: t(loc, "nav.more"),
                icon: if sidebar_open {
                    AppIcon::Close
                } else {
                    AppIcon::Ellipsis
                },
                is_active: sidebar_open || in_more_screens,
                badge_count: state.unread_notifications_count,
                action: FooterTabAction::ToggleSidebar,
            },
        ];

        Self {
            tabs,
            show_money_subbar: !sidebar_open
                && matches!(
                    current,
                    Screen::Transactions | Screen::Loans | Screen::Plans
                ),
            is_family,
        }
    }
}

pub struct AppFooter;

impl AppFooter {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let model = AppFooterModel::compute(state);
        let bottom_inset = state.safe_area.bottom.max(0.0);

        let mut root = div()
            .id(id)
            .w_full()
            .flex_shrink_0()
            .flex()
            .flex_col()
            .bg(theme.bg_surface)
            .border_t_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            });

        // Contextual one-thumb Money switcher + Quick Log bar when viewing Money screens
        if model.show_money_subbar {
            root = root.child(Self::render_money_subbar(state, theme, cx));
        }

        // Primary 5-tab Android navigation dock
        root.child(
            div()
                .w_full()
                .px(px(6.0))
                .pb(px(6.0 + bottom_inset))
                .flex()
                .flex_row()
                .items_stretch()
                .justify_between()
                .children(
                    model
                        .tabs
                        .into_iter()
                        .map(|spec| Self::render_tab(spec, theme, cx)),
                ),
        )
    }

    fn render_money_subbar(
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let loc = state.locale;
        let current = state.current_screen;
        let is_dark = theme.mode.is_dark();

        let pills = [
            (
                "footer-money-txs",
                Screen::Transactions,
                AppIcon::Transactions,
                t(loc, "nav.transactions"),
            ),
            (
                "footer-money-loans",
                Screen::Loans,
                AppIcon::Loans,
                t(loc, "nav.loans"),
            ),
            (
                "footer-money-plans",
                Screen::Plans,
                AppIcon::FuturePlans,
                t(loc, "nav.plans"),
            ),
        ];

        div()
            .w_full()
            .px(px(10.0))
            .py(px(7.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(px(6.0))
            .bg(theme.bg_canvas)
            .border_b_1()
            .border_color(theme.border_subtle)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .children(pills.into_iter().map(|(id, screen, icon, label)| {
                        let active = current == screen;
                        let bg = if active {
                            if is_dark {
                                theme.primary_deep
                            } else {
                                theme.primary.opacity(0.12)
                            }
                        } else {
                            theme.bg_surface
                        };
                        let border = if active {
                            theme.primary.opacity(0.75)
                        } else {
                            theme.border_subtle
                        };
                        let text_col = if active {
                            if is_dark {
                                theme.text_primary
                            } else {
                                theme.accent_primary
                            }
                        } else {
                            theme.text_muted
                        };
                        let tone = if active {
                            if is_dark {
                                IconTone::Default
                            } else {
                                IconTone::AccentPrimary
                            }
                        } else {
                            IconTone::Muted
                        };

                        div()
                            .id(id)
                            .flex_1()
                            .min_w_0()
                            .h(px(30.0))
                            .px(px(8.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_center()
                            .gap(px(5.0))
                            .rounded_sm()
                            .bg(bg)
                            .border_1()
                            .border_color(border)
                            .cursor_pointer()
                            .hover(|s| s.border_color(theme.primary.opacity(0.60)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.select_screen(screen, cx);
                            }))
                            .child(app_icon(icon, IconSize::Xs, tone, theme))
                            .child(
                                div()
                                    .min_w_0()
                                    .truncate()
                                    .text_size(px(11.0))
                                    .font_weight(if active {
                                        FontWeight::BOLD
                                    } else {
                                        FontWeight::MEDIUM
                                    })
                                    .text_color(text_col)
                                    .child(label),
                            )
                    })),
            )
            // One-thumb Quick Entry trigger right in the Money sub-bar
            .child(
                div()
                    .id("footer-money-quick-add")
                    .flex_shrink_0()
                    .size(px(30.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_sm()
                    .bg(theme.primary)
                    .border_1()
                    .border_color(theme.primary_pressed)
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.primary_hover))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.open_quick_entry(window, cx);
                    }))
                    .child(app_icon(
                        AppIcon::Add,
                        IconSize::Sm,
                        if is_dark {
                            IconTone::Default
                        } else {
                            IconTone::OnPrimaryButton
                        },
                        theme,
                    )),
            )
    }

    fn render_tab(
        spec: FooterTabSpec,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_dark = theme.mode.is_dark();
        let active = spec.is_active;
        let action = spec.action;

        let pill_bg = if active {
            theme.primary.opacity(if is_dark { 0.22 } else { 0.12 })
        } else {
            gpui_kit::transparent_black()
        };
        let pill_border = if active {
            theme.primary.opacity(0.45)
        } else {
            gpui_kit::transparent_black()
        };
        let icon_tone = if active {
            IconTone::AccentPrimary
        } else {
            IconTone::Muted
        };
        let label_color = if active {
            if is_dark {
                theme.text_primary
            } else {
                theme.accent_primary
            }
        } else {
            theme.text_muted
        };

        let badge_text = if spec.badge_count > 9 {
            Some("9+".to_string())
        } else if spec.badge_count > 0 {
            Some(spec.badge_count.to_string())
        } else {
            None
        };

        div()
            .id(spec.id)
            .flex_1()
            .min_w_0()
            .pt(px(2.0))
            .pb(px(2.0))
            .px(px(2.0))
            .flex()
            .flex_col()
            .items_center()
            .justify_between()
            .gap(px(3.0))
            .cursor_pointer()
            .hover(|s| s.bg(theme.primary.opacity(0.06)))
            .on_click(cx.listener(move |this, _, _, cx| match action {
                FooterTabAction::Navigate(screen) => {
                    this.select_screen(screen, cx);
                }
                FooterTabAction::OpenMoneyGroup => {
                    let target = if matches!(
                        this.state.current_screen,
                        Screen::Transactions | Screen::Loans | Screen::Plans
                    ) {
                        this.state.current_screen
                    } else {
                        Screen::Transactions
                    };
                    this.state.money_expanded = true;
                    this.select_screen(target, cx);
                }
                FooterTabAction::ToggleSidebar => {
                    this.toggle_sidebar(cx);
                }
            }))
            // Top active crimson indicator bar
            .child(
                div()
                    .w(px(26.0))
                    .h(px(2.0))
                    .rounded(PresetRadius::FULL)
                    .bg(if active {
                        theme.primary
                    } else {
                        gpui_kit::transparent_black()
                    }),
            )
            // Icon capsule + optional unread badge
            .child(
                div()
                    .relative()
                    .w(px(42.0))
                    .h(px(26.0))
                    .mt(px(2.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_sm()
                    .bg(pill_bg)
                    .border_1()
                    .border_color(pill_border)
                    .child(app_icon(spec.icon, IconSize::Md, icon_tone, theme))
                    .children(badge_text.map(|txt| {
                        div()
                            .absolute()
                            .top(px(-3.0))
                            .right(px(0.0))
                            .min_w(px(15.0))
                            .h(px(15.0))
                            .px(px(3.0))
                            .rounded(PresetRadius::FULL)
                            .bg(theme.primary)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(9.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(gpui_kit::white())
                            .child(txt)
                    })),
            )
            // Tab label
            .child(
                div()
                    .w_full()
                    .truncate()
                    .text_center()
                    .text_size(px(10.5))
                    .font_weight(if active {
                        FontWeight::BOLD
                    } else {
                        FontWeight::MEDIUM
                    })
                    .text_color(label_color)
                    .child(spec.label),
            )
    }
}

#[cfg(test)]
mod tests {
    use umuryango_core_domain::scope::LedgerScope;
    use umuryango_i18n::Locale;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn family_footer_renders_home_money_projects_reports_more_in_both_locales() {
        let mut state = AppState {
            locale: Locale::En,
            active_scope: LedgerScope::Family {
                family_id: Uuid::new_v4(),
            },
            current_screen: Screen::Dashboard,
            sidebar_open: false,
            ..Default::default()
        };

        let en = AppFooterModel::compute(&state);
        assert_eq!(en.tabs[0].label, "Home");
        assert_eq!(en.tabs[1].label, "Money");
        assert_eq!(en.tabs[2].label, "Projects");
        assert_eq!(en.tabs[3].label, "Reports");
        assert_eq!(en.tabs[4].label, "More");
        assert!(en.tabs[0].is_active);
        assert!(!en.show_money_subbar);

        state.locale = Locale::Rw;
        let rw = AppFooterModel::compute(&state);
        assert_eq!(rw.tabs[0].label, "Ahabanza");
        assert_eq!(rw.tabs[1].label, "Amafaranga");
        assert_eq!(rw.tabs[2].label, "Imishinga");
        assert_eq!(rw.tabs[3].label, "Raporo");
        assert_eq!(rw.tabs[4].label, "Ibindi");
    }

    #[test]
    fn money_group_and_sidebar_more_states_track_active_tab_and_badges() {
        let mut state = AppState {
            locale: Locale::En,
            active_scope: LedgerScope::Family {
                family_id: Uuid::new_v4(),
            },
            current_screen: Screen::Loans,
            sidebar_open: false,
            unread_notifications_count: 4,
            ..Default::default()
        };

        let money = AppFooterModel::compute(&state);
        assert!(money.tabs[1].is_active);
        assert!(money.show_money_subbar);
        assert_eq!(money.tabs[4].badge_count, 4);

        state.sidebar_open = true;
        let drawer_open = AppFooterModel::compute(&state);
        assert!(!drawer_open.tabs[1].is_active);
        assert!(drawer_open.tabs[4].is_active);
        assert_eq!(drawer_open.tabs[4].icon, AppIcon::Close);
        assert!(!drawer_open.show_money_subbar);
    }
}

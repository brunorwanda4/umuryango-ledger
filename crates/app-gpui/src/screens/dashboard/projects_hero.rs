//! Crimson Hero Card (`Active Family Projects` / `Future Plans`) for Row 3 of the Dashboard,
//! including the pure geometric house + rising-bars architectural banner and `Progress` rows.

use gpui_kit::prelude::*;
use gpui_kit::{Context, FontWeight, Hsla, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Translator, terms};

use crate::RootView;
use crate::common::{
    Badge, Button, ButtonSize, Card, CardSize, EmptyDescription, Progress, Separator,
    StyleComposeExt,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{ActiveModal, AppState, Screen};
use crate::theme::SemanticTheme;

use super::metrics::{DashboardSnapshot, HeroGoalItem};

pub fn render_hero_projects_card(
    state: &AppState,
    snap: &DashboardSnapshot,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let locale = state.locale;
    let is_dark = theme.mode.is_dark();
    let hero_fg: Hsla = if is_dark {
        theme.text_primary
    } else {
        theme.bg_surface
    };
    let title = if snap.is_family {
        Translator::t(locale, terms::ACTIVE_PROJECTS, None)
    } else {
        Translator::t(locale, terms::NAV_PLANS, None)
    };
    let subtitle = umuryango_i18n::t(locale, "dashboard.overview_year")
        .replace("{year}", &snap.current_year.to_string());
    let is_family = snap.is_family;

    Card::new()
        .size(CardSize::Sm)
        .h_full()
        .p(px(18.0))
        .py(px(18.0))
        .gap(px(14.0))
        .justify_between()
        .bg(theme.primary)
        .border_1()
        .border_color(theme.primary_pressed)
        .theme(theme)
        .child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .justify_between()
                .gap(px(10.0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::BOLD)
                                .text_color(hero_fg)
                                .child(title),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(hero_fg.opacity(0.82))
                                .child(subtitle),
                        ),
                )
                .child(
                    div()
                        .id("dash-hero-add-goal-btn")
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(28.0))
                        .bg(theme.primary_deep.opacity(0.65))
                        .border_1()
                        .border_color(hero_fg.opacity(0.25))
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.primary_deep))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            let modal = if is_family {
                                ActiveModal::AddProject
                            } else {
                                ActiveModal::AddPlan
                            };
                            this.open_modal(modal, window, cx);
                        }))
                        .child(app_icon(
                            AppIcon::Add,
                            IconSize::Xs,
                            if is_dark {
                                IconTone::Default
                            } else {
                                IconTone::OnPrimaryButton
                            },
                            theme,
                        )),
                ),
        )
        .child(render_architectural_banner(hero_fg, is_dark, theme))
        .child(
            div()
                .flex()
                .flex_col()
                .w_full()
                .bg(theme.bg_surface)
                .border_1()
                .border_color(theme.primary_deep)
                .p(px(16.0))
                .gap(px(14.0))
                .child(if snap.hero_goals.is_empty() {
                    let add_lbl = if is_family {
                        umuryango_i18n::t(locale, "projects.new_project")
                    } else {
                        umuryango_i18n::t(locale, "future_plans.new_plan")
                    };
                    let empty_lbl = if is_family {
                        umuryango_i18n::t(locale, "projects.no_projects")
                    } else {
                        umuryango_i18n::t(locale, "future_plans.no_plans")
                    };
                    div()
                        .flex()
                        .flex_col()
                        .items_start()
                        .gap(px(12.0))
                        .child(EmptyDescription::new(empty_lbl).text_xs().theme(theme))
                        .child(
                            Button::new("dash-hero-empty-create-btn")
                                .size(ButtonSize::Xs)
                                .icon_start(AppIcon::Add)
                                .label(add_lbl)
                                .theme(theme)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    let modal = if is_family {
                                        ActiveModal::AddProject
                                    } else {
                                        ActiveModal::AddPlan
                                    };
                                    this.open_modal(modal, window, cx);
                                })),
                        )
                        .into_any_element()
                } else {
                    let mut list = div().flex().flex_col().gap(px(14.0));
                    for (idx, goal) in snap.hero_goals.iter().enumerate() {
                        if idx > 0 {
                            list = list.child(Separator::horizontal().theme(theme));
                        }
                        list = list.child(render_hero_goal_row(idx, goal, is_family, theme, cx));
                    }
                    list.into_any_element()
                }),
        )
}

fn render_architectural_banner(
    hero_fg: Hsla,
    is_dark: bool,
    theme: &SemanticTheme,
) -> impl IntoElement {
    let icon_tone = if is_dark {
        IconTone::Default
    } else {
        IconTone::OnPrimaryButton
    };

    div()
        .flex()
        .flex_row()
        .items_end()
        .justify_center()
        .gap(px(28.0))
        .w_full()
        .h(px(116.0))
        .px(px(20.0))
        .py(px(16.0))
        .bg(theme.primary_deep)
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .w(px(52.0))
                        .h(px(8.0))
                        .bg(theme.primary_hover)
                        .border_t_1()
                        .border_color(hero_fg.opacity(0.85)),
                )
                .child(
                    div()
                        .w(px(74.0))
                        .h(px(10.0))
                        .bg(theme.primary)
                        .border_1()
                        .border_color(hero_fg.opacity(0.85)),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_end()
                        .justify_between()
                        .w(px(62.0))
                        .h(px(42.0))
                        .px(px(10.0))
                        .pt(px(8.0))
                        .bg(theme.primary_deep)
                        .border_l_1()
                        .border_r_1()
                        .border_b_1()
                        .border_color(hero_fg.opacity(0.85))
                        .child(
                            div()
                                .w(px(14.0))
                                .h(px(26.0))
                                .bg(theme.bg_canvas)
                                .border_t_1()
                                .border_l_1()
                                .border_r_1()
                                .border_color(hero_fg.opacity(0.45)),
                        )
                        .child(
                            div()
                                .mb(px(12.0))
                                .size(px(14.0))
                                .bg(hero_fg)
                                .border_1()
                                .border_color(theme.primary_hover),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_end()
                .gap(px(6.0))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.0))
                        .child(app_icon(AppIcon::Income, IconSize::Sm, icon_tone, theme))
                        .child(
                            div()
                                .size(px(7.0))
                                .border_1()
                                .border_color(hero_fg)
                                .bg(theme.primary_hover),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_end()
                        .gap(px(7.0))
                        .child(
                            div()
                                .w(px(13.0))
                                .h(px(20.0))
                                .bg(theme.primary_pressed)
                                .border_1()
                                .border_color(hero_fg.opacity(0.55)),
                        )
                        .child(
                            div()
                                .w(px(13.0))
                                .h(px(32.0))
                                .bg(theme.primary_hover)
                                .border_1()
                                .border_color(hero_fg.opacity(0.75)),
                        )
                        .child(div().w(px(13.0)).h(px(44.0)).bg(hero_fg.opacity(0.92)))
                        .child(div().w(px(13.0)).h(px(56.0)).bg(hero_fg)),
                ),
        )
}

fn render_hero_goal_row(
    idx: usize,
    goal: &HeroGoalItem,
    is_family: bool,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let is_dark = theme.mode.is_dark();
    let bar_fill = if is_dark {
        theme.primary_hover
    } else {
        theme.primary
    };
    let badge_bg = if idx == 0 {
        if is_dark {
            theme.primary_deep.opacity(0.55)
        } else {
            theme.primary.opacity(0.12)
        }
    } else {
        theme.bg_canvas
    };

    div()
        .id(format!("dash-hero-goal-{}", idx))
        .flex()
        .flex_col()
        .gap(px(10.0))
        .cursor_pointer()
        .on_click(cx.listener(move |this, _, _, cx| {
            this.select_screen(
                if is_family {
                    Screen::Projects
                } else {
                    Screen::Plans
                },
                cx,
            );
        }))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap(px(10.0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .min_w_0()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .truncate()
                                .child(goal.title.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(goal.amount_label.clone()),
                        ),
                )
                .child(
                    Badge::new(format!("{}%", goal.pct))
                        .cx_when(idx == 0, |b| b.destructive())
                        .px(px(8.0))
                        .py(px(4.0))
                        .bg(badge_bg)
                        .border_1()
                        .border_color(if idx == 0 {
                            theme.primary.opacity(0.45)
                        } else {
                            theme.border_subtle
                        })
                        .theme(theme),
                ),
        )
        .child(
            Progress::new(goal.pct_f32)
                .indicator_color(bar_fill)
                .theme(theme),
        )
}

//! Row 2 of the Dashboard:
//! - Left (2/3 on Wide): Cash Flow Updates diverging inflow/outflow `BarChart` + summary rail
//! - Right Top (1/3): Yearly Breakup with Donut `PieChart`
//! - Right Bottom (1/3): Monthly Net Savings with full-bleed `AreaChart` sparkline

use chrono::{DateTime, Utc};
use gpui_kit::prelude::*;
use gpui_kit::{
    Context, ElementId, FontWeight, Hsla, IntoElement, ParentElement, Styled, div,
    linear_color_stop, linear_gradient, px, relative, transparent_black,
};
use umuryango_i18n::Locale;

use crate::RootView;
use crate::common::popover::PopoverAlign;
use crate::common::{
    AreaChart, BarChart, Button, ButtonSize, Card, CardAction, CardContent, CardHeader, CardSize,
    ChartConfig, ChartContainer, DropdownMenu, DropdownMenuContent, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuSeparator, PieChart, PresetRadius, StyleComposeExt,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::Screen;
use crate::theme::SemanticTheme;

use super::metrics::{
    DashboardLayoutTier, DashboardPeriod, DashboardSnapshot, format_compact_axis, format_rwf,
};

#[derive(Clone)]
struct DonutSlice {
    value: f32,
    color: Hsla,
}

/// Renders the shared period selector dropdown (`November 2026 ▾`) used in Row 2 and Row 4.
pub fn render_period_dropdown(
    id: impl Into<ElementId>,
    active_period: DashboardPeriod,
    active_label: &str,
    locale: Locale,
    now: DateTime<Utc>,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let trigger = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .h(px(30.0))
        .px(px(12.0))
        .rounded(PresetRadius::NONE)
        .border_1()
        .border_color(theme.border_subtle)
        .bg(theme.bg_canvas)
        .hover(|s| s.border_color(theme.primary))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.text_primary)
                .whitespace_nowrap()
                .child(active_label.to_string()),
        )
        .child(app_icon(
            AppIcon::ChevronDown,
            IconSize::Xs,
            IconTone::Muted,
            theme,
        ));

    let mut content = DropdownMenuContent::new()
        .align(PopoverAlign::End)
        .theme(theme)
        .child(
            DropdownMenuLabel::new(umuryango_i18n::t(locale, "dashboard.period_filter_label"))
                .theme(theme),
        )
        .child(DropdownMenuSeparator::new().theme(theme));

    for (idx, period) in DashboardPeriod::ALL.into_iter().enumerate() {
        let lbl = period.label(locale, now);
        let is_selected = period == active_period;
        let mut item = DropdownMenuItem::new(format!("dash-period-opt-{}", idx), lbl)
            .uppercase(false)
            .theme(theme)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.set_dashboard_period(period, cx);
            }));
        if is_selected {
            item = item.icon(AppIcon::Check);
        }
        content = content.child(item);
    }

    DropdownMenu::new(id).trigger(trigger).content(content)
}

/// Renders Row 2 (`Cash Flow Updates` + `Yearly Breakup` + `Monthly Net Savings`).
pub fn render_cashflow_row(
    snap: &DashboardSnapshot,
    tier: DashboardLayoutTier,
    locale: Locale,
    now: DateTime<Utc>,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let gap = px(tier.grid_gap_px());
    let cashflow_card = render_cashflow_card(snap, tier, locale, now, theme, cx);
    let yearly_card = render_yearly_breakup_card(snap, locale, theme, cx);
    let savings_card = render_net_savings_card(snap, locale, theme, cx);

    match tier {
        DashboardLayoutTier::Wide => div()
            .flex()
            .flex_row()
            .items_stretch()
            .w_full()
            .gap(gap)
            .child(div().flex_1().min_w_0().child(cashflow_card))
            .child(
                div()
                    .w(relative(0.324))
                    .flex_shrink_0()
                    .flex()
                    .flex_col()
                    .gap(gap)
                    .child(yearly_card)
                    .child(savings_card),
            )
            .into_any_element(),
        DashboardLayoutTier::Medium => div()
            .flex()
            .flex_col()
            .w_full()
            .gap(gap)
            .child(cashflow_card)
            .child(
                div()
                    .grid()
                    .grid_cols(2)
                    .gap(gap)
                    .child(yearly_card)
                    .child(savings_card),
            )
            .into_any_element(),
        DashboardLayoutTier::Compact | DashboardLayoutTier::NarrowMobile => div()
            .flex()
            .flex_col()
            .w_full()
            .gap(gap)
            .child(cashflow_card)
            .child(yearly_card)
            .child(savings_card)
            .into_any_element(),
    }
}

fn render_cashflow_card(
    snap: &DashboardSnapshot,
    tier: DashboardLayoutTier,
    locale: Locale,
    now: DateTime<Utc>,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let is_dark = theme.mode.is_dark();
    let inflow_color = if is_dark {
        theme.primary_hover
    } else {
        theme.primary
    };
    let outflow_color = if is_dark {
        theme.primary_pressed.opacity(0.62)
    } else {
        theme.primary_deep.opacity(0.78)
    };
    let side_by_side = matches!(
        tier,
        DashboardLayoutTier::Wide | DashboardLayoutTier::Medium
    );

    let chart = BarChart::new(snap.cashflow_bars.clone())
        .id("dash-cashflow-diverging-chart")
        .interactive(false)
        .band(|d| d.band.clone())
        .value(|d| d.value)
        .label_axis(true)
        .band_tick_count(0)
        .value_axis(true)
        .value_tick_count(5)
        .value_tick_format(format_compact_axis)
        .grid(true)
        .grid_dashed(false)
        .padding_inner(0.52)
        .max_band_width(px(16.0))
        .fill(move |d, _, _, _| {
            if d.is_anchor {
                transparent_black()
            } else if d.is_inflow {
                inflow_color
            } else {
                outflow_color
            }
        });

    let plot_column = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .child(
            ChartContainer::new("dash-cashflow-chart-frame", ChartConfig::new())
                .min_height(210.0)
                .h(px(210.0))
                .theme(theme)
                .child(chart),
        )
        .child(
            div()
                .grid()
                .grid_cols(snap.cashflow_band_labels.len().max(1) as u16)
                .pl(px(44.0))
                .pt(px(6.0))
                .children(snap.cashflow_band_labels.iter().map(|lbl| {
                    div()
                        .flex()
                        .justify_center()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(lbl.clone())
                })),
        );

    let tile_bg = if is_dark {
        theme.primary_deep.opacity(0.38)
    } else {
        theme.primary.opacity(0.08)
    };

    let summary_rail = div()
        .cx_when(side_by_side, |d| d.w(px(228.0)).flex_shrink_0())
        .cx_when(!side_by_side, |d| {
            d.w_full()
                .pt(px(14.0))
                .border_t_1()
                .border_color(theme.border_subtle)
        })
        .flex()
        .flex_col()
        .justify_between()
        .gap(px(18.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .child(
                            div()
                                .flex()
                                .flex_shrink_0()
                                .items_center()
                                .justify_center()
                                .size(px(38.0))
                                .bg(tile_bg)
                                .border_1()
                                .border_color(theme.primary.opacity(0.45))
                                .child(app_icon(
                                    AppIcon::PersonalWallet,
                                    IconSize::Sm,
                                    IconTone::Expense,
                                    theme,
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(format_rwf(snap.total_custody)),
                                )
                                .child(div().text_xs().text_color(theme.text_muted).child(
                                    umuryango_i18n::t(locale, "dashboard.total_balance_short"),
                                )),
                        ),
                )
                .child(render_swatch_metric(
                    inflow_color,
                    &umuryango_i18n::t(locale, "dashboard.inflow_this_month"),
                    &format_rwf(snap.period_inflow),
                    theme,
                ))
                .child(render_swatch_metric(
                    outflow_color,
                    &umuryango_i18n::t(locale, "dashboard.expense_this_month"),
                    &format_rwf(snap.period_expense),
                    theme,
                )),
        )
        .child(
            Button::new("dash-view-full-report-btn")
                .size(ButtonSize::Sm)
                .w_full()
                .label(umuryango_i18n::t(locale, "dashboard.view_full_report"))
                .theme(theme)
                .on_click(cx.listener(|this, _, _, cx| {
                    this.select_screen(Screen::Reports, cx);
                })),
        );

    Card::new()
        .size(CardSize::Sm)
        .h_full()
        .gap(px(18.0))
        .py(px(20.0))
        .border_1()
        .border_color(theme.border_subtle)
        .theme(theme)
        .child(
            CardHeader::new()
                .size(CardSize::Sm)
                .action(CardAction::new().child(render_period_dropdown(
                    "dash-cashflow-period-menu",
                    snap.active_period,
                    &snap.active_period_label,
                    locale,
                    now,
                    theme,
                    cx,
                )))
                .theme(theme)
                .child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(umuryango_i18n::t(locale, "dashboard.cash_flow_updates")),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(umuryango_i18n::t(
                            locale,
                            "dashboard.inflow_outflow_overview",
                        )),
                ),
        )
        .child(
            CardContent::new().size(CardSize::Sm).child(
                div()
                    .flex()
                    .cx_when(side_by_side, |d| d.flex_row().items_stretch().gap(px(24.0)))
                    .cx_when(!side_by_side, |d| d.flex_col().gap(px(16.0)))
                    .child(plot_column)
                    .child(summary_rail),
            ),
        )
}

fn render_swatch_metric(
    swatch: Hsla,
    label: &str,
    value: &str,
    theme: &SemanticTheme,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(px(10.0))
        .child(div().mt(px(4.0)).size(px(8.0)).flex_shrink_0().bg(swatch))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(label.to_string()),
                )
                .child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(value.to_string()),
                ),
        )
}

fn render_yearly_breakup_card(
    snap: &DashboardSnapshot,
    locale: Locale,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let is_dark = theme.mode.is_dark();
    let curr_color = if is_dark {
        theme.primary_hover
    } else {
        theme.primary
    };
    let prev_color = if is_dark {
        theme.primary_deep
    } else {
        theme.border_subtle
    };

    let c_val = (snap.yearly_share_pct.clamp(0, 100)) as f32;
    let slices = if snap.yearly_current_total.is_zero()
        && snap.yearly_prev_total.is_zero()
        && snap.total_custody.is_zero()
    {
        vec![DonutSlice {
            value: 1.0,
            color: theme.border_subtle,
        }]
    } else {
        vec![
            DonutSlice {
                value: c_val.max(2.0),
                color: curr_color,
            },
            DonutSlice {
                value: (100.0 - c_val).max(0.0),
                color: prev_color,
            },
        ]
    };

    let display_yearly = if snap.yearly_current_total.is_zero() {
        snap.total_custody
    } else {
        snap.yearly_current_total
    };

    let donut = div()
        .relative()
        .flex_shrink_0()
        .size(px(104.0))
        .child(
            PieChart::new(slices)
                .id("dash-yearly-breakup-donut")
                .interactive(false)
                .inner_radius_ratio(0.68)
                .outer_radius(46.0)
                .pad_angle(0.03)
                .value(|s| s.value)
                .color(|s| s.color),
        )
        .child(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child(format!("{}%", snap.yearly_share_pct)),
        );

    div()
        .id("dash-yearly-breakup-card")
        .border_1()
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(|s| s.border_color(theme.primary.opacity(0.6)))
        .on_click(cx.listener(|this, _, _, cx| {
            this.select_screen(Screen::Reports, cx);
        }))
        .child(
            Card::new()
                .size(CardSize::Sm)
                .py(px(18.0))
                .gap(px(0.0))
                .theme(theme)
                .child(
                    CardContent::new().size(CardSize::Sm).child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(10.0))
                                    .min_w_0()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(umuryango_i18n::t(
                                                locale,
                                                "dashboard.yearly_breakup",
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(format_rwf(display_yearly)),
                                    )
                                    .child(render_delta_pill(
                                        snap.yearly_yoy_pct,
                                        &umuryango_i18n::t(locale, "dashboard.last_year"),
                                        theme,
                                    ))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(14.0))
                                            .pt(px(2.0))
                                            .child(render_year_dot(
                                                curr_color,
                                                snap.current_year,
                                                theme,
                                            ))
                                            .child(render_year_dot(
                                                prev_color,
                                                snap.previous_year,
                                                theme,
                                            )),
                                    ),
                            )
                            .child(donut),
                    ),
                ),
        )
}

fn render_net_savings_card(
    snap: &DashboardSnapshot,
    locale: Locale,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let is_dark = theme.mode.is_dark();
    let line_color = if is_dark {
        theme.primary_hover
    } else {
        theme.primary
    };
    let on_primary = if is_dark {
        IconTone::Default
    } else {
        IconTone::OnPrimaryButton
    };

    let spark = AreaChart::new(snap.savings_sparkline.clone())
        .id("dash-net-savings-sparkline")
        .interactive(false)
        .x(|p| p.label.clone())
        .y(|p| p.value)
        .y_domain(0.0, snap.savings_spark_max)
        .stroke(line_color)
        .fill(linear_gradient(
            180.0,
            linear_color_stop(theme.primary.opacity(0.32), 0.0),
            linear_color_stop(theme.primary.opacity(0.02), 1.0),
        ))
        .linear()
        .x_axis(false)
        .y_axis(false)
        .grid(false);

    div()
        .id("dash-net-savings-card")
        .flex_1()
        .border_1()
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(|s| s.border_color(theme.primary.opacity(0.6)))
        .on_click(cx.listener(|this, _, _, cx| {
            this.select_screen(Screen::Transactions, cx);
        }))
        .child(
            Card::new()
                .size(CardSize::Sm)
                .h_full()
                .pt(px(18.0))
                .pb(px(0.0))
                .gap(px(12.0))
                .justify_between()
                .theme(theme)
                .child(
                    CardContent::new().size(CardSize::Sm).child(
                        div()
                            .flex()
                            .flex_row()
                            .items_start()
                            .justify_between()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .min_w_0()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(umuryango_i18n::t(
                                                locale,
                                                "dashboard.monthly_net_savings",
                                            )),
                                    )
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(format_rwf(snap.net_savings)),
                                    )
                                    .child(render_delta_pill(
                                        snap.net_savings_mom_pct,
                                        &umuryango_i18n::t(locale, "dashboard.last_month"),
                                        theme,
                                    )),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_shrink_0()
                                    .items_center()
                                    .justify_center()
                                    .size(px(34.0))
                                    .bg(theme.primary)
                                    .child(app_icon(
                                        AppIcon::Cash,
                                        IconSize::Sm,
                                        on_primary,
                                        theme,
                                    )),
                            ),
                    ),
                )
                .child(
                    ChartContainer::new("dash-net-savings-spark-frame", ChartConfig::new())
                        .min_height(50.0)
                        .h(px(50.0))
                        .theme(theme)
                        .child(spark),
                ),
        )
}

pub fn render_delta_pill(delta_pct: i32, suffix: &str, theme: &SemanticTheme) -> impl IntoElement {
    let is_dark = theme.mode.is_dark();
    let is_non_neg = delta_pct >= 0;
    let badge_bg = if is_dark {
        theme.primary_deep.opacity(0.55)
    } else {
        theme.primary.opacity(0.12)
    };
    let sign = if delta_pct >= 0 { "+" } else { "" };

    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(6.0))
        .child(
            div()
                .flex()
                .flex_shrink_0()
                .items_center()
                .justify_center()
                .size(px(18.0))
                .bg(badge_bg)
                .child(app_icon(
                    if is_non_neg {
                        AppIcon::Income
                    } else {
                        AppIcon::Expense
                    },
                    IconSize::Xs,
                    IconTone::Expense,
                    theme,
                )),
        )
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child(format!("{}{}%", sign, delta_pct)),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .child(suffix.to_string()),
        )
}

fn render_year_dot(color: Hsla, year: i32, theme: &SemanticTheme) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(6.0))
        .child(div().size(px(7.0)).bg(color))
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .child(year.to_string()),
        )
}

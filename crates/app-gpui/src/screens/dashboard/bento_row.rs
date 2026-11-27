//! Row 3 of the Dashboard (3-Column Bento Grid):
//! - Column 1: Monthly Custody Flow (6-month `BarChart` + Pool vs Cash Custody split)
//! - Column 2: Wallets (`AreaChart` sparkline) + Handovers (`BarChart`) + Mother's Report Ready (`Avatar`s + Share `Button`)
//! - Column 3: Active Family Projects / Future Plans Crimson Hero `Card` (`projects_hero.rs`)

use chrono::Duration;
use gpui_kit::prelude::*;
use gpui_kit::{
    Context, FontWeight, Hsla, IntoElement, ParentElement, Styled, div, linear_color_stop,
    linear_gradient, px,
};
use umuryango_core_domain::reporting::{LocalizedExporter, generate_mothers_report};

use crate::RootView;
use crate::common::{
    AreaChart, Avatar, AvatarFallback, AvatarSize, Badge, BarChart, Button, ButtonSize, Card,
    CardContent, CardHeader, CardSize, ChartConfig, ChartContainer, PresetRadius, Separator,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{ActiveModal, AppState, Screen};
use crate::theme::SemanticTheme;

use super::metrics::{DashboardLayoutTier, DashboardSnapshot, SimpleBarDatum, format_rwf};
use super::projects_hero::render_hero_projects_card;

/// Renders Row 3 of the Dashboard, adapting from 3 columns (`Wide`) to 2+1 (`Medium`) to 1 (`Compact`).
pub fn render_bento_row(
    state: &AppState,
    snap: &DashboardSnapshot,
    tier: DashboardLayoutTier,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let gap = px(tier.grid_gap_px());
    let custody_flow_card = render_monthly_custody_flow_card(state, snap, tier, theme, cx);
    let center_stack = render_center_bento_stack(state, snap, tier, theme, cx);
    let hero_projects_card = render_hero_projects_card(state, snap, theme, cx);

    match tier {
        DashboardLayoutTier::Wide => div()
            .grid()
            .grid_cols(3)
            .gap(gap)
            .w_full()
            .child(custody_flow_card)
            .child(center_stack)
            .child(hero_projects_card)
            .into_any_element(),
        DashboardLayoutTier::Medium => div()
            .flex()
            .flex_col()
            .gap(gap)
            .w_full()
            .child(
                div()
                    .grid()
                    .grid_cols(2)
                    .gap(gap)
                    .w_full()
                    .child(custody_flow_card)
                    .child(center_stack),
            )
            .child(hero_projects_card)
            .into_any_element(),
        DashboardLayoutTier::Compact | DashboardLayoutTier::NarrowMobile => div()
            .flex()
            .flex_col()
            .gap(gap)
            .w_full()
            .child(custody_flow_card)
            .child(center_stack)
            .child(hero_projects_card)
            .into_any_element(),
    }
}

fn render_monthly_custody_flow_card(
    state: &AppState,
    snap: &DashboardSnapshot,
    tier: DashboardLayoutTier,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let locale = state.locale;
    let is_dark = theme.mode.is_dark();
    let max_vol = snap
        .monthly_flow_bars
        .iter()
        .map(|b| b.value)
        .fold(0.0_f64, f64::max);

    let plot_bars: Vec<SimpleBarDatum> = if max_vol <= 0.0 {
        snap.monthly_flow_bars
            .iter()
            .map(|b| SimpleBarDatum {
                band: b.band.clone(),
                value: 1.0,
                highlighted: b.highlighted,
            })
            .collect()
    } else {
        snap.monthly_flow_bars.clone()
    };

    let active_bar_color = if is_dark {
        theme.primary_hover
    } else {
        theme.primary
    };
    let muted_bar_color = if max_vol <= 0.0 {
        theme.border_subtle.opacity(0.45)
    } else if is_dark {
        theme.text_muted.opacity(0.22)
    } else {
        theme.border_subtle
    };

    let chart = BarChart::new(plot_bars)
        .id("dash-monthly-custody-bars")
        .interactive(false)
        .band(|d| d.band.clone())
        .value(|d| d.value)
        .label_axis(false)
        .value_axis(false)
        .grid(false)
        .padding_inner(0.36)
        .max_band_width(px(28.0))
        .min_length(8.0)
        .fill(move |d, _, _, _| {
            if d.highlighted {
                active_bar_color
            } else {
                muted_bar_color
            }
        });

    let pool_label = if snap.is_family {
        umuryango_i18n::t(locale, "dashboard.family_pool")
    } else {
        umuryango_i18n::t(locale, "dashboard.bank_momo_pool")
    };
    let cash_label = umuryango_i18n::t(locale, "dashboard.cash_custody");
    let stack_footer = matches!(tier, DashboardLayoutTier::NarrowMobile);

    div()
        .id("dash-monthly-custody-card")
        .h_full()
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
                .py(px(20.0))
                .gap(px(16.0))
                .justify_between()
                .theme(theme)
                .child(
                    CardHeader::new()
                        .size(CardSize::Sm)
                        .theme(theme)
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(umuryango_i18n::t(locale, "dashboard.monthly_custody_flow")),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(umuryango_i18n::t(locale, "dashboard.every_month")),
                        ),
                )
                .child(
                    CardContent::new().size(CardSize::Sm).child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(10.0))
                            .child(
                                ChartContainer::new(
                                    "dash-monthly-custody-frame",
                                    ChartConfig::new(),
                                )
                                .min_height(155.0)
                                .h(px(155.0))
                                .theme(theme)
                                .child(chart),
                            )
                            .child(
                                div()
                                    .grid()
                                    .grid_cols(snap.monthly_flow_bars.len().max(1) as u16)
                                    .children(snap.monthly_flow_bars.iter().map(|b| {
                                        div()
                                            .flex()
                                            .justify_center()
                                            .text_xs()
                                            .font_weight(if b.highlighted {
                                                FontWeight::BOLD
                                            } else {
                                                FontWeight::NORMAL
                                            })
                                            .text_color(if b.highlighted {
                                                theme.text_primary
                                            } else {
                                                theme.text_muted
                                            })
                                            .child(b.band.clone())
                                    })),
                            ),
                    ),
                )
                .child(
                    CardContent::new().size(CardSize::Sm).child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(14.0))
                            .child(Separator::horizontal().theme(theme))
                            .child(
                                div()
                                    .grid()
                                    .grid_cols(if stack_footer { 1 } else { 2 })
                                    .gap(px(12.0))
                                    .child(render_custody_split_tile(
                                        if snap.is_family {
                                            AppIcon::Members
                                        } else {
                                            AppIcon::Bank
                                        },
                                        IconTone::Expense,
                                        &pool_label,
                                        &format_rwf(snap.pool_custody_amount),
                                        theme,
                                    ))
                                    .child(render_custody_split_tile(
                                        AppIcon::PersonalWallet,
                                        IconTone::Muted,
                                        &cash_label,
                                        &format_rwf(snap.cash_custody_amount),
                                        theme,
                                    )),
                            ),
                    ),
                ),
        )
}

fn render_custody_split_tile(
    icon: AppIcon,
    tone: IconTone,
    label: &str,
    amount: &str,
    theme: &SemanticTheme,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(10.0))
        .min_w_0()
        .child(
            div()
                .flex()
                .flex_shrink_0()
                .items_center()
                .justify_center()
                .size(px(30.0))
                .bg(theme.bg_canvas)
                .border_1()
                .border_color(theme.border_subtle)
                .child(app_icon(icon, IconSize::Xs, tone, theme)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .min_w_0()
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(theme.text_muted)
                        .truncate()
                        .child(label.to_string()),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .truncate()
                        .child(amount.to_string()),
                ),
        )
}

fn render_center_bento_stack(
    state: &AppState,
    snap: &DashboardSnapshot,
    tier: DashboardLayoutTier,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let gap = px(tier.grid_gap_px());
    let mini_cols = if matches!(tier, DashboardLayoutTier::NarrowMobile) {
        1
    } else {
        2
    };

    div()
        .flex()
        .flex_col()
        .justify_between()
        .h_full()
        .gap(gap)
        .child(
            div()
                .grid()
                .grid_cols(mini_cols)
                .gap(gap)
                .child(render_wallets_mini_card(state.locale, snap, theme, cx))
                .child(render_handovers_mini_card(state.locale, snap, theme, cx)),
        )
        .child(render_mothers_report_card(state, snap, theme, cx))
}

fn render_wallets_mini_card(
    locale: umuryango_i18n::Locale,
    snap: &DashboardSnapshot,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let is_dark = theme.mode.is_dark();
    let accent = if is_dark {
        theme.primary_hover
    } else {
        theme.primary
    };
    let active_sub = umuryango_i18n::t(locale, "dashboard.active_count")
        .replace("{count}", &snap.active_wallets_count.to_string());

    let spark = AreaChart::new(snap.wallet_sparkline.clone())
        .id("dash-wallets-mini-spark")
        .interactive(false)
        .x(|p| p.label.clone())
        .y(|p| p.value)
        .y_domain(0.0, snap.wallet_spark_max)
        .stroke(accent)
        .fill(linear_gradient(
            180.0,
            linear_color_stop(theme.primary.opacity(0.28), 0.0),
            linear_color_stop(theme.primary.opacity(0.02), 1.0),
        ))
        .linear()
        .x_axis(false)
        .y_axis(false)
        .grid(false);

    div()
        .id("dash-wallets-mini-card")
        .border_1()
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(|s| s.border_color(theme.primary.opacity(0.65)))
        .on_click(cx.listener(|this, _, window, cx| {
            this.open_modal(ActiveModal::AddWallet, window, cx);
        }))
        .child(
            Card::new()
                .size(CardSize::Sm)
                .h_full()
                .pt(px(16.0))
                .pb(px(0.0))
                .gap(px(14.0))
                .justify_between()
                .theme(theme)
                .child(
                    CardContent::new().size(CardSize::Sm).px(px(16.0)).child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(umuryango_i18n::t(locale, "dashboard.wallets_label")),
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(snap.total_wallets_count.to_string()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(accent)
                                    .child(active_sub),
                            ),
                    ),
                )
                .child(
                    ChartContainer::new("dash-wallets-mini-frame", ChartConfig::new())
                        .min_height(48.0)
                        .h(px(48.0))
                        .theme(theme)
                        .child(spark),
                ),
        )
}

fn render_handovers_mini_card(
    locale: umuryango_i18n::Locale,
    snap: &DashboardSnapshot,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let is_dark = theme.mode.is_dark();
    let accent = if is_dark {
        theme.primary_hover
    } else {
        theme.primary
    };
    let max_h = snap
        .handover_bars
        .iter()
        .map(|b| b.value)
        .fold(0.0_f64, f64::max);
    let bars_data: Vec<SimpleBarDatum> = if max_h <= 0.0 {
        snap.handover_bars
            .iter()
            .map(|b| SimpleBarDatum {
                band: b.band.clone(),
                value: 1.0,
                highlighted: b.highlighted,
            })
            .collect()
    } else {
        snap.handover_bars.clone()
    };

    let muted_col = if max_h <= 0.0 {
        theme.border_subtle.opacity(0.45)
    } else if is_dark {
        theme.text_muted.opacity(0.28)
    } else {
        theme.border_subtle
    };

    let bars = BarChart::new(bars_data)
        .id("dash-handovers-mini-bars")
        .interactive(false)
        .band(|d| d.band.clone())
        .value(|d| d.value)
        .label_axis(false)
        .value_axis(false)
        .grid(false)
        .padding_inner(0.35)
        .max_band_width(px(10.0))
        .min_length(8.0)
        .fill(move |d, _, _, _| if d.highlighted { accent } else { muted_col });

    let sign = if snap.handovers_delta_pct >= 0 {
        "+"
    } else {
        ""
    };

    div()
        .id("dash-handovers-mini-card")
        .border_1()
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(|s| s.border_color(theme.primary.opacity(0.65)))
        .on_click(cx.listener(|this, _, window, cx| {
            this.open_quick_handover(window, cx);
        }))
        .child(
            Card::new()
                .size(CardSize::Sm)
                .h_full()
                .py(px(16.0))
                .gap(px(14.0))
                .justify_between()
                .theme(theme)
                .child(
                    CardContent::new().size(CardSize::Sm).px(px(16.0)).child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(umuryango_i18n::t(locale, "dashboard.handovers_label")),
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(snap.handovers_count.to_string()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(accent)
                                    .child(format!("{}{}%", sign, snap.handovers_delta_pct)),
                            ),
                    ),
                )
                .child(
                    CardContent::new().size(CardSize::Sm).px(px(16.0)).child(
                        ChartContainer::new("dash-handovers-mini-frame", ChartConfig::new())
                            .min_height(40.0)
                            .h(px(40.0))
                            .theme(theme)
                            .child(bars),
                    ),
                ),
        )
}

fn render_mothers_report_card(
    state: &AppState,
    snap: &DashboardSnapshot,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let locale = state.locale;
    let is_dark = theme.mode.is_dark();
    let on_crimson: Hsla = if is_dark {
        theme.text_primary
    } else {
        theme.bg_surface
    };
    let mr_bg = if is_dark {
        theme.primary_deep
    } else {
        theme.primary
    };
    let badge_bg = if is_dark {
        theme.primary_deep.opacity(0.45)
    } else {
        theme.primary.opacity(0.10)
    };

    let now = chrono::Utc::now();
    let personal_share = if !snap.is_family {
        let rep = generate_mothers_report(
            state.active_scope,
            &state.wallets,
            &state.transactions,
            &state.projects,
            &state.loans,
            &state.future_plans,
            now - Duration::days(30),
            now,
        );
        Some((
            LocalizedExporter::format_mothers_report(&rep, locale),
            umuryango_i18n::t(locale, "report.share_copied"),
        ))
    } else {
        None
    };

    let mr_avatar = Avatar::new()
        .size(AvatarSize::Lg)
        .rounded(PresetRadius::NONE)
        .border_color(theme.primary.opacity(0.55))
        .theme(theme)
        .child(
            AvatarFallback::new("MR")
                .size(AvatarSize::Lg)
                .rounded(PresetRadius::NONE)
                .bg(mr_bg)
                .text_color(on_crimson)
                .font_weight(FontWeight::BOLD)
                .theme(theme),
        );

    let is_family = snap.is_family;

    Card::new()
        .size(CardSize::Sm)
        .py(px(16.0))
        .gap(px(14.0))
        .border_1()
        .border_color(theme.border_subtle)
        .theme(theme)
        .child(
            CardContent::new().size(CardSize::Sm).px(px(16.0)).child(
                div()
                    .id("dash-mothers-report-open")
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(px(12.0))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.select_screen(Screen::Reports, cx);
                    }))
                    .child(mr_avatar)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .min_w_0()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .truncate()
                                    .child(umuryango_i18n::t(
                                        locale,
                                        "dashboard.mothers_report_ready",
                                    )),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(snap.report_date_label.clone()),
                            )
                            .child(
                                Badge::new(umuryango_i18n::t(
                                    locale,
                                    "dashboard.zero_login_active",
                                ))
                                .destructive()
                                .mt(px(2.0))
                                .px(px(8.0))
                                .py(px(3.0))
                                .bg(badge_bg)
                                .border_1()
                                .border_color(theme.primary.opacity(0.45))
                                .theme(theme),
                            ),
                    ),
            ),
        )
        .child(
            CardContent::new().size(CardSize::Sm).px(px(16.0)).child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(Separator::horizontal().theme(theme))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("dash-mothers-members-strip")
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(6.0))
                                    .cursor_pointer()
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.select_screen(
                                            if is_family {
                                                Screen::Members
                                            } else {
                                                Screen::Settings
                                            },
                                            cx,
                                        );
                                    }))
                                    .children(snap.member_chips.iter().map(|chip| {
                                        let chip_bg = if chip.primary_accent {
                                            theme.primary
                                        } else {
                                            theme.bg_canvas
                                        };
                                        let chip_fg = if chip.primary_accent {
                                            on_crimson
                                        } else {
                                            theme.text_primary
                                        };
                                        let chip_border = if chip.primary_accent {
                                            theme.primary_pressed
                                        } else {
                                            theme.border_subtle
                                        };
                                        Avatar::new()
                                            .size(AvatarSize::Sm)
                                            .w(px(28.0))
                                            .h(px(28.0))
                                            .rounded(PresetRadius::NONE)
                                            .border_color(chip_border)
                                            .theme(theme)
                                            .child(
                                                AvatarFallback::new(chip.initials.clone())
                                                    .size(AvatarSize::Sm)
                                                    .rounded(PresetRadius::NONE)
                                                    .bg(chip_bg)
                                                    .text_color(chip_fg)
                                                    .font_weight(FontWeight::BOLD)
                                                    .theme(theme),
                                            )
                                    })),
                            )
                            .child(
                                Button::new("dash-share-mothers-report-btn")
                                    .size(ButtonSize::IconXs)
                                    .w(px(30.0))
                                    .h(px(30.0))
                                    .icon(AppIcon::Share)
                                    .theme(theme)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        if let Some((txt, feedback)) = personal_share.as_ref() {
                                            this.copy_to_clipboard(
                                                txt.clone(),
                                                feedback.clone(),
                                                cx,
                                            );
                                        } else {
                                            this.handle_create_magic_link(cx);
                                        }
                                    })),
                            ),
                    ),
            ),
        )
}

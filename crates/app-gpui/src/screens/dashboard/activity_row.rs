//! Row 4 of the Dashboard (Asymmetric 1:2 Split):
//! - Left (1/3 on Wide): Weekly Activity `AreaChart` + Top Inflow / Top Expense / Latest Handover highlights
//! - Right (2/3 on Wide): Recent Custody Activity `Table` with monogram `Avatar`s, semantic `Badge`s, and period filter

use chrono::{DateTime, Utc};
use gpui_kit::prelude::*;
use gpui_kit::{
    Context, FontWeight, Hsla, IntoElement, ParentElement, Styled, div, linear_color_stop,
    linear_gradient, px, relative,
};
use umuryango_core_domain::models::TransactionType;
use umuryango_i18n::Locale;

use crate::RootView;
use crate::common::{
    AreaChart, Avatar, AvatarFallback, AvatarSize, Badge, Button, ButtonSize, Card, CardAction,
    CardContent, CardHeader, CardSize, ChartConfig, ChartContainer, Empty, EmptyDescription,
    EmptyHeader, EmptyMedia, PresetRadius, StyleComposeExt, Table, TableBody, TableCell, TableHead,
    TableHeader, TableRow,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::Screen;
use crate::theme::SemanticTheme;

use super::cashflow_row::render_period_dropdown;
use super::metrics::{DashboardLayoutTier, DashboardSnapshot, RecentActivityRow, WeeklyHighlight};

/// Renders Row 4 (`Weekly Activity` + `Recent Custody Activity`).
pub fn render_activity_row(
    snap: &DashboardSnapshot,
    tier: DashboardLayoutTier,
    locale: Locale,
    now: DateTime<Utc>,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let gap = px(tier.grid_gap_px());
    let weekly_card = render_weekly_activity_card(snap, locale, theme, cx);
    let recent_table_card = render_recent_activity_card(snap, locale, now, theme, cx);

    match tier {
        DashboardLayoutTier::Wide => div()
            .flex()
            .flex_row()
            .items_stretch()
            .w_full()
            .gap(gap)
            .child(div().w(relative(0.324)).flex_shrink_0().child(weekly_card))
            .child(div().flex_1().min_w_0().child(recent_table_card))
            .into_any_element(),
        DashboardLayoutTier::Medium
        | DashboardLayoutTier::Compact
        | DashboardLayoutTier::NarrowMobile => div()
            .flex()
            .flex_col()
            .w_full()
            .gap(gap)
            .child(weekly_card)
            .child(recent_table_card)
            .into_any_element(),
    }
}

fn render_weekly_activity_card(
    snap: &DashboardSnapshot,
    locale: Locale,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let is_dark = theme.mode.is_dark();
    let accent = if is_dark {
        theme.primary_hover
    } else {
        theme.primary
    };
    let on_primary = if is_dark {
        IconTone::Default
    } else {
        IconTone::OnPrimaryButton
    };

    let chart = AreaChart::new(snap.weekly_points.clone())
        .id("dash-weekly-activity-area")
        .interactive(false)
        .x(|p| p.label.clone())
        .y(|p| p.value)
        .y_domain(0.0, snap.weekly_max)
        .stroke(accent)
        .fill(linear_gradient(
            180.0,
            linear_color_stop(theme.primary.opacity(0.30), 0.0),
            linear_color_stop(theme.primary.opacity(0.02), 1.0),
        ))
        .linear()
        .x_axis(false)
        .y_axis(false)
        .grid(true)
        .grid_dashed(false);

    let inflow_pill_bg = if is_dark {
        theme.primary_deep.opacity(0.55)
    } else {
        theme.primary.opacity(0.10)
    };
    let expense_pill_bg = if is_dark {
        theme.primary_deep.opacity(0.35)
    } else {
        theme.bg_canvas
    };

    Card::new()
        .size(CardSize::Sm)
        .h_full()
        .py(px(20.0))
        .gap(px(18.0))
        .justify_between()
        .border_1()
        .border_color(theme.border_subtle)
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
                        .child(umuryango_i18n::t(locale, "dashboard.weekly_activity")),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(umuryango_i18n::t(locale, "dashboard.average_spending")),
                ),
        )
        .child(
            CardContent::new().size(CardSize::Sm).child(
                ChartContainer::new("dash-weekly-activity-frame", ChartConfig::new())
                    .min_height(136.0)
                    .h(px(136.0))
                    .theme(theme)
                    .child(chart),
            ),
        )
        .child(
            CardContent::new().size(CardSize::Sm).child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(render_weekly_highlight_row(
                        "dash-wk-inflow",
                        AppIcon::ArrowUp,
                        on_primary,
                        theme.primary,
                        theme.primary_pressed,
                        &umuryango_i18n::t(locale, "dashboard.top_inflow"),
                        &snap.top_inflow,
                        inflow_pill_bg,
                        theme.primary.opacity(0.45),
                        true,
                        theme,
                        cx,
                    ))
                    .child(render_weekly_highlight_row(
                        "dash-wk-expense",
                        AppIcon::ArrowDown,
                        if is_dark {
                            IconTone::Default
                        } else {
                            IconTone::Expense
                        },
                        if is_dark {
                            theme.primary_deep
                        } else {
                            theme.primary.opacity(0.12)
                        },
                        theme.primary.opacity(0.40),
                        &umuryango_i18n::t(locale, "dashboard.top_expense"),
                        &snap.top_expense,
                        expense_pill_bg,
                        theme.primary.opacity(0.35),
                        false,
                        theme,
                        cx,
                    ))
                    .child(render_weekly_highlight_row(
                        "dash-wk-handover",
                        AppIcon::Transactions,
                        IconTone::Default,
                        theme.bg_canvas,
                        theme.border_subtle,
                        &umuryango_i18n::t(locale, "dashboard.latest_handover"),
                        &snap.latest_handover,
                        theme.bg_canvas,
                        theme.border_subtle,
                        false,
                        theme,
                        cx,
                    )),
            ),
        )
}

#[allow(clippy::too_many_arguments)]
fn render_weekly_highlight_row(
    id: &'static str,
    icon: AppIcon,
    icon_tone: IconTone,
    icon_bg: Hsla,
    icon_border: Hsla,
    title: &str,
    item: &WeeklyHighlight,
    badge_bg: Hsla,
    badge_border: Hsla,
    destructive_badge: bool,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(px(10.0))
        .cursor_pointer()
        .on_click(cx.listener(|this, _, _, cx| {
            this.select_screen(Screen::Transactions, cx);
        }))
        .child(
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
                        .size(px(32.0))
                        .bg(icon_bg)
                        .border_1()
                        .border_color(icon_border)
                        .child(app_icon(icon, IconSize::Xs, icon_tone, theme)),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .truncate()
                                .child(title.to_string()),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(theme.text_muted)
                                .truncate()
                                .child(item.subtitle.clone()),
                        ),
                ),
        )
        .child(
            Badge::new(item.badge_text.clone())
                .cx_when(destructive_badge, |b| b.destructive())
                .px(px(8.0))
                .py(px(4.0))
                .bg(badge_bg)
                .border_1()
                .border_color(badge_border)
                .theme(theme),
        )
}

fn render_recent_activity_card(
    snap: &DashboardSnapshot,
    locale: Locale,
    now: DateTime<Utc>,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let subtitle_key = if snap.is_family {
        "dashboard.authorized_family_txs"
    } else {
        "dashboard.personal_ledger_txs"
    };

    let actions = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .child(render_period_dropdown(
            "dash-recent-activity-period-menu",
            snap.active_period,
            &snap.active_period_label,
            locale,
            now,
            theme,
            cx,
        ))
        .child(
            Button::new("dash-recent-quick-add-btn")
                .size(ButtonSize::IconXs)
                .w(px(30.0))
                .h(px(30.0))
                .icon(AppIcon::Add)
                .theme(theme)
                .on_click(cx.listener(|this, _, window, cx| {
                    this.open_quick_entry(window, cx);
                })),
        );

    Card::new()
        .size(CardSize::Sm)
        .h_full()
        .py(px(20.0))
        .gap(px(16.0))
        .border_1()
        .border_color(theme.border_subtle)
        .theme(theme)
        .child(
            CardHeader::new()
                .size(CardSize::Sm)
                .action(CardAction::new().child(actions))
                .theme(theme)
                .child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(umuryango_i18n::t(
                            locale,
                            "dashboard.recent_custody_activity",
                        )),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(umuryango_i18n::t(locale, subtitle_key)),
                ),
        )
        .child(
            CardContent::new()
                .size(CardSize::Sm)
                .child(if snap.recent_rows.is_empty() {
                    Empty::new()
                        .bordered(true)
                        .p(px(28.0))
                        .bg(theme.bg_canvas)
                        .theme(theme)
                        .child(
                            EmptyHeader::new()
                                .child(EmptyMedia::from_icon(AppIcon::Transactions).theme(theme))
                                .child(
                                    EmptyDescription::new(umuryango_i18n::t(
                                        locale,
                                        "dashboard.no_recent_activity",
                                    ))
                                    .theme(theme),
                                ),
                        )
                        .child(
                            Button::new("dash-empty-log-tx-btn")
                                .size(ButtonSize::Sm)
                                .icon_start(AppIcon::Add)
                                .label(umuryango_i18n::t(locale, "dashboard.quick_log_btn"))
                                .theme(theme)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.open_quick_entry(window, cx);
                                })),
                        )
                        .into_any_element()
                } else {
                    let row_count = snap.recent_rows.len();
                    Table::new("dash-recent-custody-table")
                        .min_w(px(540.0))
                        .theme(theme)
                        .child(
                            TableHeader::new().theme(theme).child(
                                TableRow::new()
                                    .border_bottom(false)
                                    .hoverable(false)
                                    .theme(theme)
                                    .child(
                                        TableHead::new()
                                            .label(umuryango_i18n::t(
                                                locale,
                                                "dashboard.col_custodian",
                                            ))
                                            .flex_none()
                                            .w(relative(0.31))
                                            .px(px(8.0))
                                            .h(px(36.0))
                                            .theme(theme),
                                    )
                                    .child(
                                        TableHead::new()
                                            .label(umuryango_i18n::t(
                                                locale,
                                                "dashboard.col_category_note",
                                            ))
                                            .px(px(8.0))
                                            .h(px(36.0))
                                            .theme(theme),
                                    )
                                    .child(
                                        TableHead::new()
                                            .label(umuryango_i18n::t(locale, "dashboard.col_type"))
                                            .flex_none()
                                            .w(px(116.0))
                                            .px(px(8.0))
                                            .h(px(36.0))
                                            .theme(theme),
                                    )
                                    .child(
                                        TableHead::new()
                                            .label(umuryango_i18n::t(
                                                locale,
                                                "dashboard.col_amount_rwf",
                                            ))
                                            .align_right()
                                            .flex_none()
                                            .w(px(148.0))
                                            .px(px(8.0))
                                            .h(px(36.0))
                                            .theme(theme),
                                    ),
                            ),
                        )
                        .child(
                            TableBody::new().children(snap.recent_rows.iter().enumerate().map(
                                |(idx, row)| {
                                    render_recent_table_row(
                                        idx,
                                        idx + 1 == row_count,
                                        row,
                                        theme,
                                        cx,
                                    )
                                },
                            )),
                        )
                        .into_any_element()
                }),
        )
}

fn render_recent_table_row(
    idx: usize,
    is_last: bool,
    row: &RecentActivityRow,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> TableRow {
    let is_dark = theme.mode.is_dark();
    let on_crimson = if is_dark {
        theme.text_primary
    } else {
        theme.bg_surface
    };
    let is_handover = row.tx_type == TransactionType::Handover;
    let is_income = row.tx_type == TransactionType::Income;

    let (avatar_bg, avatar_fg, avatar_border) = if is_handover {
        (theme.bg_canvas, theme.text_primary, theme.border_subtle)
    } else {
        (theme.primary, on_crimson, theme.primary_pressed)
    };

    let (badge_bg, badge_border) = match row.tx_type {
        TransactionType::Income => (
            if is_dark {
                theme.primary_deep.opacity(0.55)
            } else {
                theme.primary.opacity(0.10)
            },
            theme.primary.opacity(0.45),
        ),
        TransactionType::Expense => (
            if is_dark {
                theme.primary_deep.opacity(0.32)
            } else {
                theme.bg_canvas
            },
            theme.primary.opacity(0.35),
        ),
        TransactionType::Handover => (theme.bg_canvas, theme.border_subtle),
    };

    let amount_color = if is_income {
        if is_dark {
            theme.primary_hover
        } else {
            theme.primary
        }
    } else {
        theme.text_primary
    };

    TableRow::new()
        .id(format!("dash-recent-tx-row-{}", idx))
        .last(is_last)
        .theme(theme)
        .on_click(cx.listener(|this, _, _, cx| {
            this.select_screen(Screen::Transactions, cx);
        }))
        // Custodian cell
        .child(
            TableCell::new()
                .flex_none()
                .w(relative(0.31))
                .px(px(8.0))
                .py(px(12.0))
                .theme(theme)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(10.0))
                        .min_w_0()
                        .child(
                            Avatar::new()
                                .size(AvatarSize::Default)
                                .rounded(PresetRadius::NONE)
                                .border_color(avatar_border)
                                .theme(theme)
                                .child(
                                    AvatarFallback::new(row.initials.clone())
                                        .size(AvatarSize::Default)
                                        .rounded(PresetRadius::NONE)
                                        .bg(avatar_bg)
                                        .text_color(avatar_fg)
                                        .font_weight(FontWeight::BOLD)
                                        .theme(theme),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .min_w_0()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .truncate()
                                        .child(row.actor_name.clone()),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(theme.text_muted)
                                        .truncate()
                                        .child(row.actor_role_label.clone()),
                                ),
                        ),
                ),
        )
        // Category / Note cell
        .child(
            TableCell::new()
                .px(px(8.0))
                .py(px(12.0))
                .theme(theme)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text_secondary)
                        .truncate()
                        .child(row.category_note.clone()),
                ),
        )
        // Type badge cell
        .child(
            TableCell::new()
                .flex_none()
                .w(px(116.0))
                .px(px(8.0))
                .py(px(12.0))
                .theme(theme)
                .child(
                    Badge::new(row.type_badge_label.clone())
                        .cx_when(is_income, |b| b.destructive())
                        .px(px(10.0))
                        .py(px(4.0))
                        .bg(badge_bg)
                        .border_1()
                        .border_color(badge_border)
                        .theme(theme),
                ),
        )
        // Right-aligned signed RWF amount cell
        .child(
            TableCell::new()
                .align_right()
                .flex_none()
                .w(px(148.0))
                .px(px(8.0))
                .py(px(12.0))
                .theme(theme)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(amount_color)
                        .child(row.amount_label.clone()),
                ),
        )
}

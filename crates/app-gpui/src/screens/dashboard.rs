//! Responsive Bento Family & Personal Dashboard (`Screen::Dashboard`).
//!
//! Composes shared UI and chart primitives (`Card`, `BarChart`, `AreaChart`, `PieChart`,
//! `ChartContainer`, `Button`, `DropdownMenu`, `Avatar`, `Badge`, `Progress`, `Separator`,
//! `Table`, `Empty`, `app_icon`) across 4 responsive analytical rows.

#[path = "dashboard/activity_row.rs"]
pub mod activity_row;
#[path = "dashboard/bento_row.rs"]
pub mod bento_row;
#[path = "dashboard/cashflow_row.rs"]
pub mod cashflow_row;
#[path = "dashboard/formatters.rs"]
pub mod formatters;
#[path = "dashboard/metrics.rs"]
pub mod metrics;
#[path = "dashboard/projects_hero.rs"]
pub mod projects_hero;

use gpui_kit::prelude::*;
use gpui_kit::{
    Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, Window, div, px,
};
use rust_decimal::Decimal;

use crate::RootView;
use crate::common::{Card, CardContent, CardSize, PresetRadius, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::hooks::use_mobile::viewport_width;
use crate::state::{ActiveModal, AppState, Screen};
use crate::theme::SemanticTheme;

use self::activity_row::render_activity_row;
use self::bento_row::render_bento_row;
use self::cashflow_row::render_cashflow_row;
use self::metrics::{DESKTOP_SIDEBAR_WIDTH_PX, DashboardLayoutTier, DashboardSnapshot, format_rwf};

#[derive(Debug, Clone, Copy)]
enum KpiBadgeStyle {
    PrimarySolid,
    DeepCrimson,
    DarkBordered,
}

#[derive(Debug, Clone, Copy)]
enum KpiTarget {
    CustodyWallets,
    Transactions,
    ProjectsOrPlans,
    Loans,
    MembersOrWallets,
}

struct KpiCardSpec<'a> {
    id: &'static str,
    icon: AppIcon,
    badge_style: KpiBadgeStyle,
    label: &'a str,
    value: &'a str,
    subtitle: &'a str,
    target: KpiTarget,
    is_family: bool,
    wallets_empty: bool,
}

pub struct DashboardScreen;

impl DashboardScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        window: &Window,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let raw_vw = viewport_width(window);
        let sidebar_w = if state.sidebar_open {
            DESKTOP_SIDEBAR_WIDTH_PX
        } else {
            0.0
        };
        let content_width = if raw_vw > 0.0 {
            (raw_vw - sidebar_w - state.safe_area.left - state.safe_area.right).max(320.0)
        } else if gpui_kit::is_mobile() {
            375.0
        } else {
            1200.0
        };

        let tier = DashboardLayoutTier::from_content_width(content_width);
        let pad = px(tier.outer_padding_px());
        let gap = px(tier.grid_gap_px());
        let now = chrono::Utc::now();
        let snap = DashboardSnapshot::compute(state, now);

        div()
            .id(id)
            .flex_1()
            .h_full()
            .overflow_y_scroll()
            .pt(pad)
            .pb(pad + state.safe_area.total_bottom_px())
            .pl(pad + state.safe_area.left_px())
            .pr(pad + state.safe_area.right_px())
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .gap(gap)
                    .child(Self::render_kpi_row(state, &snap, tier, theme, cx))
                    .child(render_cashflow_row(
                        &snap,
                        tier,
                        state.locale,
                        now,
                        theme,
                        cx,
                    ))
                    .child(render_bento_row(state, &snap, tier, theme, cx))
                    .child(render_activity_row(
                        &snap,
                        tier,
                        state.locale,
                        now,
                        theme,
                        cx,
                    )),
            )
    }

    fn render_kpi_row(
        state: &AppState,
        snap: &DashboardSnapshot,
        tier: DashboardLayoutTier,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let locale = state.locale;
        let gap = px(tier.grid_gap_px());

        let total_custody_label = umuryango_i18n::t(locale, "dashboard.kpi_total_custody");
        let total_custody_value = format_rwf(snap.total_custody);
        let total_custody_sub = format!(
            "{} {}",
            snap.active_wallets_count,
            umuryango_i18n::t(locale, "dashboard.kpi_active_wallets")
        );

        let inflow_label = umuryango_i18n::t(locale, "dashboard.kpi_monthly_inflow");
        let inflow_value = format_rwf(snap.period_inflow);

        let projects_kpi_label = if snap.is_family {
            umuryango_i18n::t(locale, "dashboard.kpi_active_projects")
        } else {
            umuryango_i18n::t(locale, "dashboard.kpi_active_plans")
        };
        let projects_value = snap.active_projects_or_plans_count.to_string();
        let projects_target_total: Decimal = if snap.is_family {
            state.active_projects_budget()
        } else {
            state.future_plans.iter().map(|p| p.estimated_cost).sum()
        };
        let projects_sub = format_rwf(projects_target_total);

        let loans_label = umuryango_i18n::t(locale, "dashboard.kpi_active_loans");
        let loans_value = snap.active_loans_count.to_string();
        let loans_remaining: Decimal = state
            .loans
            .iter()
            .map(|l| l.balance_remaining.max(Decimal::ZERO))
            .sum();
        let loans_sub = format_rwf(loans_remaining);

        let members_kpi_label = if snap.is_family {
            umuryango_i18n::t(locale, "dashboard.kpi_family_members")
        } else {
            umuryango_i18n::t(locale, "dashboard.kpi_active_wallets")
        };
        let members_value = snap.members_or_wallets_count.to_string();
        let members_sub = if snap.is_family {
            state
                .active_family_name
                .clone()
                .unwrap_or_else(|| umuryango_i18n::t(locale, "dashboard.family_pool"))
        } else {
            umuryango_i18n::t(locale, "dashboard.bank_momo_pool")
        };

        div()
            .grid()
            .grid_cols(tier.kpi_columns())
            .gap(gap)
            .w_full()
            .child(Self::render_kpi_card(
                KpiCardSpec {
                    id: "dash-kpi-total-custody",
                    icon: AppIcon::PersonalWallet,
                    badge_style: KpiBadgeStyle::PrimarySolid,
                    label: &total_custody_label,
                    value: &total_custody_value,
                    subtitle: &total_custody_sub,
                    target: KpiTarget::CustodyWallets,
                    is_family: snap.is_family,
                    wallets_empty: snap.total_wallets_count == 0,
                },
                theme,
                cx,
            ))
            .child(Self::render_kpi_card(
                KpiCardSpec {
                    id: "dash-kpi-monthly-inflow",
                    icon: AppIcon::Income,
                    badge_style: KpiBadgeStyle::DeepCrimson,
                    label: &inflow_label,
                    value: &inflow_value,
                    subtitle: &snap.active_period_label,
                    target: KpiTarget::Transactions,
                    is_family: snap.is_family,
                    wallets_empty: false,
                },
                theme,
                cx,
            ))
            .child(Self::render_kpi_card(
                KpiCardSpec {
                    id: "dash-kpi-active-projects",
                    icon: if snap.is_family {
                        AppIcon::Projects
                    } else {
                        AppIcon::FuturePlans
                    },
                    badge_style: KpiBadgeStyle::PrimarySolid,
                    label: &projects_kpi_label,
                    value: &projects_value,
                    subtitle: &projects_sub,
                    target: KpiTarget::ProjectsOrPlans,
                    is_family: snap.is_family,
                    wallets_empty: false,
                },
                theme,
                cx,
            ))
            .child(Self::render_kpi_card(
                KpiCardSpec {
                    id: "dash-kpi-active-loans",
                    icon: AppIcon::Cash,
                    badge_style: KpiBadgeStyle::DarkBordered,
                    label: &loans_label,
                    value: &loans_value,
                    subtitle: &loans_sub,
                    target: KpiTarget::Loans,
                    is_family: snap.is_family,
                    wallets_empty: false,
                },
                theme,
                cx,
            ))
            .child(Self::render_kpi_card(
                KpiCardSpec {
                    id: "dash-kpi-family-members",
                    icon: if snap.is_family {
                        AppIcon::Members
                    } else {
                        AppIcon::PersonalWallet
                    },
                    badge_style: KpiBadgeStyle::PrimarySolid,
                    label: &members_kpi_label,
                    value: &members_value,
                    subtitle: &members_sub,
                    target: KpiTarget::MembersOrWallets,
                    is_family: snap.is_family,
                    wallets_empty: false,
                },
                theme,
                cx,
            ))
    }

    fn render_kpi_card(
        spec: KpiCardSpec<'_>,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_dark = theme.mode.is_dark();
        let on_primary = if is_dark {
            IconTone::Default
        } else {
            IconTone::OnPrimaryButton
        };

        let (badge_bg, badge_border, icon_tone) = match spec.badge_style {
            KpiBadgeStyle::PrimarySolid => (theme.primary, theme.primary_pressed, on_primary),
            KpiBadgeStyle::DeepCrimson => (
                if is_dark {
                    theme.primary_deep
                } else {
                    theme.primary_pressed
                },
                theme.primary.opacity(0.60),
                on_primary,
            ),
            KpiBadgeStyle::DarkBordered => (
                theme.bg_canvas,
                theme.primary.opacity(0.40),
                IconTone::Expense,
            ),
        };

        let target = spec.target;
        let is_family = spec.is_family;
        let wallets_empty = spec.wallets_empty;

        div()
            .id(spec.id)
            .h_full()
            .border_1()
            .border_color(theme.border_subtle)
            .cursor_pointer()
            .hover(|s| s.border_color(theme.primary.opacity(0.65)))
            .on_click(cx.listener(move |this, _, window, cx| match target {
                KpiTarget::CustodyWallets => {
                    if wallets_empty {
                        this.open_modal(ActiveModal::AddWallet, window, cx);
                    } else {
                        this.select_screen(Screen::Reports, cx);
                    }
                }
                KpiTarget::Transactions => this.select_screen(Screen::Transactions, cx),
                KpiTarget::ProjectsOrPlans => this.select_screen(
                    if is_family {
                        Screen::Projects
                    } else {
                        Screen::Plans
                    },
                    cx,
                ),
                KpiTarget::Loans => this.select_screen(Screen::Loans, cx),
                KpiTarget::MembersOrWallets => {
                    if is_family {
                        this.select_screen(Screen::Members, cx);
                    } else {
                        this.open_modal(ActiveModal::AddWallet, window, cx);
                    }
                }
            }))
            .child(
                Card::new()
                    .size(CardSize::Sm)
                    .h_full()
                    .py(px(16.0))
                    .gap(px(0.0))
                    .theme(theme)
                    .child(
                        CardContent::new()
                            .size(CardSize::Sm)
                            .px(px(18.0))
                            .h_full()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .justify_between()
                                    .items_start()
                                    .w_full()
                                    .h_full()
                                    .gap(px(14.0))
                                    // Top Row: Uppercase Label (left) + Square Accent Icon Badge (right)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .gap(px(10.0))
                                            .w_full()
                                            .child(
                                                div()
                                                    .min_w_0()
                                                    .flex_1()
                                                    .truncate()
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.text_muted)
                                                    .child(to_preset_uppercase(spec.label)),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_shrink_0()
                                                    .items_center()
                                                    .justify_center()
                                                    .size(px(36.0))
                                                    .rounded(PresetRadius::NONE)
                                                    .bg(badge_bg)
                                                    .border_1()
                                                    .border_color(badge_border)
                                                    .child(app_icon(
                                                        spec.icon,
                                                        IconSize::Md,
                                                        icon_tone,
                                                        theme,
                                                    )),
                                            ),
                                    )
                                    // Bottom Stack: Bold Primary Value + Supporting Subtitle
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_start()
                                            .gap(px(4.0))
                                            .w_full()
                                            .min_w_0()
                                            .child(
                                                div()
                                                    .w_full()
                                                    .truncate()
                                                    .text_xl()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.text_primary)
                                                    .child(spec.value.to_string()),
                                            )
                                            .child(
                                                div()
                                                    .w_full()
                                                    .truncate()
                                                    .text_xs()
                                                    .text_color(theme.text_muted)
                                                    .child(spec.subtitle.to_string()),
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}

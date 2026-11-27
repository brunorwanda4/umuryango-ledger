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

        let projects_kpi_label = if snap.is_family {
            umuryango_i18n::t(locale, "dashboard.kpi_active_projects")
        } else {
            umuryango_i18n::t(locale, "dashboard.kpi_active_plans")
        };
        let members_kpi_label = if snap.is_family {
            umuryango_i18n::t(locale, "dashboard.kpi_family_members")
        } else {
            umuryango_i18n::t(locale, "dashboard.kpi_active_wallets")
        };

        div()
            .grid()
            .grid_cols(tier.kpi_columns())
            .gap(gap)
            .w_full()
            .child(Self::render_kpi_card(
                "dash-kpi-total-custody",
                AppIcon::PersonalWallet,
                KpiBadgeStyle::PrimarySolid,
                &umuryango_i18n::t(locale, "dashboard.kpi_total_custody"),
                &format_rwf(snap.total_custody),
                KpiTarget::CustodyWallets,
                snap.is_family,
                snap.total_wallets_count == 0,
                theme,
                cx,
            ))
            .child(Self::render_kpi_card(
                "dash-kpi-monthly-inflow",
                AppIcon::Income,
                KpiBadgeStyle::DeepCrimson,
                &umuryango_i18n::t(locale, "dashboard.kpi_monthly_inflow"),
                &format_rwf(snap.period_inflow),
                KpiTarget::Transactions,
                snap.is_family,
                false,
                theme,
                cx,
            ))
            .child(Self::render_kpi_card(
                "dash-kpi-active-projects",
                if snap.is_family {
                    AppIcon::Projects
                } else {
                    AppIcon::FuturePlans
                },
                KpiBadgeStyle::PrimarySolid,
                &projects_kpi_label,
                &snap.active_projects_or_plans_count.to_string(),
                KpiTarget::ProjectsOrPlans,
                snap.is_family,
                false,
                theme,
                cx,
            ))
            .child(Self::render_kpi_card(
                "dash-kpi-active-loans",
                AppIcon::Cash,
                KpiBadgeStyle::DarkBordered,
                &umuryango_i18n::t(locale, "dashboard.kpi_active_loans"),
                &snap.active_loans_count.to_string(),
                KpiTarget::Loans,
                snap.is_family,
                false,
                theme,
                cx,
            ))
            .child(Self::render_kpi_card(
                "dash-kpi-family-members",
                if snap.is_family {
                    AppIcon::Members
                } else {
                    AppIcon::PersonalWallet
                },
                KpiBadgeStyle::PrimarySolid,
                &members_kpi_label,
                &snap.members_or_wallets_count.to_string(),
                KpiTarget::MembersOrWallets,
                snap.is_family,
                false,
                theme,
                cx,
            ))
    }

    #[allow(clippy::too_many_arguments)]
    fn render_kpi_card(
        id: &'static str,
        icon: AppIcon,
        badge_style: KpiBadgeStyle,
        label: &str,
        value: &str,
        target: KpiTarget,
        is_family: bool,
        wallets_empty: bool,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_dark = theme.mode.is_dark();
        let on_primary = if is_dark {
            IconTone::Default
        } else {
            IconTone::OnPrimaryButton
        };

        let (badge_bg, badge_border, icon_tone) = match badge_style {
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

        div()
            .id(id)
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
                    .py(px(18.0))
                    .gap(px(0.0))
                    .theme(theme)
                    .child(
                        CardContent::new().size(CardSize::Sm).px(px(14.0)).child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .gap(px(10.0))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .size(px(36.0))
                                        .rounded(PresetRadius::NONE)
                                        .bg(badge_bg)
                                        .border_1()
                                        .border_color(badge_border)
                                        .child(app_icon(icon, IconSize::Sm, icon_tone, theme)),
                                )
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_muted)
                                        .child(to_preset_uppercase(label)),
                                )
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(value.to_string()),
                                ),
                        ),
                    ),
            )
    }
}

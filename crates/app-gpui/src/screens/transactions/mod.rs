pub mod analytics;
pub mod table;

use gpui_kit::prelude::*;
use gpui_kit::{
    Context, ElementId, FontWeight, Hsla, IntoElement, ParentElement, Styled, Window, div, px,
};
use umuryango_core_domain::models::CategoryKind;
use umuryango_i18n::{t, t_args};

use crate::RootView;
use crate::common::{Button, ButtonSize, Tabs, TabsContent, TabsTrigger};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::components::money::{MoneyCategoryForm, MoneyCategoryInputs};
use crate::components::quick_entry::QuickEntryMode;
use crate::hooks::use_mobile::Breakpoint;
use crate::screens::transactions::analytics::{
    compute_money_metrics, format_rwf, render_cashflow_card, render_categories_tab,
    render_category_share_card,
};
use crate::screens::transactions::table::{ActivityTableMode, render_activity_table_card};
use crate::state::{AppState, MoneyTab};
use crate::theme::SemanticTheme;

#[derive(Clone, Copy)]
enum KpiIconBadge {
    IncomeBordered,
    ExpenseBordered,
    PrimarySolid,
    MutedBordered,
}

pub struct TransactionsScreen;

impl TransactionsScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        category_inputs: &MoneyCategoryInputs,
        theme: &SemanticTheme,
        window: &Window,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let locale = state.locale;
        let is_family = state.active_scope.is_family();
        let bp = Breakpoint::from_window(window);
        let is_mobile = bp.is_mobile();
        let pad = if is_mobile { px(16.0) } else { px(24.0) };

        let page_title = if is_family {
            t(locale, "money_page.family_title")
        } else {
            t(locale, "money_page.personal_title")
        };
        let page_subtitle = if is_family {
            t(locale, "money_page.family_subtitle")
        } else {
            t(locale, "money_page.personal_subtitle")
        };

        let metrics = compute_money_metrics(state);
        let inc_cnt_str = metrics.income_count.to_string();
        let exp_cnt_str = metrics.expense_count.to_string();
        let total_cat_str = metrics.total_categories.to_string();
        let inc_cat_str = metrics.income_category_count.to_string();
        let exp_cat_str = metrics.expense_category_count.to_string();

        let kpi1 = Self::render_kpi_card(
            "money-kpi-income",
            if is_family {
                t(locale, "money_page.kpi_family_income")
            } else {
                t(locale, "money_page.kpi_personal_income")
            },
            format!("{} RWF", format_rwf(metrics.total_income)),
            theme.income_color,
            t_args(
                locale,
                "money_page.kpi_inflows_sub",
                &[("count", &inc_cnt_str)],
            ),
            AppIcon::Income,
            KpiIconBadge::IncomeBordered,
            MoneyTab::Income,
            theme,
            cx,
        );

        let kpi2 = Self::render_kpi_card(
            "money-kpi-expenses",
            if is_family {
                t(locale, "money_page.kpi_family_expenses")
            } else {
                t(locale, "money_page.kpi_personal_expenses")
            },
            format!("{} RWF", format_rwf(metrics.total_expenses)),
            theme.expense_color,
            if is_family {
                t_args(
                    locale,
                    "money_page.kpi_family_outflows_sub",
                    &[("count", &exp_cnt_str)],
                )
            } else {
                t_args(
                    locale,
                    "money_page.kpi_personal_outflows_sub",
                    &[("count", &exp_cnt_str)],
                )
            },
            AppIcon::Expense,
            KpiIconBadge::ExpenseBordered,
            MoneyTab::Expenses,
            theme,
            cx,
        );

        let kpi3 = Self::render_kpi_card(
            "money-kpi-net",
            if is_family {
                t(locale, "money_page.kpi_family_net")
            } else {
                t(locale, "money_page.kpi_personal_net")
            },
            format!("{} RWF", format_rwf(metrics.net_balance)),
            theme.text_primary,
            if is_family {
                t(locale, "money_page.kpi_family_net_sub")
            } else {
                t(locale, "money_page.kpi_personal_net_sub")
            },
            AppIcon::PersonalWallet,
            KpiIconBadge::PrimarySolid,
            MoneyTab::Overview,
            theme,
            cx,
        );

        let kpi4 = Self::render_kpi_card(
            "money-kpi-categories",
            t(locale, "money_page.kpi_active_categories"),
            t_args(
                locale,
                "money_page.kpi_categories_value",
                &[("count", &total_cat_str)],
            ),
            theme.text_primary,
            t_args(
                locale,
                "money_page.kpi_categories_sub",
                &[("income", &inc_cat_str), ("expenses", &exp_cat_str)],
            ),
            AppIcon::Transactions,
            KpiIconBadge::MutedBordered,
            MoneyTab::Categories,
            theme,
            cx,
        );

        let kpi_grid = match bp {
            Breakpoint::Desktop => div()
                .w_full()
                .flex()
                .flex_row()
                .gap(px(16.0))
                .child(div().flex_1().min_w_0().child(kpi1))
                .child(div().flex_1().min_w_0().child(kpi2))
                .child(div().flex_1().min_w_0().child(kpi3))
                .child(div().flex_1().min_w_0().child(kpi4)),
            Breakpoint::Tablet => div()
                .w_full()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    div()
                        .w_full()
                        .flex()
                        .flex_row()
                        .gap(px(12.0))
                        .child(div().flex_1().min_w_0().child(kpi1))
                        .child(div().flex_1().min_w_0().child(kpi2)),
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .flex_row()
                        .gap(px(12.0))
                        .child(div().flex_1().min_w_0().child(kpi3))
                        .child(div().flex_1().min_w_0().child(kpi4)),
                ),
            Breakpoint::Mobile => div()
                .w_full()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(kpi1)
                .child(kpi2)
                .child(kpi3)
                .child(kpi4),
        };

        let overview_charts_row = if matches!(bp, Breakpoint::Desktop) {
            div()
                .w_full()
                .flex()
                .flex_row()
                .items_stretch()
                .gap(px(16.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .child(render_cashflow_card(state, is_family, false, theme)),
                )
                .child(
                    div()
                        .w(px(430.0))
                        .flex_shrink_0()
                        .child(render_category_share_card(
                            "money-overview-pie",
                            state,
                            is_family,
                            theme,
                            cx,
                        )),
                )
        } else {
            div()
                .w_full()
                .flex()
                .flex_col()
                .gap(px(16.0))
                .child(render_cashflow_card(state, is_family, is_mobile, theme))
                .child(render_category_share_card(
                    "money-overview-pie",
                    state,
                    is_family,
                    theme,
                    cx,
                ))
        };

        let overview_panel = div()
            .w_full()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .child(overview_charts_row)
            .child(render_activity_table_card(
                "money-overview-activity-table",
                ActivityTableMode::OverviewRecent,
                state,
                is_family,
                theme,
                cx,
            ));

        let income_panel = render_activity_table_card(
            "money-income-ledger-table",
            ActivityTableMode::IncomeLedger,
            state,
            is_family,
            theme,
            cx,
        );

        let expenses_panel = render_activity_table_card(
            "money-expenses-ledger-table",
            ActivityTableMode::ExpensesLedger,
            state,
            is_family,
            theme,
            cx,
        );

        let categories_panel =
            render_categories_tab(state, category_inputs, is_family, is_mobile, theme, cx);

        let view_for_tabs = cx.entity().clone();
        let tabs = Tabs::new("money-screen-tabs")
            .line()
            .active_index(state.money_tab.to_index())
            .theme(theme)
            .on_change(move |idx, _window, cx| {
                let next_tab = MoneyTab::from_index(*idx);
                view_for_tabs.update(cx, |this, cx| {
                    this.set_money_tab(next_tab, cx);
                });
            })
            .tab(
                TabsTrigger::new(t(locale, "money_page.tab_overview")),
                TabsContent::new().child(overview_panel),
            )
            .tab(
                TabsTrigger::new(t(locale, "money_page.tab_income")),
                TabsContent::new().child(income_panel),
            )
            .tab(
                TabsTrigger::new(t(locale, "money_page.tab_expenses")),
                TabsContent::new().child(expenses_panel),
            )
            .tab(
                TabsTrigger::new(t(locale, "money_page.tab_categories")),
                TabsContent::new().child(categories_panel),
            );

        div()
            .id(id)
            .flex_1()
            .h_full()
            .pt(pad)
            .pb(pad + state.safe_area.total_bottom_px())
            .pl(pad + state.safe_area.left_px())
            .pr(pad + state.safe_area.right_px())
            .overflow_y_scroll()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            .flex()
            .flex_col()
            .gap(px(20.0))
            // Top Page Header + 3 Action Buttons
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .justify_between()
                    .items_center()
                    .gap(px(12.0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(page_title),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(page_subtitle),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap(px(8.0))
                            .child(MoneyCategoryForm::render(
                                "transactions-header-category-form",
                                CategoryKind::Expense,
                                state,
                                category_inputs,
                                theme,
                                cx,
                            ))
                            .child(
                                Button::new("money-header-add-income")
                                    .outline()
                                    .size(ButtonSize::Sm)
                                    .icon_start(AppIcon::Income)
                                    .label(t(locale, "money_page.add_income_btn"))
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_money_entry(QuickEntryMode::Income, window, cx);
                                    })),
                            )
                            .child(
                                Button::new("money-header-log-expense")
                                    .size(ButtonSize::Sm)
                                    .icon_start(AppIcon::Add)
                                    .label(t(locale, "money_page.log_expense_btn"))
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_money_entry(QuickEntryMode::Expense, window, cx);
                                    })),
                            ),
                    ),
            )
            // 4 KPI Summary Cards
            .child(kpi_grid)
            // 4 Underline Tabs (OVERVIEW, INCOME, EXPENSES, CATEGORIES)
            .child(tabs)
    }

    #[allow(clippy::too_many_arguments)]
    fn render_kpi_card(
        id: &'static str,
        label: String,
        value: String,
        value_color: Hsla,
        subtitle: String,
        icon: AppIcon,
        badge: KpiIconBadge,
        target_tab: MoneyTab,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_dark = theme.mode.is_dark();
        let (badge_bg, badge_border, tone) = match badge {
            KpiIconBadge::IncomeBordered => {
                (theme.bg_canvas, theme.border_subtle, IconTone::Success)
            }
            KpiIconBadge::ExpenseBordered => {
                (theme.bg_canvas, theme.border_subtle, IconTone::Expense)
            }
            KpiIconBadge::PrimarySolid => (
                theme.primary,
                theme.primary_pressed,
                if is_dark {
                    IconTone::Default
                } else {
                    IconTone::OnPrimaryButton
                },
            ),
            KpiIconBadge::MutedBordered => (theme.bg_canvas, theme.border_subtle, IconTone::Muted),
        };

        div()
            .id(id)
            .w_full()
            .h_full()
            .p(px(18.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .cursor_pointer()
            .hover(|s| s.border_color(theme.primary.opacity(0.65)))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.set_money_tab(target_tab, cx);
            }))
            .flex()
            .flex_col()
            .justify_between()
            .gap(px(14.0))
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_muted)
                            .truncate()
                            .child(label),
                    )
                    .child(
                        div()
                            .w(px(32.0))
                            .h(px(32.0))
                            .flex_shrink_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(badge_bg)
                            .border_1()
                            .border_color(badge_border)
                            .child(app_icon(icon, IconSize::Sm, tone, theme)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(value_color)
                            .child(value),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .truncate()
                            .child(subtitle),
                    ),
            )
    }
}

impl RootView {
    pub fn set_money_tab(&mut self, tab: MoneyTab, cx: &mut Context<Self>) {
        if self.state.money_tab != tab {
            self.state.money_tab = tab;
            cx.notify();
        }
    }

    pub fn set_money_share_kind(&mut self, kind: CategoryKind, cx: &mut Context<Self>) {
        if self.state.money_share_kind != kind {
            self.state.money_share_kind = kind;
            cx.notify();
        }
    }
}

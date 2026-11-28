use std::collections::HashMap;

use chrono::{Datelike, Utc};
use gpui_kit::{Context, FontWeight, Hsla, IntoElement, ParentElement, Styled, div, px};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use umuryango_core_domain::models::{CategoryKind, TransactionType};
use umuryango_i18n::t;

use crate::RootView;
use crate::common::{
    Button, ButtonGroup, ButtonSize, ButtonVariant, Empty, EmptyDescription, EmptyHeader, PieChart,
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};
use crate::components::money::{MoneyCategoryForm, MoneyCategoryInputs};
use crate::components::quick_entry::QuickEntryDialog;
use crate::state::AppState;
use crate::theme::SemanticTheme;

#[derive(Debug, Clone)]
pub struct MoneyMetrics {
    pub total_income: Decimal,
    pub income_count: usize,
    pub total_expenses: Decimal,
    pub expense_count: usize,
    pub net_balance: Decimal,
    pub income_category_count: usize,
    pub expense_category_count: usize,
    pub total_categories: usize,
}

#[derive(Debug, Clone)]
pub struct MonthlyFlow {
    pub month_label: String,
    pub income: Decimal,
    pub expenses: Decimal,
}

#[derive(Debug, Clone)]
pub struct CategorySlice {
    pub name: String,
    pub kind: CategoryKind,
    pub amount: Decimal,
    pub count: usize,
    pub share_pct: i32,
    pub value_f32: f32,
    pub color: Hsla,
}

pub fn format_rwf(amount: Decimal) -> String {
    let rounded = amount.round().to_i64().unwrap_or(0);
    let is_neg = rounded < 0;
    let digits = rounded.unsigned_abs().to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3 + 1);
    if is_neg {
        out.push('-');
    }
    for (idx, ch) in digits.chars().enumerate() {
        if idx > 0 && (digits.len() - idx) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

pub fn format_compact_rwf(amount: Decimal) -> String {
    let val = amount.round().to_i64().unwrap_or(0).abs();
    if val == 0 {
        "0".to_string()
    } else if val >= 10_000_000 {
        format!("{:.1}M", (val as f64) / 1_000_000.0)
    } else if val >= 1_000 {
        format!("{}k", val / 1_000)
    } else {
        val.to_string()
    }
}

pub fn compute_money_metrics(state: &AppState) -> MoneyMetrics {
    let mut total_income = Decimal::ZERO;
    let mut income_count = 0usize;
    let mut total_expenses = Decimal::ZERO;
    let mut expense_count = 0usize;

    let mut inc_cats = QuickEntryDialog::categories_for_kind(state, CategoryKind::Income);
    let mut exp_cats = QuickEntryDialog::categories_for_kind(state, CategoryKind::Expense);

    for tx in &state.transactions {
        match tx.tx_type {
            TransactionType::Income => {
                total_income += tx.amount;
                income_count += 1;
                if !tx.category.trim().is_empty()
                    && !inc_cats
                        .iter()
                        .any(|c| c.eq_ignore_ascii_case(&tx.category))
                {
                    inc_cats.push(tx.category.clone());
                }
            }
            TransactionType::Expense => {
                total_expenses += tx.amount;
                expense_count += 1;
                if !tx.category.trim().is_empty()
                    && !exp_cats
                        .iter()
                        .any(|c| c.eq_ignore_ascii_case(&tx.category))
                {
                    exp_cats.push(tx.category.clone());
                }
            }
            TransactionType::Handover => {}
        }
    }

    let income_category_count = inc_cats.len();
    let expense_category_count = exp_cats.len();

    MoneyMetrics {
        total_income,
        income_count,
        total_expenses,
        expense_count,
        net_balance: state.total_balance(),
        income_category_count,
        expense_category_count,
        total_categories: income_category_count + expense_category_count,
    }
}

const MONTH_ABBR: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub fn compute_monthly_cashflow(state: &AppState) -> Vec<MonthlyFlow> {
    let anchor = state
        .transactions
        .iter()
        .map(|tx| tx.tx_date)
        .max()
        .unwrap_or_else(Utc::now);

    let anchor_year = anchor.year();
    let anchor_month = anchor.month() as i32;

    let mut buckets: Vec<((i32, u32), MonthlyFlow)> = Vec::with_capacity(6);
    for offset in (0..6).rev() {
        let total_months = anchor_year * 12 + (anchor_month - 1) - offset;
        let yr = total_months.div_euclid(12);
        let mo0 = total_months.rem_euclid(12) as usize;
        let mo = (mo0 + 1) as u32;
        buckets.push((
            (yr, mo),
            MonthlyFlow {
                month_label: MONTH_ABBR[mo0].to_string(),
                income: Decimal::ZERO,
                expenses: Decimal::ZERO,
            },
        ));
    }

    for tx in &state.transactions {
        let key = (tx.tx_date.year(), tx.tx_date.month());
        if let Some((_, bucket)) = buckets.iter_mut().find(|(k, _)| *k == key) {
            match tx.tx_type {
                TransactionType::Income => bucket.income += tx.amount,
                TransactionType::Expense => bucket.expenses += tx.amount,
                TransactionType::Handover => {}
            }
        }
    }

    buckets.into_iter().map(|(_, b)| b).collect()
}

fn slice_color(kind: CategoryKind, index: usize, theme: &SemanticTheme) -> Hsla {
    match kind {
        CategoryKind::Expense => match index % 5 {
            0 => theme.primary,
            1 => theme.primary_hover,
            2 => theme.primary_pressed,
            3 => theme.primary_deep,
            _ => theme.text_muted,
        },
        CategoryKind::Income => match index % 4 {
            0 => theme.income_color,
            1 => theme.income_color.opacity(0.78),
            2 => theme.income_color.opacity(0.56),
            _ => theme.text_muted,
        },
    }
}

pub fn compute_category_shares(
    state: &AppState,
    kind: CategoryKind,
    include_zero_categories: bool,
    theme: &SemanticTheme,
) -> (Decimal, Vec<CategorySlice>) {
    let mut map: HashMap<String, (String, Decimal, usize)> = HashMap::new();

    if include_zero_categories {
        for cat in QuickEntryDialog::categories_for_kind(state, kind) {
            map.insert(cat.to_lowercase(), (cat, Decimal::ZERO, 0));
        }
    }

    let mut grand_total = Decimal::ZERO;
    for tx in &state.transactions {
        let matches_kind = matches!(
            (kind, tx.tx_type),
            (CategoryKind::Income, TransactionType::Income)
                | (CategoryKind::Expense, TransactionType::Expense)
        );
        if matches_kind && tx.amount > Decimal::ZERO {
            let key = tx.category.trim().to_lowercase();
            let display = if tx.category.trim().is_empty() {
                "Other".to_string()
            } else {
                tx.category.trim().to_string()
            };
            let entry = map.entry(key).or_insert((display, Decimal::ZERO, 0));
            entry.1 += tx.amount;
            entry.2 += 1;
            grand_total += tx.amount;
        }
    }

    let mut rows: Vec<(String, Decimal, usize)> = map.into_values().collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let slices = rows
        .into_iter()
        .enumerate()
        .map(|(idx, (name, amount, count))| {
            let share_pct = if grand_total > Decimal::ZERO && amount > Decimal::ZERO {
                ((amount / grand_total) * Decimal::from(100))
                    .round()
                    .to_i32()
                    .unwrap_or(0)
            } else {
                0
            };
            CategorySlice {
                name,
                kind,
                amount,
                count,
                share_pct,
                value_f32: amount.to_f32().unwrap_or(0.0).max(0.0),
                color: slice_color(kind, idx, theme),
            }
        })
        .collect();

    (grand_total, slices)
}

pub fn render_cashflow_card(
    state: &AppState,
    is_family: bool,
    is_mobile: bool,
    theme: &SemanticTheme,
) -> impl IntoElement {
    let locale = state.locale;
    let subtitle = if is_family {
        t(locale, "money_page.cashflow_family_sub")
    } else {
        t(locale, "money_page.cashflow_personal_sub")
    };

    let buckets = compute_monthly_cashflow(state);
    let raw_max = buckets
        .iter()
        .flat_map(|b| [b.income, b.expenses])
        .max()
        .unwrap_or(Decimal::ZERO);

    let y_max_f32 = {
        let m = raw_max.to_f32().unwrap_or(0.0);
        if m <= 0.0 {
            1_600_000.0
        } else {
            let step = 400_000.0_f32;
            ((m / step).ceil().max(1.0)) * step
        }
    };

    let chart_h = if is_mobile { 168.0_f32 } else { 196.0_f32 };
    let bar_w = if is_mobile { 12.0_f32 } else { 20.0_f32 };
    let ticks: [f32; 5] = [
        y_max_f32,
        y_max_f32 * 0.75,
        y_max_f32 * 0.50,
        y_max_f32 * 0.25,
        0.0,
    ];

    div()
        .w_full()
        .h_full()
        .p(px(20.0))
        .bg(theme.bg_surface)
        .border_1()
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .justify_between()
        .gap(px(18.0))
        .child(
            div()
                .w_full()
                .flex()
                .flex_row()
                .flex_wrap()
                .justify_between()
                .items_start()
                .gap(px(12.0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(t(locale, "money_page.cashflow_title")),
                        )
                        .child(div().text_xs().text_color(theme.text_muted).child(subtitle)),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(14.0))
                        .child(render_legend_dot(
                            theme.income_color,
                            t(locale, "money_page.legend_income"),
                            theme,
                        ))
                        .child(render_legend_dot(
                            theme.primary,
                            t(locale, "money_page.legend_expenses"),
                            theme,
                        )),
                ),
        )
        .child(
            div()
                .w_full()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .w_full()
                        .h(px(chart_h))
                        .flex()
                        .flex_row()
                        .gap(px(12.0))
                        .child(
                            div()
                                .w(px(44.0))
                                .h_full()
                                .flex_shrink_0()
                                .flex()
                                .flex_col()
                                .justify_between()
                                .items_end()
                                .children(ticks.into_iter().map(|v| {
                                    let dec = Decimal::from(v.round() as i64);
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(format_compact_rwf(dec))
                                })),
                        )
                        .child(
                            div()
                                .relative()
                                .flex_1()
                                .h_full()
                                .child(
                                    div()
                                        .absolute()
                                        .inset_0()
                                        .flex()
                                        .flex_col()
                                        .justify_between()
                                        .children((0..5).map(|_| {
                                            div()
                                                .w_full()
                                                .h(px(1.0))
                                                .bg(theme.border_subtle.opacity(0.45))
                                        })),
                                )
                                .child(
                                    div()
                                        .relative()
                                        .w_full()
                                        .h_full()
                                        .px(px(8.0))
                                        .flex()
                                        .flex_row()
                                        .justify_around()
                                        .items_end()
                                        .children(buckets.iter().map(|b| {
                                            let inc_ratio = (b.income.to_f32().unwrap_or(0.0)
                                                / y_max_f32)
                                                .clamp(0.0, 1.0);
                                            let exp_ratio = (b.expenses.to_f32().unwrap_or(0.0)
                                                / y_max_f32)
                                                .clamp(0.0, 1.0);
                                            let inc_h = if b.income > Decimal::ZERO {
                                                (inc_ratio * chart_h).max(4.0)
                                            } else {
                                                2.0
                                            };
                                            let exp_h = if b.expenses > Decimal::ZERO {
                                                (exp_ratio * chart_h).max(4.0)
                                            } else {
                                                2.0
                                            };
                                            div()
                                                .h_full()
                                                .flex()
                                                .flex_row()
                                                .items_end()
                                                .gap(px(4.0))
                                                .child(div().w(px(bar_w)).h(px(inc_h)).bg(
                                                    if b.income > Decimal::ZERO {
                                                        theme.income_color
                                                    } else {
                                                        theme.border_subtle.opacity(0.35)
                                                    },
                                                ))
                                                .child(div().w(px(bar_w)).h(px(exp_h)).bg(
                                                    if b.expenses > Decimal::ZERO {
                                                        theme.primary
                                                    } else {
                                                        theme.border_subtle.opacity(0.35)
                                                    },
                                                ))
                                        })),
                                ),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .pl(px(56.0))
                        .px(px(8.0))
                        .flex()
                        .flex_row()
                        .justify_around()
                        .children(buckets.into_iter().map(|b| {
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(b.month_label)
                        })),
                ),
        )
}

fn render_legend_dot(color: Hsla, label: String, theme: &SemanticTheme) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(6.0))
        .child(div().w(px(8.0)).h(px(8.0)).bg(color))
        .child(
            div()
                .text_xs()
                .text_color(theme.text_secondary)
                .child(label),
        )
}

pub fn render_category_share_card(
    chart_id: &'static str,
    state: &AppState,
    is_family: bool,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let locale = state.locale;
    let active_kind = state.money_share_kind;
    let subtitle = if is_family {
        t(locale, "money_page.share_family_sub")
    } else {
        t(locale, "money_page.share_personal_sub")
    };

    let (grand_total, mut slices) = compute_category_shares(state, active_kind, false, theme);
    slices.truncate(6);

    let pie_data: Vec<CategorySlice> = if slices.is_empty() {
        vec![CategorySlice {
            name: String::new(),
            kind: active_kind,
            amount: Decimal::ZERO,
            count: 0,
            share_pct: 100,
            value_f32: 1.0,
            color: theme.border_subtle.opacity(0.45),
        }]
    } else {
        slices.clone()
    };

    let pie = PieChart::new(pie_data)
        .id(chart_id)
        .interactive(!slices.is_empty())
        .inner_radius(42.0)
        .outer_radius(66.0)
        .pad_angle(if slices.len() > 1 { 0.03 } else { 0.0 })
        .value(|d| d.value_f32)
        .color(|d| d.color);

    div()
        .w_full()
        .h_full()
        .p(px(20.0))
        .bg(theme.bg_surface)
        .border_1()
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .justify_between()
        .gap(px(18.0))
        .child(
            div()
                .w_full()
                .flex()
                .flex_row()
                .flex_wrap()
                .justify_between()
                .items_start()
                .gap(px(12.0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(t(locale, "money_page.share_title")),
                        )
                        .child(div().text_xs().text_color(theme.text_muted).child(subtitle)),
                )
                .child(
                    ButtonGroup::horizontal()
                        .child(
                            Button::new(format!("{chart_id}-toggle-exp"))
                                .variant(if active_kind == CategoryKind::Expense {
                                    ButtonVariant::Default
                                } else {
                                    ButtonVariant::Outline
                                })
                                .size(ButtonSize::Xs)
                                .label(t(locale, "money_page.legend_expenses"))
                                .theme(theme)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.set_money_share_kind(CategoryKind::Expense, cx);
                                })),
                        )
                        .child(
                            Button::new(format!("{chart_id}-toggle-inc"))
                                .variant(if active_kind == CategoryKind::Income {
                                    ButtonVariant::Default
                                } else {
                                    ButtonVariant::Outline
                                })
                                .size(ButtonSize::Xs)
                                .label(t(locale, "money_page.legend_income"))
                                .theme(theme)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.set_money_share_kind(CategoryKind::Income, cx);
                                })),
                        ),
                ),
        )
        .child(
            div()
                .w_full()
                .flex_1()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .justify_center()
                .gap(px(24.0))
                .child(
                    div()
                        .relative()
                        .w(px(156.0))
                        .h(px(156.0))
                        .flex_shrink_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(div().size_full().child(pie))
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(format_compact_rwf(grand_total)),
                                )
                                .child(div().text_xs().text_color(theme.text_muted).child("RWF")),
                        ),
                )
                .child(if slices.is_empty() {
                    div()
                        .flex_1()
                        .min_w(px(190.0))
                        .child(
                            Empty::new().bordered(false).p(px(8.0)).theme(theme).child(
                                EmptyHeader::new().child(
                                    EmptyDescription::new(t(locale, "money_page.share_empty"))
                                        .theme(theme),
                                ),
                            ),
                        )
                        .into_any_element()
                } else {
                    div()
                        .flex_1()
                        .min_w(px(210.0))
                        .flex()
                        .flex_col()
                        .gap(px(10.0))
                        .children(slices.into_iter().map(|s| {
                            div()
                                .w_full()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .gap(px(10.0))
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(8.0))
                                        .child(
                                            div().w(px(8.0)).h(px(8.0)).flex_shrink_0().bg(s.color),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_secondary)
                                                .truncate()
                                                .child(s.name),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex_shrink_0()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(10.0))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child(format!("{} RWF", format_rwf(s.amount))),
                                        )
                                        .child(
                                            div()
                                                .w(px(34.0))
                                                .flex()
                                                .justify_end()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(format!("{}%", s.share_pct)),
                                        ),
                                )
                        }))
                        .into_any_element()
                }),
        )
}

pub fn render_categories_tab(
    state: &AppState,
    category_inputs: &MoneyCategoryInputs,
    is_family: bool,
    is_mobile: bool,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let locale = state.locale;
    let (_, mut exp_rows) = compute_category_shares(state, CategoryKind::Expense, true, theme);
    let (_, inc_rows) = compute_category_shares(state, CategoryKind::Income, true, theme);
    exp_rows.extend(inc_rows);
    exp_rows.sort_by(|a, b| b.amount.cmp(&a.amount).then_with(|| a.name.cmp(&b.name)));
    let total_len = exp_rows.len();

    let pie_panel = render_category_share_card("money-cat-tab-pie", state, is_family, theme, cx);

    let breakdown_card = div()
        .w_full()
        .bg(theme.bg_surface)
        .border_1()
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .child(
            div()
                .w_full()
                .p(px(20.0))
                .border_b_1()
                .border_color(theme.border_subtle)
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
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(t(locale, "money_page.categories_list_title")),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(t(locale, "money_page.categories_list_sub")),
                        ),
                )
                .child(MoneyCategoryForm::render(
                    "categories-tab-add-form",
                    state.money_share_kind,
                    state,
                    category_inputs,
                    theme,
                    cx,
                )),
        )
        .child(
            Table::new("money-categories-breakdown-table")
                .theme(theme)
                .child(
                    TableHeader::new().theme(theme).child(
                        TableRow::new()
                            .hoverable(false)
                            .border_bottom(false)
                            .theme(theme)
                            .child(
                                TableHead::new()
                                    .label(t(locale, "money_page.col_category"))
                                    .flex_1()
                                    .min_w(px(160.0))
                                    .theme(theme),
                            )
                            .child(
                                TableHead::new()
                                    .label(t(locale, "money_page.col_type"))
                                    .flex_none()
                                    .w(px(130.0))
                                    .theme(theme),
                            )
                            .child(
                                TableHead::new()
                                    .label(t(locale, "money_page.col_tx_count"))
                                    .flex_none()
                                    .w(px(96.0))
                                    .theme(theme),
                            )
                            .child(
                                TableHead::new()
                                    .label(t(locale, "money_page.col_share"))
                                    .flex_none()
                                    .w(px(88.0))
                                    .theme(theme),
                            )
                            .child(
                                TableHead::new()
                                    .label(t(locale, "money_page.col_amount"))
                                    .flex_none()
                                    .w(px(150.0))
                                    .align_right()
                                    .theme(theme),
                            ),
                    ),
                )
                .child(
                    TableBody::new().children(exp_rows.into_iter().enumerate().map(
                        |(idx, row)| {
                            let is_inc = row.kind == CategoryKind::Income;
                            let type_label = if is_inc {
                                t(locale, "money_page.legend_income")
                            } else {
                                t(locale, "money_page.legend_expenses")
                            };
                            let (badge_bg, badge_border, badge_fg) = if is_inc {
                                (
                                    theme.income_color.opacity(0.14),
                                    theme.income_color.opacity(0.45),
                                    theme.income_color,
                                )
                            } else {
                                (
                                    theme.primary_deep.opacity(0.45),
                                    theme.primary.opacity(0.50),
                                    theme.expense_color,
                                )
                            };
                            let cat_name_for_click = row.name.clone();
                            let row_kind = row.kind;

                            TableRow::new()
                                .id(format!("cat-breakdown-row-{idx}"))
                                .last(idx + 1 == total_len)
                                .theme(theme)
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.open_quick_entry_for_category(
                                        row_kind,
                                        cat_name_for_click.clone(),
                                        window,
                                        cx,
                                    );
                                }))
                                .child(
                                    TableCell::new()
                                        .flex_1()
                                        .min_w(px(160.0))
                                        .theme(theme)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap(px(8.0))
                                                .child(
                                                    div()
                                                        .w(px(8.0))
                                                        .h(px(8.0))
                                                        .flex_shrink_0()
                                                        .bg(row.color),
                                                )
                                                .child(
                                                    div()
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.text_primary)
                                                        .child(row.name),
                                                ),
                                        ),
                                )
                                .child(
                                    TableCell::new()
                                        .flex_none()
                                        .w(px(130.0))
                                        .theme(theme)
                                        .child(
                                            div()
                                                .px(px(8.0))
                                                .py(px(3.0))
                                                .bg(badge_bg)
                                                .border_1()
                                                .border_color(badge_border)
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(badge_fg)
                                                .child(type_label),
                                        ),
                                )
                                .child(
                                    TableCell::new()
                                        .flex_none()
                                        .w(px(96.0))
                                        .text_color(theme.text_muted)
                                        .text(row.count.to_string())
                                        .theme(theme),
                                )
                                .child(
                                    TableCell::new()
                                        .flex_none()
                                        .w(px(88.0))
                                        .text_color(theme.text_muted)
                                        .text(format!("{}%", row.share_pct))
                                        .theme(theme),
                                )
                                .child(
                                    TableCell::new()
                                        .flex_none()
                                        .w(px(150.0))
                                        .align_right()
                                        .theme(theme)
                                        .child(
                                            div()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(if row.amount == Decimal::ZERO {
                                                    theme.text_muted
                                                } else if is_inc {
                                                    theme.income_color
                                                } else {
                                                    theme.expense_color
                                                })
                                                .child(format!("{} RWF", format_rwf(row.amount))),
                                        ),
                                )
                        },
                    )),
                ),
        );

    let _ = is_mobile;
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(pie_panel)
        .child(breakdown_card)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_rwf_adds_thousands_commas_and_compact_scales() {
        assert_eq!(format_rwf(Decimal::new(1_450_000, 0)), "1,450,000");
        assert_eq!(format_rwf(Decimal::new(825_000, 0)), "825,000");
        assert_eq!(format_rwf(Decimal::ZERO), "0");
        assert_eq!(format_compact_rwf(Decimal::new(825_000, 0)), "825k");
        assert_eq!(format_compact_rwf(Decimal::new(1_600_000, 0)), "1600k");
        assert_eq!(format_compact_rwf(Decimal::ZERO), "0");
    }

    #[test]
    fn default_metrics_count_ten_active_categories() {
        let state = AppState::default();
        let metrics = compute_money_metrics(&state);
        assert_eq!(metrics.income_category_count, 4);
        assert_eq!(metrics.expense_category_count, 6);
        assert_eq!(metrics.total_categories, 10);
    }
}

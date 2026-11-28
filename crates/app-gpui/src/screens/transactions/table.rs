use gpui_kit::{Context, FontWeight, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::{Transaction, TransactionType};
use umuryango_i18n::t;

use crate::RootView;
use crate::common::{
    Button, ButtonSize, Empty, EmptyDescription, EmptyHeader, EmptyMedia, Table, TableBody,
    TableCell, TableHead, TableHeader, TableRow,
};
use crate::components::icon::AppIcon;
use crate::components::quick_entry::QuickEntryMode;
use crate::screens::transactions::analytics::format_rwf;
use crate::state::{AppState, MoneyTab};
use crate::theme::SemanticTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityTableMode {
    OverviewRecent,
    IncomeLedger,
    ExpensesLedger,
}

pub fn render_activity_table_card(
    table_id: &'static str,
    mode: ActivityTableMode,
    state: &AppState,
    is_family: bool,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let locale = state.locale;

    let (title, subtitle) = match mode {
        ActivityTableMode::OverviewRecent => (
            if is_family {
                t(locale, "money_page.recent_family_title")
            } else {
                t(locale, "money_page.recent_personal_title")
            },
            if is_family {
                t(locale, "money_page.recent_family_sub")
            } else {
                t(locale, "money_page.recent_personal_sub")
            },
        ),
        ActivityTableMode::IncomeLedger => (
            if is_family {
                t(locale, "money_page.income_family_title")
            } else {
                t(locale, "money_page.income_personal_title")
            },
            t(locale, "money_page.income_sub"),
        ),
        ActivityTableMode::ExpensesLedger => (
            if is_family {
                t(locale, "money_page.expenses_family_title")
            } else {
                t(locale, "money_page.expenses_personal_title")
            },
            if is_family {
                t(locale, "money_page.expenses_family_sub")
            } else {
                t(locale, "money_page.expenses_personal_sub")
            },
        ),
    };

    let filtered: Vec<&Transaction> = match mode {
        ActivityTableMode::OverviewRecent => state.transactions.iter().take(12).collect(),
        ActivityTableMode::IncomeLedger => state
            .transactions
            .iter()
            .filter(|tx| tx.tx_type == TransactionType::Income)
            .collect(),
        ActivityTableMode::ExpensesLedger => state
            .transactions
            .iter()
            .filter(|tx| tx.tx_type == TransactionType::Expense)
            .collect(),
    };

    let header_actions = match mode {
        ActivityTableMode::OverviewRecent => div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.0))
            .child(
                Button::new(format!("{table_id}-view-all-inc"))
                    .outline()
                    .size(ButtonSize::Xs)
                    .label(t(locale, "money_page.view_all_income"))
                    .theme(theme)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.set_money_tab(MoneyTab::Income, cx);
                    })),
            )
            .child(
                Button::new(format!("{table_id}-view-all-exp"))
                    .outline()
                    .size(ButtonSize::Xs)
                    .label(t(locale, "money_page.view_all_expenses"))
                    .theme(theme)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.set_money_tab(MoneyTab::Expenses, cx);
                    })),
            )
            .into_any_element(),
        ActivityTableMode::IncomeLedger => Button::new(format!("{table_id}-add-inc"))
            .outline()
            .size(ButtonSize::Sm)
            .icon_start(AppIcon::Income)
            .label(t(locale, "money_page.add_income_btn"))
            .theme(theme)
            .on_click(cx.listener(|this, _, window, cx| {
                this.open_money_entry(QuickEntryMode::Income, window, cx);
            }))
            .into_any_element(),
        ActivityTableMode::ExpensesLedger => Button::new(format!("{table_id}-add-exp"))
            .size(ButtonSize::Sm)
            .icon_start(AppIcon::Add)
            .label(t(locale, "money_page.log_expense_btn"))
            .theme(theme)
            .on_click(cx.listener(|this, _, window, cx| {
                this.open_money_entry(QuickEntryMode::Expense, window, cx);
            }))
            .into_any_element(),
    };

    let actor_col_label = match mode {
        ActivityTableMode::OverviewRecent => t(locale, "money_page.col_added_authorized"),
        ActivityTableMode::IncomeLedger => t(locale, "money_page.col_added_by"),
        ActivityTableMode::ExpensesLedger => t(locale, "money_page.col_authorized_by"),
    };

    let body = if filtered.is_empty() {
        div()
            .w_full()
            .p(px(28.0))
            .child(
                Empty::new().bordered(false).p(px(16.0)).theme(theme).child(
                    EmptyHeader::new()
                        .child(EmptyMedia::new().icon(AppIcon::Transactions).theme(theme))
                        .child(
                            EmptyDescription::new(t(locale, "transactions.no_transactions"))
                                .theme(theme),
                        ),
                ),
            )
            .into_any_element()
    } else {
        let row_count = filtered.len();
        Table::new(table_id)
            .theme(theme)
            .child(
                TableHeader::new().theme(theme).child(
                    TableRow::new()
                        .hoverable(false)
                        .border_bottom(false)
                        .theme(theme)
                        .child(
                            TableHead::new()
                                .label(t(locale, "money_page.col_date"))
                                .flex_none()
                                .w(px(120.0))
                                .theme(theme),
                        )
                        .child(
                            TableHead::new()
                                .label(t(locale, "money_page.col_description"))
                                .flex_1()
                                .min_w(px(210.0))
                                .theme(theme),
                        )
                        .child(
                            TableHead::new()
                                .label(t(locale, "money_page.col_category"))
                                .flex_none()
                                .w(px(184.0))
                                .theme(theme),
                        )
                        .child(
                            TableHead::new()
                                .label(t(locale, "money_page.col_wallet"))
                                .flex_none()
                                .w(px(168.0))
                                .theme(theme),
                        )
                        .children(is_family.then(|| {
                            TableHead::new()
                                .label(actor_col_label)
                                .flex_none()
                                .w(px(164.0))
                                .theme(theme)
                        }))
                        .child(
                            TableHead::new()
                                .label(t(locale, "money_page.col_amount"))
                                .flex_none()
                                .w(px(152.0))
                                .align_right()
                                .theme(theme),
                        ),
                ),
            )
            .child(
                TableBody::new().children(filtered.into_iter().enumerate().map(|(idx, tx)| {
                    render_tx_row(idx, idx + 1 == row_count, tx, state, is_family, theme)
                })),
            )
            .into_any_element()
    };

    div()
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
                                .child(title),
                        )
                        .child(div().text_xs().text_color(theme.text_muted).child(subtitle)),
                )
                .child(header_actions),
        )
        .child(body)
}

fn render_tx_row(
    idx: usize,
    is_last: bool,
    tx: &Transaction,
    state: &AppState,
    is_family: bool,
    theme: &SemanticTheme,
) -> impl IntoElement {
    let date_str = tx.tx_date.format("%d %b %Y").to_string();
    let desc = tx
        .description
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| tx.paid_to.as_deref().filter(|s| !s.trim().is_empty()))
        .unwrap_or(tx.category.as_str())
        .to_string();

    let wallet_str = if tx.tx_type == TransactionType::Handover
        && let Some(dst) = tx.destination_wallet_id
    {
        format!(
            "{} -> {}",
            state.wallet_name(tx.source_wallet_id),
            state.wallet_name(dst)
        )
    } else {
        state.wallet_name(tx.source_wallet_id)
    };

    let (badge_bg, badge_border, badge_fg, amt_prefix, amt_color) = match tx.tx_type {
        TransactionType::Income => (
            theme.income_color.opacity(0.14),
            theme.income_color.opacity(0.45),
            theme.income_color,
            "+",
            theme.income_color,
        ),
        TransactionType::Expense => (
            theme.primary_deep.opacity(0.45),
            theme.primary.opacity(0.50),
            theme.expense_color,
            "-",
            theme.expense_color,
        ),
        TransactionType::Handover => (
            theme.bg_canvas,
            theme.border_subtle,
            theme.text_muted,
            "",
            theme.text_secondary,
        ),
    };

    let actor_str = state.actor_name(tx.authorized_by).unwrap_or_else(|| {
        if tx.authorized_by == state.user_id {
            t(state.locale, "money_page.self_actor")
        } else {
            "—".to_string()
        }
    });

    TableRow::new()
        .id(format!("money-tx-row-{idx}"))
        .last(is_last)
        .theme(theme)
        .child(
            TableCell::new()
                .flex_none()
                .w(px(120.0))
                .text_xs()
                .text_color(theme.text_muted)
                .text(date_str)
                .theme(theme),
        )
        .child(
            TableCell::new()
                .flex_1()
                .min_w(px(210.0))
                .theme(theme)
                .child(
                    div()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_primary)
                        .truncate()
                        .child(desc),
                ),
        )
        .child(
            TableCell::new()
                .flex_none()
                .w(px(184.0))
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
                        .truncate()
                        .child(tx.category.clone()),
                ),
        )
        .child(
            TableCell::new()
                .flex_none()
                .w(px(168.0))
                .theme(theme)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text_secondary)
                        .truncate()
                        .child(wallet_str),
                ),
        )
        .children(is_family.then(|| {
            TableCell::new()
                .flex_none()
                .w(px(164.0))
                .theme(theme)
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .truncate()
                        .child(actor_str),
                )
        }))
        .child(
            TableCell::new()
                .flex_none()
                .w(px(152.0))
                .align_right()
                .theme(theme)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(amt_color)
                        .child(format!("{amt_prefix}{} RWF", format_rwf(tx.amount))),
                ),
        )
}

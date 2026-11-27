use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::CategoryKind;
use umuryango_i18n::{Translator, terms};

use crate::RootView;
use crate::common::{Button, ButtonSize, Empty, EmptyDescription, EmptyHeader, EmptyMedia};
use crate::components::icon::AppIcon;
use crate::components::money::{MoneyCategoryForm, MoneyCategoryInputs};
use crate::components::tx_row::TxRow;
use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct TransactionsScreen;

impl TransactionsScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        category_inputs: &MoneyCategoryInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let title = Translator::t(state.locale, terms::NAV_TRANSACTIONS, None);

        div()
            .id(id)
            .flex_1()
            .h_full()
            .pt(px(24.0))
            .pb(px(24.0) + state.safe_area.total_bottom_px())
            .pl(px(24.0) + state.safe_area.left_px())
            .pr(px(24.0) + state.safe_area.right_px())
            .overflow_y_scroll()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            // Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .justify_between()
                    .items_center()
                    .gap(px(12.0))
                    .mb(px(16.0))
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(title),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap(px(8.0))
                            .child(MoneyCategoryForm::render(
                                "txs-money-category-form",
                                CategoryKind::Expense,
                                state,
                                category_inputs,
                                theme,
                                cx,
                            ))
                            .child(
                                Button::new("transactions-refresh-btn")
                                    .outline()
                                    .size(ButtonSize::Sm)
                                    .icon_start(AppIcon::SyncPending)
                                    .label(umuryango_i18n::t(state.locale, "common.refresh"))
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.refresh_active_scope(cx);
                                    })),
                            )
                            .child(
                                Button::new("transactions-quick-entry-btn")
                                    .size(ButtonSize::Sm)
                                    .icon_start(AppIcon::Add)
                                    .label(umuryango_i18n::t(
                                        state.locale,
                                        "dashboard.quick_log_btn",
                                    ))
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_quick_entry(window, cx);
                                    })),
                            ),
                    ),
            )
            // Saved custom categories bar (quick tap logs a transaction with that category)
            .children((!state.categories.is_empty()).then(|| {
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(px(6.0))
                    .mb(px(16.0))
                    .children(state.categories.iter().map(|cat| {
                        let kind = cat.kind;
                        let cat_name = cat.name.clone();
                        let icon = match kind {
                            CategoryKind::Income => AppIcon::Income,
                            CategoryKind::Expense => AppIcon::Expense,
                        };
                        Button::new(format!("saved-cat-chip-{}", cat.id))
                            .outline()
                            .size(ButtonSize::Xs)
                            .icon_start(icon)
                            .label(cat.name.clone())
                            .theme(theme)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.open_quick_entry_for_category(
                                    kind,
                                    cat_name.clone(),
                                    window,
                                    cx,
                                );
                            }))
                    }))
            }))
            // Transactions full list or empty state
            .child(div().flex().flex_col().gap(px(8.0)).children(
                if state.transactions.is_empty() {
                    vec![
                        Empty::new()
                            .bordered(true)
                            .p(px(32.0))
                            .bg(theme.bg_surface)
                            .theme(theme)
                            .child(
                                EmptyHeader::new()
                                    .child(
                                        EmptyMedia::from_icon(AppIcon::Transactions).theme(theme),
                                    )
                                    .child(
                                        EmptyDescription::new(umuryango_i18n::t(
                                            state.locale,
                                            "transactions.no_transactions",
                                        ))
                                        .theme(theme),
                                    ),
                            )
                            .into_any_element(),
                    ]
                } else {
                    state
                        .transactions
                        .iter()
                        .enumerate()
                        .map(|(idx, tx)| {
                            let actor = state.actor_name(tx.authorized_by);
                            TxRow::render(
                                format!("txs-row-{}", idx),
                                tx,
                                actor.as_deref(),
                                state.locale,
                                theme,
                            )
                            .into_any_element()
                        })
                        .collect()
                },
            ))
    }
}

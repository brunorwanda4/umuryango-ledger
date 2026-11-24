use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Translator, terms};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::components::tx_row::TxRow;
use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct TransactionsScreen;

impl TransactionsScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
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
                    .justify_between()
                    .items_center()
                    .mb(px(20.0))
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(title),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .id("transactions-refresh-btn")
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .bg(theme.bg_surface)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_color(theme.text_primary)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.refresh_active_scope(cx);
                                    }))
                                    .child(app_icon(
                                        AppIcon::SyncPending,
                                        IconSize::Xs,
                                        IconTone::Muted,
                                        theme,
                                    ))
                                    .child(umuryango_i18n::t(state.locale, "common.refresh")),
                            )
                            .child(
                                div()
                                    .id("transactions-quick-entry-btn")
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .px(px(14.0))
                                    .py(px(8.0))
                                    .bg(theme.accent_primary)
                                    .text_color(theme.bg_canvas)
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_quick_entry(window, cx);
                                    }))
                                    .child(app_icon(
                                        AppIcon::Add,
                                        IconSize::Xs,
                                        IconTone::OnPrimaryButton,
                                        theme,
                                    ))
                                    .child(umuryango_i18n::t(
                                        state.locale,
                                        "dashboard.quick_log_btn",
                                    )),
                            ),
                    ),
            )
            // Transactions full list or empty state
            .child(div().flex().flex_col().gap(px(8.0)).children(
                if state.transactions.is_empty() {
                    vec![
                        div()
                            .p(px(24.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_sm()
                            .text_center()
                            .text_color(theme.text_muted)
                            .child(umuryango_i18n::t(
                                state.locale,
                                "transactions.no_transactions",
                            ))
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

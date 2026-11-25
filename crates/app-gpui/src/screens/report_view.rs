use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::reporting::{LocalizedExporter, generate_mothers_report};
use umuryango_i18n::{Translator, terms};

use crate::RootView;
use crate::common::{
    Button, ButtonSize, Card, CardAction, CardContent, CardHeader, CardSize, CardTitle, Empty,
    EmptyDescription, EmptyHeader, EmptyMedia, Item, ItemActions, ItemContent, ItemMedia, ItemSize,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct ReportViewScreen;

impl ReportViewScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let now = chrono::Utc::now();
        let period_start = now - chrono::Duration::days(30);

        let report = generate_mothers_report(
            state.active_scope,
            &state.wallets,
            &state.transactions,
            &state.projects,
            &state.loans,
            &state.future_plans,
            period_start,
            now,
        );

        let whatsapp_text = LocalizedExporter::format_mothers_report(&report, state.locale);
        let title = Translator::t(state.locale, terms::MOTHERS_REPORT_TITLE, None);

        let share_text = whatsapp_text.clone();
        let share_success = umuryango_i18n::t(state.locale, "report.share_copied");
        let copy_preview_text = whatsapp_text.clone();
        let copy_preview_success = share_success.clone();

        div()
            .id(id)
            .flex_1()
            .h_full()
            .overflow_y_scroll()
            .pt(px(24.0))
            .pb(px(24.0) + state.safe_area.total_bottom_px())
            .pl(px(24.0) + state.safe_area.left_px())
            .pr(px(24.0) + state.safe_area.right_px())
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            // Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .mb(px(24.0))
                    .child(
                        div()
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(title),
                            )
                            .child(
                                div().text_xs().text_color(theme.text_muted).child(
                                    umuryango_i18n::t(state.locale, "report.report_subtitle"),
                                ),
                            ),
                    )
                    // Export Actions: WhatsApp Share & Real API Magic Link
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(10.0))
                            .child(
                                Button::new("report-whatsapp-share-btn")
                                    .size(ButtonSize::Sm)
                                    .icon_start(AppIcon::Share)
                                    .label(umuryango_i18n::t(state.locale, "report.whatsapp_share"))
                                    .bg(theme.income_color)
                                    .theme(theme)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.copy_to_clipboard(
                                            share_text.clone(),
                                            share_success.clone(),
                                            cx,
                                        );
                                    })),
                            )
                            .child(
                                Button::new("report-magic-link-btn")
                                    .size(ButtonSize::Sm)
                                    .icon_start(AppIcon::Language)
                                    .label(umuryango_i18n::t(state.locale, "report.magic_link"))
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.handle_create_magic_link(cx);
                                    })),
                            ),
                    ),
            )
            // KPI Summary Cards
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(16.0))
                    .mb(px(24.0))
                    .child(Self::render_kpi_card(
                        Translator::t(state.locale, terms::TOTAL_INCOME, None),
                        format!("{} RWF", report.total_income),
                        AppIcon::Income,
                        IconTone::Income,
                        theme.income_color,
                        theme,
                    ))
                    .child(Self::render_kpi_card(
                        Translator::t(state.locale, terms::TOTAL_EXPENSES, None),
                        format!("{} RWF", report.total_expenses),
                        AppIcon::Expense,
                        IconTone::Expense,
                        theme.expense_color,
                        theme,
                    ))
                    .child(Self::render_kpi_card(
                        Translator::t(state.locale, terms::CLOSING_BALANCE, None),
                        format!("{} RWF", report.closing_balance),
                        AppIcon::Reports,
                        IconTone::AccentPrimary,
                        theme.accent_primary,
                        theme,
                    )),
            )
            // Cash Breakdown Held by Each Custodian
            .child(
                Card::new()
                    .size(CardSize::Sm)
                    .gap(px(12.0))
                    .py(px(18.0))
                    .mb(px(20.0))
                    .border_1()
                    .border_color(theme.border_subtle)
                    .theme(theme)
                    .child(
                        CardHeader::new().size(CardSize::Sm).theme(theme).child(
                            CardTitle::new(Translator::t(
                                state.locale,
                                terms::CASH_CUSTODY_BREAKDOWN,
                                None,
                            ))
                            .text_base()
                            .theme(theme),
                        ),
                    )
                    .child(CardContent::new().size(CardSize::Sm).child(
                        if report.custodian_balances.is_empty() {
                            Empty::new()
                                .p(px(20.0))
                                .theme(theme)
                                .child(
                                    EmptyHeader::new()
                                        .child(
                                            EmptyMedia::from_icon(AppIcon::PersonalWallet)
                                                .theme(theme),
                                        )
                                        .child(
                                            EmptyDescription::new(umuryango_i18n::t(
                                                state.locale,
                                                "report.no_wallets_in_report",
                                            ))
                                            .theme(theme),
                                        ),
                                )
                                .into_any_element()
                        } else {
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(8.0))
                                .children(report.custodian_balances.iter().map(|c| {
                                    Item::new()
                                        .muted()
                                        .size(ItemSize::Xs)
                                        .bg(theme.bg_canvas)
                                        .justify_between()
                                        .theme(theme)
                                        .child(ItemMedia::new().theme(theme).child(app_icon(
                                            AppIcon::Members,
                                            IconSize::Xs,
                                            IconTone::Muted,
                                            theme,
                                        )))
                                        .child(
                                            ItemContent::new().child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.text_primary)
                                                    .child(c.wallet_name.clone()),
                                            ),
                                        )
                                        .child(
                                            ItemActions::new().child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.income_color)
                                                    .child(format!("{} RWF", c.balance)),
                                            ),
                                        )
                                }))
                                .into_any_element()
                        },
                    )),
            )
            // WhatsApp Formatted Preview Box
            .child(
                Card::new()
                    .size(CardSize::Sm)
                    .gap(px(12.0))
                    .py(px(18.0))
                    .border_1()
                    .border_color(theme.border_subtle)
                    .theme(theme)
                    .child(
                        CardHeader::new()
                            .size(CardSize::Sm)
                            .action(
                                CardAction::new().child(
                                    Button::new("report-copy-preview-btn")
                                        .outline()
                                        .size(ButtonSize::Xs)
                                        .icon_start(AppIcon::Copy)
                                        .label(umuryango_i18n::t(state.locale, "report.copy_text"))
                                        .theme(theme)
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.copy_to_clipboard(
                                                copy_preview_text.clone(),
                                                copy_preview_success.clone(),
                                                cx,
                                            );
                                        })),
                                ),
                            )
                            .theme(theme)
                            .child(
                                CardTitle::new(umuryango_i18n::t(
                                    state.locale,
                                    "report.whatsapp_preview",
                                ))
                                .text_sm()
                                .theme(theme),
                            ),
                    )
                    .child(
                        CardContent::new().size(CardSize::Sm).child(
                            div()
                                .p(px(14.0))
                                .bg(theme.bg_canvas)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .child(whatsapp_text),
                                ),
                        ),
                    ),
            )
    }

    fn render_kpi_card(
        label: String,
        value: String,
        icon: AppIcon,
        tone: IconTone,
        value_color: gpui_kit::Hsla,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        Card::new()
            .size(CardSize::Sm)
            .flex_1()
            .gap(px(8.0))
            .py(px(18.0))
            .border_1()
            .border_color(theme.border_subtle)
            .theme(theme)
            .child(
                CardHeader::new()
                    .size(CardSize::Sm)
                    .action(CardAction::new().child(app_icon(icon, IconSize::Sm, tone, theme)))
                    .theme(theme)
                    .child(div().text_xs().text_color(theme.text_muted).child(label)),
            )
            .child(
                CardContent::new().size(CardSize::Sm).child(
                    div()
                        .text_xl()
                        .font_weight(FontWeight::BOLD)
                        .text_color(value_color)
                        .child(value),
                ),
            )
    }
}

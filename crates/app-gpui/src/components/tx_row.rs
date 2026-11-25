use gpui_kit::{ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::{Transaction, TransactionType};
use umuryango_i18n::{Locale, Translator, terms};

use crate::common::{Badge, BadgeVariant, Item, ItemActions, ItemContent, ItemSize};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::SemanticTheme;

pub struct TxRow;

impl TxRow {
    pub fn amount_display(tx: &Transaction) -> String {
        let prefix = match tx.tx_type {
            TransactionType::Income => "+",
            TransactionType::Expense => "-",
            TransactionType::Handover => "~",
        };
        format!("{} {} RWF", prefix, tx.amount)
    }

    pub fn type_label(tx_type: TransactionType, locale: Locale) -> String {
        match tx_type {
            TransactionType::Income => Translator::t(locale, terms::INCOME, None),
            TransactionType::Expense => Translator::t(locale, terms::EXPENSE, None),
            TransactionType::Handover => Translator::t(locale, terms::HANDOVER, None),
        }
    }

    pub const fn type_icon(tx_type: TransactionType) -> AppIcon {
        match tx_type {
            TransactionType::Income => AppIcon::Income,
            TransactionType::Expense => AppIcon::Expense,
            TransactionType::Handover => AppIcon::Handover,
        }
    }

    pub fn render(
        id: impl Into<ElementId>,
        tx: &Transaction,
        authorizer_name: Option<&str>,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let type_str = Self::type_label(tx.tx_type, locale);
        let type_ico = Self::type_icon(tx.tx_type);
        let amount_str = Self::amount_display(tx);
        let amount_color = match tx.tx_type {
            TransactionType::Income => theme.income_color,
            TransactionType::Expense => theme.expense_color,
            TransactionType::Handover => theme.handover_color,
        };
        let badge_variant = match tx.tx_type {
            TransactionType::Income => BadgeVariant::Outline,
            TransactionType::Expense => BadgeVariant::Destructive,
            TransactionType::Handover => BadgeVariant::Secondary,
        };
        let type_bg = match tx.tx_type {
            TransactionType::Income => theme.income_color.opacity(0.15),
            TransactionType::Expense => theme.expense_color.opacity(0.15),
            TransactionType::Handover => theme.accent_primary.opacity(0.15),
        };

        Item::new()
            .id(id)
            .outline()
            .size(ItemSize::Sm)
            .bg(theme.bg_surface)
            .justify_between()
            .theme(theme)
            .child(
                Badge::new(type_str)
                    .variant(badge_variant)
                    .icon_start(type_ico)
                    .px(px(8.0))
                    .py(px(3.0))
                    .bg(type_bg)
                    .text_color(amount_color)
                    .theme(theme),
            )
            .child(
                ItemContent::new()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.text_primary)
                            .child(tx.category.clone()),
                    )
                    .child(
                        div().text_xs().text_color(theme.text_muted).child(
                            tx.description
                                .clone()
                                .unwrap_or_else(|| tx.tx_date.format("%Y-%m-%d").to_string()),
                        ),
                    ),
            )
            .child(
                ItemActions::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .items_end()
                        .gap(px(2.0))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(amount_color)
                                .child(amount_str),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(4.0))
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(app_icon(
                                    AppIcon::Check,
                                    IconSize::Xs,
                                    IconTone::Success,
                                    theme,
                                ))
                                .child(authorizer_name.unwrap_or("Authorized").to_string()),
                        ),
                ),
            )
    }
}

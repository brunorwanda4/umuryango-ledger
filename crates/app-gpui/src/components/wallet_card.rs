use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_core_domain::models::{Wallet, WalletChannel};
use umuryango_i18n::{terms, Locale, Translator};

use crate::theme::SemanticTheme;

pub struct WalletCard;

impl WalletCard {
    pub fn channel_label(channel: WalletChannel, locale: Locale) -> String {
        match channel {
            WalletChannel::Bank => {
                if locale == Locale::Rw {
                    "Banki".to_string()
                } else {
                    "Bank".to_string()
                }
            }
            WalletChannel::MobileMoney => "MoMo".to_string(),
            WalletChannel::CashInHand => {
                if locale == Locale::Rw {
                    "Amafaranga mu Ntoki".to_string()
                } else {
                    "Cash in Hand".to_string()
                }
            }
        }
    }

    pub fn channel_icon(channel: WalletChannel) -> &'static str {
        match channel {
            WalletChannel::Bank => "🏦",
            WalletChannel::MobileMoney => "📱",
            WalletChannel::CashInHand => "💵",
        }
    }

    pub fn format_balance(wallet: &Wallet) -> String {
        format!("{} RWF", wallet.balance)
    }

    pub fn render(
        id: impl Into<ElementId>,
        wallet: &Wallet,
        custodian_name: Option<&str>,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let channel_str = Self::channel_label(wallet.channel, locale);
        let channel_ico = Self::channel_icon(wallet.channel);
        let balance_str = Self::format_balance(wallet);
        let is_family = wallet.scope.is_family();

        div()
            .id(id)
            .flex()
            .flex_col()
            .justify_between()
            .p(px(16.0))
            .rounded(px(12.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            // Header: channel badge & icon
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .child(div().text_base().child(channel_ico))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child(wallet.name.clone()),
                            ),
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(2.0))
                            .rounded(px(6.0))
                            .bg(theme.bg_elevated)
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(channel_str),
                    ),
            )
            // Balance row
            .child(
                div()
                    .my(px(12.0))
                    .text_xl()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .text_color(theme.income_color)
                    .child(balance_str),
            )
            // Footer: Custodian (in Family mode)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(if is_family {
                        let custodian_label = Translator::t(locale, terms::HELD_BY, None);
                        let name = custodian_name.unwrap_or("Family Custodian");
                        format!("{}: 👤 {}", custodian_label, name)
                    } else {
                        Translator::t(locale, terms::PERSONAL_SCOPE, None)
                    }),
            )
    }
}

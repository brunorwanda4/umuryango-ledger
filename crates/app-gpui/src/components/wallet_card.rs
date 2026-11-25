use gpui_kit::{
    ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement, Styled, div, px,
};
use umuryango_core_domain::models::{Wallet, WalletChannel};
use umuryango_i18n::{Locale, Translator, terms};

use crate::common::{Badge, Card, CardAction, CardContent, CardFooter, CardHeader, CardSize};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::SemanticTheme;

pub struct WalletCard;

impl WalletCard {
    pub fn channel_label(channel: WalletChannel, locale: Locale) -> String {
        match channel {
            WalletChannel::Bank => umuryango_i18n::t(locale, "wallets.channel_bank"),
            WalletChannel::MobileMoney => umuryango_i18n::t(locale, "wallets.channel_momo"),
            WalletChannel::CashInHand => umuryango_i18n::t(locale, "wallets.channel_cash"),
        }
    }

    pub const fn channel_icon(channel: WalletChannel) -> AppIcon {
        match channel {
            WalletChannel::Bank => AppIcon::Bank,
            WalletChannel::MobileMoney => AppIcon::MobileMoney,
            WalletChannel::CashInHand => AppIcon::Cash,
        }
    }

    pub fn format_balance(wallet: &Wallet) -> String {
        format!("{} {}", wallet.balance, wallet.currency)
    }

    pub fn render<T: Into<ElementId>>(
        id: T,
        wallet: &Wallet,
        custodian_name: Option<&str>,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement + use<T> {
        let channel_str = Self::channel_label(wallet.channel, locale);
        let channel_ico = Self::channel_icon(wallet.channel);
        let balance_str = Self::format_balance(wallet);
        let is_family = wallet.scope.is_family();

        div().id(id).child(
            Card::new()
                .size(CardSize::Sm)
                .gap(px(12.0))
                .py(px(16.0))
                .border_1()
                .border_color(theme.border_subtle)
                .theme(theme)
                .child(
                    CardHeader::new()
                        .size(CardSize::Sm)
                        .px(px(16.0))
                        .action(
                            CardAction::new().child(
                                Badge::new(channel_str)
                                    .secondary()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .theme(theme),
                            ),
                        )
                        .theme(theme)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(app_icon(
                                    channel_ico,
                                    IconSize::Md,
                                    IconTone::AccentPrimary,
                                    theme,
                                ))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child(wallet.name.clone()),
                                ),
                        ),
                )
                .child(
                    CardContent::new().size(CardSize::Sm).px(px(16.0)).child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.income_color)
                            .child(balance_str),
                    ),
                )
                .child(
                    CardFooter::new()
                        .size(CardSize::Sm)
                        .px(px(16.0))
                        .text_xs()
                        .text_color(theme.text_muted)
                        .theme(theme)
                        .children(if is_family {
                            let custodian_label = Translator::t(locale, terms::HELD_BY, None);
                            let fallback =
                                umuryango_i18n::t(locale, "wallets.unassigned_custodian");
                            let name = custodian_name.unwrap_or(&fallback);
                            vec![
                                app_icon(AppIcon::Members, IconSize::Xs, IconTone::Muted, theme)
                                    .into_any_element(),
                                div()
                                    .child(format!("{}: {}", custodian_label, name))
                                    .into_any_element(),
                            ]
                        } else {
                            vec![
                                div()
                                    .child(Translator::t(locale, terms::PERSONAL_SCOPE, None))
                                    .into_any_element(),
                            ]
                        }),
                ),
        )
    }
}

use gpui_kit::{Context, ElementId, InteractiveElement, IntoElement, ParentElement, Styled, div};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{Locale, Translator, terms};
use uuid::Uuid;

use crate::RootView;
use crate::common::{Button, ButtonGroup, ButtonSize, ButtonVariant};
use crate::components::icon::AppIcon;
use crate::theme::SemanticTheme;

pub struct ScopeSwitcher;

impl ScopeSwitcher {
    pub fn render(
        id: impl Into<ElementId>,
        current_scope: &LedgerScope,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_family = current_scope.is_family();
        let personal_label = Translator::t(locale, terms::PERSONAL_SCOPE, None);
        let family_label = Translator::t(locale, terms::FAMILY_SCOPE, None);

        div().id(id).w_full().child(
            ButtonGroup::horizontal()
                .w_full()
                .child(
                    Button::new("scope-family-btn")
                        .variant(if is_family {
                            ButtonVariant::Default
                        } else {
                            ButtonVariant::Outline
                        })
                        .size(ButtonSize::Xs)
                        .icon_start(AppIcon::FamilyLedger)
                        .label(family_label)
                        .flex_1()
                        .theme(theme)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.switch_to_family_scope(cx);
                        })),
                )
                .child(
                    Button::new("scope-personal-btn")
                        .variant(if !is_family {
                            ButtonVariant::Default
                        } else {
                            ButtonVariant::Outline
                        })
                        .size(ButtonSize::Xs)
                        .icon_start(AppIcon::PersonalWallet)
                        .label(personal_label)
                        .flex_1()
                        .theme(theme)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.switch_to_personal_scope(cx);
                        })),
                ),
        )
    }

    pub fn toggle(
        current_scope: &LedgerScope,
        default_family_id: Uuid,
        user_id: Uuid,
    ) -> LedgerScope {
        match current_scope {
            LedgerScope::Personal { .. } => LedgerScope::Family {
                family_id: default_family_id,
            },
            LedgerScope::Family { .. } => LedgerScope::Personal { user_id },
        }
    }
}

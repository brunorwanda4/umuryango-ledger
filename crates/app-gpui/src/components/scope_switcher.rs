use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{terms, Locale, Translator};
use uuid::Uuid;

use crate::theme::SemanticTheme;

pub struct ScopeSwitcher;

impl ScopeSwitcher {
    pub fn render(
        id: impl Into<ElementId>,
        current_scope: &LedgerScope,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let is_family = current_scope.is_family();
        let personal_label = Translator::t(locale, terms::PERSONAL_SCOPE, None);
        let family_label = Translator::t(locale, terms::FAMILY_SCOPE, None);

        let active_bg = theme.accent_primary;
        let inactive_bg = theme.bg_elevated;
        let active_text = theme.bg_canvas;
        let inactive_text = theme.text_muted;

        div()
            .id(id)
            .flex()
            .flex_row()
            .items_center()
            .p(px(3.0))
            .rounded(px(8.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            // Family scope button
            .child(
                div()
                    .flex_1()
                    .py(px(6.0))
                    .px(px(12.0))
                    .rounded(px(6.0))
                    .text_sm()
                    .font_weight(if is_family {
                        gpui_kit::FontWeight::SEMIBOLD
                    } else {
                        gpui_kit::FontWeight::NORMAL
                    })
                    .bg(if is_family { active_bg } else { inactive_bg })
                    .text_color(if is_family {
                        active_text
                    } else {
                        inactive_text
                    })
                    .cursor_pointer()
                    .child(family_label),
            )
            // Personal scope button
            .child(
                div()
                    .flex_1()
                    .py(px(6.0))
                    .px(px(12.0))
                    .rounded(px(6.0))
                    .text_sm()
                    .font_weight(if !is_family {
                        gpui_kit::FontWeight::SEMIBOLD
                    } else {
                        gpui_kit::FontWeight::NORMAL
                    })
                    .bg(if !is_family { active_bg } else { inactive_bg })
                    .text_color(if !is_family {
                        active_text
                    } else {
                        inactive_text
                    })
                    .cursor_pointer()
                    .child(personal_label),
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

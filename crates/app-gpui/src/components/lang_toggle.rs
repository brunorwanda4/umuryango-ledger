use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_i18n::Locale;

use crate::theme::SemanticTheme;

pub struct LangToggle;

impl LangToggle {
    pub fn next_locale(current: Locale) -> Locale {
        match current {
            Locale::Rw => Locale::En,
            Locale::En => Locale::Rw,
        }
    }

    pub fn display_badge(current: Locale) -> &'static str {
        match current {
            Locale::Rw => "🇷🇼 RW",
            Locale::En => "🇬🇧 EN",
        }
    }

    pub fn render(
        id: impl Into<ElementId>,
        current: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let label = Self::display_badge(current);

        div()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(6.0))
            .bg(theme.bg_elevated)
            .border_1()
            .border_color(theme.border_subtle)
            .text_color(theme.text_primary)
            .font_family(theme.font_family)
            .text_sm()
            .font_weight(gpui_kit::FontWeight::MEDIUM)
            .cursor_pointer()
            .child(label)
    }
}

use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, px};
use umuryango_i18n::Locale;

use crate::RootView;
use crate::common::{Button, ButtonSize};
use crate::components::icon::locale_flag;
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
            Locale::Rw => "RW",
            Locale::En => "EN",
        }
    }

    pub fn render(
        id: impl Into<ElementId>,
        current: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let label = Self::display_badge(current);

        Button::new(id)
            .outline()
            .size(ButtonSize::Sm)
            .pl(px(12.0))
            .gap(px(6.0))
            .child(locale_flag(current, 16.0))
            .child(label)
            .theme(theme)
            .on_click(cx.listener(|this, _, _, cx| {
                this.toggle_locale(cx);
            }))
    }
}

use gpui_kit::{Context, ElementId, IntoElement};
use umuryango_i18n::Locale;

use crate::RootView;
use crate::common::{Button, ButtonSize};
use crate::components::icon::AppIcon;
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
            .icon_start(AppIcon::Language)
            .label(label)
            .theme(theme)
            .on_click(cx.listener(|this, _, _, cx| {
                this.toggle_locale(cx);
            }))
    }
}

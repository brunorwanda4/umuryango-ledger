use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::Locale;

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
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

        div()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .gap(px(6.0))
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
            .on_click(cx.listener(|this, _, _, cx| {
                this.toggle_locale(cx);
            }))
            .child(app_icon(
                AppIcon::Language,
                IconSize::Xs,
                IconTone::Default,
                theme,
            ))
            .child(label)
    }
}

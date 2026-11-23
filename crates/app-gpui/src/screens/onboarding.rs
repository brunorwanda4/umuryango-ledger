use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_i18n::Locale;

use crate::theme::SemanticTheme;

pub struct OnboardingScreen {
    pub step: u8,
    pub selected_language: Locale,
}

impl Default for OnboardingScreen {
    fn default() -> Self {
        Self {
            step: 1,
            selected_language: Locale::Rw,
        }
    }
}

impl OnboardingScreen {
    pub fn render(
        id: impl Into<ElementId>,
        screen: &OnboardingScreen,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let is_rw = screen.selected_language == Locale::Rw;

        div()
            .id(id)
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            .child(
                div()
                    .w(px(460.0))
                    .p(px(28.0))
                    .rounded(px(16.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(16.0))
                    // Official Application Emblem
                    .child(
                        gpui_kit::img("crates/app-gpui/assets/icons/icon-256.png")
                            .w(px(64.0))
                            .h(px(64.0))
                            .rounded(px(14.0)),
                    )
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(if is_rw {
                                "Murakaza neza muri Umuryango Ledger"
                            } else {
                                "Welcome to Umuryango Ledger"
                            }),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_center()
                            .text_color(theme.text_muted)
                            .child(if is_rw {
                                "Uburyo bwo gucunga no kwizerana mu mafaranga y'umuryango n'ayawe bwite."
                            } else {
                                "The multi-tenant personal & family financial accountability platform."
                            }),
                    )
                    // Button: Create or Join
                    .child(
                        div()
                            .w_full()
                            .py(px(10.0))
                            .rounded(px(8.0))
                            .bg(theme.accent_primary)
                            .text_center()
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .child(if is_rw { "Tangira nonaha" } else { "Get Started" }),
                    ),
            )
    }
}

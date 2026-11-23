use gpui_kit::prelude::*;
use gpui_kit::{div, px, ElementId, IntoElement, ParentElement, Styled};
use umuryango_core_domain::models::FuturePlan;
use umuryango_i18n::{terms, Locale, Translator};

use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct FuturePlansScreen;

impl FuturePlansScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let title = Translator::t(state.locale, terms::FUTURE_PLANS, None);

        div()
            .id(id)
            .flex_1()
            .h_full()
            .p(px(24.0))
            .overflow_y_scroll()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            // Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .mb(px(20.0))
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(title),
                    )
                    .child(
                        div()
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(theme.accent_primary)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .child(if state.locale == Locale::Rw {
                                "+ Umugambi Mushya"
                            } else {
                                "+ New Goal"
                            }),
                    ),
            )
            // Plans list
            .child(
                div().flex().flex_col().gap(px(12.0)).children(
                    state
                        .future_plans
                        .iter()
                        .map(|p| Self::render_plan_card(p, state.locale, theme)),
                ),
            )
    }

    fn render_plan_card(
        plan: &FuturePlan,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let status_badge = if plan.is_achieved {
            ("✓ Byagezweho / Achieved", theme.income_color)
        } else {
            ("⏳ Biracyategerejwe / Planned", theme.text_muted)
        };

        div()
            .p(px(16.0))
            .rounded(px(12.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .mb(px(6.0))
                    .child(
                        div()
                            .text_base()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(plan.title.clone()),
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .bg(theme.bg_elevated)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(status_badge.1)
                            .child(status_badge.0),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .mb(px(10.0))
                    .child(plan.description.clone().unwrap_or_default()),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.income_color)
                            .child(format!("Estimated: {} RWF", plan.estimated_cost)),
                    )
                    .child(
                        div()
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(theme.accent_primary)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .child(if locale == Locale::Rw {
                                "Kuzigama"
                            } else {
                                "Allocate Funds"
                            }),
                    ),
            )
    }
}

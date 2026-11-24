use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::FuturePlan;
use umuryango_i18n::{Locale, Translator, terms};

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct FuturePlansScreen;

impl FuturePlansScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let title = Translator::t(state.locale, terms::FUTURE_PLANS, None);

        div()
            .id(id)
            .flex_1()
            .h_full()
            .pt(px(24.0))
            .pb(px(24.0) + state.safe_area.total_bottom_px())
            .pl(px(24.0) + state.safe_area.left_px())
            .pr(px(24.0) + state.safe_area.right_px())
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
                            .id("future-plans-add-btn")
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(12.0))
                            .py(px(6.0))
                            .rounded(px(6.0))
                            .bg(theme.accent_primary)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.open_quick_entry(cx);
                            }))
                            .child(app_icon(
                                AppIcon::Add,
                                IconSize::Xs,
                                IconTone::OnPrimaryButton,
                                theme,
                            ))
                            .child(umuryango_i18n::t(state.locale, "future_plans.new_plan")),
                    ),
            )
            // Plans list or empty state
            .child(div().flex().flex_col().gap(px(12.0)).children(
                if state.future_plans.is_empty() {
                    vec![
                        div()
                            .p(px(24.0))
                            .rounded(px(8.0))
                            .bg(theme.bg_surface)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_sm()
                            .text_center()
                            .text_color(theme.text_muted)
                            .child(umuryango_i18n::t(state.locale, "future_plans.no_plans"))
                            .into_any_element(),
                    ]
                } else {
                    state
                        .future_plans
                        .iter()
                        .map(|p| {
                            Self::render_plan_card(p, state.locale, theme, cx).into_any_element()
                        })
                        .collect()
                },
            ))
    }

    fn render_plan_card(
        plan: &FuturePlan,
        locale: Locale,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let status_badge = if plan.is_achieved {
            ("Byagezweho / Achieved", theme.income_color)
        } else {
            ("Biracyategerejwe / Planned", theme.text_muted)
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
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(app_icon(
                                AppIcon::FuturePlans,
                                IconSize::Sm,
                                IconTone::AccentPrimary,
                                theme,
                            ))
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(plan.title.clone()),
                            ),
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
                            .id(format!("plan-alloc-{}", plan.id))
                            .px(px(10.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(theme.accent_primary)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.open_quick_entry(cx);
                            }))
                            .child(umuryango_i18n::t(locale, "future_plans.allocate_funds")),
                    ),
            )
    }
}

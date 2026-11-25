use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::models::FuturePlan;
use umuryango_i18n::{Locale, Translator, terms};

use crate::RootView;
use crate::common::{
    Badge, Button, ButtonSize, Card, CardAction, CardDescription, CardFooter, CardHeader, CardSize,
    CardTitle, Empty, EmptyDescription, EmptyHeader, EmptyMedia,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{ActiveModal, AppState};
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
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(title),
                    )
                    .child(
                        Button::new("future-plans-add-btn")
                            .size(ButtonSize::Sm)
                            .icon_start(AppIcon::Add)
                            .label(umuryango_i18n::t(state.locale, "future_plans.new_plan"))
                            .theme(theme)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.open_modal(ActiveModal::AddPlan, window, cx);
                            })),
                    ),
            )
            // Plans list or empty state
            .child(div().flex().flex_col().gap(px(12.0)).children(
                if state.future_plans.is_empty() {
                    vec![
                        Empty::new()
                            .bordered(true)
                            .p(px(32.0))
                            .bg(theme.bg_surface)
                            .theme(theme)
                            .child(
                                EmptyHeader::new()
                                    .child(EmptyMedia::from_icon(AppIcon::FuturePlans).theme(theme))
                                    .child(
                                        EmptyDescription::new(umuryango_i18n::t(
                                            state.locale,
                                            "future_plans.no_plans",
                                        ))
                                        .theme(theme),
                                    ),
                            )
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
        let (status_label, status_color) = if plan.is_achieved {
            (
                umuryango_i18n::t(locale, "future_plans.status_achieved"),
                theme.income_color,
            )
        } else {
            (
                umuryango_i18n::t(locale, "future_plans.status_planned"),
                theme.text_muted,
            )
        };
        let plan_id = plan.id;
        let plan_title = plan.title.clone();

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
                            Badge::new(status_label)
                                .secondary()
                                .px(px(8.0))
                                .py(px(2.0))
                                .text_color(status_color)
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
                                AppIcon::FuturePlans,
                                IconSize::Sm,
                                IconTone::AccentPrimary,
                                theme,
                            ))
                            .child(CardTitle::new(plan.title.clone()).text_base().theme(theme)),
                    )
                    .children(
                        plan.description
                            .as_ref()
                            .filter(|d| !d.is_empty())
                            .map(|desc| CardDescription::new(desc.clone()).text_xs().theme(theme)),
                    ),
            )
            .child(
                CardFooter::new()
                    .size(CardSize::Sm)
                    .px(px(16.0))
                    .justify_between()
                    .theme(theme)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.income_color)
                            .child(format!(
                                "{}: {} RWF",
                                umuryango_i18n::t(locale, "future_plans.estimated_cost"),
                                plan.estimated_cost
                            )),
                    )
                    .child(
                        Button::new(format!("plan-alloc-{}", plan_id))
                            .size(ButtonSize::Xs)
                            .label(umuryango_i18n::t(locale, "future_plans.allocate_funds"))
                            .theme(theme)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.open_modal(
                                    ActiveModal::AllocatePlan {
                                        plan_id,
                                        title: plan_title.clone(),
                                    },
                                    window,
                                    cx,
                                );
                            })),
                    ),
            )
    }
}

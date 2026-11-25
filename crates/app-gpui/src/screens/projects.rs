use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use umuryango_core_domain::models::Project;
use umuryango_i18n::{Translator, terms};

use crate::RootView;
use crate::common::{
    Badge, Button, ButtonSize, Card, CardAction, CardContent, CardDescription, CardHeader,
    CardSize, CardTitle, Empty, EmptyDescription, EmptyHeader, EmptyMedia, Progress, ProgressLabel,
    ProgressValue,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{ActiveModal, AppState};
use crate::theme::SemanticTheme;

pub struct ProjectsScreen;

impl ProjectsScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let title = Translator::t(state.locale, terms::ACTIVE_PROJECTS, None);

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
                        Button::new("projects-add-btn")
                            .size(ButtonSize::Sm)
                            .icon_start(AppIcon::Add)
                            .label(umuryango_i18n::t(state.locale, "projects.new_project"))
                            .theme(theme)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.open_modal(ActiveModal::AddProject, window, cx);
                            })),
                    ),
            )
            // Projects Grid or Empty State
            .child(if state.projects.is_empty() {
                Empty::new()
                    .bordered(true)
                    .p(px(32.0))
                    .bg(theme.bg_surface)
                    .theme(theme)
                    .child(
                        EmptyHeader::new()
                            .child(EmptyMedia::from_icon(AppIcon::Projects).theme(theme))
                            .child(
                                EmptyDescription::new(if state.active_scope.is_family() {
                                    umuryango_i18n::t(state.locale, "projects.no_projects")
                                } else {
                                    umuryango_i18n::t(
                                        state.locale,
                                        "projects.family_scope_required",
                                    )
                                })
                                .theme(theme),
                            ),
                    )
                    .into_any_element()
            } else {
                div()
                    .grid()
                    .grid_cols(2)
                    .gap(px(16.0))
                    .children(
                        state
                            .projects
                            .iter()
                            .map(|p| Self::render_project_card(p, theme)),
                    )
                    .into_any_element()
            })
    }

    fn render_project_card(project: &Project, theme: &SemanticTheme) -> impl IntoElement {
        let pct_f32 = if project.target_amount > Decimal::ZERO {
            ((project.allocated_amount / project.target_amount)
                .to_f32()
                .unwrap_or(0.0)
                .clamp(0.0, 1.0))
                * 100.0
        } else {
            0.0
        };
        let pct = pct_f32.round() as i32;

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
                            Badge::new(project.status.as_str())
                                .secondary()
                                .px(px(6.0))
                                .py(px(2.0))
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
                                AppIcon::Projects,
                                IconSize::Sm,
                                IconTone::AccentPrimary,
                                theme,
                            ))
                            .child(
                                CardTitle::new(project.name.clone())
                                    .text_base()
                                    .theme(theme),
                            ),
                    )
                    .children(
                        project
                            .description
                            .as_ref()
                            .filter(|d| !d.is_empty())
                            .map(|desc| CardDescription::new(desc.clone()).text_xs().theme(theme)),
                    ),
            )
            .child(
                CardContent::new().size(CardSize::Sm).px(px(16.0)).child(
                    Progress::new(pct_f32)
                        .indicator_color(theme.income_color)
                        .gap(px(6.0))
                        .theme(theme)
                        .child(
                            ProgressLabel::new(format!(
                                "{} / {} RWF",
                                project.allocated_amount, project.target_amount
                            ))
                            .theme(theme),
                        )
                        .child(ProgressValue::new(format!("{}%", pct)).theme(theme)),
                ),
            )
    }
}

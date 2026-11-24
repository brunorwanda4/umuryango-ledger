use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px, relative};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use umuryango_core_domain::models::Project;
use umuryango_i18n::{Translator, terms};

use crate::RootView;
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
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(title),
                    )
                    .child(
                        div()
                            .id("projects-add-btn")
                            .flex()
                            .items_center()
                            .gap(px(6.0))
                            .px(px(12.0))
                            .py(px(6.0))
                            .bg(theme.accent_primary)
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::BOLD)
                            .text_color(theme.bg_canvas)
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.open_modal(ActiveModal::AddProject, window, cx);
                            }))
                            .child(app_icon(
                                AppIcon::Add,
                                IconSize::Xs,
                                IconTone::OnPrimaryButton,
                                theme,
                            ))
                            .child(umuryango_i18n::t(state.locale, "projects.new_project")),
                    ),
            )
            // Projects Grid or Empty State
            .child(if state.projects.is_empty() {
                div()
                    .p(px(24.0))
                    .bg(theme.bg_surface)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .text_sm()
                    .text_center()
                    .text_color(theme.text_muted)
                    .child(if state.active_scope.is_family() {
                        umuryango_i18n::t(state.locale, "projects.no_projects")
                    } else {
                        umuryango_i18n::t(state.locale, "projects.family_scope_required")
                    })
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
        let ratio = if project.target_amount > Decimal::ZERO {
            (project.allocated_amount / project.target_amount)
                .to_f32()
                .unwrap_or(0.0)
                .clamp(0.0, 1.0)
        } else {
            0.0
        };
        let pct = (ratio * 100.0).round() as i32;

        div()
            .p(px(16.0))
            .bg(theme.bg_surface)
            .border_1()
            .border_color(theme.border_subtle)
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .mb(px(8.0))
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
                                div()
                                    .text_base()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(project.name.clone()),
                            ),
                    )
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(2.0))
                            .bg(theme.bg_elevated)
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(project.status.as_str()),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .mb(px(12.0))
                    .child(project.description.clone().unwrap_or_default()),
            )
            // Progress row
            .child(
                div()
                    .flex()
                    .justify_between()
                    .text_xs()
                    .text_color(theme.text_primary)
                    .mb(px(4.0))
                    .child(format!(
                        "{} / {} RWF",
                        project.allocated_amount, project.target_amount
                    ))
                    .child(format!("{}%", pct)),
            )
            .child(
                div()
                    .w_full()
                    .h(px(8.0))
                    .bg(theme.bg_elevated)
                    .child(div().h_full().bg(theme.income_color).w(relative(ratio))),
            )
    }
}

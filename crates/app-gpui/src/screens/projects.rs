use gpui_kit::prelude::*;
use gpui_kit::{ElementId, IntoElement, ParentElement, Styled, div, px};
use rust_decimal::Decimal;
use umuryango_core_domain::models::Project;
use umuryango_i18n::{Locale, Translator, terms};

use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::AppState;
use crate::theme::SemanticTheme;

pub struct ProjectsScreen;

impl ProjectsScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
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
                            .child(app_icon(
                                AppIcon::Add,
                                IconSize::Xs,
                                IconTone::OnPrimaryButton,
                                theme,
                            ))
                            .child(if state.locale == Locale::Rw {
                                "Umushinga Mushya"
                            } else {
                                "New Project"
                            }),
                    ),
            )
            // Projects Grid
            .child(
                div().grid().grid_cols(2).gap(px(16.0)).children(
                    state
                        .projects
                        .iter()
                        .map(|p| Self::render_project_card(p, theme)),
                ),
            )
    }

    fn render_project_card(project: &Project, theme: &SemanticTheme) -> impl IntoElement {
        let pct = if project.target_amount > Decimal::ZERO {
            ((project.allocated_amount / project.target_amount) * Decimal::new(100, 0)).to_string()
        } else {
            "0".to_string()
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
                            .rounded(px(4.0))
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
                    .rounded(px(4.0))
                    .bg(theme.bg_elevated)
                    .child(
                        div()
                            .h_full()
                            .rounded(px(4.0))
                            .bg(theme.income_color)
                            .w(px(140.0)),
                    ),
            )
    }
}

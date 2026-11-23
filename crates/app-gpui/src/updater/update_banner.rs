use gpui_kit::prelude::*;
use gpui_kit::{ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Locale, Translator, terms};

use super::UpdateStatus;
use crate::theme::SemanticTheme;

pub struct UpdateBanner;

impl UpdateBanner {
    pub fn render(
        id: impl Into<ElementId>,
        status: &UpdateStatus,
        locale: Locale,
        theme: &SemanticTheme,
    ) -> Option<impl IntoElement> {
        match status {
            UpdateStatus::Idle | UpdateStatus::Checking => None,
            UpdateStatus::Available {
                version,
                is_mandatory,
                ..
            } => {
                let title = if *is_mandatory {
                    Translator::t(locale, terms::UPDATE_MANDATORY_TITLE, None)
                } else {
                    Translator::t(locale, terms::UPDATE_READY_BANNER, None)
                };

                let download_btn_text = if locale == Locale::Rw {
                    "Kura Verisiyo Nshya"
                } else {
                    "Download Update"
                };

                let banner_bg = if *is_mandatory {
                    theme.expense_color
                } else {
                    theme.income_color
                };

                Some(
                    div()
                        .id(id)
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px(px(16.0))
                        .py(px(10.0))
                        .bg(banner_bg)
                        .text_color(theme.bg_canvas)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .font_weight(gpui_kit::FontWeight::BOLD)
                                        .text_sm()
                                        .child(format!("v{}", version)),
                                )
                                .child(div().text_sm().child(title)),
                        )
                        .child(
                            div().flex().items_center().gap(px(8.0)).child(
                                div()
                                    .px(px(12.0))
                                    .py(px(4.0))
                                    .rounded(px(6.0))
                                    .bg(theme.bg_surface)
                                    .text_color(theme.text_primary)
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_xs()
                                    .cursor_pointer()
                                    .child(download_btn_text),
                            ),
                        ),
                )
            }
            UpdateStatus::Downloading { progress } => {
                let progress_pct = (progress * 100.0) as u32;
                let downloading_text = Translator::t(locale, terms::UPDATE_DOWNLOADING, None);

                Some(
                    div()
                        .id(id)
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px(px(16.0))
                        .py(px(10.0))
                        .bg(theme.bg_elevated)
                        .text_color(theme.text_primary)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(div().text_sm().child(downloading_text))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(gpui_kit::FontWeight::BOLD)
                                        .child(format!("{}%", progress_pct)),
                                ),
                        ),
                )
            }
            UpdateStatus::ReadyToRestart { version, .. } => {
                let restart_text = Translator::t(locale, terms::UPDATE_RESTART_NOW, None);
                let ready_msg = if locale == Locale::Rw {
                    format!("Verisiyo {} iriteguye. Funga wongere ufungure.", version)
                } else {
                    format!("Version {} ready. Restart application to apply.", version)
                };

                Some(
                    div()
                        .id(id)
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px(px(16.0))
                        .py(px(10.0))
                        .bg(theme.accent_secondary)
                        .text_color(theme.bg_canvas)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(div().text_sm().child(ready_msg)),
                        )
                        .child(
                            div()
                                .px(px(12.0))
                                .py(px(4.0))
                                .rounded(px(6.0))
                                .bg(theme.bg_surface)
                                .text_color(theme.text_primary)
                                .font_weight(gpui_kit::FontWeight::BOLD)
                                .text_xs()
                                .cursor_pointer()
                                .child(restart_text),
                        ),
                )
            }
            UpdateStatus::Failed { error } => {
                let err_label = if locale == Locale::Rw {
                    format!("Kuvugurura byanze: {}", error)
                } else {
                    format!("Update failed: {}", error)
                };

                Some(
                    div()
                        .id(id)
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px(px(16.0))
                        .py(px(10.0))
                        .bg(theme.expense_color)
                        .text_color(theme.bg_canvas)
                        .child(div().text_xs().child(err_label)),
                )
            }
        }
    }
}

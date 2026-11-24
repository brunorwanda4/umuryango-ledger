use gpui_kit::prelude::*;
use gpui_kit::{ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Locale, Translator, terms};

use super::UpdateStatus;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
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

                let download_btn_text = umuryango_i18n::t(locale, "updater.download_now");

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
                                .child(app_icon(
                                    AppIcon::UpdateAvailable,
                                    IconSize::Sm,
                                    IconTone::OnPrimaryButton,
                                    theme,
                                ))
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
                                .child(app_icon(
                                    AppIcon::SyncPending,
                                    IconSize::Sm,
                                    IconTone::Default,
                                    theme,
                                ))
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
                let ready_msg = umuryango_i18n::t_args(
                    locale,
                    "updater.restart_ready",
                    &[("version", version.as_str())],
                );

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
                                .child(app_icon(
                                    AppIcon::Check,
                                    IconSize::Sm,
                                    IconTone::OnPrimaryButton,
                                    theme,
                                ))
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
                let err_label =
                    umuryango_i18n::t_args(locale, "updater.failed", &[("error", error.as_str())]);

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
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(app_icon(
                                    AppIcon::Error,
                                    IconSize::Sm,
                                    IconTone::OnPrimaryButton,
                                    theme,
                                ))
                                .child(div().text_xs().child(err_label)),
                        ),
                )
            }
        }
    }
}

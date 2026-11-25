use gpui_kit::{ElementId, InteractiveElement, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Locale, Translator, terms};

use super::UpdateStatus;
use crate::common::{
    Alert, AlertAction, AlertDescription, AlertTitle, AlertVariant, Badge, Progress, ProgressValue,
};
use crate::components::icon::AppIcon;
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
                let download_badge = umuryango_i18n::t(locale, "updater.download_now");

                Some(
                    div().id(id).w_full().child(
                        Alert::new()
                            .variant(if *is_mandatory {
                                AlertVariant::Destructive
                            } else {
                                AlertVariant::Default
                            })
                            .icon(AppIcon::UpdateAvailable)
                            .action(
                                AlertAction::new().child(
                                    Badge::new(download_badge)
                                        .outline()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .theme(theme),
                                ),
                            )
                            .theme(theme)
                            .child(AlertTitle::new(format!("v{} — {}", version, title))),
                    ),
                )
            }
            UpdateStatus::Downloading { progress } => {
                let progress_pct = (progress * 100.0).clamp(0.0, 100.0);
                let downloading_text = Translator::t(locale, terms::UPDATE_DOWNLOADING, None);

                Some(
                    div().id(id).w_full().child(
                        Alert::new()
                            .icon(AppIcon::SyncPending)
                            .theme(theme)
                            .child(AlertTitle::new(downloading_text))
                            .child(
                                Progress::new(progress_pct).theme(theme).child(
                                    ProgressValue::new(format!("{}%", progress_pct as u32))
                                        .theme(theme),
                                ),
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
                    div().id(id).w_full().child(
                        Alert::new()
                            .icon(AppIcon::Check)
                            .action(
                                AlertAction::new().child(
                                    Badge::new(restart_text)
                                        .secondary()
                                        .px(px(8.0))
                                        .py(px(3.0))
                                        .theme(theme),
                                ),
                            )
                            .theme(theme)
                            .child(AlertTitle::new(ready_msg)),
                    ),
                )
            }
            UpdateStatus::Failed { error } => {
                let err_label =
                    umuryango_i18n::t_args(locale, "updater.failed", &[("error", error.as_str())]);

                Some(
                    div().id(id).w_full().child(
                        Alert::new()
                            .destructive()
                            .icon(AppIcon::Error)
                            .theme(theme)
                            .child(
                                AlertDescription::new(err_label)
                                    .destructive(true)
                                    .theme(theme),
                            ),
                    ),
                )
            }
        }
    }
}

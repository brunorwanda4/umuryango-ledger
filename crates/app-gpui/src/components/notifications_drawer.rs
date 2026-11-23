use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::realtime::RealtimeEvent;

use crate::RootView;
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::state::{AppState, NotificationItem};
use crate::theme::SemanticTheme;

pub struct NotificationsDrawer;

impl NotificationsDrawer {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let title = umuryango_i18n::t(state.locale, "notifications.title");
        let mark_all_label = umuryango_i18n::t(state.locale, "notifications.mark_all_read");
        let empty_text = umuryango_i18n::t(state.locale, "notifications.empty");

        div()
            .id(id)
            .w(px(380.0))
            .h_full()
            .bg(theme.bg_surface)
            .border_l_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            .flex()
            .flex_col()
            .justify_between()
            // Drawer Header
            .child(
                div()
                    .p(px(16.0))
                    .border_b_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            .child(app_icon(
                                AppIcon::Bell,
                                IconSize::Sm,
                                IconTone::AccentPrimary,
                                theme,
                            ))
                            .child(
                                div()
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_base()
                                    .text_color(theme.text_primary)
                                    .child(title),
                            )
                            .children(if state.unread_notifications_count > 0 {
                                Some(
                                    div()
                                        .px(px(6.0))
                                        .py(px(2.0))
                                        .rounded_full()
                                        .bg(theme.accent_primary)
                                        .text_xs()
                                        .font_weight(gpui_kit::FontWeight::BOLD)
                                        .text_color(theme.bg_canvas)
                                        .child(state.unread_notifications_count.to_string()),
                                )
                            } else {
                                None
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(10.0))
                            .child(
                                div()
                                    .id("drawer-mark-all-read")
                                    .cursor_pointer()
                                    .text_xs()
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .text_color(theme.income_color)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.mark_all_notifications_read(cx);
                                    }))
                                    .child(mark_all_label),
                            )
                            .child(
                                div()
                                    .id("drawer-close-btn")
                                    .p(px(4.0))
                                    .rounded(px(4.0))
                                    .cursor_pointer()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.close_notifications(cx);
                                    }))
                                    .child(app_icon(
                                        AppIcon::Close,
                                        IconSize::Xs,
                                        IconTone::Default,
                                        theme,
                                    )),
                            ),
                    ),
            )
            // Notifications List
            .child(
                div()
                    .id("notifications-scroll-area")
                    .flex_1()
                    .overflow_y_scroll()
                    .p(px(16.0))
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .children(if state.notifications.is_empty() {
                        vec![
                            div()
                                .py(px(40.0))
                                .text_center()
                                .text_sm()
                                .text_color(theme.text_muted)
                                .child(empty_text)
                                .into_any_element(),
                        ]
                    } else {
                        state
                            .notifications
                            .iter()
                            .map(|item| {
                                Self::render_notification_item(item, theme).into_any_element()
                            })
                            .collect()
                    }),
            )
            // Drawer Footer
            .child(
                div()
                    .p(px(12.0))
                    .border_t_1()
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("Umuryango Real-Time Hub"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.income_color)
                            .child("Live"),
                    ),
            )
    }

    fn render_notification_item(
        item: &NotificationItem,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let (icon, icon_tone) = match &item.event {
            RealtimeEvent::IncomeAdded { .. } => (AppIcon::Income, IconTone::Income),
            RealtimeEvent::ExpenseLogged { .. } => (AppIcon::Expense, IconTone::Expense),
            RealtimeEvent::HandoverCompleted { .. } => {
                (AppIcon::Handover, IconTone::AccentSecondary)
            }
            RealtimeEvent::ProjectOrLoanUpdated { .. } => {
                (AppIcon::Projects, IconTone::AccentSecondary)
            }
            RealtimeEvent::AppUpdateAvailable { is_mandatory, .. } => (
                AppIcon::UpdateAvailable,
                if *is_mandatory {
                    IconTone::Expense
                } else {
                    IconTone::Income
                },
            ),
        };

        let card_bg = if item.is_read {
            theme.bg_canvas
        } else {
            theme.bg_elevated
        };

        let formatted_time = item.timestamp.format("%H:%M").to_string();

        div()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(12.0))
            .p(px(10.0))
            .rounded(px(8.0))
            .bg(card_bg)
            .border_1()
            .border_color(if !item.is_read {
                theme.accent_primary
            } else {
                theme.border_subtle
            })
            // Vector Icon badge
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(32.0))
                    .h(px(32.0))
                    .rounded(px(6.0))
                    .bg(theme.bg_surface)
                    .child(app_icon(icon, IconSize::Sm, icon_tone, theme)),
            )
            // Content
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(if !item.is_read {
                                gpui_kit::FontWeight::SEMIBOLD
                            } else {
                                gpui_kit::FontWeight::NORMAL
                            })
                            .text_color(theme.text_primary)
                            .child(item.message.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(formatted_time),
                    ),
            )
    }
}

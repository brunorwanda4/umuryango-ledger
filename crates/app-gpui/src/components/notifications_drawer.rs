use gpui_kit::prelude::*;
use gpui_kit::{ElementId, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::realtime::RealtimeEvent;
use umuryango_i18n::Locale;

use crate::state::{AppState, NotificationItem};
use crate::theme::SemanticTheme;

pub struct NotificationsDrawer;

impl NotificationsDrawer {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let (title, mark_all_label, empty_text) = match state.locale {
            Locale::Rw => (
                "Imenyesha ry'Imari",
                "Soma Byose",
                "Nta bimenyesha bishya bihari.",
            ),
            Locale::En => (
                "Financial Notifications",
                "Mark All as Read",
                "No notifications yet.",
            ),
        };

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
                            .cursor_pointer()
                            .text_xs()
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.income_color)
                            .child(mark_all_label),
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
        let (icon, icon_bg, icon_color) = match &item.event {
            RealtimeEvent::IncomeAdded { .. } => ("+", theme.bg_elevated, theme.income_color),
            RealtimeEvent::ExpenseLogged { .. } => ("-", theme.bg_elevated, theme.expense_color),
            RealtimeEvent::HandoverCompleted { .. } => {
                ("->", theme.bg_elevated, theme.handover_color)
            }
            RealtimeEvent::ProjectOrLoanUpdated { .. } => {
                ("#", theme.bg_elevated, theme.accent_secondary)
            }
            RealtimeEvent::AppUpdateAvailable { is_mandatory, .. } => {
                if *is_mandatory {
                    ("!", theme.bg_elevated, theme.expense_color)
                } else {
                    ("^", theme.bg_elevated, theme.income_color)
                }
            }
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
            // Icon
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(32.0))
                    .h(px(32.0))
                    .rounded(px(6.0))
                    .bg(icon_bg)
                    .text_sm()
                    .font_weight(gpui_kit::FontWeight::BOLD)
                    .text_color(icon_color)
                    .child(icon),
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

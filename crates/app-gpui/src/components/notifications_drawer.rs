use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use umuryango_core_domain::realtime::RealtimeEvent;

use crate::RootView;
use crate::common::{
    Badge, Button, ButtonSize, Empty, EmptyDescription, EmptyHeader, EmptyMedia, Item, ItemContent,
    ItemMedia, ItemSize, Separator, SheetContent, SheetFooter, SheetHeader, SheetOverlay,
    SheetSide, SheetTitle,
};
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

        let view_for_close = cx.entity().clone();

        SheetOverlay::new("notifications-sheet-overlay")
            .side(SheetSide::Right)
            .on_click(cx.listener(|this, _, _, cx| {
                this.close_notifications(cx);
            }))
            .child(
                SheetContent::new(id)
                    .side(SheetSide::Right)
                    .show_close_button(true)
                    .on_close(move |_window, cx| {
                        view_for_close.update(cx, |this, cx| {
                            this.close_notifications(cx);
                        });
                    })
                    .w(px(380.0))
                    .theme(theme)
                    // Drawer Header
                    .child(
                        SheetHeader::new()
                            .p(px(16.0))
                            .pr(px(56.0))
                            .gap(px(10.0))
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
                                    .child(SheetTitle::new(title).text_base().theme(theme))
                                    .children(if state.unread_notifications_count > 0 {
                                        Some(
                                            Badge::new(
                                                state.unread_notifications_count.to_string(),
                                            )
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .bg(theme.accent_primary)
                                            .text_color(theme.bg_canvas)
                                            .theme(theme),
                                        )
                                    } else {
                                        None
                                    }),
                            )
                            .child(
                                Button::new("drawer-mark-all-read")
                                    .ghost()
                                    .size(ButtonSize::Xs)
                                    .label(mark_all_label)
                                    .text_color(theme.income_color)
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.mark_all_notifications_read(cx);
                                    })),
                            ),
                    )
                    .child(Separator::horizontal().theme(theme))
                    // Notifications List
                    .child(
                        div()
                            .id("notifications-scroll-area")
                            .flex_1()
                            .overflow_y_scroll()
                            .p(px(16.0))
                            .flex()
                            .flex_col()
                            .gap(px(10.0))
                            .children(if state.notifications.is_empty() {
                                vec![
                                    Empty::new()
                                        .p(px(28.0))
                                        .theme(theme)
                                        .child(
                                            EmptyHeader::new()
                                                .child(
                                                    EmptyMedia::from_icon(AppIcon::Bell)
                                                        .theme(theme),
                                                )
                                                .child(
                                                    EmptyDescription::new(empty_text).theme(theme),
                                                ),
                                        )
                                        .into_any_element(),
                                ]
                            } else {
                                state
                                    .notifications
                                    .iter()
                                    .map(|item| {
                                        Self::render_notification_item(item, theme)
                                            .into_any_element()
                                    })
                                    .collect()
                            }),
                    )
                    .child(Separator::horizontal().theme(theme))
                    // Drawer Footer
                    .child(
                        SheetFooter::new()
                            .p(px(12.0))
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
                                Badge::new("Live")
                                    .outline()
                                    .text_color(theme.income_color)
                                    .theme(theme),
                            ),
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

        let formatted_time = item.timestamp.format("%H:%M").to_string();

        Item::new()
            .outline()
            .size(ItemSize::Xs)
            .items_start()
            .bg(if item.is_read {
                theme.bg_canvas
            } else {
                theme.bg_elevated
            })
            .border_color(if !item.is_read {
                theme.accent_primary
            } else {
                theme.border_subtle
            })
            .theme(theme)
            .child(
                ItemMedia::new()
                    .w(px(32.0))
                    .h(px(32.0))
                    .bg(theme.bg_surface)
                    .theme(theme)
                    .child(app_icon(icon, IconSize::Sm, icon_tone, theme)),
            )
            .child(
                ItemContent::new()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(if !item.is_read {
                                FontWeight::SEMIBOLD
                            } else {
                                FontWeight::NORMAL
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

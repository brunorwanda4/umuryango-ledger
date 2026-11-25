//! Toast notification stack and manager primitives ported from
//! `web/src/components/ui/toast.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `ToastViewport`: `top-4 inset-x-4 max-w-sm mx-auto` on small screens (`< 768px`),
//!   `bottom-4 right-4 max-w-sm` on larger screens (`>= 768px`) via `is_mobile`
//! - `Toast`: `w-full rounded-none border bg-popover text-popover-foreground shadow-lg`
//! - `ToastContent`: `flex items-center gap-3 p-4`
//! - `ToastIcon`: `Success` | `Info` | `Warning` | `Error` (`app_icon`) | `Loading` ([`Spinner`])
//! - `ToastTitle`: `text-sm font-medium`, `ToastDescription`: `text-sm text-muted-foreground`
//! - `ToastAction` (`Button` `Outline` `Sm`) & `ToastClose` (`Button` `Ghost` `IconSm` `AppIcon::Close`).

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::notification::{
    Notification as KitNotification, NotificationList as KitNotificationList,
    NotificationSettings as KitNotificationSettings, NotificationType as KitNotificationType,
};
use gpui_kit::{
    AnyElement, App, AppContext as _, ClickEvent, ElementId, Entity, FontWeight,
    InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::dialog::DialogPortal;
use crate::common::spinner::Spinner;
use crate::common::utils::{PresetRadius, PresetTokens, overlay_surface_shadow};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::hooks::use_mobile::{Breakpoint, is_mobile};
use crate::safe_area::SafeAreaInsets;
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Semantic status variant matching `ToastIcon` in `toast.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastType {
    #[default]
    Default,
    Success,
    Info,
    Warning,
    Error,
    Loading,
}

fn resolve_theme(explicit: Option<&SemanticTheme>, cx: &App) -> SemanticTheme {
    explicit.cloned().unwrap_or_else(|| {
        SemanticTheme::from_config(
            &ThemeConfig::default(),
            if gpui_kit::component::Theme::global(cx).mode.is_dark() {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        )
    })
}

/// Data model for a queued toast item inside [`ToastManager`].
#[derive(Clone)]
pub struct ToastRecord {
    pub id: u64,
    pub toast_type: ToastType,
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub action_label: Option<SharedString>,
    pub on_action: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl ToastRecord {
    pub fn new(id: u64, title: impl Into<SharedString>) -> Self {
        Self {
            id,
            toast_type: ToastType::Default,
            title: title.into(),
            description: None,
            action_label: None,
            on_action: None,
        }
    }

    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.toast_type = toast_type;
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.action_label = Some(label.into());
        self.on_action = Some(Rc::new(handler));
        self
    }
}

/// Reactive GPUI state manager (`createToastManager` in `toast.tsx`).
#[derive(Default)]
pub struct ToastManager {
    next_id: u64,
    items: Vec<ToastRecord>,
    max_visible: usize,
}

impl ToastManager {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            items: Vec::new(),
            max_visible: 5,
        }
    }

    pub fn max_visible(mut self, max_visible: usize) -> Self {
        self.max_visible = max_visible.max(1);
        self
    }

    pub fn toasts(&self) -> &[ToastRecord] {
        &self.items
    }

    pub fn push(&mut self, mut record: ToastRecord) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        record.id = id;
        self.items.push(record);
        if self.items.len() > self.max_visible.max(1) {
            let excess = self.items.len() - self.max_visible.max(1);
            self.items.drain(0..excess);
        }
        id
    }

    pub fn success(
        &mut self,
        title: impl Into<SharedString>,
        description: Option<SharedString>,
    ) -> u64 {
        let mut rec = ToastRecord::new(0, title).toast_type(ToastType::Success);
        rec.description = description;
        self.push(rec)
    }

    pub fn error(
        &mut self,
        title: impl Into<SharedString>,
        description: Option<SharedString>,
    ) -> u64 {
        let mut rec = ToastRecord::new(0, title).toast_type(ToastType::Error);
        rec.description = description;
        self.push(rec)
    }

    pub fn warning(
        &mut self,
        title: impl Into<SharedString>,
        description: Option<SharedString>,
    ) -> u64 {
        let mut rec = ToastRecord::new(0, title).toast_type(ToastType::Warning);
        rec.description = description;
        self.push(rec)
    }

    pub fn info(
        &mut self,
        title: impl Into<SharedString>,
        description: Option<SharedString>,
    ) -> u64 {
        let mut rec = ToastRecord::new(0, title).toast_type(ToastType::Info);
        rec.description = description;
        self.push(rec)
    }

    pub fn dismiss(&mut self, id: u64) {
        self.items.retain(|t| t.id != id);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

/// Creates a GPUI [`Entity<ToastManager>`] (`createToastManager` in `toast.tsx`).
pub fn create_toast_manager(cx: &mut App) -> Entity<ToastManager> {
    cx.new(|_| ToastManager::new())
}

/// Deferred GPUI overlay layer (`ToastPortal` in `toast.tsx`).
pub type ToastPortal = DialogPortal;

/// Structural provider wrapper (`ToastProvider` in `toast.tsx`).
#[derive(IntoElement)]
pub struct ToastProvider {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ToastProvider {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for ToastProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ToastProvider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ToastProvider {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ToastProvider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().refine_style(&self.style).children(self.children)
    }
}

/// Responsive viewport placement for toasts: top on small screens (`< 768px`),
/// bottom-right on larger screens (`>= 768px`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastPlacement {
    Top,
    BottomRight,
}

impl ToastPlacement {
    #[inline]
    pub const fn from_breakpoint(bp: Breakpoint) -> Self {
        if bp.is_mobile() {
            Self::Top
        } else {
            Self::BottomRight
        }
    }

    #[inline]
    pub fn from_width(width_px: f32) -> Self {
        Self::from_breakpoint(Breakpoint::from_width(width_px))
    }

    #[inline]
    pub fn from_window(window: &Window) -> Self {
        if is_mobile(window) {
            Self::Top
        } else {
            Self::BottomRight
        }
    }
}

/// Viewport stack container (`ToastViewport` in `toast.tsx`).
#[derive(IntoElement)]
pub struct ToastViewport {
    safe_area: SafeAreaInsets,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ToastViewport {
    pub fn new() -> Self {
        Self {
            safe_area: SafeAreaInsets::zero(),
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn safe_area(mut self, safe_area: SafeAreaInsets) -> Self {
        self.safe_area = safe_area;
        self
    }
}

impl Default for ToastViewport {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ToastViewport {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ToastViewport {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ToastViewport {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        match ToastPlacement::from_window(window) {
            ToastPlacement::Top => div()
                .absolute()
                .top(px(16.0) + self.safe_area.top_px())
                .left(px(16.0) + self.safe_area.left_px())
                .right(px(16.0) + self.safe_area.right_px())
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .w_full()
                        .max_w(px(384.0))
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .refine_style(&self.style)
                        .children(self.children),
                ),
            ToastPlacement::BottomRight => div()
                .absolute()
                .bottom(px(16.0) + self.safe_area.total_bottom_px())
                .right(px(16.0) + self.safe_area.right_px())
                .w(px(384.0))
                .flex()
                .flex_col()
                .gap(px(12.0))
                .refine_style(&self.style)
                .children(self.children),
        }
    }
}

/// Individual toast surface (`Toast` in `toast.tsx`).
#[derive(IntoElement)]
pub struct Toast {
    id: ElementId,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Toast {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Toast {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Toast {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Toast {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .id(self.id)
            .occlude()
            .w_full()
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(tokens.border)
            .bg(tokens.popover)
            .font_family(FONT_FAMILY)
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Inner content layout (`ToastContent` in `toast.tsx`).
#[derive(IntoElement)]
pub struct ToastContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ToastContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for ToastContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ToastContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ToastContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ToastContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .items_center()
            .gap(px(12.0))
            .overflow_hidden()
            .p(px(16.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Status icon (`ToastIcon` in `toast.tsx`).
#[derive(IntoElement)]
pub struct ToastIcon {
    toast_type: ToastType,
    theme: Option<SemanticTheme>,
}

impl ToastIcon {
    pub fn new(toast_type: ToastType) -> Self {
        Self {
            toast_type,
            theme: None,
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl RenderOnce for ToastIcon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);

        match self.toast_type {
            ToastType::Default => div().into_any_element(),
            ToastType::Success => div()
                .flex_shrink_0()
                .child(app_icon(
                    AppIcon::Success,
                    IconSize::Md,
                    IconTone::Success,
                    &theme,
                ))
                .into_any_element(),
            ToastType::Info => div()
                .flex_shrink_0()
                .child(app_icon(
                    AppIcon::Info,
                    IconSize::Md,
                    IconTone::Info,
                    &theme,
                ))
                .into_any_element(),
            ToastType::Warning => div()
                .flex_shrink_0()
                .child(app_icon(
                    AppIcon::Warning,
                    IconSize::Md,
                    IconTone::Warning,
                    &theme,
                ))
                .into_any_element(),
            ToastType::Error => div()
                .flex_shrink_0()
                .child(app_icon(
                    AppIcon::Error,
                    IconSize::Md,
                    IconTone::Error,
                    &theme,
                ))
                .into_any_element(),
            ToastType::Loading => div()
                .flex_shrink_0()
                .child(Spinner::new().theme(&theme))
                .into_any_element(),
        }
    }
}

/// Toast heading (`ToastTitle` in `toast.tsx`).
#[derive(IntoElement)]
pub struct ToastTitle {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ToastTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ToastTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ToastTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .font_family(FONT_FAMILY)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.popover_foreground)
            .child(self.text)
            .refine_style(&self.style)
    }
}

/// Toast supporting text (`ToastDescription` in `toast.tsx`).
#[derive(IntoElement)]
pub struct ToastDescription {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ToastDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ToastDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ToastDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.muted_foreground)
            .child(self.text)
            .refine_style(&self.style)
    }
}

/// Action button (`ToastAction` in `toast.tsx`).
#[derive(IntoElement)]
pub struct ToastAction {
    id: ElementId,
    label: SharedString,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl ToastAction {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for ToastAction {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ToastAction {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut btn = Button::new(self.id)
            .variant(ButtonVariant::Outline)
            .size(ButtonSize::Sm)
            .label(self.label)
            .theme(&theme)
            .flex_shrink_0();
        if let Some(cb) = self.on_click {
            btn = btn.on_click(move |ev, window, cx| cb(ev, window, cx));
        }
        btn.refine_style(&self.style)
    }
}

/// Dismiss button (`ToastClose` in `toast.tsx`).
#[derive(IntoElement)]
pub struct ToastClose {
    id: ElementId,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl ToastClose {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for ToastClose {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ToastClose {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut btn = Button::new(self.id)
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::IconSm)
            .icon(AppIcon::Close)
            .theme(&theme)
            .flex_shrink_0();
        if let Some(cb) = self.on_click {
            btn = btn.on_click(move |ev, window, cx| cb(ev, window, cx));
        }
        btn.refine_style(&self.style)
    }
}

/// Root toast stack renderer bound to [`Entity<ToastManager>`] (`Toaster` in `toast.tsx`).
#[derive(IntoElement)]
pub struct Toaster {
    manager: Entity<ToastManager>,
    safe_area: SafeAreaInsets,
    theme: Option<SemanticTheme>,
    children: Vec<AnyElement>,
}

impl Toaster {
    pub fn new(manager: &Entity<ToastManager>) -> Self {
        Self {
            manager: manager.clone(),
            safe_area: SafeAreaInsets::zero(),
            theme: None,
            children: Vec::new(),
        }
    }

    pub fn safe_area(mut self, safe_area: SafeAreaInsets) -> Self {
        self.safe_area = safe_area;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl ParentElement for Toaster {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Toaster {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let records: Vec<ToastRecord> = self.manager.read(cx).toasts().to_vec();

        let mut provider = ToastProvider::new().children(self.children);

        if !records.is_empty() {
            let mut viewport = ToastViewport::new().safe_area(self.safe_area);
            for item in records {
                let id = item.id;
                let mgr_close = self.manager.clone();
                let mgr_act = self.manager.clone();

                let mut text_col = div()
                    .flex()
                    .min_w_0()
                    .flex_1()
                    .flex_col()
                    .gap(px(4.0))
                    .child(ToastTitle::new(item.title).theme(&theme));

                if let Some(desc) = item.description {
                    text_col = text_col.child(ToastDescription::new(desc).theme(&theme));
                }

                let mut content = ToastContent::new();
                if !matches!(item.toast_type, ToastType::Default) {
                    content = content.child(ToastIcon::new(item.toast_type).theme(&theme));
                }
                content = content.child(text_col);

                if let Some(act_lbl) = item.action_label {
                    let cb = item.on_action;
                    content = content.child(
                        ToastAction::new(
                            ElementId::Name(format!("toast-act-{id}").into()),
                            act_lbl,
                        )
                        .theme(&theme)
                        .on_click(move |ev, window, cx| {
                            if let Some(f) = &cb {
                                f(ev, window, cx);
                            }
                            mgr_act.update(cx, |m, cx| {
                                m.dismiss(id);
                                cx.notify();
                            });
                        }),
                    );
                }

                content = content.child(
                    ToastClose::new(ElementId::Name(format!("toast-close-{id}").into()))
                        .theme(&theme)
                        .on_click(move |_ev, _window, cx| {
                            mgr_close.update(cx, |m, cx| {
                                m.dismiss(id);
                                cx.notify();
                            });
                        }),
                );

                viewport = viewport.child(
                    Toast::new(ElementId::Name(format!("toast-item-{id}").into()))
                        .theme(&theme)
                        .child(content),
                );
            }

            provider = provider.child(ToastPortal::new().priority(20).child(viewport));
        }

        provider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_placement_is_top_on_small_screen_and_bottom_right_on_big_screen() {
        assert_eq!(ToastPlacement::from_width(375.0), ToastPlacement::Top);
        assert_eq!(ToastPlacement::from_width(767.9), ToastPlacement::Top);
        assert_eq!(
            ToastPlacement::from_width(768.0),
            ToastPlacement::BottomRight
        );
        assert_eq!(
            ToastPlacement::from_width(1440.0),
            ToastPlacement::BottomRight
        );
    }
}

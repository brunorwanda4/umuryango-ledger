//! Auto-scrolling conversation feed and jump-to-edge button ported from
//! `web/src/components/ui/message-scroller.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `MessageScrollerProvider` / `MessageScroller`: `relative flex size-full min-h-0 flex-col overflow-hidden`
//! - `MessageScrollerViewport`: `size-full min-h-0 min-w-0 overflow-y-scroll` tracked via GPUI [`ScrollHandle`]
//! - `MessageScrollerContent`: `flex min-h-full flex-col gap-8`
//! - `MessageScrollerItem`: `min-w-0 shrink-0`
//! - `MessageScrollerButton`: floating [`Button`] (`ButtonVariant::Secondary`, `ButtonSize::IconSm`,
//!   `border-border bg-background text-foreground hover:bg-muted`) anchored at `bottom-4` (`End` with
//!   `AppIcon::ArrowDown`) or `top-4` (`Start` with `AppIcon::ArrowUp`) that jumps to bottom or top.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::message_scroller::{
    MessageScroller as KitMessageScroller, MessageScrollerState as KitMessageScrollerState,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    RenderOnce, ScrollHandle, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    point, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::utils::{PresetRadius, PresetTokens};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Jump direction matching `direction` (`"end" | "start"`) in `message-scroller.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageScrollerDirection {
    #[default]
    End,
    Start,
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

/// Returns `true` when the viewport has scrollable overflow (`useMessageScrollerScrollable`).
pub fn message_scroller_is_scrollable(handle: &ScrollHandle) -> bool {
    handle.max_offset().y > px(1.0)
}

/// Returns `true` when the jump button for `direction` should be visible (`useMessageScrollerVisibility`).
///
/// In GPUI, vertical scroll `offset().y` starts at `0px` (top) and becomes negative as the user
/// scrolls down toward `-max_offset().y` (bottom).
pub fn message_scroller_button_visible(
    handle: &ScrollHandle,
    direction: MessageScrollerDirection,
    threshold: Pixels,
) -> bool {
    let max_y = handle.max_offset().y;
    if max_y <= threshold {
        return false;
    }
    let scrolled_down = (-handle.offset().y).max(px(0.0));
    match direction {
        MessageScrollerDirection::End => (max_y - scrolled_down) > threshold,
        MessageScrollerDirection::Start => scrolled_down > threshold,
    }
}

/// Jumps `handle` to either the bottom (`End`) or top (`Start`) of the conversation feed.
pub fn scroll_message_scroller_to(handle: &ScrollHandle, direction: MessageScrollerDirection) {
    match direction {
        MessageScrollerDirection::End => handle.scroll_to_bottom(),
        MessageScrollerDirection::Start => handle.set_offset(point(px(0.0), px(0.0))),
    }
}

/// Context/layout wrapper matching `MessageScrollerProvider` in `message-scroller.tsx`.
#[derive(IntoElement)]
pub struct MessageScrollerProvider {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageScrollerProvider {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for MessageScrollerProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageScrollerProvider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageScrollerProvider {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageScrollerProvider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .min_h_0()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Outer positioning frame (`data-slot="message-scroller"`).
#[derive(IntoElement)]
pub struct MessageScroller {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageScroller {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for MessageScroller {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageScroller {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageScroller {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageScroller {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .relative()
            .flex()
            .flex_col()
            .size_full()
            .min_h_0()
            .overflow_hidden()
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Scrollable message viewport (`data-slot="message-scroller-viewport"`).
#[derive(IntoElement)]
pub struct MessageScrollerViewport {
    id: ElementId,
    scroll_handle: Option<ScrollHandle>,
    pending_scroll: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageScrollerViewport {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            scroll_handle: None,
            pending_scroll: false,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(handle.clone());
        self
    }

    /// Hides initial unpositioned frame while computing initial scroll offset (`data-pending-scroll:invisible`).
    pub fn pending_scroll(mut self, pending: bool) -> Self {
        self.pending_scroll = pending;
        self
    }
}

impl Styled for MessageScrollerViewport {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageScrollerViewport {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageScrollerViewport {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut viewport = div()
            .id(self.id)
            .size_full()
            .min_h_0()
            .min_w_0()
            .overflow_y_scroll()
            .when(self.pending_scroll, |d| d.opacity(0.0));

        if let Some(handle) = self.scroll_handle {
            viewport = viewport.track_scroll(&handle);
        }

        viewport.refine_style(&self.style).children(self.children)
    }
}

/// Inner content stack (`data-slot="message-scroller-content"`, `flex min-h-full flex-col gap-8`).
#[derive(IntoElement)]
pub struct MessageScrollerContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageScrollerContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for MessageScrollerContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageScrollerContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageScrollerContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageScrollerContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .min_h_full()
            .gap(px(32.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Individual turn/item wrapper (`data-slot="message-scroller-item"`).
#[derive(IntoElement)]
pub struct MessageScrollerItem {
    scroll_anchor: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageScrollerItem {
    pub fn new() -> Self {
        Self {
            scroll_anchor: false,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn scroll_anchor(mut self, scroll_anchor: bool) -> Self {
        self.scroll_anchor = scroll_anchor;
        self
    }
}

impl Default for MessageScrollerItem {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageScrollerItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageScrollerItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageScrollerItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let _ = self.scroll_anchor;
        div()
            .min_w_0()
            .flex_shrink_0()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Floating centered jump button (`data-slot="message-scroller-button"`).
#[derive(IntoElement)]
pub struct MessageScrollerButton {
    id: ElementId,
    direction: MessageScrollerDirection,
    variant: ButtonVariant,
    size: ButtonSize,
    active: Option<bool>,
    scroll_handle: Option<ScrollHandle>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageScrollerButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            direction: MessageScrollerDirection::End,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::IconSm,
            active: None,
            scroll_handle: None,
            on_click: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn direction(mut self, direction: MessageScrollerDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Explicitly sets whether the jump button is shown. When omitted and `scroll_handle`
    /// is attached, visibility is computed automatically via [`message_scroller_button_visible`].
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    pub fn scroll_handle(mut self, handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(handle.clone());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for MessageScrollerButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageScrollerButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageScrollerButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_active = self.active.unwrap_or_else(|| {
            self.scroll_handle
                .as_ref()
                .map(|h| message_scroller_button_visible(h, self.direction, px(24.0)))
                .unwrap_or(true)
        });

        if !is_active {
            return div().into_any_element();
        }

        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let direction = self.direction;
        let handle = self.scroll_handle;
        let user_on_click = self.on_click;

        let default_icon = match direction {
            MessageScrollerDirection::End => AppIcon::ArrowDown,
            MessageScrollerDirection::Start => AppIcon::ArrowUp,
        };

        let mut btn = Button::new(self.id)
            .variant(self.variant)
            .size(self.size)
            .theme(&theme)
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(tokens.border)
            .bg(tokens.background)
            .text_color(tokens.foreground)
            .on_click(move |ev, win, app| {
                if let Some(ref h) = handle {
                    scroll_message_scroller_to(h, direction);
                    win.refresh();
                }
                if let Some(ref cb) = user_on_click {
                    cb(ev, win, app);
                }
            });

        if self.children.is_empty() {
            btn = btn.child(app_icon(
                default_icon,
                IconSize::Sm,
                IconTone::Default,
                &theme,
            ));
        } else {
            btn = btn.children(self.children);
        }

        div()
            .absolute()
            .left_0()
            .right_0()
            .flex()
            .justify_center()
            .when_else(
                direction == MessageScrollerDirection::End,
                |d| d.bottom(px(16.0)),
                |d| d.top(px(16.0)),
            )
            .refine_style(&self.style)
            .child(btn)
            .into_any_element()
    }
}

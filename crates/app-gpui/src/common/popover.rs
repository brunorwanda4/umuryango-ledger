//! Anchored floating popover primitives ported from `web/src/components/ui/popover.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Popover`: controlled or uncontrolled click-toggled floating surface with outside-click
//!   and `Escape` dismissal (native GPUI `deferred`, zero DOM portals)
//! - `PopoverTrigger`: interactive trigger slot
//! - `PopoverContent`: `flex w-72 flex-col gap-4 rounded-none bg-popover p-4 text-sm text-popover-foreground shadow-md ring-1 ring-foreground/10`
//! - `PopoverHeader`: `flex flex-col gap-1 text-sm`
//! - `PopoverTitle`: `text-xs font-semibold uppercase`
//! - `PopoverDescription`: `mt-0.5 text-sm leading-relaxed text-muted-foreground`.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::popover::{Popover as KitPopover, PopoverState as KitPopoverState};
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, deferred, div, px, relative,
};

use crate::common::utils::{
    PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow, to_preset_uppercase,
};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Placement side matching `side = "bottom" | "top" | "left" | "right"` in `popover.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverSide {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

/// Cross-axis alignment matching `align = "start" | "center" | "end"` in `popover.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverAlign {
    Start,
    #[default]
    Center,
    End,
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

/// Interactive trigger slot (`PopoverTrigger` in `popover.tsx`).
#[derive(IntoElement)]
pub struct PopoverTrigger {
    id: ElementId,
    disabled: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl PopoverTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            disabled: false,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

impl Styled for PopoverTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for PopoverTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PopoverTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div()
            .id(self.id)
            .flex()
            .items_center()
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .children(self.children);

        if !self.disabled {
            el = el.cursor_pointer();
            if let Some(on_click) = self.on_click {
                el = el.on_click(move |ev, window, cx| on_click(ev, window, cx));
            }
        }
        el
    }
}

/// Floating popover surface (`PopoverContent` in `popover.tsx`).
#[derive(IntoElement)]
pub struct PopoverContent {
    side: PopoverSide,
    side_offset: f32,
    align: PopoverAlign,
    align_offset: f32,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl PopoverContent {
    pub fn new() -> Self {
        Self {
            side: PopoverSide::Bottom,
            side_offset: 4.0,
            align: PopoverAlign::Center,
            align_offset: 0.0,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for PopoverContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for PopoverContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for PopoverContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PopoverContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .flex_col()
            .w(px(288.0))
            .gap(px(16.0))
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .p(px(16.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Header section inside [`PopoverContent`] (`PopoverHeader` in `popover.tsx`).
#[derive(IntoElement)]
pub struct PopoverHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl PopoverHeader {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for PopoverHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for PopoverHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for PopoverHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PopoverHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Editorial uppercase heading inside [`PopoverHeader`] (`PopoverTitle` in `popover.tsx`).
#[derive(IntoElement)]
pub struct PopoverTitle {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl PopoverTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
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

impl Styled for PopoverTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for PopoverTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PopoverTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.popover_foreground);

        if let Some(text) = &self.text {
            el = el.child(to_preset_uppercase(text.as_ref()));
        }

        el.children(self.children).refine_style(&self.style)
    }
}

/// Muted body copy inside [`PopoverHeader`] (`PopoverDescription` in `popover.tsx`).
#[derive(IntoElement)]
pub struct PopoverDescription {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl PopoverDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
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

impl Styled for PopoverDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for PopoverDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PopoverDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .mt(px(2.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .line_height(relative(1.625))
            .text_color(tokens.muted_foreground);

        if let Some(text) = self.text {
            el = el.child(text);
        }

        el.children(self.children).refine_style(&self.style)
    }
}

/// Anchored popover root (`Popover` in `popover.tsx`).
#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<PopoverContent>,
    open: Option<bool>,
    default_open: bool,
    disabled: bool,
    close_on_outside_click: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Popover {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: None,
            default_open: false,
            disabled: false,
            close_on_outside_click: true,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content(mut self, content: PopoverContent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn close_on_outside_click(mut self, close: bool) -> Self {
        self.close_on_outside_click = close;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Popover {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

pub(crate) fn apply_anchored_position<E: Styled>(
    mut pos: E,
    side: PopoverSide,
    side_offset: f32,
    align: PopoverAlign,
    align_offset: f32,
) -> E {
    pos = match side {
        PopoverSide::Bottom => pos.top_full().mt(px(side_offset)),
        PopoverSide::Top => pos.bottom_full().mb(px(side_offset)),
        PopoverSide::Left => pos.right_full().mr(px(side_offset)),
        PopoverSide::Right => pos.left_full().ml(px(side_offset)),
    };

    match side {
        PopoverSide::Bottom | PopoverSide::Top => match align {
            PopoverAlign::Start | PopoverAlign::Center => pos.left(px(align_offset)),
            PopoverAlign::End => pos.right(px(-align_offset)),
        },
        PopoverSide::Left | PopoverSide::Right => match align {
            PopoverAlign::Start | PopoverAlign::Center => pos.top(px(align_offset)),
            PopoverAlign::End => pos.bottom(px(-align_offset)),
        },
    }
}

impl RenderOnce for Popover {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_controlled = self.open.is_some();
        let default_open = self.default_open;
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-popover-open", self.id).into()),
            cx,
            |_w, _cx| default_open,
        );
        let is_open = self.open.unwrap_or_else(|| *state.read(cx));

        let on_change_toggle = self.on_open_change.clone();
        let state_toggle = state.clone();
        let trigger_id = ElementId::Name(format!("{}-popover-trigger", self.id).into());

        let mut trigger_slot = div().id(trigger_id).flex().items_center();
        if let Some(trig) = self.trigger {
            trigger_slot = trigger_slot.child(trig);
        }
        if !self.disabled {
            trigger_slot = trigger_slot
                .cursor_pointer()
                .on_click(move |_ev, window, cx| {
                    let next = !is_open;
                    if !is_controlled {
                        state_toggle.update(cx, |v, cx| {
                            *v = next;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_change_toggle {
                        cb(&next, window, cx);
                    }
                });
        }

        let mut root = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .items_center()
            .refine_style(&self.style)
            .child(trigger_slot);

        if is_open && let Some(content) = self.content {
            let side = content.side;
            let side_offset = content.side_offset;
            let align = content.align;
            let align_offset = content.align_offset;

            let on_change_out = self.on_open_change.clone();
            let state_out = state.clone();
            let on_change_esc = self.on_open_change;
            let state_esc = state;
            let close_outside = self.close_on_outside_click;

            let mut popup_box = apply_anchored_position(
                div()
                    .id(ElementId::Name(format!("{}-popover-popup", self.id).into()))
                    .absolute(),
                side,
                side_offset,
                align,
                align_offset,
            )
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_key_down(move |ev: &KeyDownEvent, window, cx| {
                if ev.keystroke.key == "escape" {
                    if !is_controlled {
                        state_esc.update(cx, |v, cx| {
                            *v = false;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_change_esc {
                        cb(&false, window, cx);
                    }
                }
            })
            .child(content);

            if close_outside {
                popup_box = popup_box.on_mouse_down_out(move |_ev, window, cx| {
                    if !is_controlled {
                        state_out.update(cx, |v, cx| {
                            *v = false;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_change_out {
                        cb(&false, window, cx);
                    }
                });
            }

            root = root.child(deferred(popup_box).with_priority(3));
        }

        root
    }
}

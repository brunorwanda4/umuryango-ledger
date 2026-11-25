//! Hover-triggered preview card primitives ported from `web/src/components/ui/hover-card.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `HoverCard`: opens on pointer hover (with tap-toggle fallback on mobile via `is_mobile`)
//! - `HoverCardTrigger`: trigger slot
//! - `HoverCardContent`: `w-72 rounded-none bg-popover p-4 text-sm text-popover-foreground shadow-md ring-1 ring-foreground/10`
//!   (`side = Bottom`, `side_offset = 4.0`, `align = Center`, `align_offset = 4.0`).

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::hover_card::{
    HoverCard as KitHoverCard, HoverCardState as KitHoverCardState,
};
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, MouseButton, ParentElement,
    RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, deferred, div, px,
};

use crate::common::popover::{PopoverAlign, PopoverSide, apply_anchored_position};
use crate::common::utils::{PresetRadius, PresetTokens, overlay_surface_shadow};
use crate::hooks::use_mobile::is_mobile;
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

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

#[derive(Debug, Clone, Copy, Default)]
struct HoverCardInternalState {
    trigger_hovered: bool,
    card_hovered: bool,
    tapped_open: bool,
}

impl HoverCardInternalState {
    fn is_open(self) -> bool {
        self.trigger_hovered || self.card_hovered || self.tapped_open
    }
}

/// Trigger slot wrapper (`HoverCardTrigger` in `hover-card.tsx`).
#[derive(IntoElement)]
pub struct HoverCardTrigger {
    id: ElementId,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl HoverCardTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for HoverCardTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for HoverCardTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for HoverCardTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .items_center()
            .cursor_pointer()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Floating preview surface (`HoverCardContent` in `hover-card.tsx`).
#[derive(IntoElement)]
pub struct HoverCardContent {
    side: PopoverSide,
    side_offset: f32,
    align: PopoverAlign,
    align_offset: f32,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl HoverCardContent {
    pub fn new() -> Self {
        Self {
            side: PopoverSide::Bottom,
            side_offset: 4.0,
            align: PopoverAlign::Center,
            align_offset: 4.0,
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

impl Default for HoverCardContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for HoverCardContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for HoverCardContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for HoverCardContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .flex_col()
            .w(px(288.0))
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

/// Hover-activated preview card (`HoverCard` in `hover-card.tsx`).
#[derive(IntoElement)]
pub struct HoverCard {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<HoverCardContent>,
    open: Option<bool>,
    default_open: bool,
    disabled: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl HoverCard {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: None,
            default_open: false,
            disabled: false,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content(mut self, content: HoverCardContent) -> Self {
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

    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for HoverCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for HoverCard {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_controlled = self.open.is_some();
        let default_open = self.default_open;
        let mobile = is_mobile(window);

        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-hover-card-state", self.id).into()),
            cx,
            |_w, _cx| HoverCardInternalState {
                trigger_hovered: false,
                card_hovered: false,
                tapped_open: default_open,
            },
        );

        let is_open = self.open.unwrap_or_else(|| state.read(cx).is_open());

        let mut trigger_slot = div()
            .id(ElementId::Name(
                format!("{}-hover-card-trigger", self.id).into(),
            ))
            .flex()
            .items_center();

        if let Some(trig) = self.trigger {
            trigger_slot = trigger_slot.child(trig);
        }

        if !self.disabled {
            let state_hover = state.clone();
            let on_change_hover = self.on_open_change.clone();
            trigger_slot = trigger_slot.on_hover(move |hovered, window, cx| {
                if !is_controlled {
                    let prev = state_hover.read(cx).is_open();
                    state_hover.update(cx, |s, cx| {
                        s.trigger_hovered = *hovered;
                        cx.notify();
                    });
                    let next = state_hover.read(cx).is_open();
                    if prev != next
                        && let Some(cb) = &on_change_hover
                    {
                        cb(&next, window, cx);
                    }
                }
            });

            if mobile {
                let state_tap = state.clone();
                let on_change_tap = self.on_open_change.clone();
                trigger_slot = trigger_slot.on_click(move |_ev, window, cx| {
                    if !is_controlled {
                        let next = !state_tap.read(cx).tapped_open;
                        state_tap.update(cx, |s, cx| {
                            s.tapped_open = next;
                            cx.notify();
                        });
                        if let Some(cb) = &on_change_tap {
                            cb(&next, window, cx);
                        }
                    }
                });
            }
        }

        let mut root = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .items_center()
            .refine_style(&self.style)
            .child(trigger_slot);

        if is_open
            && !self.disabled
            && let Some(content) = self.content
        {
            let side = content.side;
            let side_offset = content.side_offset;
            let align = content.align;
            let align_offset = content.align_offset;

            let state_card = state.clone();
            let on_change_card = self.on_open_change.clone();
            let state_out = state;
            let on_change_out = self.on_open_change;

            let popup_box = apply_anchored_position(
                div()
                    .id(ElementId::Name(
                        format!("{}-hover-card-popup", self.id).into(),
                    ))
                    .absolute(),
                side,
                side_offset,
                align,
                align_offset,
            )
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_hover(move |hovered, window, cx| {
                if !is_controlled {
                    let prev = state_card.read(cx).is_open();
                    state_card.update(cx, |s, cx| {
                        s.card_hovered = *hovered;
                        cx.notify();
                    });
                    let next = state_card.read(cx).is_open();
                    if prev != next
                        && let Some(cb) = &on_change_card
                    {
                        cb(&next, window, cx);
                    }
                }
            })
            .on_mouse_down_out(move |_ev, window, cx| {
                if !is_controlled {
                    let prev = state_out.read(cx).is_open();
                    state_out.update(cx, |s, cx| {
                        s.tapped_open = false;
                        s.trigger_hovered = false;
                        s.card_hovered = false;
                        cx.notify();
                    });
                    if prev && let Some(cb) = &on_change_out {
                        cb(&false, window, cx);
                    }
                }
            })
            .child(content);

            root = root.child(deferred(popup_box).with_priority(3));
        }

        root
    }
}

//! Disclosure collapsible primitives ported from `web/src/components/ui/collapsible.tsx`.
//!
//! Exports `Collapsible`, `CollapsibleTrigger`, and `CollapsibleContent` supporting
//! both controlled (`open` + `on_open_change`) and uncontrolled (`default_open` via
//! `window.use_keyed_state`) expand/collapse behavior, plus keyboard Space/Enter toggle.

use std::rc::Rc;

pub use gpui_kit::component::collapsible::Collapsible as KitCollapsible;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ElementId, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent,
    ParentElement, RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
};

use crate::common::utils::StyleComposeExt;

/// Expandable disclosure container (`Collapsible` in `web/src/components/ui/collapsible.tsx`).
#[derive(IntoElement)]
pub struct Collapsible {
    id: ElementId,
    open: Option<bool>,
    default_open: bool,
    disabled: bool,
    trigger: Option<CollapsibleTrigger>,
    content: Option<CollapsibleContent>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Collapsible {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: None,
            default_open: false,
            disabled: false,
            trigger: None,
            content: None,
            on_open_change: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
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

    pub fn trigger(mut self, trigger: CollapsibleTrigger) -> Self {
        self.trigger = Some(trigger);
        self
    }

    pub fn content(mut self, content: CollapsibleContent) -> Self {
        self.content = Some(content);
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

impl Styled for Collapsible {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Collapsible {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Collapsible {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let initial = self.default_open;
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| initial);
        let is_open = self.open.unwrap_or_else(|| *state.read(cx));
        let next_open = !is_open;
        let is_controlled = self.open.is_some();

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .children(self.children);

        if let Some(mut trig) = self.trigger {
            trig.disabled = self.disabled || trig.disabled;
            if !trig.disabled {
                let st = state.clone();
                let cb = self.on_open_change.clone();
                trig = trig.on_toggle(move |window, cx| {
                    if !is_controlled {
                        st.update(cx, |v, cx| {
                            *v = next_open;
                            cx.notify();
                        });
                    }
                    if let Some(ref handler) = cb {
                        handler(&next_open, window, cx);
                    }
                });
            }
            root = root.child(trig);
        }

        if is_open {
            if let Some(panel) = self.content {
                root = root.child(panel);
            }
        }

        root
    }
}

/// Click & keyboard trigger toggling a [`Collapsible`] (`CollapsibleTrigger`).
#[derive(IntoElement)]
pub struct CollapsibleTrigger {
    id: ElementId,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    on_toggle: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CollapsibleTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            disabled: false,
            focus_handle: None,
            on_toggle: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn track_focus(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl Styled for CollapsibleTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CollapsibleTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CollapsibleTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div()
            .id(self.id)
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .children(self.children);

        if let Some(ref fh) = self.focus_handle {
            el = el.track_focus(fh);
        }

        if !self.disabled {
            el = el.cursor_pointer();
            if let Some(cb) = self.on_toggle {
                let click_cb = cb.clone();
                el = el.on_click(move |_, window, cx| {
                    click_cb(window, cx);
                });
                el = el.on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    if key == "enter" || key == "space" {
                        cb(window, cx);
                    }
                });
            }
        }

        el
    }
}

/// Revealed body panel of a [`Collapsible`] (`CollapsibleContent` in `collapsible.tsx`).
#[derive(IntoElement)]
pub struct CollapsibleContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for CollapsibleContent {
    fn default() -> Self {
        Self::new()
    }
}

impl CollapsibleContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for CollapsibleContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CollapsibleContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CollapsibleContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .overflow_hidden()
            .refine_style(&self.style)
            .children(self.children)
    }
}

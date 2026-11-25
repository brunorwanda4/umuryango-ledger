//! Right-click context menu primitives ported from `web/src/components/ui/context-menu.tsx`.
//!
//! Reuses the shared `--preset b1oaDsAa` menu item/label/separator/shortcut/submenu primitives
//! from [`crate::common::dropdown_menu`] per Rule 2 (Strict Component Reuse), and attaches
//! native GPUI `MouseButton::Right` pointer-positioned popup handling.

use std::cell::Cell;
use std::rc::Rc;

pub use gpui_kit::component::menu::{
    ContextMenu as KitContextMenu, ContextMenuExt, ContextMenuState,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, KeyDownEvent, MouseButton,
    MouseDownEvent, ParentElement, Pixels, Point, RenderOnce, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, canvas, deferred, div, point, px,
};

use crate::common::dropdown_menu::{
    DropdownMenuCheckboxItem, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuItemVariant, DropdownMenuLabel, DropdownMenuPortal, DropdownMenuRadioGroup,
    DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuSub,
    DropdownMenuSubContent, DropdownMenuSubTrigger,
};

/// Deferred GPUI overlay layer (`ContextMenuPortal` in `context-menu.tsx`).
pub type ContextMenuPortal = DropdownMenuPortal;

/// Group wrapper (`ContextMenuGroup` in `context-menu.tsx`).
pub type ContextMenuGroup = DropdownMenuGroup;

/// Uppercase section heading (`ContextMenuLabel` in `context-menu.tsx`).
pub type ContextMenuLabel = DropdownMenuLabel;

/// Actionable context menu row (`ContextMenuItem` in `context-menu.tsx`).
pub type ContextMenuItem = DropdownMenuItem;

/// Visual variant (`Default` | `Destructive`) for [`ContextMenuItem`].
pub type ContextMenuItemVariant = DropdownMenuItemVariant;

/// Checkable context menu row (`ContextMenuCheckboxItem` in `context-menu.tsx`).
pub type ContextMenuCheckboxItem = DropdownMenuCheckboxItem;

/// Radio group inside [`ContextMenuContent`] (`ContextMenuRadioGroup` in `context-menu.tsx`).
pub type ContextMenuRadioGroup = DropdownMenuRadioGroup;

/// Radio item inside [`ContextMenuRadioGroup`] (`ContextMenuRadioItem` in `context-menu.tsx`).
pub type ContextMenuRadioItem = DropdownMenuRadioItem;

/// Thin divider (`ContextMenuSeparator` in `context-menu.tsx`).
pub type ContextMenuSeparator = DropdownMenuSeparator;

/// Right-aligned shortcut hint (`ContextMenuShortcut` in `context-menu.tsx`).
pub type ContextMenuShortcut = DropdownMenuShortcut;

/// Nested submenu container (`ContextMenuSub` in `context-menu.tsx`).
pub type ContextMenuSub = DropdownMenuSub;

/// Nested submenu trigger (`ContextMenuSubTrigger` in `context-menu.tsx`).
pub type ContextMenuSubTrigger = DropdownMenuSubTrigger;

/// Nested submenu popup (`ContextMenuSubContent` in `context-menu.tsx`).
pub type ContextMenuSubContent = DropdownMenuSubContent;

/// Floating context menu surface (`ContextMenuContent` in `context-menu.tsx`).
pub type ContextMenuContent = DropdownMenuContent;

/// Right-clickable trigger region (`ContextMenuTrigger` in `context-menu.tsx`).
#[derive(IntoElement)]
pub struct ContextMenuTrigger {
    id: ElementId,
    disabled: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ContextMenuTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            disabled: false,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for ContextMenuTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ContextMenuTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ContextMenuTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .refine_style(&self.style)
            .children(self.children)
    }
}

#[derive(Debug, Clone, Copy)]
struct ContextMenuRuntimeState {
    open: bool,
    local_offset: Point<Pixels>,
}

impl Default for ContextMenuRuntimeState {
    fn default() -> Self {
        Self {
            open: false,
            local_offset: point(px(0.0), px(0.0)),
        }
    }
}

/// Right-click context menu root (`ContextMenu` in `context-menu.tsx`).
#[derive(IntoElement)]
pub struct ContextMenu {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<ContextMenuContent>,
    open: Option<bool>,
    disabled: bool,
    close_on_select: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl ContextMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: None,
            disabled: false,
            close_on_select: true,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content(mut self, content: ContextMenuContent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn close_on_select(mut self, close: bool) -> Self {
        self.close_on_select = close;
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

impl Styled for ContextMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_controlled = self.open.is_some();
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-ctx-menu-state", self.id).into()),
            cx,
            |_w, _cx| ContextMenuRuntimeState::default(),
        );
        let snap = *state.read(cx);
        let is_open = self.open.unwrap_or(snap.open);

        let origin_cell = Rc::new(Cell::new(point(px(0.0), px(0.0))));
        let origin_writer = Rc::clone(&origin_cell);
        let probe = canvas(
            move |bounds, _window, _cx| {
                origin_writer.set(bounds.origin);
            },
            |_bounds, _state, _window, _cx| {},
        )
        .absolute()
        .inset_0();

        let mut trig_box = div()
            .id(ElementId::Name(format!("{}-ctx-trig", self.id).into()))
            .relative()
            .child(probe);

        if let Some(trig) = self.trigger {
            trig_box = trig_box.child(trig);
        }

        if !self.disabled {
            let state_rc = state.clone();
            let on_change_rc = self.on_open_change.clone();
            let origin_reader = Rc::clone(&origin_cell);
            trig_box = trig_box.on_mouse_down(
                MouseButton::Right,
                move |ev: &MouseDownEvent, window, cx| {
                    cx.stop_propagation();
                    let origin = origin_reader.get();
                    let offset = point(ev.position.x - origin.x, ev.position.y - origin.y);
                    state_rc.update(cx, |s, cx| {
                        s.local_offset = offset;
                        if !is_controlled {
                            s.open = true;
                        }
                        cx.notify();
                    });
                    if let Some(cb) = &on_change_rc {
                        cb(&true, window, cx);
                    }
                },
            );
        }

        let mut root = div()
            .id(self.id.clone())
            .relative()
            .refine_style(&self.style)
            .child(trig_box);

        if is_open
            && !self.disabled
            && let Some(content) = self.content
        {
            let state_out = state.clone();
            let on_change_out = self.on_open_change.clone();
            let state_esc = state.clone();
            let on_change_esc = self.on_open_change.clone();
            let state_sel = state;
            let on_change_sel = self.on_open_change;
            let close_on_select = self.close_on_select;

            let mut popup = div()
                .id(ElementId::Name(format!("{}-ctx-popup", self.id).into()))
                .absolute()
                .left(snap.local_offset.x + px(4.0))
                .top(snap.local_offset.y)
                .on_mouse_down(MouseButton::Left, |_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_down(MouseButton::Right, |_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_down_out(move |_ev, window, cx| {
                    if !is_controlled {
                        state_out.update(cx, |s, cx| {
                            s.open = false;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_change_out {
                        cb(&false, window, cx);
                    }
                })
                .on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    if ev.keystroke.key == "escape" {
                        if !is_controlled {
                            state_esc.update(cx, |s, cx| {
                                s.open = false;
                                cx.notify();
                            });
                        }
                        if let Some(cb) = &on_change_esc {
                            cb(&false, window, cx);
                        }
                    }
                })
                .child(content);

            if close_on_select {
                popup = popup.on_click(move |_ev, window, cx| {
                    if !is_controlled {
                        state_sel.update(cx, |s, cx| {
                            s.open = false;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_change_sel {
                        cb(&false, window, cx);
                    }
                });
            }

            root = root.child(deferred(popup).with_priority(6));
        }

        root
    }
}

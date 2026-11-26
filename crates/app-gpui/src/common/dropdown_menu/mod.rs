//! Anchored dropdown menu primitives ported from `web/src/components/ui/dropdown-menu.tsx`.
//!
//! Split into `mod.rs` and `items.rs` to stay well under the 1,000-line file limit and allow
//! `ContextMenu` and `Menubar` to reuse item/label/separator/submenu primitives directly.

mod items;

use std::rc::Rc;

pub use gpui_kit::component::menu::{
    DropdownMenu as KitDropdownMenu, PopupMenu as KitPopupMenu, PopupMenuItem as KitPopupMenuItem,
};
pub use items::{
    DropdownMenuCheckboxItem, DropdownMenuGroup, DropdownMenuItem, DropdownMenuItemVariant,
    DropdownMenuLabel, DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator,
    DropdownMenuShortcut, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger,
};

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, KeyDownEvent, MouseButton,
    ParentElement, RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    deferred, div, px,
};

use crate::common::dialog::DialogPortal;
use crate::common::popover::{PopoverAlign, PopoverSide, PopoverTrigger, apply_anchored_position};
use crate::common::utils::{PresetRadius, PresetTokens, overlay_surface_shadow};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

pub(super) fn resolve_theme(explicit: Option<&SemanticTheme>, cx: &App) -> SemanticTheme {
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

/// Deferred GPUI overlay layer (`DropdownMenuPortal` in `dropdown-menu.tsx`).
pub type DropdownMenuPortal = DialogPortal;

/// Interactive trigger slot (`DropdownMenuTrigger` in `dropdown-menu.tsx`).
pub type DropdownMenuTrigger = PopoverTrigger;

/// Floating menu popup (`DropdownMenuContent` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuContent {
    pub(crate) side: PopoverSide,
    pub(crate) side_offset: f32,
    pub(crate) align: PopoverAlign,
    pub(crate) align_offset: f32,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DropdownMenuContent {
    pub fn new() -> Self {
        Self {
            side: PopoverSide::Bottom,
            side_offset: 4.0,
            align: PopoverAlign::Start,
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

impl Default for DropdownMenuContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DropdownMenuContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DropdownMenuContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DropdownMenuContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .id("dropdown-menu-content")
            .flex()
            .flex_col()
            .min_w(px(192.0))
            .max_h(px(360.0))
            .overflow_y_scroll()
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .p(px(6.0))
            .font_family(FONT_FAMILY)
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Anchored dropdown menu root (`DropdownMenu` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenu {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<DropdownMenuContent>,
    open: Option<bool>,
    default_open: bool,
    disabled: bool,
    close_on_select: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl DropdownMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: None,
            default_open: false,
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

    pub fn content(mut self, content: DropdownMenuContent) -> Self {
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

impl Styled for DropdownMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_controlled = self.open.is_some();
        let default_open = self.default_open;
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-dropdown-open", self.id).into()),
            cx,
            |_w, _cx| default_open,
        );
        let is_open = self.open.unwrap_or_else(|| *state.read(cx));

        let mut trig = div()
            .id(ElementId::Name(format!("{}-dropdown-trig", self.id).into()))
            .flex()
            .items_center();

        if let Some(t) = self.trigger {
            trig = trig.child(t);
        }

        if !self.disabled {
            trig = trig.cursor_pointer();
            if !(is_open && self.content.is_some()) {
                let state_toggle = state.clone();
                let on_change_toggle = self.on_open_change.clone();
                trig = trig.on_click(move |_ev, window, cx| {
                    if !is_controlled {
                        state_toggle.update(cx, |v, cx| {
                            *v = true;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_change_toggle {
                        cb(&true, window, cx);
                    }
                });
            }
        }

        let mut root = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_col()
            .refine_style(&self.style)
            .child(trig);

        if is_open
            && !self.disabled
            && let Some(content) = self.content
        {
            let side = content.side;
            let side_offset = content.side_offset;
            let align = content.align;
            let align_offset = content.align_offset;

            let state_out = state.clone();
            let on_change_out = self.on_open_change.clone();
            let state_esc = state.clone();
            let on_change_esc = self.on_open_change.clone();
            let state_sel = state;
            let on_change_sel = self.on_open_change;
            let close_on_select = self.close_on_select;

            let mut popup = apply_anchored_position(
                div()
                    .id(ElementId::Name(
                        format!("{}-dropdown-popup", self.id).into(),
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
            .on_mouse_down_out(move |_ev, window, cx| {
                if !is_controlled {
                    state_out.update(cx, |v, cx| {
                        *v = false;
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

            if close_on_select {
                popup = popup.on_click(move |_ev, window, cx| {
                    if !is_controlled {
                        state_sel.update(cx, |v, cx| {
                            *v = false;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_change_sel {
                        cb(&false, window, cx);
                    }
                });
            }

            root = root.child(deferred(popup).with_priority(5));
        }

        root
    }
}

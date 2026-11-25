//! Edge-anchored drawer primitives ported from `web/src/components/ui/drawer.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `DrawerDirection`: `Down` (default bottom sheet), `Up`, `Left`, `Right`
//! - `DrawerSwipeHandle`: `h-1.5 w-[100px] rounded-none bg-muted` (Y axis) or `h-[100px] w-1.5` (X axis)
//! - `DrawerContent`: `rounded-none bg-popover text-sm text-popover-foreground` with edge border
//!   (`border-t`/`border-b`/`border-r`/`border-l`) and responsive sizing (`max-h-[calc(100vh-6rem)]`
//!   for Y axis, `w-3/4 sm:w-96` for X axis via `is_mobile`)
//! - `DrawerHeader`: `flex shrink-0 flex-col gap-1 md:gap-2 p-4 pb-0`
//! - `DrawerFooter`: `mt-auto flex shrink-0 flex-col gap-2 p-4 pt-0`
//! - `DrawerTitle` & `DrawerDescription`: reused directly from [`crate::common::dialog`].

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, KeyDownEvent, MouseButton,
    ParentElement, RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    px,
};

use crate::common::dialog::{
    DialogClose, DialogDescription, DialogPortal, DialogTitle, DialogTrigger,
};
use crate::common::sheet::{SheetOverlay, SheetSide};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow};
use crate::hooks::use_mobile::is_mobile;
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Swipe/dock direction matching `swipeDirection = "down" | "up" | "left" | "right"` in `drawer.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerDirection {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

impl DrawerDirection {
    pub const fn is_vertical_axis(self) -> bool {
        matches!(self, Self::Down | Self::Up)
    }

    pub const fn to_sheet_side(self) -> SheetSide {
        match self {
            Self::Down => SheetSide::Bottom,
            Self::Up => SheetSide::Top,
            Self::Left => SheetSide::Left,
            Self::Right => SheetSide::Right,
        }
    }
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

/// Deferred GPUI overlay layer (`DrawerPortal` in `drawer.tsx`).
pub type DrawerPortal = DialogPortal;

/// Semi-transparent backdrop (`DrawerOverlay` in `drawer.tsx`).
pub type DrawerOverlay = SheetOverlay;

/// Trigger slot (`DrawerTrigger` in `drawer.tsx`).
pub type DrawerTrigger = DialogTrigger;

/// Close button (`DrawerClose` in `drawer.tsx`).
pub type DrawerClose = DialogClose;

/// Editorial uppercase heading (`DrawerTitle` in `drawer.tsx`).
pub type DrawerTitle = DialogTitle;

/// Muted body copy (`DrawerDescription` in `drawer.tsx`).
pub type DrawerDescription = DialogDescription;

/// Architectural zero-radius drag handle bar (`DrawerSwipeHandle` in `drawer.tsx`).
#[derive(IntoElement)]
pub struct DrawerSwipeHandle {
    direction: DrawerDirection,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl DrawerSwipeHandle {
    pub fn new() -> Self {
        Self {
            direction: DrawerDirection::Down,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn direction(mut self, direction: DrawerDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for DrawerSwipeHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DrawerSwipeHandle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DrawerSwipeHandle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_y = self.direction.is_vertical_axis();

        let bar = div()
            .flex_shrink_0()
            .rounded(PresetRadius::NONE)
            .bg(tokens.muted)
            .cx_when(is_y, |d| d.h(px(6.0)).w(px(100.0)))
            .cx_when(!is_y, |d| d.h(px(100.0)).w(px(6.0)));

        div()
            .flex()
            .flex_shrink_0()
            .cursor_grab()
            .cx_when(is_y, |d| {
                d.h(px(16.0))
                    .w_full()
                    .justify_center()
                    .items_center()
                    .pt(px(8.0))
            })
            .cx_when(!is_y, |d| {
                d.w(px(16.0)).h_full().items_center().justify_center()
            })
            .child(bar)
            .refine_style(&self.style)
    }
}

/// Edge-anchored drawer surface (`DrawerContent` in `drawer.tsx`).
#[derive(IntoElement)]
pub struct DrawerContent {
    id: ElementId,
    direction: DrawerDirection,
    show_swipe_handle: bool,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DrawerContent {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            direction: DrawerDirection::Down,
            show_swipe_handle: false,
            on_close: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn direction(mut self, direction: DrawerDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn show_swipe_handle(mut self, show: bool) -> Self {
        self.show_swipe_handle = show;
        self
    }

    pub fn on_close(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for DrawerContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DrawerContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DrawerContent {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let mobile = is_mobile(window);
        let viewport = window.viewport_size();
        let max_y_h = (viewport.height - px(96.0)).max(px(200.0));
        let side_w = if mobile {
            (viewport.width * 0.75).max(px(240.0))
        } else {
            px(384.0)
        };

        let is_y = self.direction.is_vertical_axis();
        let on_esc = self.on_close;

        let handle_first = matches!(self.direction, DrawerDirection::Down | DrawerDirection::Right);

        let inner_body = div()
            .flex()
            .flex_1()
            .min_h_0()
            .flex_col()
            .overflow_hidden()
            .children(self.children);

        let mut popup = div()
            .id(self.id)
            .occlude()
            .relative()
            .flex()
            .cx_when(is_y, |d| d.flex_col())
            .cx_when(!is_y, |d| d.flex_row())
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
            .cx_when(matches!(self.direction, DrawerDirection::Down), |d| {
                d.w_full()
                    .max_h(max_y_h)
                    .border_t_1()
                    .border_color(tokens.border)
            })
            .cx_when(matches!(self.direction, DrawerDirection::Up), |d| {
                d.w_full()
                    .max_h(max_y_h)
                    .border_b_1()
                    .border_color(tokens.border)
            })
            .cx_when(matches!(self.direction, DrawerDirection::Left), |d| {
                d.h_full()
                    .w(side_w)
                    .border_r_1()
                    .border_color(tokens.border)
            })
            .cx_when(matches!(self.direction, DrawerDirection::Right), |d| {
                d.h_full()
                    .w(side_w)
                    .border_l_1()
                    .border_color(tokens.border)
            })
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_click(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_key_down(move |ev: &KeyDownEvent, window, cx| {
                if ev.keystroke.key == "escape"
                    && let Some(cb) = &on_esc
                {
                    cb(window, cx);
                }
            });

        if self.show_swipe_handle && handle_first {
            popup = popup.child(
                DrawerSwipeHandle::new()
                    .direction(self.direction)
                    .theme(&theme),
            );
        }

        popup = popup.child(inner_body);

        if self.show_swipe_handle && !handle_first {
            popup = popup.child(
                DrawerSwipeHandle::new()
                    .direction(self.direction)
                    .theme(&theme),
            );
        }

        popup.refine_style(&self.style)
    }
}

/// Header block inside [`DrawerContent`] (`DrawerHeader` in `drawer.tsx`).
#[derive(IntoElement)]
pub struct DrawerHeader {
    direction: DrawerDirection,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DrawerHeader {
    pub fn new() -> Self {
        Self {
            direction: DrawerDirection::Down,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn direction(mut self, direction: DrawerDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl Default for DrawerHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DrawerHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DrawerHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DrawerHeader {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mobile = is_mobile(window);
        let centered = mobile && self.direction.is_vertical_axis();

        div()
            .flex()
            .flex_shrink_0()
            .flex_col()
            .gap(px(if mobile { 4.0 } else { 8.0 }))
            .p(px(16.0))
            .pb(px(0.0))
            .cx_when(centered, |d| d.items_center())
            .cx_when(!centered, |d| d.items_start())
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Footer block inside [`DrawerContent`] (`DrawerFooter` in `drawer.tsx`).
#[derive(IntoElement)]
pub struct DrawerFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DrawerFooter {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for DrawerFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DrawerFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DrawerFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DrawerFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .mt_auto()
            .flex()
            .flex_shrink_0()
            .flex_col()
            .gap(px(8.0))
            .p(px(16.0))
            .pt(px(0.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Edge-docked drawer root (`Drawer` in `drawer.tsx`).
#[derive(IntoElement)]
pub struct Drawer {
    id: ElementId,
    direction: DrawerDirection,
    modal: bool,
    show_swipe_handle: bool,
    trigger: Option<AnyElement>,
    content: Option<DrawerContent>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Drawer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            direction: DrawerDirection::Down,
            modal: true,
            show_swipe_handle: false,
            trigger: None,
            content: None,
            open: None,
            default_open: false,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn swipe_direction(mut self, direction: DrawerDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn show_swipe_handle(mut self, show: bool) -> Self {
        self.show_swipe_handle = show;
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content(mut self, content: DrawerContent) -> Self {
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

    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Drawer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Drawer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_controlled = self.open.is_some();
        let default_open = self.default_open;
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-drawer-open", self.id).into()),
            cx,
            |_w, _cx| default_open,
        );
        let is_open = self.open.unwrap_or_else(|| *state.read(cx));

        let mut root = div().id(self.id.clone()).refine_style(&self.style);

        if let Some(trig) = self.trigger {
            let state_open = state.clone();
            let on_change_open = self.on_open_change.clone();
            root = root.child(
                div()
                    .id(ElementId::Name(format!("{}-drawer-trig", self.id).into()))
                    .cursor_pointer()
                    .on_click(move |_ev, window, cx| {
                        if !is_controlled {
                            state_open.update(cx, |v, cx| {
                                *v = true;
                                cx.notify();
                            });
                        }
                        if let Some(cb) = &on_change_open {
                            cb(&true, window, cx);
                        }
                    })
                    .child(trig),
            );
        }

        if is_open
            && let Some(mut content) = self.content
        {
            content.direction = self.direction;
            if self.show_swipe_handle {
                content.show_swipe_handle = true;
            }

            let existing_close = content.on_close.take();
            let state_close = state.clone();
            let on_change_close = self.on_open_change.clone();

            content = content.on_close(move |window, cx| {
                if let Some(prev) = &existing_close {
                    prev(window, cx);
                }
                if !is_controlled {
                    state_close.update(cx, |v, cx| {
                        *v = false;
                        cx.notify();
                    });
                }
                if let Some(cb) = &on_change_close {
                    cb(&false, window, cx);
                }
            });

            let side = self.direction.to_sheet_side();
            if self.modal {
                let state_bd = state;
                let on_change_bd = self.on_open_change;
                let overlay = DrawerOverlay::new(ElementId::Name(
                    format!("{}-drawer-overlay", self.id).into(),
                ))
                .side(side)
                .on_click(move |_ev, window, cx| {
                    if !is_controlled {
                        state_bd.update(cx, |v, cx| {
                            *v = false;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_change_bd {
                        cb(&false, window, cx);
                    }
                })
                .child(content);

                root = root.child(DrawerPortal::new().child(overlay));
            } else {
                let mut non_modal = div().absolute().inset_0().flex();
                non_modal = match side {
                    SheetSide::Bottom => non_modal.flex_col().justify_end().items_stretch(),
                    SheetSide::Top => non_modal.flex_col().justify_start().items_stretch(),
                    SheetSide::Left => non_modal.justify_start().items_stretch(),
                    SheetSide::Right => non_modal.justify_end().items_stretch(),
                };
                root = root.child(DrawerPortal::new().child(non_modal.child(content)));
            }
        }

        root
    }
}

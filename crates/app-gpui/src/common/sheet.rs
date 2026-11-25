//! Edge-docked slide-over sheet primitives ported from `web/src/components/ui/sheet.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `SheetSide`: `Top`, `Right` (default), `Bottom`, `Left`
//! - `SheetOverlay`: `inset-0 bg-black/20` occluding backdrop
//! - `SheetContent`: `flex flex-col bg-popover text-sm text-popover-foreground shadow-md`
//!   (`w-3/4 sm:max-w-sm` on `Left`/`Right`, `w-full h-auto` on `Top`/`Bottom`),
//!   edge border (`border-l`/`border-r`/`border-t`/`border-b`), and `show_close_button`
//!   (`Button` `Ghost` `IconSm` `bg-secondary` at `top-4 right-4` with `AppIcon::Close`)
//! - `SheetHeader`: `flex flex-col gap-1.5 p-8`
//! - `SheetFooter`: `mt-auto flex flex-col gap-2 p-8`
//! - `SheetTitle` & `SheetDescription`: reused directly from [`crate::common::dialog`].

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::sheet::Sheet as KitSheet;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, KeyDownEvent,
    MouseButton, ParentElement, RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, hsla, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::dialog::{
    DialogClose, DialogDescription, DialogPortal, DialogTitle, DialogTrigger,
};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow};
use crate::components::icon::AppIcon;
use crate::hooks::use_mobile::is_mobile;
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Edge side matching `side = "top" | "right" | "bottom" | "left"` in `sheet.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
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

/// Trigger slot (`SheetTrigger` in `sheet.tsx`).
pub type SheetTrigger = DialogTrigger;

/// Close button (`SheetClose` in `sheet.tsx`).
pub type SheetClose = DialogClose;

/// Deferred GPUI overlay layer (`SheetPortal` in `sheet.tsx`).
pub type SheetPortal = DialogPortal;

/// Editorial uppercase heading (`SheetTitle` in `sheet.tsx`).
pub type SheetTitle = DialogTitle;

/// Muted body copy (`SheetDescription` in `sheet.tsx`).
pub type SheetDescription = DialogDescription;

/// Full-viewport backdrop docking its sheet child to `side` (`SheetOverlay` in `sheet.tsx`).
#[derive(IntoElement)]
pub struct SheetOverlay {
    id: ElementId,
    side: SheetSide,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SheetOverlay {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            side: SheetSide::Right,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
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

impl Styled for SheetOverlay {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SheetOverlay {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetOverlay {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut backdrop = div()
            .id(self.id)
            .occlude()
            .absolute()
            .inset_0()
            .flex()
            .bg(hsla(0.0, 0.0, 0.0, 0.20));

        backdrop = match self.side {
            SheetSide::Right => backdrop.justify_end().items_stretch(),
            SheetSide::Left => backdrop.justify_start().items_stretch(),
            SheetSide::Top => backdrop.flex_col().justify_start().items_stretch(),
            SheetSide::Bottom => backdrop.flex_col().justify_end().items_stretch(),
        };

        if let Some(on_click) = self.on_click {
            backdrop = backdrop.on_click(move |ev, window, cx| on_click(ev, window, cx));
        }

        backdrop.refine_style(&self.style).children(self.children)
    }
}

/// Edge-docked panel surface (`SheetContent` in `sheet.tsx`).
#[derive(IntoElement)]
pub struct SheetContent {
    id: ElementId,
    side: SheetSide,
    show_close_button: bool,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SheetContent {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            side: SheetSide::Right,
            show_close_button: true,
            on_close: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
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

impl Styled for SheetContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SheetContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetContent {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let mobile = is_mobile(window);
        let viewport_w = window.viewport_size().width;
        let side_w = if mobile {
            (viewport_w * 0.75).max(px(240.0))
        } else {
            (viewport_w * 0.75).min(px(384.0))
        };

        let close_btn_id = ElementId::Name(format!("{}-sheet-close", self.id).into());
        let on_close_btn = self.on_close.clone();
        let on_close_esc = self.on_close;

        let mut panel = div()
            .id(self.id)
            .occlude()
            .relative()
            .flex()
            .flex_col()
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
            .cx_when(matches!(self.side, SheetSide::Right), |d| {
                d.h_full()
                    .w(side_w)
                    .border_l_1()
                    .border_color(tokens.border)
            })
            .cx_when(matches!(self.side, SheetSide::Left), |d| {
                d.h_full()
                    .w(side_w)
                    .border_r_1()
                    .border_color(tokens.border)
            })
            .cx_when(matches!(self.side, SheetSide::Top), |d| {
                d.w_full().border_b_1().border_color(tokens.border)
            })
            .cx_when(matches!(self.side, SheetSide::Bottom), |d| {
                d.w_full().border_t_1().border_color(tokens.border)
            })
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_click(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_key_down(move |ev: &KeyDownEvent, window, cx| {
                if ev.keystroke.key == "escape"
                    && let Some(cb) = &on_close_esc
                {
                    cb(window, cx);
                }
            })
            .children(self.children);

        if self.show_close_button {
            let mut close_btn = Button::new(close_btn_id)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::IconSm)
                .icon(AppIcon::Close)
                .theme(&theme)
                .absolute()
                .top(px(16.0))
                .right(px(16.0))
                .bg(tokens.secondary);

            if let Some(cb) = on_close_btn {
                close_btn = close_btn.on_click(move |_ev, window, cx| cb(window, cx));
            }
            panel = panel.child(close_btn);
        }

        panel.refine_style(&self.style)
    }
}

/// Header block inside [`SheetContent`] (`flex flex-col gap-1.5 p-8`, `SheetHeader` in `sheet.tsx`).
#[derive(IntoElement)]
pub struct SheetHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SheetHeader {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SheetHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SheetHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SheetHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .p(px(32.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Footer block inside [`SheetContent`] (`mt-auto flex flex-col gap-2 p-8`, `SheetFooter` in `sheet.tsx`).
#[derive(IntoElement)]
pub struct SheetFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SheetFooter {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SheetFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SheetFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SheetFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SheetFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .mt_auto()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .p(px(32.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Slide-over sheet root (`Sheet` in `sheet.tsx`).
#[derive(IntoElement)]
pub struct Sheet {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<SheetContent>,
    open: Option<bool>,
    default_open: bool,
    dismiss_on_backdrop: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Sheet {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: None,
            default_open: false,
            dismiss_on_backdrop: true,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content(mut self, content: SheetContent) -> Self {
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

    pub fn dismiss_on_backdrop(mut self, dismiss: bool) -> Self {
        self.dismiss_on_backdrop = dismiss;
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

impl Styled for Sheet {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Sheet {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_controlled = self.open.is_some();
        let default_open = self.default_open;
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-sheet-open", self.id).into()),
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
                    .id(ElementId::Name(format!("{}-sheet-trig", self.id).into()))
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

        if is_open && let Some(mut content) = self.content {
            let side = content.side;
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

            let mut overlay =
                SheetOverlay::new(ElementId::Name(format!("{}-sheet-overlay", self.id).into()))
                    .side(side);

            if self.dismiss_on_backdrop {
                let state_bd = state;
                let on_change_bd = self.on_open_change;
                overlay = overlay.on_click(move |_ev, window, cx| {
                    if !is_controlled {
                        state_bd.update(cx, |v, cx| {
                            *v = false;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_change_bd {
                        cb(&false, window, cx);
                    }
                });
            }

            root = root.child(SheetPortal::new().child(overlay.child(content)));
        }

        root
    }
}

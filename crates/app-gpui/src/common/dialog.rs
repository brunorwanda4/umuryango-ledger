//! Modal dialog primitives ported from `web/src/components/ui/dialog.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `DialogOverlay`: `inset-0 bg-black/20` occluding backdrop
//! - `DialogContent`: `w-full max-w-[calc(100%-2rem)] sm:max-w-md gap-6 rounded-none bg-popover p-6 text-sm text-popover-foreground shadow-md ring-1 ring-foreground/10`
//!   with optional `show_close_button` (`Button` `Ghost` `IconSm` `bg-secondary` at `top-5 right-5` with `AppIcon::Close`)
//! - `DialogHeader`: `flex flex-col gap-2`
//! - `DialogFooter`: `flex flex-col-reverse gap-2 sm:flex-row sm:justify-end` with optional `show_close_button`
//! - `DialogTitle`: `text-lg leading-none font-semibold uppercase`
//! - `DialogDescription`: `mt-0.5 text-sm leading-relaxed text-muted-foreground`.

use std::rc::Rc;

pub use gpui_kit::component::dialog::Dialog as KitDialog;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, deferred, div, hsla, px, relative,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::utils::{
    PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow, to_preset_uppercase,
};
use crate::components::icon::AppIcon;
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

/// Trigger slot (`DialogTrigger` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct DialogTrigger {
    id: ElementId,
    disabled: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogTrigger {
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

impl Styled for DialogTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogTrigger {
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

/// Deferred GPUI overlay layer (`DialogPortal` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct DialogPortal {
    priority: usize,
    children: Vec<AnyElement>,
}

impl DialogPortal {
    pub fn new() -> Self {
        Self {
            priority: 10,
            children: Vec::new(),
        }
    }

    pub fn priority(mut self, priority: usize) -> Self {
        self.priority = priority;
        self
    }
}

impl Default for DialogPortal {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for DialogPortal {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogPortal {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        deferred(div().absolute().inset_0().children(self.children)).with_priority(self.priority)
    }
}

/// Close control (`DialogClose` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct DialogClose {
    id: ElementId,
    label: Option<SharedString>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogClose {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            on_click: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
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

impl Styled for DialogClose {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogClose {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogClose {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);

        if self.children.is_empty() {
            let mut btn = Button::new(self.id)
                .variant(ButtonVariant::Outline)
                .theme(&theme);
            if let Some(label) = self.label {
                btn = btn.label(label);
            }
            if let Some(on_click) = self.on_click {
                btn = btn.on_click(move |ev, window, cx| on_click(ev, window, cx));
            }
            btn.refine_style(&self.style).into_any_element()
        } else {
            let mut wrapper = div()
                .id(self.id)
                .cursor_pointer()
                .refine_style(&self.style)
                .children(self.children);
            if let Some(on_click) = self.on_click {
                wrapper = wrapper.on_click(move |ev, window, cx| on_click(ev, window, cx));
            }
            wrapper.into_any_element()
        }
    }
}

/// Semi-transparent backdrop (`DialogOverlay` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct DialogOverlay {
    id: ElementId,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogOverlay {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for DialogOverlay {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogOverlay {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogOverlay {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut backdrop = div()
            .id(self.id)
            .occlude()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .p(px(16.0))
            .bg(hsla(0.0, 0.0, 0.0, 0.20))
            .refine_style(&self.style)
            .children(self.children);

        if let Some(on_click) = self.on_click {
            backdrop = backdrop.on_click(move |ev, window, cx| on_click(ev, window, cx));
        }

        backdrop
    }
}

/// Centered modal card (`DialogContent` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct DialogContent {
    id: ElementId,
    show_close_button: bool,
    on_close: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogContent {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            show_close_button: true,
            on_close: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
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

impl Styled for DialogContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogContent {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let mobile = is_mobile(window);
        let viewport_w = window.viewport_size().width;
        let max_w = if mobile {
            (viewport_w - px(32.0)).max(px(260.0))
        } else {
            px(448.0)
        };

        let close_btn_id = ElementId::Name(format!("{}-close-btn", self.id).into());
        let on_close_btn = self.on_close.clone();
        let on_close_esc = self.on_close;

        let mut popup = div()
            .id(self.id)
            .occlude()
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .max_w(max_w)
            .gap(px(24.0))
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .p(px(24.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
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
                .top(px(20.0))
                .right(px(20.0))
                .bg(tokens.secondary);

            if let Some(cb) = on_close_btn {
                close_btn = close_btn.on_click(move |_ev, window, cx| cb(window, cx));
            }
            popup = popup.child(close_btn);
        }

        popup.refine_style(&self.style)
    }
}

/// Header block inside [`DialogContent`] (`DialogHeader` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct DialogHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogHeader {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for DialogHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Footer action row inside [`DialogContent`] (`DialogFooter` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct DialogFooter {
    show_close_button: bool,
    close_label: Option<SharedString>,
    on_close: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogFooter {
    pub fn new() -> Self {
        Self {
            show_close_button: false,
            close_label: None,
            on_close: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn show_close_button(mut self, show: bool, label: impl Into<SharedString>) -> Self {
        self.show_close_button = show;
        self.close_label = Some(label.into());
        self
    }

    pub fn on_close(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for DialogFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DialogFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogFooter {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mobile = is_mobile(window);

        let mut footer = div()
            .flex()
            .gap(px(8.0))
            .cx_when(mobile, |d| d.flex_col_reverse())
            .cx_when(!mobile, |d| d.flex_row().justify_end())
            .children(self.children);

        if self.show_close_button {
            let mut close_btn = Button::new("dialog-footer-close")
                .variant(ButtonVariant::Outline)
                .theme(&theme);
            if let Some(lbl) = self.close_label {
                close_btn = close_btn.label(lbl);
            }
            if let Some(on_close) = self.on_close {
                close_btn = close_btn.on_click(move |ev, window, cx| on_close(ev, window, cx));
            }
            footer = footer.child(close_btn);
        }

        footer.refine_style(&self.style)
    }
}

/// Uppercase editorial title (`DialogTitle` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct DialogTitle {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogTitle {
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

impl Styled for DialogTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .font_family(FONT_FAMILY)
            .text_lg()
            .line_height(relative(1.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.popover_foreground);

        if let Some(text) = &self.text {
            el = el.child(to_preset_uppercase(text.as_ref()));
        }

        el.children(self.children).refine_style(&self.style)
    }
}

/// Muted supporting description (`DialogDescription` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct DialogDescription {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DialogDescription {
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

impl Styled for DialogDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DialogDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogDescription {
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

/// Modal dialog root (`Dialog` in `dialog.tsx`).
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<DialogContent>,
    open: Option<bool>,
    default_open: bool,
    dismiss_on_backdrop: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Dialog {
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

    pub fn content(mut self, content: DialogContent) -> Self {
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

impl Styled for Dialog {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_controlled = self.open.is_some();
        let default_open = self.default_open;
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-dialog-open", self.id).into()),
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
                    .id(ElementId::Name(format!("{}-dialog-trig", self.id).into()))
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
                DialogOverlay::new(ElementId::Name(format!("{}-overlay", self.id).into()));
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

            root = root.child(DialogPortal::new().child(overlay.child(content)));
        }

        root
    }
}

//! Blocking confirmation alert dialog primitives ported from
//! `web/src/components/ui/alert-dialog.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `AlertDialogContent`: `Default` (`max-w-xs sm:max-w-md`) | `Sm` (`max-w-xs`),
//!   `gap-6 rounded-none bg-popover p-6 shadow-md ring-1 ring-foreground/10`
//! - `AlertDialogMedia`: `size-16 items-center justify-center rounded-none bg-muted`
//! - `AlertDialogTitle`: `text-lg font-semibold uppercase`
//! - `AlertDialogDescription`: `mt-0.5 text-sm leading-relaxed text-muted-foreground`
//! - `AlertDialogAction` & `AlertDialogCancel`: composed directly on [`Button`].

use std::rc::Rc;

pub use gpui_kit::component::dialog::AlertDialog as KitAlertDialog;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, KeyDownEvent,
    MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::dialog::{
    DialogDescription, DialogOverlay, DialogPortal, DialogTitle, DialogTrigger,
};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::hooks::use_mobile::is_mobile;
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Width preset matching `size = "default" | "sm"` in `alert-dialog.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertDialogSize {
    #[default]
    Default,
    Sm,
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

/// Trigger slot (`AlertDialogTrigger` in `alert-dialog.tsx`).
pub type AlertDialogTrigger = DialogTrigger;

/// Deferred GPUI overlay layer (`AlertDialogPortal` in `alert-dialog.tsx`).
pub type AlertDialogPortal = DialogPortal;

/// Semi-transparent backdrop (`AlertDialogOverlay` in `alert-dialog.tsx`).
pub type AlertDialogOverlay = DialogOverlay;

/// Editorial uppercase heading (`AlertDialogTitle` in `alert-dialog.tsx`).
pub type AlertDialogTitle = DialogTitle;

/// Muted body copy (`AlertDialogDescription` in `alert-dialog.tsx`).
pub type AlertDialogDescription = DialogDescription;

///Centered confirmation popup (`AlertDialogContent` in `alert-dialog.tsx`).
#[derive(IntoElement)]
pub struct AlertDialogContent {
    id: ElementId,
    size: AlertDialogSize,
    on_escape: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl AlertDialogContent {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: AlertDialogSize::Default,
            on_escape: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: AlertDialogSize) -> Self {
        self.size = size;
        self
    }

    pub fn sm(self) -> Self {
        self.size(AlertDialogSize::Sm)
    }

    pub fn on_escape(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_escape = Some(Rc::new(handler));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for AlertDialogContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AlertDialogContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialogContent {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let mobile = is_mobile(window);
        let max_w = match (self.size, mobile) {
            (AlertDialogSize::Sm, _) | (AlertDialogSize::Default, true) => px(320.0),
            (AlertDialogSize::Default, false) => px(448.0),
        };
        let on_esc = self.on_escape;

        div()
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
                    && let Some(cb) = &on_esc
                {
                    cb(window, cx);
                }
            })
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Icon/media badge (`size-16 rounded-none bg-muted`, `AlertDialogMedia` in `alert-dialog.tsx`).
#[derive(IntoElement)]
pub struct AlertDialogMedia {
    icon: Option<AppIcon>,
    icon_tone: IconTone,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl AlertDialogMedia {
    pub fn new() -> Self {
        Self {
            icon: None,
            icon_tone: IconTone::Default,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_tone(mut self, tone: IconTone) -> Self {
        self.icon_tone = tone;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for AlertDialogMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for AlertDialogMedia {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AlertDialogMedia {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialogMedia {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut box_el = div()
            .mb(px(8.0))
            .flex()
            .flex_shrink_0()
            .w(px(64.0))
            .h(px(64.0))
            .items_center()
            .justify_center()
            .rounded(PresetRadius::NONE)
            .bg(tokens.muted);

        if let Some(icon) = self.icon {
            box_el = box_el.child(app_icon(icon, IconSize::Xl, self.icon_tone, &theme));
        }

        box_el.children(self.children).refine_style(&self.style)
    }
}

/// Header block inside [`AlertDialogContent`] (`AlertDialogHeader` in `alert-dialog.tsx`).
#[derive(IntoElement)]
pub struct AlertDialogHeader {
    size: AlertDialogSize,
    media: Option<AlertDialogMedia>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl AlertDialogHeader {
    pub fn new() -> Self {
        Self {
            size: AlertDialogSize::Default,
            media: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: AlertDialogSize) -> Self {
        self.size = size;
        self
    }

    pub fn media(mut self, media: AlertDialogMedia) -> Self {
        self.media = Some(media);
        self
    }
}

impl Default for AlertDialogHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for AlertDialogHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AlertDialogHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialogHeader {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let centered = is_mobile(window) || matches!(self.size, AlertDialogSize::Sm);

        if let Some(media) = self.media {
            if centered {
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(8.0))
                    .child(media)
                    .children(self.children)
                    .refine_style(&self.style)
            } else {
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(px(24.0))
                    .child(media.mb(px(0.0)))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w_0()
                            .gap(px(8.0))
                            .children(self.children),
                    )
                    .refine_style(&self.style)
            }
        } else {
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .cx_when(centered, |d| d.items_center())
                .cx_when(!centered, |d| d.items_start())
                .refine_style(&self.style)
                .children(self.children)
        }
    }
}

/// Action bar inside [`AlertDialogContent`] (`AlertDialogFooter` in `alert-dialog.tsx`).
#[derive(IntoElement)]
pub struct AlertDialogFooter {
    size: AlertDialogSize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl AlertDialogFooter {
    pub fn new() -> Self {
        Self {
            size: AlertDialogSize::Default,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: AlertDialogSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for AlertDialogFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for AlertDialogFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AlertDialogFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertDialogFooter {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mobile = is_mobile(window);
        let is_sm = matches!(self.size, AlertDialogSize::Sm);

        div()
            .flex()
            .gap(px(8.0))
            .cx_when(is_sm, |d| d.flex_row().w_full())
            .cx_when(!is_sm && mobile, |d| d.flex_col_reverse())
            .cx_when(!is_sm && !mobile, |d| d.flex_row().justify_end())
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Primary confirmation button (`AlertDialogAction` in `alert-dialog.tsx`).
#[derive(IntoElement)]
pub struct AlertDialogAction {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    loading: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl AlertDialogAction {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            disabled: false,
            loading: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn destructive(mut self) -> Self {
        self.variant = ButtonVariant::Destructive;
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
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

impl Styled for AlertDialogAction {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AlertDialogAction {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut btn = Button::new(self.id)
            .label(self.label)
            .variant(self.variant)
            .size(self.size)
            .disabled(self.disabled)
            .loading(self.loading)
            .theme(&theme);

        if let Some(on_click) = self.on_click {
            btn = btn.on_click(move |ev, window, cx| on_click(ev, window, cx));
        }

        btn.refine_style(&self.style)
    }
}

/// Secondary cancel button (`AlertDialogCancel` in `alert-dialog.tsx`).
#[derive(IntoElement)]
pub struct AlertDialogCancel {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl AlertDialogCancel {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Outline,
            size: ButtonSize::Default,
            disabled: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

impl Styled for AlertDialogCancel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AlertDialogCancel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut btn = Button::new(self.id)
            .label(self.label)
            .variant(self.variant)
            .size(self.size)
            .disabled(self.disabled)
            .theme(&theme);

        if let Some(on_click) = self.on_click {
            btn = btn.on_click(move |ev, window, cx| on_click(ev, window, cx));
        }

        btn.refine_style(&self.style)
    }
}

/// Blocking confirmation dialog root (`AlertDialog` in `alert-dialog.tsx`).
#[derive(IntoElement)]
pub struct AlertDialog {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<AlertDialogContent>,
    open: Option<bool>,
    default_open: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl AlertDialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: None,
            default_open: false,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn content(mut self, content: AlertDialogContent) -> Self {
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

impl Styled for AlertDialog {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AlertDialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_controlled = self.open.is_some();
        let default_open = self.default_open;
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-alert-dialog-open", self.id).into()),
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
                    .id(ElementId::Name(
                        format!("{}-alert-dialog-trig", self.id).into(),
                    ))
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
            let existing_esc = content.on_escape.take();
            let state_esc = state;
            let on_change_esc = self.on_open_change;

            content = content.on_escape(move |window, cx| {
                if let Some(prev) = &existing_esc {
                    prev(window, cx);
                }
                if !is_controlled {
                    state_esc.update(cx, |v, cx| {
                        *v = false;
                        cx.notify();
                    });
                }
                if let Some(cb) = &on_change_esc {
                    cb(&false, window, cx);
                }
            });

            let overlay = AlertDialogOverlay::new(ElementId::Name(
                format!("{}-alert-overlay", self.id).into(),
            ));
            root = root.child(AlertDialogPortal::new().child(overlay.child(content)));
        }

        root
    }
}

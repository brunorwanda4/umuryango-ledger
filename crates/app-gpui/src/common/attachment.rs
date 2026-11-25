//! File and media attachment card primitives ported from
//! `web/src/components/ui/attachment.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Attachment`: `rounded-none border bg-card text-card-foreground`, states `Idle` (`border-dashed`),
//!   `Uploading` / `Processing` ([`Spinner`]), `Error` (`border-destructive/30`), `Done`
//! - Sizes: `Default` (`text-sm`, media `w-10`), `Sm` (`text-xs`, media `w-8`), `Xs` (`text-xs`, media `w-7`)
//! - Orientations: `Horizontal` (`min-w-40 items-center`) | `Vertical` (`w-24 flex-col`)
//! - `AttachmentMedia`, `AttachmentContent`, `AttachmentTitle`, `AttachmentDescription`,
//!   `AttachmentActions`, `AttachmentAction`, `AttachmentTrigger`, and `AttachmentGroup`.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::attachment::{
    Attachment as KitAttachment, AttachmentActions as KitAttachmentActions,
    AttachmentContent as KitAttachmentContent, AttachmentDescription as KitAttachmentDescription,
    AttachmentGroup as KitAttachmentGroup, AttachmentMedia as KitAttachmentMedia,
    AttachmentStatus as KitAttachmentStatus, AttachmentTitle as KitAttachmentTitle,
};
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement, IntoElement, ObjectFit,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    StyledImage as _, Window, div, img, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::spinner::Spinner;
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Upload/processing lifecycle state matching `state` in `attachment.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttachmentState {
    Idle,
    Uploading,
    Processing,
    Error,
    #[default]
    Done,
}

/// Density preset matching `attachmentVariants.size` in `attachment.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttachmentSize {
    #[default]
    Default,
    Sm,
    Xs,
}

/// Card orientation matching `attachmentVariants.orientation` in `attachment.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttachmentOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Media preview mode matching `attachmentMediaVariants.variant` in `attachment.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttachmentMediaVariant {
    #[default]
    Icon,
    Image,
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

/// Horizontal scrolling strip of attachments (`AttachmentGroup` in `attachment.tsx`).
#[derive(IntoElement)]
pub struct AttachmentGroup {
    id: ElementId,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl AttachmentGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for AttachmentGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AttachmentGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AttachmentGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .min_w_0()
            .gap(px(12.0))
            .overflow_x_scroll()
            .py(px(4.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Root attachment card (`Attachment` in `attachment.tsx`).
#[derive(IntoElement)]
pub struct Attachment {
    id: ElementId,
    state: AttachmentState,
    size: AttachmentSize,
    orientation: AttachmentOrientation,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Attachment {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state: AttachmentState::Done,
            size: AttachmentSize::Default,
            orientation: AttachmentOrientation::Horizontal,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn state(mut self, state: AttachmentState) -> Self {
        self.state = state;
        self
    }

    pub fn size(mut self, size: AttachmentSize) -> Self {
        self.size = size;
        self
    }

    pub fn orientation(mut self, orientation: AttachmentOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Attachment {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Attachment {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Attachment {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_err = matches!(self.state, AttachmentState::Error);
        let is_idle = matches!(self.state, AttachmentState::Idle);
        let is_vert = matches!(self.orientation, AttachmentOrientation::Vertical);

        let gap_px = match self.size {
            AttachmentSize::Default => 8.0,
            AttachmentSize::Sm => 10.0,
            AttachmentSize::Xs => 6.0,
        };

        let border_c = if is_err {
            tokens.destructive.opacity(0.30)
        } else {
            tokens.border
        };

        div()
            .id(self.id)
            .relative()
            .flex()
            .flex_shrink_0()
            .min_w_0()
            .gap(px(gap_px))
            .rounded(PresetRadius::NONE)
            .border_1()
            .cx_when(is_idle, |d| d.border_dashed())
            .border_color(border_c)
            .bg(tokens.card)
            .font_family(FONT_FAMILY)
            .cx_when(matches!(self.size, AttachmentSize::Default), |d| {
                d.text_sm()
            })
            .cx_when(!matches!(self.size, AttachmentSize::Default), |d| {
                d.text_xs()
            })
            .text_color(tokens.card_foreground)
            .cx_when(!is_vert, |d| d.min_w(px(160.0)).items_center())
            .cx_when(is_vert, |d| d.w(px(120.0)).flex_col())
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Thumbnail or icon preview slot (`AttachmentMedia` in `attachment.tsx`).
#[derive(IntoElement)]
pub struct AttachmentMedia {
    variant: AttachmentMediaVariant,
    icon: Option<AppIcon>,
    image_src: Option<SharedString>,
    state: AttachmentState,
    size: AttachmentSize,
    orientation: AttachmentOrientation,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl AttachmentMedia {
    pub fn new() -> Self {
        Self {
            variant: AttachmentMediaVariant::Icon,
            icon: None,
            image_src: None,
            state: AttachmentState::Done,
            size: AttachmentSize::Default,
            orientation: AttachmentOrientation::Horizontal,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.variant = AttachmentMediaVariant::Icon;
        self.icon = Some(icon);
        self
    }

    pub fn image(mut self, src: impl Into<SharedString>) -> Self {
        self.variant = AttachmentMediaVariant::Image;
        self.image_src = Some(src.into());
        self
    }

    pub fn state(mut self, state: AttachmentState) -> Self {
        self.state = state;
        self
    }

    pub fn size(mut self, size: AttachmentSize) -> Self {
        self.size = size;
        self
    }

    pub fn orientation(mut self, orientation: AttachmentOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for AttachmentMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for AttachmentMedia {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AttachmentMedia {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AttachmentMedia {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_err = matches!(self.state, AttachmentState::Error);
        let is_busy = matches!(
            self.state,
            AttachmentState::Uploading | AttachmentState::Processing
        );
        let is_vert = matches!(self.orientation, AttachmentOrientation::Vertical);

        let dim = match self.size {
            AttachmentSize::Default => 40.0,
            AttachmentSize::Sm => 32.0,
            AttachmentSize::Xs => 28.0,
        };

        let mut box_el = div()
            .relative()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .rounded(PresetRadius::NONE)
            .bg(if is_err {
                tokens.destructive_soft
            } else {
                tokens.muted
            })
            .cx_when(!is_vert, |d| d.w(px(dim)).h(px(dim)))
            .cx_when(is_vert, |d| d.w_full().h(px(88.0)));

        if is_busy {
            box_el = box_el.child(Spinner::new().theme(&theme));
        } else if matches!(self.variant, AttachmentMediaVariant::Image)
            && let Some(src) = self.image_src
        {
            box_el = box_el.child(img(src).size_full().object_fit(ObjectFit::Cover));
        } else if self.children.is_empty() {
            let ic = self.icon.unwrap_or(AppIcon::Paperclip);
            let tone = if is_err {
                IconTone::Error
            } else {
                IconTone::Default
            };
            let ic_sz = match (is_vert, self.size) {
                (true, _) => IconSize::Lg,
                (false, AttachmentSize::Xs) => IconSize::Sm,
                _ => IconSize::Md,
            };
            box_el = box_el.child(app_icon(ic, ic_sz, tone, &theme));
        }

        box_el.children(self.children).refine_style(&self.style)
    }
}

/// Text body inside [`Attachment`] (`AttachmentContent` in `attachment.tsx`).
#[derive(IntoElement)]
pub struct AttachmentContent {
    size: AttachmentSize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl AttachmentContent {
    pub fn new() -> Self {
        Self {
            size: AttachmentSize::Default,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: AttachmentSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for AttachmentContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for AttachmentContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AttachmentContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AttachmentContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (px_v, py_v) = match self.size {
            AttachmentSize::Default => (12.0, 10.0),
            AttachmentSize::Sm => (10.0, 8.0),
            AttachmentSize::Xs => (8.0, 6.0),
        };

        div()
            .flex()
            .flex_col()
            .min_w_0()
            .flex_1()
            .px(px(px_v))
            .py(px(py_v))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Filename or attachment label (`AttachmentTitle` in `attachment.tsx`).
#[derive(IntoElement)]
pub struct AttachmentTitle {
    text: SharedString,
    style: StyleRefinement,
}

impl AttachmentTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for AttachmentTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AttachmentTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .min_w_0()
            .truncate()
            .font_weight(FontWeight::MEDIUM)
            .child(self.text)
            .refine_style(&self.style)
    }
}

/// File size or status subtitle (`AttachmentDescription` in `attachment.tsx`).
#[derive(IntoElement)]
pub struct AttachmentDescription {
    text: SharedString,
    error: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl AttachmentDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            error: false,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for AttachmentDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AttachmentDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .mt(px(2.0))
            .min_w_0()
            .truncate()
            .text_xs()
            .text_color(if self.error {
                tokens.destructive.opacity(0.80)
            } else {
                tokens.muted_foreground
            })
            .child(self.text)
            .refine_style(&self.style)
    }
}

/// Action buttons container (`AttachmentActions` in `attachment.tsx`).
#[derive(IntoElement)]
pub struct AttachmentActions {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl AttachmentActions {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for AttachmentActions {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for AttachmentActions {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AttachmentActions {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AttachmentActions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(4.0))
            .pr(px(8.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Compact icon action button (`AttachmentAction` in `attachment.tsx`).
#[derive(IntoElement)]
pub struct AttachmentAction {
    id: ElementId,
    icon: AppIcon,
    variant: ButtonVariant,
    size: ButtonSize,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl AttachmentAction {
    pub fn new(id: impl Into<ElementId>, icon: AppIcon) -> Self {
        Self {
            id: id.into(),
            icon,
            variant: ButtonVariant::Ghost,
            size: ButtonSize::IconXs,
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

impl Styled for AttachmentAction {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AttachmentAction {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut btn = Button::new(self.id)
            .variant(self.variant)
            .size(self.size)
            .icon(self.icon)
            .theme(&theme);
        if let Some(cb) = self.on_click {
            btn = btn.on_click(move |ev, window, cx| cb(ev, window, cx));
        }
        btn.refine_style(&self.style)
    }
}

/// Full-card overlay click target (`AttachmentTrigger` in `attachment.tsx`).
#[derive(IntoElement)]
pub struct AttachmentTrigger {
    id: ElementId,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl AttachmentTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            on_click: None,
            style: StyleRefinement::default(),
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

impl Styled for AttachmentTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AttachmentTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div()
            .id(self.id)
            .absolute()
            .inset_0()
            .cursor_pointer()
            .refine_style(&self.style);
        if let Some(cb) = self.on_click {
            el = el.on_click(move |ev, window, cx| cb(ev, window, cx));
        }
        el
    }
}

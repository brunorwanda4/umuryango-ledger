//! Pagination primitives ported from `web/src/components/ui/pagination.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Pagination`: `mx-auto flex w-full justify-center`
//! - `PaginationContent`: `flex items-center gap-1`
//! - `PaginationItem`: list item wrapper
//! - `PaginationLink`: wraps [`Button`] with `Outline` when `is_active` and `Ghost` otherwise (`ButtonSize::Icon` default)
//! - `PaginationPrevious`: `ChevronLeft` + optional label (`hidden sm:block`, responsive via `is_mobile(window)`)
//! - `PaginationNext`: optional label (`hidden sm:block`) + `ChevronRight`
//! - `PaginationEllipsis`: `flex size-9 items-center justify-center` with `AppIcon::Ellipsis`.

use std::rc::Rc;

pub use gpui_kit::component::pagination::Pagination as KitPagination;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
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

/// Root pagination navigation container (`Pagination` in `pagination.tsx`).
#[derive(IntoElement)]
pub struct Pagination {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Pagination {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Pagination {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Pagination {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .justify_center()
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Row list of pagination controls (`PaginationContent` in `pagination.tsx`).
#[derive(IntoElement)]
pub struct PaginationContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl PaginationContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for PaginationContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for PaginationContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for PaginationContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PaginationContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(4.0))
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Individual pagination item slot (`PaginationItem` in `pagination.tsx`).
#[derive(IntoElement)]
pub struct PaginationItem {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl PaginationItem {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for PaginationItem {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for PaginationItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for PaginationItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PaginationItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Page link button (`PaginationLink` in `pagination.tsx`).
#[derive(IntoElement)]
pub struct PaginationLink {
    id: ElementId,
    label: Option<SharedString>,
    is_active: bool,
    size: ButtonSize,
    icon_start: Option<AppIcon>,
    icon_end: Option<AppIcon>,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl PaginationLink {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            is_active: false,
            size: ButtonSize::Icon,
            icon_start: None,
            icon_end: None,
            disabled: false,
            focus_handle: None,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn page(id: impl Into<ElementId>, page_number: usize) -> Self {
        Self::new(id).label(page_number.to_string())
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn icon_start(mut self, icon: AppIcon) -> Self {
        self.icon_start = Some(icon);
        self
    }

    pub fn icon_end(mut self, icon: AppIcon) -> Self {
        self.icon_end = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn track_focus(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
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

impl Styled for PaginationLink {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for PaginationLink {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PaginationLink {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let variant = if self.is_active {
            ButtonVariant::Outline
        } else {
            ButtonVariant::Ghost
        };

        let mut btn = Button::new(self.id)
            .variant(variant)
            .size(self.size)
            .disabled(self.disabled)
            .theme(&theme);

        if let Some(label) = self.label {
            btn = btn.label(label);
        }
        if let Some(icon) = self.icon_start {
            btn = btn.icon_start(icon);
        }
        if let Some(icon) = self.icon_end {
            btn = btn.icon_end(icon);
        }
        if let Some(fh) = &self.focus_handle {
            btn = btn.track_focus(fh);
        }
        if let Some(on_click) = self.on_click {
            btn = btn.on_click(move |ev, window, cx| on_click(ev, window, cx));
        }

        btn.children(self.children).refine_style(&self.style)
    }
}

/// Previous-page control (`PaginationPrevious` in `pagination.tsx`).
#[derive(IntoElement)]
pub struct PaginationPrevious {
    id: ElementId,
    text: SharedString,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl PaginationPrevious {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            disabled: false,
            focus_handle: None,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
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

impl Styled for PaginationPrevious {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PaginationPrevious {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mobile = is_mobile(window);

        let mut link = PaginationLink::new(self.id)
            .size(if mobile {
                ButtonSize::Icon
            } else {
                ButtonSize::Default
            })
            .icon_start(AppIcon::ChevronLeft)
            .disabled(self.disabled)
            .theme(&theme);

        if !mobile {
            link = link.label(self.text).pl(px(8.0));
        }
        if let Some(fh) = &self.focus_handle {
            link = link.track_focus(fh);
        }
        if let Some(on_click) = self.on_click {
            link = link.on_click(move |ev, window, cx| on_click(ev, window, cx));
        }

        link.refine_style(&self.style)
    }
}

/// Next-page control (`PaginationNext` in `pagination.tsx`).
#[derive(IntoElement)]
pub struct PaginationNext {
    id: ElementId,
    text: SharedString,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl PaginationNext {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            disabled: false,
            focus_handle: None,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
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

impl Styled for PaginationNext {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PaginationNext {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mobile = is_mobile(window);

        let mut link = PaginationLink::new(self.id)
            .size(if mobile {
                ButtonSize::Icon
            } else {
                ButtonSize::Default
            })
            .icon_end(AppIcon::ChevronRight)
            .disabled(self.disabled)
            .theme(&theme);

        if !mobile {
            link = link.label(self.text).pr(px(8.0));
        }
        if let Some(fh) = &self.focus_handle {
            link = link.track_focus(fh);
        }
        if let Some(on_click) = self.on_click {
            link = link.on_click(move |ev, window, cx| on_click(ev, window, cx));
        }

        link.refine_style(&self.style)
    }
}

/// Ellipsis indicator for skipped pages (`PaginationEllipsis` in `pagination.tsx`).
#[derive(IntoElement)]
pub struct PaginationEllipsis {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl PaginationEllipsis {
    pub fn new() -> Self {
        Self {
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for PaginationEllipsis {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for PaginationEllipsis {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PaginationEllipsis {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);

        div()
            .flex()
            .w(px(36.0))
            .h(px(36.0))
            .items_center()
            .justify_center()
            .child(app_icon(
                AppIcon::Ellipsis,
                IconSize::Md,
                IconTone::Muted,
                &theme,
            ))
            .refine_style(&self.style)
    }
}

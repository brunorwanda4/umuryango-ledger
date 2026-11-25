//! Breadcrumb navigation primitives ported from `web/src/components/ui/breadcrumb.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `BreadcrumbList`: `flex flex-wrap items-center gap-1.5 sm:gap-2.5 text-xs uppercase text-muted-foreground`
//! - `BreadcrumbItem`: `inline-flex items-center gap-1.5`
//! - `BreadcrumbLink`: interactive link (`transition-colors hover:text-foreground`) with keyboard Enter/Space support
//! - `BreadcrumbPage`: `font-normal text-foreground` (`aria-current="page"`)
//! - `BreadcrumbSeparator`: `[&>svg]:size-3.5` defaulting to `AppIcon::ChevronRight`
//! - `BreadcrumbEllipsis`: `flex size-5 items-center justify-center [&>svg]:size-4` with `AppIcon::Ellipsis`.

use std::rc::Rc;

pub use gpui_kit::component::breadcrumb::{
    Breadcrumb as KitBreadcrumb, BreadcrumbItem as KitBreadcrumbItem,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, KeyDownEvent, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{
    PresetRadius, PresetTokens, StyleComposeExt, focus_ring_shadow, to_preset_uppercase,
};
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

/// Outer breadcrumb navigation container (`Breadcrumb` in `breadcrumb.tsx`).
#[derive(IntoElement)]
pub struct Breadcrumb {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Breadcrumb {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Breadcrumb {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Ordered breadcrumb trail (`BreadcrumbList` in `breadcrumb.tsx`).
#[derive(IntoElement)]
pub struct BreadcrumbList {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl BreadcrumbList {
    pub fn new() -> Self {
        Self {
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn theme(mut self, theme: SemanticTheme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Default for BreadcrumbList {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BreadcrumbList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for BreadcrumbList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BreadcrumbList {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let gap_px = if is_mobile(window) { 6.0 } else { 10.0 };

        div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(gap_px))
            .font_family(FONT_FAMILY)
            .text_xs()
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Individual breadcrumb item wrapper (`BreadcrumbItem` in `breadcrumb.tsx`).
#[derive(IntoElement)]
pub struct BreadcrumbItem {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl BreadcrumbItem {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for BreadcrumbItem {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BreadcrumbItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for BreadcrumbItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BreadcrumbItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Interactive ancestor link (`BreadcrumbLink` in `breadcrumb.tsx`).
#[derive(IntoElement)]
pub struct BreadcrumbLink {
    id: ElementId,
    label: Option<SharedString>,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl BreadcrumbLink {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            disabled: false,
            focus_handle: None,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }

    pub fn theme(mut self, theme: SemanticTheme) -> Self {
        self.theme = Some(theme);
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

impl Styled for BreadcrumbLink {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for BreadcrumbLink {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BreadcrumbLink {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let focus_ring = focus_ring_shadow(&theme);
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));

        let mut el = div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(4.0))
            .rounded(PresetRadius::NONE)
            .font_family(FONT_FAMILY)
            .text_xs()
            .text_color(tokens.muted_foreground)
            .cx_when(!self.disabled, |d| {
                d.cursor_pointer()
                    .hover(|s| s.text_color(tokens.foreground))
            })
            .cx_when(is_focused, |d| {
                d.text_color(tokens.foreground).shadow(focus_ring)
            })
            .preset_disabled(self.disabled);

        if let Some(fh) = &self.focus_handle {
            el = el.track_focus(fh);
        }

        if let Some(label) = &self.label {
            el = el.child(to_preset_uppercase(label.as_ref()));
        }

        el = el.children(self.children).refine_style(&self.style);

        if !self.disabled
            && let Some(on_click) = self.on_click
        {
            let on_key = Rc::clone(&on_click);
            el = el
                .on_click(move |ev, window, cx| on_click(ev, window, cx))
                .on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    if key == "enter" || key == "space" || key == " " {
                        on_key(&ClickEvent::default(), window, cx);
                    }
                });
        }

        el
    }
}

/// Active current page label (`BreadcrumbPage` in `breadcrumb.tsx`).
#[derive(IntoElement)]
pub struct BreadcrumbPage {
    label: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl BreadcrumbPage {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: Some(label.into()),
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            label: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn theme(mut self, theme: SemanticTheme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Styled for BreadcrumbPage {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for BreadcrumbPage {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BreadcrumbPage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .flex()
            .items_center()
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::NORMAL)
            .text_color(tokens.foreground);

        if let Some(label) = &self.label {
            el = el.child(to_preset_uppercase(label.as_ref()));
        }

        el.children(self.children).refine_style(&self.style)
    }
}

/// Visual separator between breadcrumb items (`BreadcrumbSeparator` in `breadcrumb.tsx`).
#[derive(IntoElement)]
pub struct BreadcrumbSeparator {
    icon: AppIcon,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl BreadcrumbSeparator {
    pub fn new() -> Self {
        Self {
            icon: AppIcon::ChevronRight,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = icon;
        self
    }

    pub fn theme(mut self, theme: SemanticTheme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Default for BreadcrumbSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BreadcrumbSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for BreadcrumbSeparator {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BreadcrumbSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut el = div().flex().items_center().justify_center();

        if self.children.is_empty() {
            el = el.child(app_icon(self.icon, IconSize::Sm, IconTone::Muted, &theme));
        } else {
            el = el.children(self.children);
        }

        el.refine_style(&self.style)
    }
}

/// Collapsed path indicator (`BreadcrumbEllipsis` in `breadcrumb.tsx`).
#[derive(IntoElement)]
pub struct BreadcrumbEllipsis {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl BreadcrumbEllipsis {
    pub fn new() -> Self {
        Self {
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: SemanticTheme) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl Default for BreadcrumbEllipsis {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BreadcrumbEllipsis {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for BreadcrumbEllipsis {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);

        div()
            .flex()
            .w(px(20.0))
            .h(px(20.0))
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

//! Inverted high-contrast tooltip primitives ported from `web/src/components/ui/tooltip.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `TooltipProvider`: structural wrapper with optional `delay_ms` (default `0`)
//! - `TooltipContent`: `inline-flex w-fit max-w-xs items-center gap-1.5 rounded-none bg-foreground px-3 py-1.5 text-xs text-background`
//! - `Tooltip` & `TooltipTrigger`: native GPUI `.tooltip(...)` hover attachment plus controlled `open` support.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::tooltip::{Tooltip as KitTooltip, TooltipDefaults};
use gpui_kit::{
    AnyElement, AnyView, App, AppContext as _, Context, ElementId, InteractiveElement, IntoElement,
    ParentElement, Render, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement,
    Styled, Window, deferred, div, px,
};

use crate::common::kbd::Kbd;
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Placement side matching `side = "top" | "bottom" | "left" | "right"` in `tooltip.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipSide {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// Cross-axis alignment matching `align = "start" | "center" | "end"` in `tooltip.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipAlign {
    Start,
    #[default]
    Center,
    End,
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

/// Group-level tooltip configuration wrapper (`TooltipProvider` in `tooltip.tsx`).
#[derive(IntoElement)]
pub struct TooltipProvider {
    delay_ms: u64,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TooltipProvider {
    pub fn new() -> Self {
        Self {
            delay_ms: 0,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn delay_ms(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub const fn delay(&self) -> u64 {
        self.delay_ms
    }
}

impl Default for TooltipProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TooltipProvider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TooltipProvider {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TooltipProvider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().refine_style(&self.style).children(self.children)
    }
}

/// Inverted `--preset b1oaDsAa` tooltip surface (`TooltipContent` in `tooltip.tsx`).
///
/// Can be rendered directly as an `IntoElement` or passed to GPUI's `.tooltip(...)`
/// via [`TooltipContent::build`].
#[derive(IntoElement)]
pub struct TooltipContent {
    text: Option<SharedString>,
    shortcut: Option<SharedString>,
    side: TooltipSide,
    side_offset: f32,
    align: TooltipAlign,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl TooltipContent {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            shortcut: None,
            side: TooltipSide::Top,
            side_offset: 4.0,
            align: TooltipAlign::Center,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    pub fn side_offset(mut self, side_offset: f32) -> Self {
        self.side_offset = side_offset;
        self
    }

    pub fn align(mut self, align: TooltipAlign) -> Self {
        self.align = align;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    /// Builds an `AnyView` suitable for GPUI's `.tooltip(|window, cx| ...)` callback.
    pub fn build(self, _window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|_| TooltipView { content: self }).into()
    }

    fn render_surface(&self, cx: &App) -> gpui_kit::Div {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let has_kbd = self.shortcut.is_some();

        let mut popup = div()
            .flex()
            .max_w(px(320.0))
            .items_center()
            .gap(px(6.0))
            .rounded(PresetRadius::NONE)
            .bg(tokens.foreground)
            .pl(px(12.0))
            .pr(px(if has_kbd { 6.0 } else { 12.0 }))
            .py(px(6.0))
            .font_family(FONT_FAMILY)
            .text_xs()
            .text_color(tokens.background)
            .shadow(overlay_surface_shadow(&theme));

        if let Some(text) = &self.text {
            popup = popup.child(text.clone());
        }

        if let Some(shortcut) = &self.shortcut {
            popup = popup.child(
                Kbd::new(shortcut.clone())
                    .theme(&theme)
                    .bg(tokens.background.opacity(0.16))
                    .text_color(tokens.background),
            );
        }

        popup.refine_style(&self.style)
    }
}

impl Styled for TooltipContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

struct TooltipView {
    content: TooltipContent,
}

impl Render for TooltipView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.content.render_surface(cx)
    }
}

impl RenderOnce for TooltipContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.render_surface(cx)
    }
}

/// Trigger slot wrapper (`TooltipTrigger` in `tooltip.tsx`).
#[derive(IntoElement)]
pub struct TooltipTrigger {
    id: ElementId,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TooltipTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for TooltipTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TooltipTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TooltipTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .items_center()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Composable tooltip wrapper (`Tooltip` in `tooltip.tsx`).
#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    text: SharedString,
    shortcut: Option<SharedString>,
    side: TooltipSide,
    side_offset: f32,
    align: TooltipAlign,
    open: Option<bool>,
    theme: Option<SemanticTheme>,
    builder: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Tooltip {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            shortcut: None,
            side: TooltipSide::Top,
            side_offset: 4.0,
            align: TooltipAlign::Center,
            open: None,
            theme: None,
            builder: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    pub fn align(mut self, align: TooltipAlign) -> Self {
        self.align = align;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn custom_view(
        mut self,
        builder: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.builder = Some(Rc::new(builder));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Tooltip {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Tooltip {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let text = self.text.clone();
        let shortcut = self.shortcut.clone();
        let side = self.side;
        let side_offset = self.side_offset;
        let align = self.align;
        let explicit_builder = self.builder.clone();
        let theme_for_hover = theme.clone();

        let mut wrapper = div()
            .id(self.id)
            .relative()
            .flex()
            .items_center()
            .refine_style(&self.style)
            .children(self.children);

        if self.open != Some(false) {
            wrapper = wrapper.tooltip(move |window, cx| {
                if let Some(custom) = &explicit_builder {
                    custom(window, cx)
                } else {
                    let mut tc = TooltipContent::new(text.clone())
                        .side(side)
                        .side_offset(side_offset)
                        .align(align)
                        .theme(&theme_for_hover);
                    if let Some(sc) = &shortcut {
                        tc = tc.shortcut(sc.clone());
                    }
                    tc.build(window, cx)
                }
            });
        }

        if self.open == Some(true) {
            let mut forced = TooltipContent::new(self.text)
                .side(self.side)
                .side_offset(self.side_offset)
                .align(self.align)
                .theme(&theme);
            if let Some(sc) = self.shortcut {
                forced = forced.shortcut(sc);
            }
            let pos = div()
                .absolute()
                .cx_when(matches!(self.side, TooltipSide::Top), |d| {
                    d.bottom_full().mb(px(self.side_offset))
                })
                .cx_when(matches!(self.side, TooltipSide::Bottom), |d| {
                    d.top_full().mt(px(self.side_offset))
                })
                .cx_when(matches!(self.side, TooltipSide::Left), |d| {
                    d.right_full().mr(px(self.side_offset))
                })
                .cx_when(matches!(self.side, TooltipSide::Right), |d| {
                    d.left_full().ml(px(self.side_offset))
                })
                .child(forced);
            wrapper = wrapper.child(deferred(pos).with_priority(2));
        }

        wrapper
    }
}

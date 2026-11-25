//! Scrollable viewport and sharp zero-radius scrollbar primitives ported from
//! `web/src/components/ui/scroll-area.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `ScrollArea`: `relative` container with full-size scrollable viewport (`size-full rounded-none`)
//!   and optional keyboard focus ring (`focus-visible:ring-ring/50`)
//! - `ScrollBar`: `w-2.5` (`vertical`) or `h-2.5` (`horizontal`), `p-px`, thumb `rounded-none bg-border`.

pub use gpui_kit::component::scroll::{
    Scrollable as KitScrollable, ScrollableElement, Scrollbar as KitScrollbar, ScrollbarAxis,
    ScrollbarHandle, ScrollbarMode,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ElementId, FocusHandle, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, ScrollHandle, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
    transparent_black,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, focus_ring_shadow};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Scroll axis orientation matching `orientation = "vertical" | "horizontal"` in `scroll-area.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollBarOrientation {
    #[default]
    Vertical,
    Horizontal,
    Both,
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

/// Custom sharp zero-radius scrollbar (`ScrollBar` in `scroll-area.tsx`).
#[derive(IntoElement)]
pub struct ScrollBar {
    scroll_handle: ScrollHandle,
    orientation: ScrollBarOrientation,
    mode: ScrollbarMode,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ScrollBar {
    pub fn new(scroll_handle: &ScrollHandle) -> Self {
        Self {
            scroll_handle: scroll_handle.clone(),
            orientation: ScrollBarOrientation::Vertical,
            mode: ScrollbarMode::Scrolling,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn vertical(scroll_handle: &ScrollHandle) -> Self {
        Self::new(scroll_handle).orientation(ScrollBarOrientation::Vertical)
    }

    pub fn horizontal(scroll_handle: &ScrollHandle) -> Self {
        Self::new(scroll_handle).orientation(ScrollBarOrientation::Horizontal)
    }

    pub fn orientation(mut self, orientation: ScrollBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn mode(mut self, mode: ScrollbarMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn always_visible(mut self) -> Self {
        self.mode = ScrollbarMode::Always;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ScrollBar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

fn build_kit_scrollbar(
    handle: &ScrollHandle,
    axis: ScrollbarAxis,
    mode: ScrollbarMode,
    tokens: &PresetTokens,
) -> KitScrollbar {
    let border = tokens.border;
    let hover_color = tokens.muted_foreground;
    let active_color = tokens.ring;

    let sb = match axis {
        ScrollbarAxis::Horizontal => KitScrollbar::horizontal(handle),
        _ => KitScrollbar::vertical(handle),
    };

    sb.viewport_from_layout().mode(mode).styles(move |styles| {
        styles
            .track(|s| s.bg(transparent_black()).width(px(10.0)))
            .thumb(move |s| {
                s.radius(PresetRadius::NONE)
                    .inset(px(1.0))
                    .width(px(8.0))
                    .bg(border)
            })
            .thumb_hover(move |s| {
                s.radius(PresetRadius::NONE)
                    .inset(px(1.0))
                    .width(px(8.0))
                    .bg(hover_color)
            })
            .thumb_active(move |s| {
                s.radius(PresetRadius::NONE)
                    .inset(px(1.0))
                    .width(px(8.0))
                    .bg(active_color)
            })
    })
}

impl RenderOnce for ScrollBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut wrapper = div().absolute().inset_0().refine_style(&self.style);
        match self.orientation {
            ScrollBarOrientation::Vertical => {
                wrapper = wrapper.child(build_kit_scrollbar(
                    &self.scroll_handle,
                    ScrollbarAxis::Vertical,
                    self.mode,
                    &tokens,
                ));
            }
            ScrollBarOrientation::Horizontal => {
                wrapper = wrapper.child(build_kit_scrollbar(
                    &self.scroll_handle,
                    ScrollbarAxis::Horizontal,
                    self.mode,
                    &tokens,
                ));
            }
            ScrollBarOrientation::Both => {
                wrapper = wrapper
                    .child(build_kit_scrollbar(
                        &self.scroll_handle,
                        ScrollbarAxis::Vertical,
                        self.mode,
                        &tokens,
                    ))
                    .child(build_kit_scrollbar(
                        &self.scroll_handle,
                        ScrollbarAxis::Horizontal,
                        self.mode,
                        &tokens,
                    ));
            }
        }
        wrapper
    }
}

/// Scrollable viewport container with overlaid `--preset b1oaDsAa` scrollbars (`ScrollArea` in `scroll-area.tsx`).
#[derive(IntoElement)]
pub struct ScrollArea {
    id: ElementId,
    orientation: ScrollBarOrientation,
    scroll_handle: Option<ScrollHandle>,
    scrollbar_mode: ScrollbarMode,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    viewport_style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ScrollArea {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            orientation: ScrollBarOrientation::Vertical,
            scroll_handle: None,
            scrollbar_mode: ScrollbarMode::Scrolling,
            focus_handle: None,
            theme: None,
            style: StyleRefinement::default(),
            viewport_style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn orientation(mut self, orientation: ScrollBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn vertical(self) -> Self {
        self.orientation(ScrollBarOrientation::Vertical)
    }

    pub fn horizontal(self) -> Self {
        self.orientation(ScrollBarOrientation::Horizontal)
    }

    pub fn both(self) -> Self {
        self.orientation(ScrollBarOrientation::Both)
    }

    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(handle.clone());
        self
    }

    pub fn scrollbar_mode(mut self, mode: ScrollbarMode) -> Self {
        self.scrollbar_mode = mode;
        self
    }

    pub fn track_focus(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
        self
    }

    pub fn viewport_style(mut self, refine: impl FnOnce(&mut StyleRefinement)) -> Self {
        refine(&mut self.viewport_style);
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ScrollArea {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ScrollArea {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ScrollArea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let focus_ring = focus_ring_shadow(&theme);

        let handle = match self.scroll_handle {
            Some(h) => h,
            None => {
                let state = window.use_keyed_state(
                    ElementId::Name(format!("{}-scroll-handle", self.id).into()),
                    cx,
                    |_w, _cx| ScrollHandle::new(),
                );
                state.read(cx).clone()
            }
        };

        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));

        let mut viewport = div()
            .id(ElementId::Name(format!("{}-viewport", self.id).into()))
            .size_full()
            .rounded(PresetRadius::NONE)
            .font_family(FONT_FAMILY)
            .track_scroll(&handle)
            .cx_when(is_focused, |d| d.shadow(focus_ring));

        viewport = match self.orientation {
            ScrollBarOrientation::Vertical => viewport.overflow_y_scroll(),
            ScrollBarOrientation::Horizontal => viewport.overflow_x_scroll(),
            ScrollBarOrientation::Both => viewport.overflow_scroll(),
        };

        if let Some(fh) = &self.focus_handle {
            viewport = viewport.track_focus(fh);
        }

        viewport = viewport
            .refine_style(&self.viewport_style)
            .children(self.children);

        let mut root = div()
            .id(self.id)
            .relative()
            .overflow_hidden()
            .rounded(PresetRadius::NONE)
            .refine_style(&self.style)
            .child(viewport);

        match self.orientation {
            ScrollBarOrientation::Vertical => {
                root = root.child(build_kit_scrollbar(
                    &handle,
                    ScrollbarAxis::Vertical,
                    self.scrollbar_mode,
                    &tokens,
                ));
            }
            ScrollBarOrientation::Horizontal => {
                root = root.child(build_kit_scrollbar(
                    &handle,
                    ScrollbarAxis::Horizontal,
                    self.scrollbar_mode,
                    &tokens,
                ));
            }
            ScrollBarOrientation::Both => {
                root = root
                    .child(build_kit_scrollbar(
                        &handle,
                        ScrollbarAxis::Vertical,
                        self.scrollbar_mode,
                        &tokens,
                    ))
                    .child(build_kit_scrollbar(
                        &handle,
                        ScrollbarAxis::Horizontal,
                        self.scrollbar_mode,
                        &tokens,
                    ));
            }
        }

        root
    }
}

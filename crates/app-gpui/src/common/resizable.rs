//! Resizable split panel primitives ported from `web/src/components/ui/resizable.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `ResizablePanelGroup`: `flex h-full w-full` (`Horizontal` row or `Vertical` column)
//! - `ResizablePanel`: individual resizable split pane with optional initial/min/max bounds
//! - `ResizableHandle`: `w-px` / `h-px` `bg-border` divider with optional `with_handle`
//!   sharp grip (`h-6 w-1 shrink-0 rounded-none bg-border`).

use std::ops::Range;
use std::rc::Rc;

pub use gpui_kit::base::{
    ResizablePanel as KitResizablePanel, ResizablePanelEvent,
    ResizablePanelGroup as KitResizablePanelGroup, ResizableState, ResizeHandleContext,
    ResizeHandleRenderer, ResizeHandleState,
};
pub use gpui_kit::component::resizable::{
    h_resizable as kit_h_resizable, resizable_panel as kit_resizable_panel,
    v_resizable as kit_v_resizable,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, Axis, ElementId, Entity, FocusHandle, InteractiveElement, IntoElement,
    ParentElement, Pixels, RenderOnce, StatefulInteractiveElement, StyleRefinement, Styled, Window,
    deferred, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, focus_ring_shadow};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Split orientation matching `orientation = "horizontal" | "vertical"` in `resizable.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResizableOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ResizableOrientation {
    pub const fn to_axis(self) -> Axis {
        match self {
            Self::Horizontal => Axis::Horizontal,
            Self::Vertical => Axis::Vertical,
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

/// Builds a `--preset b1oaDsAa` sharp resize handle renderer (`w-px bg-border` + `h-6 w-1 rounded-none`).
pub fn preset_resize_handle_appearance(
    with_handle: bool,
    explicit_theme: Option<SemanticTheme>,
) -> ResizeHandleRenderer {
    Rc::new(move |handle: &ResizeHandleContext, _window: &mut Window, cx: &mut App| {
        let theme = resolve_theme(explicit_theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let axis = handle.axis();
        let state = handle.state();

        let show_grip = with_handle || !matches!(state, ResizeHandleState::Idle);
        let grip_color = match state {
            ResizeHandleState::Dragging | ResizeHandleState::Pressed => tokens.ring,
            ResizeHandleState::Hovered => tokens.muted_foreground,
            ResizeHandleState::Idle => tokens.border,
        };

        let mut line = div().flex_none().flex().bg(tokens.border);
        line = match axis {
            Axis::Horizontal => line.w(px(1.0)).h_full().items_center(),
            Axis::Vertical => line.h(px(1.0)).w_full().items_start().justify_center(),
        };

        if show_grip {
            let grip = match axis {
                Axis::Horizontal => div()
                    .flex_none()
                    .rounded(PresetRadius::NONE)
                    .bg(grip_color)
                    .w(px(4.0))
                    .h(px(24.0))
                    .ml(px(-1.5)),
                Axis::Vertical => div()
                    .flex_none()
                    .rounded(PresetRadius::NONE)
                    .bg(grip_color)
                    .h(px(4.0))
                    .w(px(24.0))
                    .mt(px(-1.5)),
            };
            line = line.child(deferred(grip).with_priority(1));
        }

        Some(line.into_any_element())
    })
}

/// Resizable split pane container (`ResizablePanelGroup` in `resizable.tsx`).
#[derive(IntoElement)]
pub struct ResizablePanelGroup {
    id: ElementId,
    orientation: ResizableOrientation,
    with_handle: bool,
    state: Option<Entity<ResizableState>>,
    cross_size: Option<Pixels>,
    panels: Vec<KitResizablePanel>,
    theme: Option<SemanticTheme>,
    on_resize: Option<Rc<dyn Fn(&Entity<ResizableState>, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl ResizablePanelGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            orientation: ResizableOrientation::Horizontal,
            with_handle: false,
            state: None,
            cross_size: None,
            panels: Vec::new(),
            theme: None,
            on_resize: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn horizontal(id: impl Into<ElementId>) -> Self {
        Self::new(id).orientation(ResizableOrientation::Horizontal)
    }

    pub fn vertical(id: impl Into<ElementId>) -> Self {
        Self::new(id).orientation(ResizableOrientation::Vertical)
    }

    pub fn orientation(mut self, orientation: ResizableOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_handle(mut self, with_handle: bool) -> Self {
        self.with_handle = with_handle;
        self
    }

    pub fn with_state(mut self, state: &Entity<ResizableState>) -> Self {
        self.state = Some(state.clone());
        self
    }

    pub fn cross_size(mut self, size: Pixels) -> Self {
        self.cross_size = Some(size);
        self
    }

    pub fn panel(mut self, panel: impl Into<KitResizablePanel>) -> Self {
        self.panels.push(panel.into());
        self
    }

    pub fn panels<I>(mut self, panels: impl IntoIterator<Item = I>) -> Self
    where
        I: Into<KitResizablePanel>,
    {
        self.panels.extend(panels.into_iter().map(Into::into));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_resize(
        mut self,
        handler: impl Fn(&Entity<ResizableState>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_resize = Some(Rc::new(handler));
        self
    }
}

impl Styled for ResizablePanelGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ResizablePanelGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut group = KitResizablePanelGroup::new(self.id)
            .axis(self.orientation.to_axis())
            .with_handle_appearance(preset_resize_handle_appearance(
                self.with_handle,
                self.theme,
            ))
            .children(self.panels);

        if let Some(state) = &self.state {
            group = group.with_state(state);
        }
        if let Some(size) = self.cross_size {
            group = group.size(size);
        }
        if let Some(on_resize) = self.on_resize {
            group = group.on_resize(move |state, window, cx| on_resize(state, window, cx));
        }

        div()
            .flex()
            .size_full()
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .child(group)
    }
}

/// Single resizable split pane (`ResizablePanel` in `resizable.tsx`).
pub struct ResizablePanel {
    initial_size: Option<Pixels>,
    size_range: Option<Range<Pixels>>,
    visible: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ResizablePanel {
    pub fn new() -> Self {
        Self {
            initial_size: None,
            size_range: None,
            visible: true,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, initial_size: Pixels) -> Self {
        self.initial_size = Some(initial_size);
        self
    }

    pub fn size_range(mut self, range: Range<Pixels>) -> Self {
        self.size_range = Some(range);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl Default for ResizablePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ResizablePanel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ResizablePanel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl From<ResizablePanel> for KitResizablePanel {
    fn from(panel: ResizablePanel) -> Self {
        let content = div()
            .size_full()
            .font_family(FONT_FAMILY)
            .refine_style(&panel.style)
            .children(panel.children);

        let mut kp = kit_resizable_panel().visible(panel.visible).child(content);
        if let Some(sz) = panel.initial_size {
            kp = kp.size(sz);
        }
        if let Some(range) = panel.size_range {
            kp = kp.size_range(range);
        }
        kp
    }
}

/// Standalone visual divider with optional `h-6 w-1` grip (`ResizableHandle` in `resizable.tsx`).
#[derive(IntoElement)]
pub struct ResizableHandle {
    id: ElementId,
    orientation: ResizableOrientation,
    with_handle: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ResizableHandle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            orientation: ResizableOrientation::Horizontal,
            with_handle: false,
            focus_handle: None,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn orientation(mut self, orientation: ResizableOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn with_handle(mut self, with_handle: bool) -> Self {
        self.with_handle = with_handle;
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
}

impl Styled for ResizableHandle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ResizableHandle {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let focus_ring = focus_ring_shadow(&theme);
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));

        let is_vertical_split = matches!(self.orientation, ResizableOrientation::Vertical);

        let mut sep = div()
            .id(self.id)
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .bg(tokens.border)
            .cx_when(!is_vertical_split, |d| d.w(px(1.0)).h_full().cursor_col_resize())
            .cx_when(is_vertical_split, |d| d.h(px(1.0)).w_full().cursor_row_resize())
            .cx_when(is_focused, |d| d.shadow(focus_ring));

        if let Some(fh) = &self.focus_handle {
            sep = sep.track_focus(fh);
        }

        if self.with_handle {
            let grip = div()
                .flex_shrink_0()
                .rounded(PresetRadius::NONE)
                .bg(tokens.border)
                .cx_when(!is_vertical_split, |d| d.h(px(24.0)).w(px(4.0)))
                .cx_when(is_vertical_split, |d| d.w(px(24.0)).h(px(4.0)));
            sep = sep.child(grip);
        }

        sep.refine_style(&self.style)
    }
}

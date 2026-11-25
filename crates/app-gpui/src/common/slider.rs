//! Range and single-value slider primitive ported from `web/src/components/ui/slider.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - Horizontal (`w-full`) & vertical (`h-full min-h-40`) orientations
//! - `bg-input/50` track, `bg-primary` (`#ba181b`) active range & square `12px` thumb
//!   with `ring-2 ring-ring/30` hover/focus ring
//! - Built on `gpui_kit::component::slider::{Slider, SliderEvent, SliderScale, SliderState, SliderValue}`.

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::slider::{
    Slider as KitSlider, SliderEvent, SliderScale, SliderState, SliderValue,
};
use gpui_kit::{
    App, Entity, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, px,
};

use crate::common::label::Label;
use crate::common::utils::StyleComposeExt;
use crate::theme::SemanticTheme;

/// Axis orientation matching `data-horizontal` / `data-vertical` in `slider.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Interactive slider matching `Slider` in `web/src/components/ui/slider.tsx`.
#[derive(IntoElement)]
pub struct Slider {
    state: Entity<SliderState>,
    orientation: SliderOrientation,
    disabled: bool,
    reverse: bool,
    aria_label: Option<SharedString>,
    style: StyleRefinement,
}

impl Slider {
    pub fn new(state: &Entity<SliderState>) -> Self {
        Self {
            state: state.clone(),
            orientation: SliderOrientation::Horizontal,
            disabled: false,
            reverse: false,
            aria_label: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn horizontal(state: &Entity<SliderState>) -> Self {
        Self::new(state).orientation(SliderOrientation::Horizontal)
    }

    pub fn vertical(state: &Entity<SliderState>) -> Self {
        Self::new(state).orientation(SliderOrientation::Vertical)
    }

    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }
}

impl Styled for Slider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Slider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut inner = match self.orientation {
            SliderOrientation::Horizontal => KitSlider::new(&self.state).horizontal().w_full(),
            SliderOrientation::Vertical => KitSlider::new(&self.state)
                .vertical()
                .h_full()
                .min_h(px(160.0)),
        }
        .disabled(self.disabled);

        if self.reverse {
            inner = inner.reverse();
        }
        if let Some(label) = self.aria_label {
            inner = inner.accessibility_label(label);
        }

        div()
            .cx_when(
                matches!(self.orientation, SliderOrientation::Horizontal),
                |d| d.w_full(),
            )
            .cx_when(
                matches!(self.orientation, SliderOrientation::Vertical),
                |d| d.h_full().min_h(px(160.0)),
            )
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .child(inner)
    }
}

/// Convenience helper pairing an uppercase [`Label`] with a controlled [`Slider`].
pub fn slider_field(
    label: impl Into<SharedString>,
    state: &Entity<SliderState>,
    theme: &SemanticTheme,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Label::new(label).theme(theme))
        .child(Slider::new(state))
}

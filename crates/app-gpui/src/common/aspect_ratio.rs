//! Aspect ratio container ported from `web/src/components/ui/aspect-ratio.tsx`.
//!
//! Matches `--preset b1oaDsAa`: `relative aspect-(--ratio)` with sharp zero-radius
//! corners (`PresetRadius::NONE`) backed directly by GPUI's native Taffy
//! `Styled::aspect_ratio(f32)` layout constraint.

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window, div,
};

use crate::common::utils::PresetRadius;

/// Proportional box container matching `AspectRatio` in `aspect-ratio.tsx`.
#[derive(IntoElement)]
pub struct AspectRatio {
    ratio: f32,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl AspectRatio {
    /// Creates an aspect-ratio container with `width / height = ratio` (clamped to `> 0`).
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio: if ratio.is_finite() && ratio > 0.0 {
                ratio
            } else {
                1.0
            },
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn square() -> Self {
        Self::new(1.0)
    }

    pub fn video() -> Self {
        Self::new(16.0 / 9.0)
    }

    pub fn standard() -> Self {
        Self::new(4.0 / 3.0)
    }

    pub fn portrait() -> Self {
        Self::new(3.0 / 4.0)
    }

    pub fn ratio(mut self, ratio: f32) -> Self {
        if ratio.is_finite() && ratio > 0.0 {
            self.ratio = ratio;
        }
        self
    }
}

impl Styled for AspectRatio {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AspectRatio {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AspectRatio {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .relative()
            .w_full()
            .rounded(PresetRadius::NONE)
            .overflow_hidden()
            .aspect_ratio(self.ratio)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Convenience helper creating an [`AspectRatio`] box for `width / height`.
pub fn aspect_ratio(ratio: f32) -> AspectRatio {
    AspectRatio::new(ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_ratio_falls_back_to_square() {
        assert_eq!(AspectRatio::new(0.0).ratio, 1.0);
        assert_eq!(AspectRatio::new(-2.0).ratio, 1.0);
        assert!((AspectRatio::video().ratio - 16.0 / 9.0).abs() < 1e-5);
    }
}

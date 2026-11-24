//! Separator primitive ported from `web/src/components/ui/separator.tsx`.
//!
//! Matches `--preset b1oaDsAa`:
//! `shrink-0 bg-border data-horizontal:h-px data-horizontal:w-full data-vertical:w-px data-vertical:self-stretch`
//! Built on `gpui_kit::component::separator::Separator` per Rule 2.

pub use gpui_kit::component::separator::SeparatorStyle;
use gpui_kit::component::separator::Separator as KitSeparator;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    App, Hsla, IntoElement, RenderOnce, SharedString, StyleRefinement, Styled, Window, px,
};

use crate::common::utils::PresetTokens;
use crate::theme::SemanticTheme;

/// Orientation matching `orientation = "horizontal" | "vertical"` in `separator.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Hairline visual divider matching `Separator` in `web/src/components/ui/separator.tsx`.
#[derive(IntoElement)]
pub struct Separator {
    orientation: SeparatorOrientation,
    color: Option<Hsla>,
    label: Option<SharedString>,
    dashed: bool,
    style: StyleRefinement,
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl Separator {
    pub fn new() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            color: None,
            label: None,
            dashed: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn horizontal() -> Self {
        Self::new().orientation(SeparatorOrientation::Horizontal)
    }

    pub fn vertical() -> Self {
        Self::new().orientation(SeparatorOrientation::Vertical)
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.color = Some(PresetTokens::from_theme(theme).border);
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }
}

impl Styled for Separator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut inner = match self.orientation {
            SeparatorOrientation::Horizontal => KitSeparator::horizontal().h(px(1.0)).w_full(),
            SeparatorOrientation::Vertical => KitSeparator::vertical().w(px(1.0)).h_full(),
        };

        if let Some(color) = self.color {
            inner = inner.color(color);
        }
        if let Some(label) = self.label {
            inner = inner.label(label);
        }
        if self.dashed {
            inner = inner.dashed();
        }

        inner.flex_shrink_0().refine_style(&self.style)
    }
}

/// Convenience constructor for a themed horizontal 1px separator.
pub fn horizontal_separator(theme: &SemanticTheme) -> Separator {
    Separator::horizontal().theme(theme)
}

/// Convenience constructor for a themed vertical 1px separator.
pub fn vertical_separator(theme: &SemanticTheme) -> Separator {
    Separator::vertical().theme(theme)
}

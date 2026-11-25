//! Skeleton loading placeholder ported from `web/src/components/ui/skeleton.tsx`.
//!
//! Matches `--preset b1oaDsAa`: `animate-pulse bg-muted` with sharp zero-radius
//! corners (`PresetRadius::NONE`). Built on `gpui_kit::component::skeleton::Skeleton`.

use gpui_kit::component::StyledExt as _;
use gpui_kit::component::skeleton::Skeleton as KitSkeleton;
use gpui_kit::{App, Hsla, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::common::utils::{PresetRadius, PresetTokens};
use crate::theme::SemanticTheme;

/// Animated pulse loading placeholder matching `Skeleton` in `skeleton.tsx`.
#[derive(IntoElement)]
pub struct Skeleton {
    bg: Option<Hsla>,
    secondary: bool,
    style: StyleRefinement,
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            bg: None,
            secondary: false,
            style: StyleRefinement::default(),
        }
    }

    /// Applies the active [`SemanticTheme`]'s muted surface color.
    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        let tokens = PresetTokens::from_theme(theme);
        self.bg = Some(if tokens.is_dark {
            theme.bg_surface
        } else {
            tokens.secondary.opacity(0.6)
        });
        self
    }

    pub fn secondary(mut self) -> Self {
        self.secondary = true;
        self
    }
}

impl Styled for Skeleton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut inner = KitSkeleton::new().rounded(PresetRadius::NONE);
        if self.secondary {
            inner = inner.secondary();
        }
        if let Some(bg) = self.bg {
            inner = inner.bg(bg);
        }
        inner.refine_style(&self.style)
    }
}

/// Convenience constructor for a themed sharp-cornered skeleton block.
pub fn skeleton(theme: &SemanticTheme) -> Skeleton {
    Skeleton::new().theme(theme)
}

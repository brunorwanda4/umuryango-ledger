use gpui_kit::prelude::*;
use gpui_kit::{div, px, Div, ElementId, IntoElement, ParentElement, Styled};
use crate::safe_area::SafeAreaInsets;

/// A layout container that applies system safe area insets (status bar, navigation bar, cutouts)
#[derive(Debug, Clone, Copy)]
pub struct SafeAreaView {
    pub insets: SafeAreaInsets,
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub include_ime: bool,
}

impl SafeAreaView {
    /// Applies insets on all four edges plus software keyboard (IME)
    pub fn all(insets: SafeAreaInsets) -> Self {
        Self {
            insets,
            top: true,
            bottom: true,
            left: true,
            right: true,
            include_ime: true,
        }
    }

    /// Applies top safe area inset only (status bar & camera notch)
    pub fn top_only(insets: SafeAreaInsets) -> Self {
        Self {
            insets,
            top: true,
            bottom: false,
            left: false,
            right: false,
            include_ime: false,
        }
    }

    /// Applies bottom safe area inset only (Android 3-button or gesture navigation bar)
    pub fn bottom_only(insets: SafeAreaInsets) -> Self {
        Self {
            insets,
            top: false,
            bottom: true,
            left: false,
            right: false,
            include_ime: true,
        }
    }

    /// Applies vertical (top and bottom) insets
    pub fn vertical(insets: SafeAreaInsets) -> Self {
        Self {
            insets,
            top: true,
            bottom: true,
            left: false,
            right: false,
            include_ime: true,
        }
    }

    /// Applies horizontal (left and right) insets for landscape cutouts
    pub fn horizontal(insets: SafeAreaInsets) -> Self {
        Self {
            insets,
            top: false,
            bottom: false,
            left: true,
            right: true,
            include_ime: false,
        }
    }

    /// Sets whether the soft keyboard (IME) height should be factored into bottom padding
    pub fn with_ime(mut self, include_ime: bool) -> Self {
        self.include_ime = include_ime;
        self
    }

    /// Applies safe area padding directly onto any Styled element
    pub fn apply<E: Styled>(&self, element: E) -> E {
        let mut el = element;
        if self.top && self.insets.top > 0.0 {
            el = el.pt(self.insets.top_px());
        }
        if self.bottom {
            let b = if self.include_ime {
                self.insets.total_bottom()
            } else {
                self.insets.bottom
            };
            if b > 0.0 {
                el = el.pb(px(b));
            }
        }
        if self.left && self.insets.left > 0.0 {
            el = el.pl(self.insets.left_px());
        }
        if self.right && self.insets.right > 0.0 {
            el = el.pr(self.insets.right_px());
        }
        el
    }

    /// Renders a wrapped container `div()` padded with the configured safe area insets
    pub fn render(
        &self,
        id: impl Into<ElementId>,
        child: impl IntoElement,
    ) -> Div {
        self.apply(
            div()
                .id(id)
                .w_full()
                .h_full()
                .child(child),
        )
    }
}

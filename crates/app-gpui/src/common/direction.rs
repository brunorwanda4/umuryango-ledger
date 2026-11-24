//! Layout direction primitives ported from `web/src/components/ui/direction.tsx`.
//!
//! Bridges shadcn's `DirectionProvider` and `useDirection` onto GPUI's native
//! [`LayoutDirection`] (`Ltr` / `Rtl`) and [`LogicalStyledExt`] (`ps`, `pe`,
//! `ms`, `me`, `border_s`, `border_e`, `inset_s`, `inset_e`, `flex_row_dir`,
//! `text_start`, `text_end`).

pub use gpui_kit::base::{LayoutDirection as Direction, LogicalStyledExt};
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window, div,
};

/// Reads the current application [`Direction`] (`Ltr` or `Rtl`), equivalent to
/// React's `useDirection()` hook in `direction.tsx`.
#[inline]
pub fn use_direction(cx: &App) -> Direction {
    Direction::global(cx)
}

/// Updates the global application [`Direction`] and refreshes open windows.
#[inline]
pub fn set_direction(direction: Direction, cx: &mut App) {
    direction.set_global(cx);
}

/// Directional layout wrapper ported from `DirectionProvider` in `direction.tsx`.
///
/// Sets the global direction when `sync_global` is enabled, and aligns text/flow
/// start edges according to the configured [`Direction`].
#[derive(IntoElement)]
pub struct DirectionProvider {
    direction: Direction,
    sync_global: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DirectionProvider {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            sync_global: false,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn ltr() -> Self {
        Self::new(Direction::Ltr)
    }

    pub fn rtl() -> Self {
        Self::new(Direction::Rtl)
    }

    /// Synchronizes the global GPUI [`Direction`] on render when changed.
    pub fn sync_global(mut self, sync: bool) -> Self {
        self.sync_global = sync;
        self
    }
}

impl Styled for DirectionProvider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DirectionProvider {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DirectionProvider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.sync_global {
            self.direction.set_global(cx);
        }

        let base = if self.direction.is_rtl() {
            div().text_right()
        } else {
            div().text_left()
        };

        base.refine_style(&self.style).children(self.children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_locale_detection() {
        assert_eq!(Direction::from_locale("en"), Direction::Ltr);
        assert_eq!(Direction::from_locale("rw"), Direction::Ltr);
        assert_eq!(Direction::from_locale("ar"), Direction::Rtl);
    }
}

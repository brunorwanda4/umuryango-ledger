//! Reactive viewport and breakpoint helpers ported from `web/src/hooks/use-mobile.ts`.
//!
//! Computes `Breakpoint` (`Mobile`, `Tablet`, `Desktop`) directly from GPUI
//! `Window` viewport bounds (`< 768px` mobile, `768px..1024px` tablet,
//! `>= 1024px` desktop) so responsive primitives (`sidebar`, `drawer`, `sheet`,
//! `dialog`, `navigation_menu`) adapt seamlessly across Desktop, WebAssembly,
//! and Android/Mobile.

use gpui_kit::{Context, Pixels, Size, Subscription, Window, px};

use crate::safe_area::SafeAreaInsets;

/// Matches `const MOBILE_BREAKPOINT = 768` in `web/src/hooks/use-mobile.ts`.
pub const MOBILE_BREAKPOINT: f32 = 768.0;

/// Tablet upper threshold (`lg` breakpoint in Tailwind CSS).
pub const TABLET_BREAKPOINT: f32 = 1024.0;

/// Responsive layout tier derived from GPUI window viewport width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Breakpoint {
    /// `< 768px` (compact phone / narrow split window)
    Mobile,
    /// `768px .. 1024px` (tablet / compact laptop)
    Tablet,
    /// `>= 1024px` (standard desktop / wide browser viewport)
    Desktop,
}

impl Breakpoint {
    /// Classifies a logical pixel width into a [`Breakpoint`].
    #[inline]
    pub fn from_width(width_px: f32) -> Self {
        if width_px < MOBILE_BREAKPOINT {
            Self::Mobile
        } else if width_px < TABLET_BREAKPOINT {
            Self::Tablet
        } else {
            Self::Desktop
        }
    }

    /// Reads the active GPUI window's viewport size and returns its [`Breakpoint`].
    #[inline]
    pub fn from_window(window: &Window) -> Self {
        let width = viewport_width(window);
        if width <= 0.0 && gpui_kit::is_mobile() {
            return Self::Mobile;
        }
        Self::from_width(width)
    }

    #[inline]
    pub const fn is_mobile(self) -> bool {
        matches!(self, Self::Mobile)
    }

    #[inline]
    pub const fn is_tablet(self) -> bool {
        matches!(self, Self::Tablet)
    }

    #[inline]
    pub const fn is_desktop(self) -> bool {
        matches!(self, Self::Desktop)
    }
}

/// Returns `true` when the active GPUI window is narrower than `768px`
/// (direct equivalent of React's `useIsMobile()` hook in `use-mobile.ts`).
#[inline]
pub fn is_mobile(window: &Window) -> bool {
    Breakpoint::from_window(window).is_mobile()
}

/// Returns `true` when the active GPUI window is in `[768px, 1024px)`.
#[inline]
pub fn is_tablet(window: &Window) -> bool {
    Breakpoint::from_window(window).is_tablet()
}

/// Returns `true` when the active GPUI window is `>= 1024px`.
#[inline]
pub fn is_desktop(window: &Window) -> bool {
    Breakpoint::from_window(window).is_desktop()
}

/// Logical viewport width of `window` in `f32` pixels.
#[inline]
pub fn viewport_width(window: &Window) -> f32 {
    f32::from(window.viewport_size().width)
}

/// Logical viewport height of `window` in `f32` pixels.
#[inline]
pub fn viewport_height(window: &Window) -> f32 {
    f32::from(window.viewport_size().height)
}

/// Snapshot of the current window viewport, active breakpoint, and safe-area insets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportMetrics {
    pub size: Size<Pixels>,
    pub breakpoint: Breakpoint,
    pub safe_area: SafeAreaInsets,
}

impl ViewportMetrics {
    pub fn from_window(window: &Window) -> Self {
        let size = window.viewport_size();
        let width = f32::from(size.width);
        let breakpoint = if width <= 0.0 && gpui_kit::is_mobile() {
            Breakpoint::Mobile
        } else {
            Breakpoint::from_width(width)
        };
        Self {
            size,
            breakpoint,
            safe_area: crate::safe_area::platform::query_system_window_insets(),
        }
    }

    #[inline]
    pub fn is_mobile(&self) -> bool {
        self.breakpoint.is_mobile()
    }

    #[inline]
    pub fn usable_width(&self) -> Pixels {
        px((f32::from(self.size.width) - self.safe_area.left - self.safe_area.right).max(0.0))
    }

    #[inline]
    pub fn usable_height(&self) -> Pixels {
        px(
            (f32::from(self.size.height) - self.safe_area.top - self.safe_area.total_bottom())
                .max(0.0),
        )
    }
}

/// Subscribes an entity to GPUI window bounds changes, invoking `on_change`
/// with updated [`ViewportMetrics`] whenever the window resizes or rotates.
pub fn observe_viewport<V: 'static>(
    window: &mut Window,
    cx: &mut Context<V>,
    mut on_change: impl FnMut(&mut V, ViewportMetrics, &mut Window, &mut Context<V>) + 'static,
) -> Subscription {
    cx.observe_window_bounds(window, move |this, window, cx| {
        let metrics = ViewportMetrics::from_window(window);
        on_change(this, metrics, window, cx);
    })
}

/// Notifies a view whenever window bounds cross a responsive breakpoint.
pub fn notify_on_resize<V: 'static>(window: &mut Window, cx: &mut Context<V>) -> Subscription {
    let mut last_bp = Breakpoint::from_window(window);
    cx.observe_window_bounds(window, move |_, window, cx| {
        let next_bp = Breakpoint::from_window(window);
        if next_bp != last_bp {
            last_bp = next_bp;
            cx.notify();
        }
    })
}

/// Convenience helper returning `mobile_val` when `is_mobile(window)` and `desktop_val` otherwise.
#[inline]
pub fn responsive<T>(window: &Window, mobile_val: T, desktop_val: T) -> T {
    if is_mobile(window) {
        mobile_val
    } else {
        desktop_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breakpoint_thresholds_match_preset() {
        assert_eq!(Breakpoint::from_width(375.0), Breakpoint::Mobile);
        assert_eq!(Breakpoint::from_width(767.9), Breakpoint::Mobile);
        assert_eq!(Breakpoint::from_width(768.0), Breakpoint::Tablet);
        assert_eq!(Breakpoint::from_width(1023.9), Breakpoint::Tablet);
        assert_eq!(Breakpoint::from_width(1024.0), Breakpoint::Desktop);
        assert_eq!(Breakpoint::from_width(1440.0), Breakpoint::Desktop);
        assert!(Breakpoint::Mobile.is_mobile());
        assert!(Breakpoint::Tablet.is_tablet());
        assert!(Breakpoint::Desktop.is_desktop());
    }
}

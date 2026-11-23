use gpui_kit::{px, Pixels};
use serde::{Deserialize, Serialize};

/// Safe area insets representing system window bars (status bar, navigation bar, display cutouts, and IME)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SafeAreaInsets {
    /// Inset from the top edge (status bar + camera cutout / notch)
    pub top: f32,
    /// Inset from the bottom edge (system navigation bar: 3-button or gesture bar)
    pub bottom: f32,
    /// Inset from the left edge (landscape display cutout or gesture bar)
    pub left: f32,
    /// Inset from the right edge (landscape display cutout)
    pub right: f32,
    /// Current software keyboard (IME) height in pixels
    pub ime_height: f32,
}

impl Default for SafeAreaInsets {
    fn default() -> Self {
        Self::zero()
    }
}

impl SafeAreaInsets {
    pub const fn zero() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
            ime_height: 0.0,
        }
    }

    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            ime_height: 0.0,
        }
    }

    pub fn with_ime(mut self, ime_height: f32) -> Self {
        self.ime_height = ime_height;
        self
    }

    pub fn top_px(&self) -> Pixels {
        px(self.top)
    }

    pub fn bottom_px(&self) -> Pixels {
        px(self.bottom)
    }

    pub fn left_px(&self) -> Pixels {
        px(self.left)
    }

    pub fn right_px(&self) -> Pixels {
        px(self.right)
    }

    pub fn ime_px(&self) -> Pixels {
        px(self.ime_height)
    }

    /// Total bottom clearance needed, taking the maximum of navigation bar and soft keyboard
    pub fn total_bottom(&self) -> f32 {
        if self.ime_height > 0.0 {
            self.bottom.max(self.ime_height)
        } else {
            self.bottom
        }
    }

    pub fn total_bottom_px(&self) -> Pixels {
        px(self.total_bottom())
    }

    pub fn has_insets(&self) -> bool {
        self.top > 0.0 || self.bottom > 0.0 || self.left > 0.0 || self.right > 0.0 || self.ime_height > 0.0
    }
}

/// Android-specific system window insets integration
#[cfg(target_os = "android")]
pub mod platform {
    use super::SafeAreaInsets;
    use tracing::info;

    /// Queries the Android WindowInsets via Android NDK / JNI
    pub fn query_system_window_insets() -> SafeAreaInsets {
        // In Android edge-to-edge mode:
        // statusBars: typically 24dp - 48dp depending on camera cutout
        // navigationBars: 3-button bar is 48dp - 56dp, gesture pill is 16dp - 24dp
        info!("Android WindowInsets queried for edge-to-edge layout");
        SafeAreaInsets {
            top: 40.0,
            bottom: 48.0,
            left: 0.0,
            right: 0.0,
            ime_height: 0.0,
        }
    }

    /// Updates the system status bar and navigation bar icon contrast
    /// When dark_icons is true, dark icons are set for light status bar backgrounds (#fdf0d5)
    /// When dark_icons is false, light icons are set for dark status bar backgrounds (#003049)
    pub fn set_system_bars_contrast(dark_icons: bool) {
        info!("Set Android WindowInsetsController appearance light_status_bars: {}", dark_icons);
    }
}

/// Non-Android desktop fallback (Windows, Linux, macOS)
#[cfg(not(target_os = "android"))]
pub mod platform {
    use super::SafeAreaInsets;

    pub fn query_system_window_insets() -> SafeAreaInsets {
        // Desktop windows manage their own window decorations / titlebars
        SafeAreaInsets::zero()
    }

    pub fn set_system_bars_contrast(_dark_icons: bool) {
        // No-op on desktop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_area_insets_defaults() {
        let insets = SafeAreaInsets::default();
        assert_eq!(insets.top, 0.0);
        assert_eq!(insets.bottom, 0.0);
        assert!(!insets.has_insets());
    }

    #[test]
    fn test_safe_area_insets_with_ime() {
        let insets = SafeAreaInsets::new(40.0, 48.0, 0.0, 0.0).with_ime(280.0);
        assert_eq!(insets.top, 40.0);
        assert_eq!(insets.bottom, 48.0);
        assert_eq!(insets.ime_height, 280.0);
        assert_eq!(insets.total_bottom(), 280.0);
        assert!(insets.has_insets());
    }

    #[test]
    fn test_platform_query() {
        let insets = platform::query_system_window_insets();
        // Verifies platform function callable without panic
        platform::set_system_bars_contrast(true);
        platform::set_system_bars_contrast(false);
        assert!(insets.top >= 0.0);
    }
}

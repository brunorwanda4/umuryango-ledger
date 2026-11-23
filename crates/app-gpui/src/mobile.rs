use crate::safe_area::SafeAreaInsets;

#[derive(Debug, Clone, Copy)]
pub struct MobileViewport {
    pub is_mobile: bool,
    pub width: f32,
    pub height: f32,
    pub safe_area: SafeAreaInsets,
}

impl Default for MobileViewport {
    fn default() -> Self {
        Self {
            is_mobile: false,
            width: 1024.0,
            height: 768.0,
            safe_area: SafeAreaInsets::zero(),
        }
    }
}

impl MobileViewport {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            is_mobile: width < 768.0,
            width,
            height,
            safe_area: crate::safe_area::platform::query_system_window_insets(),
        }
    }

    pub fn with_safe_area(mut self, insets: SafeAreaInsets) -> Self {
        self.safe_area = insets;
        self
    }

    pub fn is_compact(&self) -> bool {
        self.width < 768.0
    }

    /// Returns the usable height excluding the top status bar and bottom navigation bar
    pub fn usable_height(&self) -> f32 {
        (self.height - self.safe_area.top - self.safe_area.total_bottom()).max(0.0)
    }

    /// Returns the usable width excluding horizontal display cutouts
    pub fn usable_width(&self) -> f32 {
        (self.width - self.safe_area.left - self.safe_area.right).max(0.0)
    }
}

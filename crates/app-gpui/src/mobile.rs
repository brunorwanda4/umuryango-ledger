#[derive(Debug, Clone, Copy)]
pub struct MobileViewport {
    pub is_mobile: bool,
    pub width: f32,
    pub height: f32,
}

impl Default for MobileViewport {
    fn default() -> Self {
        Self {
            is_mobile: false,
            width: 1024.0,
            height: 768.0,
        }
    }
}

impl MobileViewport {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            is_mobile: width < 768.0,
            width,
            height,
        }
    }

    pub fn is_compact(&self) -> bool {
        self.width < 768.0
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SyncStatus {
    Synced,
    PendingSync(usize),
    Offline,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self::Synced
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserThemePrefs {
    pub theme_mode: String, // "light" | "dark"
    pub primary: String,
    pub primary_hover: String,
    pub primary_pressed: String,
    pub primary_deep: String,
    pub dark_canvas: String,
    pub dark_surface: String,
    pub text_muted: String,
    pub light_border: String,
    pub light_canvas: String,
    pub light_surface: String,
}

impl Default for UserThemePrefs {
    fn default() -> Self {
        Self {
            theme_mode: "light".to_string(),
            primary: "#ba181b".to_string(),
            primary_hover: "#e5383b".to_string(),
            primary_pressed: "#a4161a".to_string(),
            primary_deep: "#660708".to_string(),
            dark_canvas: "#0b090a".to_string(),
            dark_surface: "#161a1d".to_string(),
            text_muted: "#b1a7a6".to_string(),
            light_border: "#d3d3d3".to_string(),
            light_canvas: "#f5f3f4".to_string(),
            light_surface: "#ffffff".to_string(),
        }
    }
}

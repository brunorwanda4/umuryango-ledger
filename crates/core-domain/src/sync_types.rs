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
    pub crimson_deep: String,
    pub crimson_bright: String,
    pub cream_warm: String,
    pub prussian_navy: String,
    pub steel_blue: String,
}

impl Default for UserThemePrefs {
    fn default() -> Self {
        Self {
            theme_mode: "light".to_string(),
            crimson_deep: "#780000".to_string(),
            crimson_bright: "#c1121f".to_string(),
            cream_warm: "#fdf0d5".to_string(),
            prussian_navy: "#003049".to_string(),
            steel_blue: "#669bbc".to_string(),
        }
    }
}

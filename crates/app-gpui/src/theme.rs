use gpui_kit::{hsla, rgb, rgba, Hsla, Rgba};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use umuryango_storage_local::UserThemePrefs;

pub const FONT_FAMILY: &str = "Archivo";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn is_dark(&self) -> bool {
        matches!(self, ThemeMode::Dark)
    }

    pub fn toggle(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}

/// Customizable palette configuration stored in state and persisted in SQLite.
/// Seeded with 5 core brand color tokens:
/// - crimson_deep: #780000
/// - crimson_bright: #c1121f
/// - cream_warm: #fdf0d5
/// - prussian_navy: #003049
/// - steel_blue: #669bbc
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub crimson_deep: String,
    pub crimson_bright: String,
    pub cream_warm: String,
    pub prussian_navy: String,
    pub steel_blue: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            crimson_deep: "#780000".to_string(),
            crimson_bright: "#c1121f".to_string(),
            cream_warm: "#fdf0d5".to_string(),
            prussian_navy: "#003049".to_string(),
            steel_blue: "#669bbc".to_string(),
        }
    }
}

impl From<&UserThemePrefs> for ThemeConfig {
    fn from(p: &UserThemePrefs) -> Self {
        Self {
            crimson_deep: p.crimson_deep.clone(),
            crimson_bright: p.crimson_bright.clone(),
            cream_warm: p.cream_warm.clone(),
            prussian_navy: p.prussian_navy.clone(),
            steel_blue: p.steel_blue.clone(),
        }
    }
}

impl ThemeConfig {
    pub fn to_prefs(&self, mode: ThemeMode) -> UserThemePrefs {
        UserThemePrefs {
            theme_mode: match mode {
                ThemeMode::Light => "light".to_string(),
                ThemeMode::Dark => "dark".to_string(),
            },
            crimson_deep: self.crimson_deep.clone(),
            crimson_bright: self.crimson_bright.clone(),
            cream_warm: self.cream_warm.clone(),
            prussian_navy: self.prussian_navy.clone(),
            steel_blue: self.steel_blue.clone(),
        }
    }
}

/// Fully derived dynamic semantic tokens for either Light or Dark mode.
#[derive(Debug, Clone)]
pub struct SemanticTheme {
    pub mode: ThemeMode,
    pub font_family: &'static str,

    // Backgrounds
    pub bg_canvas: Hsla,
    pub bg_surface: Hsla,
    pub bg_elevated: Hsla,

    // Typography & Borders
    pub text_primary: Hsla,
    pub text_muted: Hsla,
    pub border_subtle: Hsla,

    // Accents & Actions
    pub accent_primary: Hsla,
    pub accent_secondary: Hsla,

    // Financial Indicators
    pub income_color: Hsla,
    pub expense_color: Hsla,
    pub handover_color: Hsla,

    // Hex strings for chart components and tooltips
    pub bg_canvas_hex: String,
    pub bg_surface_hex: String,
    pub text_primary_hex: String,
    pub text_muted_hex: String,
    pub income_hex: String,
    pub expense_hex: String,
    pub handover_hex: String,
}

impl SemanticTheme {
    pub fn from_config(config: &ThemeConfig, mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => {
                // Light mode: cream_warm canvas, prussian_navy text, steel_blue income, crimson expenses
                Self {
                    mode,
                    font_family: FONT_FAMILY,
                    bg_canvas: hex_to_hsla(&config.cream_warm),
                    bg_surface: hex_to_hsla("#ffffff"),
                    bg_elevated: hex_to_hsla("#f5e6c8"),
                    text_primary: hex_to_hsla(&config.prussian_navy),
                    text_muted: hex_to_hsla("#4a6070"),
                    border_subtle: hex_to_hsla("#e5d4b8"),
                    accent_primary: hex_to_hsla(&config.crimson_deep),
                    accent_secondary: hex_to_hsla(&config.crimson_bright),
                    income_color: hex_to_hsla(&config.steel_blue),
                    expense_color: hex_to_hsla(&config.crimson_bright),
                    handover_color: hex_to_hsla(&config.crimson_deep),

                    bg_canvas_hex: config.cream_warm.clone(),
                    bg_surface_hex: "#ffffff".to_string(),
                    text_primary_hex: config.prussian_navy.clone(),
                    text_muted_hex: "#4a6070".to_string(),
                    income_hex: config.steel_blue.clone(),
                    expense_hex: config.crimson_bright.clone(),
                    handover_hex: config.crimson_deep.clone(),
                }
            }
            ThemeMode::Dark => {
                // Dark mode: prussian_navy canvas, cream_warm text, steel_blue income, crimson highlights
                Self {
                    mode,
                    font_family: FONT_FAMILY,
                    bg_canvas: hex_to_hsla("#001b2a"),
                    bg_surface: hex_to_hsla(&config.prussian_navy),
                    bg_elevated: hex_to_hsla("#0b3d5b"),
                    text_primary: hex_to_hsla(&config.cream_warm),
                    text_muted: hex_to_hsla("#a8c5d6"),
                    border_subtle: hex_to_hsla("#0e4b6e"),
                    accent_primary: hex_to_hsla(&config.crimson_bright),
                    accent_secondary: hex_to_hsla(&config.crimson_deep),
                    income_color: hex_to_hsla(&config.steel_blue),
                    expense_color: hex_to_hsla(&config.crimson_bright),
                    handover_color: hex_to_hsla(&config.cream_warm),

                    bg_canvas_hex: "#001b2a".to_string(),
                    bg_surface_hex: config.prussian_navy.clone(),
                    text_primary_hex: config.cream_warm.clone(),
                    text_muted_hex: "#a8c5d6".to_string(),
                    income_hex: config.steel_blue.clone(),
                    expense_hex: config.crimson_bright.clone(),
                    handover_hex: config.cream_warm.clone(),
                }
            }
        }
    }
}

/// Helper converting `#RRGGBB` or `#RRGGBBAA` strings into `gpui::Rgba`.
pub fn hex_to_rgba(hex: &str) -> Rgba {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() >= 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0);
        let a = if clean.len() >= 8 {
            u8::from_str_radix(&clean[6..8], 16).unwrap_or(255)
        } else {
            255
        };
        let val = ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32);
        rgba(val)
    } else {
        rgb(0x000000)
    }
}

/// Helper converting a hex string into `gpui::Hsla`.
pub fn hex_to_hsla(hex: &str) -> Hsla {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() >= 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0) as f32 / 255.0;
        let a = if clean.len() >= 8 {
            u8::from_str_radix(&clean[6..8], 16).unwrap_or(255) as f32 / 255.0
        } else {
            1.0
        };

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if (max - min).abs() < 1e-5 {
            hsla(0.0, 0.0, l, a)
        } else {
            let d = max - min;
            let s = if l > 0.5 {
                d / (2.0 - max - min)
            } else {
                d / (max + min)
            };
            let mut h = if (max - r).abs() < 1e-5 {
                (g - b) / d + (if g < b { 6.0 } else { 0.0 })
            } else if (max - g).abs() < 1e-5 {
                (b - r) / d + 2.0
            } else {
                (r - g) / d + 4.0
            };
            h /= 6.0;
            hsla(h, s, l, a)
        }
    } else {
        hsla(0.0, 0.0, 0.0, 1.0)
    }
}

/// Bundles the 4 Archivo TrueType font weights at compile time
pub fn archivo_font_bytes() -> Vec<Cow<'static, [u8]>> {
    vec![
        Cow::Borrowed(include_bytes!("../assets/fonts/Archivo-Regular.ttf")),
        Cow::Borrowed(include_bytes!("../assets/fonts/Archivo-Medium.ttf")),
        Cow::Borrowed(include_bytes!("../assets/fonts/Archivo-SemiBold.ttf")),
        Cow::Borrowed(include_bytes!("../assets/fonts/Archivo-Bold.ttf")),
    ]
}

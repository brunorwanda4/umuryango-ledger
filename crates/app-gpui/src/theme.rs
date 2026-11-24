use gpui_kit::{Hsla, Rgba, hsla, rgb, rgba};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use umuryango_core_domain::UserThemePrefs;

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

/// Customizable 10-color palette configuration stored in state and persisted in SQLite.
/// Seeded with the 10-color red primary palette:
/// ["#0b090a", "#161a1d", "#660708", "#a4161a", "#ba181b", "#e5383b", "#b1a7a6", "#d3d3d3", "#f5f3f4", "#ffffff"]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub primary: String, // #ba181b (Main primary buttons, active navigation, primary brand accents)
    pub primary_hover: String, // #e5383b (Hover states, active highlights, bright red emphasis)
    pub primary_pressed: String, // #a4161a (Pressed button states, strong red badges)
    pub primary_deep: String, // #660708 (Deep crimson containers, dark-tinted active pills, high-contrast red accents)
    pub dark_canvas: String,  // #0b090a (Dark Mode main app background)
    pub dark_surface: String, // #161a1d (Dark Mode cards, sidebar, modals, inputs)
    pub text_muted: String,   // #b1a7a6 (Muted text / subtle dividers)
    pub light_border: String, // #d3d3d3 (Light Mode input/card borders, subtle elevated headers)
    pub light_canvas: String, // #f5f3f4 (Light Mode main app background)
    pub light_surface: String, // #ffffff (Light Mode cards, sidebar, modals, inputs)
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
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

impl From<&UserThemePrefs> for ThemeConfig {
    fn from(p: &UserThemePrefs) -> Self {
        Self {
            primary: p.primary.clone(),
            primary_hover: p.primary_hover.clone(),
            primary_pressed: p.primary_pressed.clone(),
            primary_deep: p.primary_deep.clone(),
            dark_canvas: p.dark_canvas.clone(),
            dark_surface: p.dark_surface.clone(),
            text_muted: p.text_muted.clone(),
            light_border: p.light_border.clone(),
            light_canvas: p.light_canvas.clone(),
            light_surface: p.light_surface.clone(),
        }
    }
}

impl ThemeConfig {
    pub fn from_prefs(p: &UserThemePrefs) -> Self {
        Self::from(p)
    }

    pub fn to_prefs(&self, mode: ThemeMode) -> UserThemePrefs {
        UserThemePrefs {
            theme_mode: match mode {
                ThemeMode::Light => "light".to_string(),
                ThemeMode::Dark => "dark".to_string(),
            },
            primary: self.primary.clone(),
            primary_hover: self.primary_hover.clone(),
            primary_pressed: self.primary_pressed.clone(),
            primary_deep: self.primary_deep.clone(),
            dark_canvas: self.dark_canvas.clone(),
            dark_surface: self.dark_surface.clone(),
            text_muted: self.text_muted.clone(),
            light_border: self.light_border.clone(),
            light_canvas: self.light_canvas.clone(),
            light_surface: self.light_surface.clone(),
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
    pub text_secondary: Hsla,
    pub text_muted: Hsla,
    pub border_subtle: Hsla,

    // Primary Red Spectrum (Main Brand Primary)
    pub primary: Hsla,
    pub primary_hover: Hsla,
    pub primary_pressed: Hsla,
    pub primary_deep: Hsla,
    pub accent_primary: Hsla,
    pub accent_secondary: Hsla,

    // Financial Indicators
    pub income_color: Hsla,
    pub expense_color: Hsla,
    pub handover_color: Hsla,

    // Hex strings for chart components and tooltips
    pub bg_canvas_hex: String,
    pub bg_surface_hex: String,
    pub bg_elevated_hex: String,
    pub text_primary_hex: String,
    pub text_secondary_hex: String,
    pub text_muted_hex: String,
    pub primary_hex: String,
    pub income_hex: String,
    pub expense_hex: String,
    pub handover_hex: String,
}

impl SemanticTheme {
    pub fn from_config(config: &ThemeConfig, mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => {
                // Light mode:
                // bg_canvas: #f5f3f4
                // bg_surface: #ffffff
                // bg_elevated: #d3d3d3 / #ffffff
                // border_subtle: #d3d3d3
                // text_primary: #0b090a
                // text_secondary: #161a1d
                // text_muted: #b1a7a6
                // accent_primary: #ba181b (with #a4161a / #660708 for deep red text/badges)
                // accent_secondary: #a4161a
                Self {
                    mode,
                    font_family: FONT_FAMILY,
                    bg_canvas: hex_to_hsla(&config.light_canvas),
                    bg_surface: hex_to_hsla(&config.light_surface),
                    bg_elevated: hex_to_hsla(&config.light_border),
                    text_primary: hex_to_hsla(&config.dark_canvas),
                    text_secondary: hex_to_hsla(&config.dark_surface),
                    text_muted: hex_to_hsla(&config.text_muted),
                    border_subtle: hex_to_hsla(&config.light_border),
                    primary: hex_to_hsla(&config.primary),
                    primary_hover: hex_to_hsla(&config.primary_hover),
                    primary_pressed: hex_to_hsla(&config.primary_pressed),
                    primary_deep: hex_to_hsla(&config.primary_deep),
                    accent_primary: hex_to_hsla(&config.primary),
                    accent_secondary: hex_to_hsla(&config.primary_pressed),
                    income_color: hex_to_hsla("#2a9d8f"),
                    expense_color: hex_to_hsla(&config.primary),
                    handover_color: hex_to_hsla(&config.primary_deep),

                    bg_canvas_hex: config.light_canvas.clone(),
                    bg_surface_hex: config.light_surface.clone(),
                    bg_elevated_hex: config.light_border.clone(),
                    text_primary_hex: config.dark_canvas.clone(),
                    text_secondary_hex: config.dark_surface.clone(),
                    text_muted_hex: config.text_muted.clone(),
                    primary_hex: config.primary.clone(),
                    income_hex: "#2a9d8f".to_string(),
                    expense_hex: config.primary.clone(),
                    handover_hex: config.primary_deep.clone(),
                }
            }
            ThemeMode::Dark => {
                // Dark mode:
                // bg_canvas: #0b090a
                // bg_surface: #161a1d
                // bg_elevated: #660708 / #161a1d
                // border_subtle: #b1a7a6 (at low opacity) / #d3d3d3
                // text_primary: #ffffff
                // text_secondary: #f5f3f4
                // text_muted: #b1a7a6
                // accent_primary: #ba181b (with #e5383b for bright contrast)
                // accent_secondary: #e5383b
                Self {
                    mode,
                    font_family: FONT_FAMILY,
                    bg_canvas: hex_to_hsla(&config.dark_canvas),
                    bg_surface: hex_to_hsla(&config.dark_surface),
                    bg_elevated: hex_to_hsla(&config.primary_deep),
                    text_primary: hex_to_hsla(&config.light_surface),
                    text_secondary: hex_to_hsla(&config.light_canvas),
                    text_muted: hex_to_hsla(&config.text_muted),
                    border_subtle: hex_to_hsla_alpha(&config.text_muted, 0.25),
                    primary: hex_to_hsla(&config.primary),
                    primary_hover: hex_to_hsla(&config.primary_hover),
                    primary_pressed: hex_to_hsla(&config.primary_pressed),
                    primary_deep: hex_to_hsla(&config.primary_deep),
                    accent_primary: hex_to_hsla(&config.primary),
                    accent_secondary: hex_to_hsla(&config.primary_hover),
                    income_color: hex_to_hsla("#2a9d8f"),
                    expense_color: hex_to_hsla(&config.primary),
                    handover_color: hex_to_hsla(&config.primary_hover),

                    bg_canvas_hex: config.dark_canvas.clone(),
                    bg_surface_hex: config.dark_surface.clone(),
                    bg_elevated_hex: config.primary_deep.clone(),
                    text_primary_hex: config.light_surface.clone(),
                    text_secondary_hex: config.light_canvas.clone(),
                    text_muted_hex: config.text_muted.clone(),
                    primary_hex: config.primary.clone(),
                    income_hex: "#2a9d8f".to_string(),
                    expense_hex: config.primary.clone(),
                    handover_hex: config.primary_hover.clone(),
                }
            }
        }
    }
}

/// Helper converting a hex string into `gpui::Hsla` with explicit alpha.
pub fn hex_to_hsla_alpha(hex: &str, alpha: f32) -> Hsla {
    let mut h = hex_to_hsla(hex);
    h.a = alpha;
    h
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

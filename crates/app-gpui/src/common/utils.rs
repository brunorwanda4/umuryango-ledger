//! Reusable GPUI style composition utilities and `--preset b1oaDsAa` (`base-sera`)
//! visual tokens ported from `web/src/lib/utils.ts`, `web/components.json`, and
//! `web/src/app/globals.css`.
//!
//! Key rules of `--preset b1oaDsAa`:
//! - Architectural zero corner radius (`--radius: 0`, `rounded-none`) across surfaces,
//!   cards, buttons, inputs, badges, menus, and dialogs; `RADIUS_FULL` reserved only
//!   for explicitly circular controls (`Avatar`, `Switch`, `RadioGroupItem`, `SliderThumb`).
//! - Editorial `Archivo` typography (`FONT_FAMILY`), uppercase `tracking-widest` /
//!   `tracking-wider` micro-labels on buttons, badges, labels, and card titles.
//! - Focus rings: `ring-2 ring-ring/30` (`focus_ring_shadow`) and destructive invalid
//!   rings `ring-2 ring-destructive/20` (`invalid_ring_shadow`).
//! - 10-color harmonious Red-Primary palette (`#ba181b` primary, `#0b090a` dark canvas,
//!   `#f5f3f4` light canvas) mapped strictly through [`SemanticTheme`].

use gpui_kit::base::box_shadow;
use gpui_kit::{BoxShadow, FontWeight, Hsla, Pixels, Styled, hsla, px};

use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeMode, hex_to_hsla};

/// Border radius scale matching `--preset b1oaDsAa` (`--radius: 0`).
pub struct PresetRadius;

impl PresetRadius {
    pub const NONE: Pixels = px(0.0);
    pub const SM: Pixels = px(0.0);
    pub const MD: Pixels = px(0.0);
    pub const LG: Pixels = px(0.0);
    pub const XL: Pixels = px(0.0);
    pub const XXL: Pixels = px(0.0);
    /// Used exclusively where a component explicitly specifies `rounded-full`.
    pub const FULL: Pixels = px(9999.0);
}

/// Control heights and container spacing scale matching `--preset b1oaDsAa`.
pub struct PresetSpacing;

impl PresetSpacing {
    /// `h-7` (`28px`) — extra-small buttons, compact badges, kbd chips
    pub const CONTROL_XS: Pixels = px(28.0);
    /// `h-8` (`32px`) — compact menu items, toolbar controls
    pub const CONTROL_COMPACT: Pixels = px(32.0);
    /// `h-9` (`36px`) — small buttons, compact select triggers
    pub const CONTROL_SM: Pixels = px(36.0);
    /// `h-10` (`40px`) — default `Button`, `Input`, `NativeSelect`
    pub const CONTROL_DEFAULT: Pixels = px(40.0);
    /// `h-11` (`44px`) — large `Button`
    pub const CONTROL_LG: Pixels = px(44.0);
    /// `--card-spacing: --spacing(8)` (`32px`)
    pub const CARD_DEFAULT: Pixels = px(32.0);
    /// `data-[size=sm]:[--card-spacing:--spacing(5)]` (`20px`)
    pub const CARD_SM: Pixels = px(20.0);
}

/// Resolved semantic shadcn/ui CSS variables mapped strictly onto Umuryango Ledger's
/// 10-color Red-Primary Light (`#f5f3f4`) & Dark (`#0b090a`) palette.
#[derive(Debug, Clone, Copy)]
pub struct PresetTokens {
    pub is_dark: bool,
    pub background: Hsla,
    pub foreground: Hsla,
    pub card: Hsla,
    pub card_foreground: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,
    pub primary: Hsla,
    pub primary_hover: Hsla,
    pub primary_pressed: Hsla,
    pub primary_deep: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_hover: Hsla,
    pub secondary_foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    pub destructive: Hsla,
    pub destructive_soft: Hsla,
    pub destructive_hover: Hsla,
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
}

impl PresetTokens {
    /// Resolves all shadcn semantic tokens from the active [`SemanticTheme`].
    pub fn from_theme(theme: &SemanticTheme) -> Self {
        let is_dark = matches!(theme.mode, ThemeMode::Dark);
        // Pure white text on #ba181b primary red in both Light & Dark modes:
        // in Light mode `theme.bg_surface` is #ffffff; in Dark mode `theme.text_primary` is #ffffff.
        let primary_foreground = if is_dark {
            theme.text_primary
        } else {
            theme.bg_surface
        };

        let (secondary, secondary_hover, muted, accent, destructive, input) = if is_dark {
            (
                theme.bg_surface,
                theme.bg_elevated.opacity(0.65),
                theme.bg_surface,
                theme.bg_elevated.opacity(0.55),
                theme.primary_hover, // #e5383b high-contrast crimson on dark surfaces
                theme.border_subtle,
            )
        } else {
            (
                theme.bg_elevated, // #d3d3d3
                theme.bg_elevated.opacity(0.8),
                theme.bg_canvas, // #f5f3f4
                theme.bg_elevated.opacity(0.55),
                theme.primary, // #ba181b
                theme.border_subtle,
            )
        };

        Self {
            is_dark,
            background: theme.bg_canvas,
            foreground: theme.text_primary,
            card: theme.bg_surface,
            card_foreground: theme.text_primary,
            popover: theme.bg_surface,
            popover_foreground: theme.text_primary,
            primary: theme.primary,
            primary_hover: theme.primary_hover,
            primary_pressed: theme.primary_pressed,
            primary_deep: theme.primary_deep,
            primary_foreground,
            secondary,
            secondary_hover,
            secondary_foreground: theme.text_primary,
            muted,
            muted_foreground: theme.text_muted,
            accent,
            accent_foreground: theme.text_primary,
            destructive,
            destructive_soft: destructive.opacity(if is_dark { 0.20 } else { 0.10 }),
            destructive_hover: destructive.opacity(if is_dark { 0.30 } else { 0.20 }),
            border: theme.border_subtle,
            input,
            ring: theme.accent_primary,
            success: theme.income_color,
            warning: hex_to_hsla("#e76f51"),
            info: theme.text_muted,
        }
    }
}

/// Builds the 2px outer focus ring (`focus-visible:ring-2 focus-visible:ring-ring/30`)
/// used across `--preset b1oaDsAa` interactive primitives.
pub fn focus_ring_shadow(theme: &SemanticTheme) -> Vec<BoxShadow> {
    let ring = PresetTokens::from_theme(theme).ring.opacity(0.30);
    vec![box_shadow(px(0.0), px(0.0), px(0.0), px(2.0), ring)]
}

/// Builds the 2px outer destructive validation ring (`aria-invalid:ring-2 aria-invalid:ring-destructive/20`).
pub fn invalid_ring_shadow(theme: &SemanticTheme) -> Vec<BoxShadow> {
    let tokens = PresetTokens::from_theme(theme);
    let alpha = if tokens.is_dark { 0.40 } else { 0.20 };
    vec![box_shadow(
        px(0.0),
        px(0.0),
        px(0.0),
        px(2.0),
        tokens.destructive.opacity(alpha),
    )]
}

/// Builds the subtle `shadow-sm ring-1 ring-foreground/5` elevation used on `Card`.
pub fn card_surface_shadow(theme: &SemanticTheme) -> Vec<BoxShadow> {
    let tokens = PresetTokens::from_theme(theme);
    let ring_color = tokens.foreground.opacity(if tokens.is_dark { 0.08 } else { 0.05 });
    let drop_color = hsla(0.0, 0.0, 0.0, if tokens.is_dark { 0.22 } else { 0.06 });
    vec![
        box_shadow(px(0.0), px(0.0), px(0.0), px(1.0), ring_color),
        box_shadow(px(0.0), px(1.0), px(2.0), px(0.0), drop_color),
    ]
}

/// Builds the `shadow-md ring-1 ring-foreground/10` elevation used on `Popover`,
/// `SelectContent`, `DropdownMenuContent`, `DialogContent`, and `TooltipContent`.
pub fn overlay_surface_shadow(theme: &SemanticTheme) -> Vec<BoxShadow> {
    let tokens = PresetTokens::from_theme(theme);
    let ring_color = tokens.foreground.opacity(0.10);
    let drop_color = hsla(0.0, 0.0, 0.0, if tokens.is_dark { 0.36 } else { 0.12 });
    vec![
        box_shadow(px(0.0), px(0.0), px(0.0), px(1.0), ring_color),
        box_shadow(px(0.0), px(4.0), px(6.0), px(-1.0), drop_color),
    ]
}

/// Fluent style composition trait (Rust GPUI counterpart to `cn(...)` in `web/src/lib/utils.ts`).
pub trait StyleComposeExt: Styled + Sized {
    /// Conditionally refines an element's style when `condition` is `true`.
    #[inline]
    fn cx_when(self, condition: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if condition { f(self) } else { self }
    }

    /// Conditionally refines an element's style when `option` is `Some(val)`.
    #[inline]
    fn cx_when_some<T>(self, option: Option<T>, f: impl FnOnce(Self, T) -> Self) -> Self {
        match option {
            Some(val) => f(self, val),
            None => self,
        }
    }

    /// Applies `--preset b1oaDsAa` sharp zero-radius corners (`rounded-none`) and `Archivo` font.
    #[inline]
    fn preset_surface(self) -> Self {
        self.rounded(PresetRadius::NONE).font_family(FONT_FAMILY)
    }

    /// Applies `--preset b1oaDsAa` editorial uppercase micro-typography (`text-xs font-semibold`).
    #[inline]
    fn preset_uppercase_xs(self) -> Self {
        self.font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
    }

    /// Applies disabled opacity (`disabled:opacity-50`) when `disabled` is `true`.
    #[inline]
    fn preset_disabled(self, disabled: bool) -> Self {
        if disabled {
            self.opacity(0.5).cursor_default()
        } else {
            self
        }
    }

    /// Applies focus or invalid ring styling based on `focused` and `invalid` flags.
    fn preset_ring(self, focused: bool, invalid: bool, theme: &SemanticTheme) -> Self {
        let tokens = PresetTokens::from_theme(theme);
        if invalid {
            self.border_color(tokens.destructive)
                .shadow(invalid_ring_shadow(theme))
        } else if focused {
            self.border_color(tokens.ring)
                .shadow(focus_ring_shadow(theme))
        } else {
            self
        }
    }
}

impl<E: Styled + Sized> StyleComposeExt for E {}

/// Converts text to uppercase matching Tailwind's `uppercase` utility on `base-sera`
/// buttons, labels, badges, and card headers.
#[inline]
pub fn to_preset_uppercase(text: &str) -> String {
    text.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{ThemeConfig, ThemeMode};

    #[test]
    fn preset_radius_is_zero_except_full() {
        assert_eq!(PresetRadius::NONE, px(0.0));
        assert_eq!(PresetRadius::SM, px(0.0));
        assert_eq!(PresetRadius::MD, px(0.0));
        assert_eq!(PresetRadius::LG, px(0.0));
        assert_eq!(PresetRadius::FULL, px(9999.0));
    }

    #[test]
    fn preset_tokens_resolve_light_and_dark_palettes() {
        let cfg = ThemeConfig::default();
        let light = SemanticTheme::from_config(&cfg, ThemeMode::Light);
        let dark = SemanticTheme::from_config(&cfg, ThemeMode::Dark);

        let lt = PresetTokens::from_theme(&light);
        let dt = PresetTokens::from_theme(&dark);

        assert!(!lt.is_dark);
        assert!(dt.is_dark);
        assert_eq!(lt.background, light.bg_canvas);
        assert_eq!(dt.background, dark.bg_canvas);
        assert_eq!(lt.primary, light.primary);
        assert_eq!(dt.primary, dark.primary);
        assert_eq!(focus_ring_shadow(&light).len(), 1);
        assert_eq!(overlay_surface_shadow(&dark).len(), 2);
    }
}

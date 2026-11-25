//! Progress bar primitives ported from `web/src/components/ui/progress.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Progress`: `flex flex-wrap gap-3` (`12px`), renders header children (`ProgressLabel` / `ProgressValue`) above `ProgressTrack`
//! - `ProgressTrack`: `relative flex h-0.5` (`2px`) `w-full overflow-hidden rounded-none bg-muted`
//! - `ProgressIndicator`: `h-full bg-primary` (`#ba181b`)
//! - `ProgressLabel`: `text-xs font-semibold uppercase`
//! - `ProgressValue`: `ml-auto text-sm text-muted-foreground`.

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::progress::{Progress as KitProgress, ProgressCircle};
use gpui_kit::{
    AnyElement, App, FontWeight, Hsla, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, relative,
};

use crate::common::utils::{PresetRadius, PresetTokens, to_preset_uppercase};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

fn resolve_theme(explicit: Option<&SemanticTheme>, cx: &App) -> SemanticTheme {
    explicit.cloned().unwrap_or_else(|| {
        SemanticTheme::from_config(
            &ThemeConfig::default(),
            if gpui_kit::component::Theme::global(cx).mode.is_dark() {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        )
    })
}

/// Determinate progress bar matching `Progress` in `web/src/components/ui/progress.tsx`.
#[derive(IntoElement)]
pub struct Progress {
    percentage: f32,
    indicator_color: Option<Hsla>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl Progress {
    /// Creates a progress bar with `percentage` in `0.0..=100.0`.
    pub fn new(percentage: f32) -> Self {
        Self {
            percentage: if percentage.is_finite() {
                percentage.clamp(0.0, 100.0)
            } else {
                0.0
            },
            indicator_color: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Computes exact integer ratio `(numerator / denominator) * 100` safely for RWF project goals.
    pub fn from_ratio(numerator: i64, denominator: i64) -> Self {
        if denominator <= 0 || numerator <= 0 {
            return Self::new(0.0);
        }
        let basis_points = ((numerator as i128 * 10_000) / denominator as i128).clamp(0, 10_000);
        Self::new(basis_points as f32 / 100.0)
    }

    pub fn value(mut self, percentage: f32) -> Self {
        if percentage.is_finite() {
            self.percentage = percentage.clamp(0.0, 100.0);
        }
        self
    }

    pub fn indicator_color(mut self, color: impl Into<Hsla>) -> Self {
        self.indicator_color = Some(color.into());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Progress {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Progress {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut indicator = ProgressIndicator::new(self.percentage).theme(&theme);
        if let Some(color) = self.indicator_color {
            indicator = indicator.color(color);
        }

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(12.0)) // gap-3
            .refine_style(&self.style);

        if !self.children.is_empty() {
            root = root.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .w_full()
                    .gap(px(8.0))
                    .children(self.children),
            );
        }

        root.child(ProgressTrack::new().theme(&theme).child(indicator))
    }
}

/// Hairline `2px` (`h-0.5`) progress trough (`ProgressTrack` in `progress.tsx`).
#[derive(IntoElement)]
pub struct ProgressTrack {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for ProgressTrack {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressTrack {
    pub fn new() -> Self {
        Self {
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ProgressTrack {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ProgressTrack {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ProgressTrack {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let track_bg = if tokens.is_dark {
            theme.bg_surface
        } else {
            tokens.secondary
        };

        div()
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .h(px(2.0)) // h-0.5
            .w_full()
            .overflow_hidden()
            .rounded(PresetRadius::NONE)
            .bg(track_bg)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Filled bar inside [`ProgressTrack`] (`ProgressIndicator` in `progress.tsx`).
#[derive(IntoElement)]
pub struct ProgressIndicator {
    percentage: f32,
    color: Option<Hsla>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ProgressIndicator {
    pub fn new(percentage: f32) -> Self {
        Self {
            percentage: percentage.clamp(0.0, 100.0),
            color: None,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ProgressIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ProgressIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let fill = self.color.unwrap_or(tokens.primary);

        div()
            .h_full()
            .w(relative(self.percentage / 100.0))
            .bg(fill)
            .refine_style(&self.style)
    }
}

/// Uppercase label inside [`Progress`] (`ProgressLabel` in `progress.tsx`).
#[derive(IntoElement)]
pub struct ProgressLabel {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ProgressLabel {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ProgressLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ProgressLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground)
            .refine_style(&self.style)
            .child(to_preset_uppercase(self.text.as_ref()))
    }
}

/// Right-aligned tabular percentage/value readout (`ProgressValue` in `progress.tsx`).
#[derive(IntoElement)]
pub struct ProgressValue {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ProgressValue {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ProgressValue {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ProgressValue {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .ml_auto()
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .child(self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_ratio_clamps_cleanly() {
        assert_eq!(Progress::from_ratio(50_000, 100_000).percentage, 50.0);
        assert_eq!(Progress::from_ratio(150_000, 100_000).percentage, 100.0);
        assert_eq!(Progress::from_ratio(0, 0).percentage, 0.0);
    }
}

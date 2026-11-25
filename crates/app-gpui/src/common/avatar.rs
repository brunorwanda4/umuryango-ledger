//! Avatar, badge, and stacked group primitives ported from `web/src/components/ui/avatar.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Avatar`: circular `rounded-full` (`Sm` `24px`, `Default` `32px`, `Lg` `40px`) with subtle `border-border`
//! - `AvatarImage`, `AvatarFallback` (`initials_from_name`), `AvatarBadge`, `AvatarGroup`, `AvatarGroupCount`.

pub use gpui_kit::component::avatar::{Avatar as KitAvatar, AvatarGroup as KitAvatarGroup};
use gpui_kit::base::box_shadow;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, FontWeight, ImageSource, IntoElement, ObjectFit, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, StyledImage as _, Window, div, img, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Avatar dimensions matching `size = "default" | "sm" | "lg"` in `avatar.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    Sm, // 24px (`size-6`)
    #[default]
    Default, // 32px (`size-8`)
    Lg, // 40px (`size-10`)
}

impl AvatarSize {
    pub const fn px(self) -> f32 {
        match self {
            Self::Sm => 24.0,
            Self::Default => 32.0,
            Self::Lg => 40.0,
        }
    }

    pub const fn badge_px(self) -> f32 {
        match self {
            Self::Sm => 8.0,      // size-2
            Self::Default => 10.0, // size-2.5
            Self::Lg => 12.0,     // size-3
        }
    }
}

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

/// Extracts up to 2 uppercase initials from a person or wallet name (e.g. `"Mama Uwase"` -> `"MU"`).
pub fn initials_from_name(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().filter(|s| !s.is_empty()).collect();
    match parts.as_slice() {
        [] => "?".to_string(),
        [single] => single
            .chars()
            .take(2)
            .collect::<String>()
            .to_uppercase(),
        [first, .., last] => {
            let mut out = String::with_capacity(2);
            if let Some(c) = first.chars().next() {
                out.extend(c.to_uppercase());
            }
            if let Some(c) = last.chars().next() {
                out.extend(c.to_uppercase());
            }
            out
        }
    }
}

/// Circular profile avatar root (`Avatar` in `web/src/components/ui/avatar.tsx`).
#[derive(IntoElement)]
pub struct Avatar {
    size: AvatarSize,
    badge: Option<AvatarBadge>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for Avatar {
    fn default() -> Self {
        Self::new()
    }
}

impl Avatar {
    pub fn new() -> Self {
        Self {
            size: AvatarSize::Default,
            badge: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Convenience constructor rendering initials fallback from `full_name`.
    pub fn from_name(full_name: &str, theme: &SemanticTheme) -> Self {
        Self::new()
            .theme(theme)
            .child(AvatarFallback::new(initials_from_name(full_name)).theme(theme))
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn badge(mut self, badge: AvatarBadge) -> Self {
        self.badge = Some(badge);
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Avatar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Avatar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let dim = px(self.size.px());

        let mut root = div()
            .relative()
            .flex()
            .flex_shrink_0()
            .size(dim)
            .rounded(PresetRadius::FULL)
            .border_1()
            .border_color(tokens.border)
            .refine_style(&self.style)
            .children(self.children);

        if let Some(badge) = self.badge {
            root = root.child(badge.parent_size(self.size).theme(&theme));
        }

        root
    }
}

/// Circular cropped image inside [`Avatar`] (`AvatarImage` in `avatar.tsx`).
#[derive(IntoElement)]
pub struct AvatarImage {
    source: ImageSource,
    style: StyleRefinement,
}

impl AvatarImage {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        Self {
            source: source.into(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for AvatarImage {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AvatarImage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        img(self.source)
            .size_full()
            .rounded(PresetRadius::FULL)
            .object_fit(ObjectFit::Cover)
            .refine_style(&self.style)
    }
}

/// Initials or placeholder inside [`Avatar`] (`AvatarFallback` in `avatar.tsx`).
#[derive(IntoElement)]
pub struct AvatarFallback {
    text: SharedString,
    size: AvatarSize,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl AvatarFallback {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            size: AvatarSize::Default,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for AvatarFallback {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AvatarFallback {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let bg = if tokens.is_dark {
            theme.bg_surface
        } else {
            tokens.secondary
        };

        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .rounded(PresetRadius::FULL)
            .bg(bg)
            .font_family(FONT_FAMILY)
            .font_weight(FontWeight::SEMIBOLD)
            .cx_when(matches!(self.size, AvatarSize::Sm), |d| d.text_xs())
            .cx_when(!matches!(self.size, AvatarSize::Sm), |d| d.text_sm())
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Bottom-right status dot on [`Avatar`] (`AvatarBadge` in `avatar.tsx`).
#[derive(IntoElement)]
pub struct AvatarBadge {
    parent_size: AvatarSize,
    color: Option<gpui_kit::Hsla>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl Default for AvatarBadge {
    fn default() -> Self {
        Self::new()
    }
}

impl AvatarBadge {
    pub fn new() -> Self {
        Self {
            parent_size: AvatarSize::Default,
            color: None,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn parent_size(mut self, size: AvatarSize) -> Self {
        self.parent_size = size;
        self
    }

    pub fn color(mut self, color: impl Into<gpui_kit::Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for AvatarBadge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AvatarBadge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let fill = self.color.unwrap_or(tokens.primary);
        let sz = px(self.parent_size.badge_px());

        div()
            .absolute()
            .right_0()
            .bottom_0()
            .size(sz)
            .rounded(PresetRadius::FULL)
            .bg(fill)
            .shadow(vec![box_shadow(
                px(0.0),
                px(0.0),
                px(0.0),
                px(2.0),
                tokens.background,
            )])
            .refine_style(&self.style)
    }
}

/// Overlapping horizontal row of avatars (`AvatarGroup` in `avatar.tsx`).
#[derive(IntoElement)]
pub struct AvatarGroup {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for AvatarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl AvatarGroup {
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

impl Styled for AvatarGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AvatarGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AvatarGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let ring = vec![box_shadow(
            px(0.0),
            px(0.0),
            px(0.0),
            px(2.0),
            tokens.background,
        )];

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .refine_style(&self.style);

        for (idx, child) in self.children.into_iter().enumerate() {
            row = row.child(
                div()
                    .rounded(PresetRadius::FULL)
                    .shadow(ring.clone())
                    .cx_when(idx > 0, |d| d.ml(px(-8.0)))
                    .child(child),
            );
        }
        row
    }
}

/// Overflow counter bubble inside [`AvatarGroup`] (`AvatarGroupCount` in `avatar.tsx`).
#[derive(IntoElement)]
pub struct AvatarGroupCount {
    count: usize,
    size: AvatarSize,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl AvatarGroupCount {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            size: AvatarSize::Default,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for AvatarGroupCount {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AvatarGroupCount {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let bg = if tokens.is_dark {
            theme.bg_surface
        } else {
            tokens.secondary
        };

        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .size(px(self.size.px()))
            .rounded(PresetRadius::FULL)
            .bg(bg)
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .child(format!("+{}", self.count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initials_extracted_properly() {
        assert_eq!(initials_from_name("Mama Uwase"), "MU");
        assert_eq!(initials_from_name("Bruno"), "BR");
        assert_eq!(initials_from_name(""), "?");
    }
}

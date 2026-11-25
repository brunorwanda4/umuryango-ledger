//! Section & timeline marker primitives ported from `web/src/components/ui/marker.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Marker`: `relative flex min-h-4 w-full items-center gap-2 text-xs text-muted-foreground uppercase`
//!   - `MarkerVariant::Default`: inline marker
//!   - `MarkerVariant::Separator`: centered marker flanked by `1px` `bg-border` hairlines
//!   - `MarkerVariant::Border`: `border-b border-border pb-2`
//! - `MarkerIcon` (`16px` `app_icon`), `MarkerContent`.

pub use gpui_kit::component::marker::Marker as KitMarker;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Visual rule style matching `markerVariants.variant` in `web/src/components/ui/marker.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerVariant {
    #[default]
    Default,
    Separator,
    Border,
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

/// Editorial uppercase timeline/date/section divider (`Marker` in `marker.tsx`).
#[derive(IntoElement)]
pub struct Marker {
    variant: MarkerVariant,
    icon: Option<AppIcon>,
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for Marker {
    fn default() -> Self {
        Self::new()
    }
}

impl Marker {
    pub fn new() -> Self {
        Self {
            variant: MarkerVariant::Default,
            icon: None,
            text: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn from_label(label: impl Into<SharedString>) -> Self {
        Self::new().text(label)
    }

    pub fn variant(mut self, variant: MarkerVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn separator(self) -> Self {
        self.variant(MarkerVariant::Separator)
    }

    pub fn border(self) -> Self {
        self.variant(MarkerVariant::Border)
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Marker {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Marker {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Marker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_sep = matches!(self.variant, MarkerVariant::Separator);

        let mut row = div()
            .relative()
            .flex()
            .flex_row()
            .min_h(px(16.0))
            .w_full()
            .items_center()
            .gap(px(8.0))
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.muted_foreground)
            .cx_when(matches!(self.variant, MarkerVariant::Border), |d| {
                d.border_b_1().border_color(tokens.border).pb(px(8.0))
            })
            .refine_style(&self.style);

        if is_sep {
            row = row.child(
                div()
                    .mr(px(4.0))
                    .h(px(1.0))
                    .min_w_0()
                    .flex_1()
                    .bg(tokens.border),
            );
        }

        if let Some(icon) = self.icon {
            row = row.child(MarkerIcon::new(icon).theme(&theme));
        }
        if let Some(text) = self.text {
            row = row.child(MarkerContent::new(text).centered(is_sep));
        }
        row = row.children(self.children);

        if is_sep {
            row = row.child(
                div()
                    .ml(px(4.0))
                    .h(px(1.0))
                    .min_w_0()
                    .flex_1()
                    .bg(tokens.border),
            );
        }

        row
    }
}

/// `16x16px` icon inside [`Marker`] (`MarkerIcon` in `marker.tsx`).
#[derive(IntoElement)]
pub struct MarkerIcon {
    icon: AppIcon,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl MarkerIcon {
    pub fn new(icon: AppIcon) -> Self {
        Self {
            icon,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for MarkerIcon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MarkerIcon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        div()
            .size(px(16.0))
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_center()
            .refine_style(&self.style)
            .child(app_icon(self.icon, IconSize::Md, IconTone::Muted, &theme))
    }
}

/// Uppercase label span inside [`Marker`] (`MarkerContent` in `marker.tsx`).
#[derive(IntoElement)]
pub struct MarkerContent {
    text: SharedString,
    centered: bool,
    style: StyleRefinement,
}

impl MarkerContent {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            centered: false,
            style: StyleRefinement::default(),
        }
    }

    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }
}

impl Styled for MarkerContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MarkerContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .min_w_0()
            .cx_when(self.centered, |d| d.flex_none().text_center())
            .refine_style(&self.style)
            .child(to_preset_uppercase(self.text.as_ref()))
    }
}

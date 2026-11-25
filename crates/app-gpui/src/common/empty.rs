//! Empty state placeholder primitives ported from `web/src/components/ui/empty.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Empty`: `flex w-full flex-1 flex-col items-center justify-center gap-4 border-dashed p-12 text-center`
//! - `EmptyHeader`, `EmptyMedia` (`Default` | `Icon` `40x40px` `rounded-none bg-muted` with `app_icon`),
//!   `EmptyTitle` (`text-lg font-semibold uppercase`), `EmptyDescription`, `EmptyContent`.

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::empty::Empty as KitEmpty;
use gpui_kit::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, relative,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Visual container style for [`EmptyMedia`] matching `emptyMediaVariants.variant` in `empty.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
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

/// Centered zero-data placeholder container (`Empty` in `web/src/components/ui/empty.tsx`).
#[derive(IntoElement)]
pub struct Empty {
    bordered: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

impl Empty {
    pub fn new() -> Self {
        Self {
            bordered: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Empty {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Empty {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Empty {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .flex_1()
            .flex_col()
            .w_full()
            .min_w_0()
            .items_center()
            .justify_center()
            .gap(px(16.0)) // gap-4
            .p(px(48.0)) // p-12
            .rounded(PresetRadius::NONE)
            .text_center()
            .cx_when(self.bordered, |d| {
                d.border_1().border_dashed().border_color(tokens.border)
            })
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Header block inside [`Empty`] (`EmptyHeader` in `empty.tsx`).
#[derive(IntoElement)]
pub struct EmptyHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for EmptyHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl EmptyHeader {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for EmptyHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for EmptyHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for EmptyHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .max_w(px(384.0)) // max-w-sm
            .items_center()
            .gap(px(8.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Icon or illustration box inside [`EmptyHeader`] (`EmptyMedia` in `empty.tsx`).
#[derive(IntoElement)]
pub struct EmptyMedia {
    variant: EmptyMediaVariant,
    icon: Option<AppIcon>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for EmptyMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl EmptyMedia {
    pub fn new() -> Self {
        Self {
            variant: EmptyMediaVariant::Default,
            icon: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Creates a `40x40px` square muted icon badge (`variant="icon"`).
    pub fn from_icon(icon: AppIcon) -> Self {
        Self::new().variant(EmptyMediaVariant::Icon).icon(icon)
    }

    pub fn variant(mut self, variant: EmptyMediaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for EmptyMedia {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for EmptyMedia {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for EmptyMedia {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let box_bg = if tokens.is_dark {
            theme.bg_surface
        } else {
            tokens.secondary
        };

        let mut base = div()
            .mb(px(8.0))
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .rounded(PresetRadius::NONE)
            .cx_when(matches!(self.variant, EmptyMediaVariant::Icon), |d| {
                d.size(px(40.0)).bg(box_bg)
            })
            .refine_style(&self.style);

        if let Some(icon) = self.icon {
            base = base.child(app_icon(icon, IconSize::Lg, IconTone::Default, &theme));
        }

        base.children(self.children)
    }
}

/// Editorial uppercase heading inside [`EmptyHeader`] (`EmptyTitle` in `empty.tsx`).
#[derive(IntoElement)]
pub struct EmptyTitle {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl EmptyTitle {
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

impl Styled for EmptyTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for EmptyTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .font_family(FONT_FAMILY)
            .text_lg()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground)
            .refine_style(&self.style)
            .child(to_preset_uppercase(self.text.as_ref()))
    }
}

/// Muted body copy inside [`EmptyHeader`] (`EmptyDescription` in `empty.tsx`).
#[derive(IntoElement)]
pub struct EmptyDescription {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl EmptyDescription {
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

impl Styled for EmptyDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for EmptyDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .mt(px(2.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .line_height(relative(1.625))
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Action column inside [`Empty`] (`EmptyContent` in `empty.tsx`).
#[derive(IntoElement)]
pub struct EmptyContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for EmptyContent {
    fn default() -> Self {
        Self::new()
    }
}

impl EmptyContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for EmptyContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for EmptyContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for EmptyContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(px(384.0))
            .min_w_0()
            .items_center()
            .gap(px(16.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .refine_style(&self.style)
            .children(self.children)
    }
}

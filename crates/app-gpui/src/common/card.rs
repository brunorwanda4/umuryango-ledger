//! Card container and sub-slot primitives ported from `web/src/components/ui/card.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Card`: `flex flex-col gap-8 py-8 bg-card text-sm text-card-foreground shadow-sm ring-1 ring-foreground/5`
//!   (`CardSize::Sm` switches spacing from `32px` to `20px`), `rounded-none`
//! - `CardHeader`, `CardTitle` (`text-lg font-semibold uppercase`), `CardDescription`,
//!   `CardAction`, `CardContent`, `CardFooter`.

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, Pixels, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, relative,
};

use crate::common::utils::{
    PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt, card_surface_shadow,
    to_preset_uppercase,
};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Spacing density matching `data-size="default" | "sm"` in `card.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardSize {
    #[default]
    Default,
    Sm,
}

impl CardSize {
    pub const fn spacing_px(self) -> Pixels {
        match self {
            Self::Default => PresetSpacing::CARD_DEFAULT, // 32px
            Self::Sm => PresetSpacing::CARD_SM,           // 20px
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

/// Elevated surface container matching `Card` in `web/src/components/ui/card.tsx`.
#[derive(IntoElement)]
pub struct Card {
    size: CardSize,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    pub fn sm(self) -> Self {
        self.size(CardSize::Sm)
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let pad = self.size.spacing_px();

        div()
            .flex()
            .flex_col()
            .gap(pad)
            .py(pad)
            .rounded(PresetRadius::NONE)
            .overflow_hidden()
            .bg(tokens.card)
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.card_foreground)
            .shadow(card_surface_shadow(&theme))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Top section of a [`Card`] (`CardHeader` in `card.tsx`).
#[derive(IntoElement)]
pub struct CardHeader {
    size: CardSize,
    bordered: bool,
    action: Option<AnyElement>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for CardHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl CardHeader {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            bordered: false,
            action: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for CardHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let pad = self.size.spacing_px();

        let body = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .children(self.children);

        div()
            .flex()
            .flex_row()
            .items_start()
            .justify_between()
            .gap(px(12.0))
            .px(pad)
            .rounded(PresetRadius::NONE)
            .cx_when(self.bordered, |d| {
                d.pb(pad).border_b_1().border_color(tokens.border)
            })
            .refine_style(&self.style)
            .child(body)
            .cx_when_some(self.action, |d, act| d.child(act))
    }
}

/// Editorial uppercase heading inside [`CardHeader`] (`CardTitle` in `card.tsx`).
#[derive(IntoElement)]
pub struct CardTitle {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl CardTitle {
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

impl Styled for CardTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CardTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .font_family(FONT_FAMILY)
            .text_lg()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.card_foreground)
            .refine_style(&self.style)
            .child(to_preset_uppercase(self.text.as_ref()))
    }
}

/// Muted supporting text inside [`CardHeader`] (`CardDescription` in `card.tsx`).
#[derive(IntoElement)]
pub struct CardDescription {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl CardDescription {
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

impl Styled for CardDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CardDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .font_family(FONT_FAMILY)
            .text_sm()
            .line_height(relative(1.625))
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Top-right action slot inside [`CardHeader`] (`CardAction` in `card.tsx`).
#[derive(IntoElement)]
pub struct CardAction {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for CardAction {
    fn default() -> Self {
        Self::new()
    }
}

impl CardAction {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for CardAction {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardAction {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardAction {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_end()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Main body container inside [`Card`] (`CardContent` in `card.tsx`).
#[derive(IntoElement)]
pub struct CardContent {
    size: CardSize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for CardContent {
    fn default() -> Self {
        Self::new()
    }
}

impl CardContent {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }
}

impl Styled for CardContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .px(self.size.spacing_px())
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Bottom action bar inside [`Card`] (`CardFooter` in `card.tsx`).
#[derive(IntoElement)]
pub struct CardFooter {
    size: CardSize,
    bordered: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for CardFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl CardFooter {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            bordered: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
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

impl Styled for CardFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let pad = self.size.spacing_px();

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .px(pad)
            .cx_when(self.bordered, |d| {
                d.pt(pad).border_t_1().border_color(tokens.border)
            })
            .refine_style(&self.style)
            .children(self.children)
    }
}

//! Inline callout alert primitives ported from `web/src/components/ui/alert.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Alert`: `relative flex w-full border bg-card px-4 py-3 text-sm rounded-none` with a
//!   signature `2px` (`w-0.5`) left accent stripe (`bg-foreground` on `Default`, `bg-destructive` on `Destructive`)
//! - `AlertTitle` (`text-sm font-semibold`), `AlertDescription` (`text-sm text-muted-foreground`),
//!   `AlertAction` (`absolute top-2.5 right-3`).

pub use gpui_kit::component::alert::Alert as KitAlert;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Visual intent matching `alertVariants.variant` in `web/src/components/ui/alert.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
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

/// Inline status banner matching `Alert` in `web/src/components/ui/alert.tsx`.
#[derive(IntoElement)]
pub struct Alert {
    variant: AlertVariant,
    icon: Option<AppIcon>,
    action: Option<AnyElement>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl Alert {
    pub fn new() -> Self {
        Self {
            variant: AlertVariant::Default,
            icon: None,
            action: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn destructive(self) -> Self {
        self.variant(AlertVariant::Destructive)
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
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

impl Styled for Alert {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Alert {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Alert {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let (fg, stripe, icon_tone) = match self.variant {
            AlertVariant::Default => (tokens.card_foreground, tokens.foreground, IconTone::Default),
            AlertVariant::Destructive => (tokens.destructive, tokens.destructive, IconTone::Expense),
        };

        let has_action = self.action.is_some();

        let mut body = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .children(self.children);

        if has_action {
            body = body.pr(px(72.0)); // has-data-[slot=alert-action]:pr-18
        }

        let mut root = div()
            .relative()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(10.0)) // gap-x-2.5
            .w_full()
            .px(px(16.0)) // px-4
            .py(px(12.0)) // py-3
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(tokens.border)
            .bg(tokens.card)
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(fg)
            .refine_style(&self.style)
            // Left 2px accent bar (`after:absolute after:-inset-y-px after:-left-px after:w-0.5`)
            .child(
                div()
                    .absolute()
                    .top(px(-1.0))
                    .bottom(px(-1.0))
                    .left(px(-1.0))
                    .w(px(2.0))
                    .bg(stripe),
            );

        if let Some(icon) = self.icon {
            root = root.child(
                div()
                    .pt(px(2.0))
                    .flex_shrink_0()
                    .child(app_icon(icon, IconSize::Md, icon_tone, &theme)),
            );
        }

        root = root.child(body);

        if let Some(action) = self.action {
            root = root.child(AlertAction::new().child(action));
        }

        root
    }
}

/// Primary heading inside [`Alert`] (`AlertTitle` in `alert.tsx`).
#[derive(IntoElement)]
pub struct AlertTitle {
    text: SharedString,
    style: StyleRefinement,
}

impl AlertTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for AlertTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AlertTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .font_family(FONT_FAMILY)
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Supporting detail inside [`Alert`] (`AlertDescription` in `alert.tsx`).
#[derive(IntoElement)]
pub struct AlertDescription {
    text: SharedString,
    destructive: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl AlertDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            destructive: false,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for AlertDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AlertDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let fg = if self.destructive {
            tokens.destructive.opacity(0.90)
        } else {
            tokens.muted_foreground
        };
        div()
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(fg)
            .cx_when(self.destructive, |d| d.text_color(fg))
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Top-right action slot inside [`Alert`] (`AlertAction` in `alert.tsx`).
#[derive(IntoElement)]
pub struct AlertAction {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for AlertAction {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertAction {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for AlertAction {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AlertAction {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AlertAction {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .absolute()
            .top(px(10.0)) // top-2.5
            .right(px(12.0)) // right-3
            .flex()
            .items_center()
            .refine_style(&self.style)
            .children(self.children)
    }
}

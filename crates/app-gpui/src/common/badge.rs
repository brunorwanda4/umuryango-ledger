//! Badge primitive ported from `web/src/components/ui/badge.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - Base: `inline-flex w-fit shrink-0 items-center justify-center gap-1.5 overflow-hidden rounded-none border-0 bg-transparent px-0 py-0 text-[0.625rem] font-semibold tracking-widest whitespace-nowrap uppercase`
//! - Variants: `Default`, `Secondary`, `Destructive`, `Outline`, `Ghost`, `Link`
//! - Leading/trailing vector icons rendered via `crate::components::icon::app_icon` (`IconSize::Xs` = `12px`).

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::badge::Badge as OverlayCountBadge;
use gpui_kit::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme};

/// Visual variants matching `badgeVariants` in `web/src/components/ui/badge.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

/// Editorial micro-badge matching `Badge` in `web/src/components/ui/badge.tsx`.
#[derive(IntoElement)]
pub struct Badge {
    id: Option<ElementId>,
    label: Option<SharedString>,
    variant: BadgeVariant,
    icon_start: Option<AppIcon>,
    icon_end: Option<AppIcon>,
    focused: bool,
    invalid: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Box<dyn Fn(&gpui_kit::ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Badge {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            id: None,
            label: Some(label.into()),
            variant: BadgeVariant::Default,
            icon_start: None,
            icon_end: None,
            focused: false,
            invalid: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            id: None,
            label: None,
            variant: BadgeVariant::Default,
            icon_start: None,
            icon_end: None,
            focused: false,
            invalid: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn secondary(self) -> Self {
        self.variant(BadgeVariant::Secondary)
    }

    pub fn destructive(self) -> Self {
        self.variant(BadgeVariant::Destructive)
    }

    pub fn outline(self) -> Self {
        self.variant(BadgeVariant::Outline)
    }

    pub fn ghost(self) -> Self {
        self.variant(BadgeVariant::Ghost)
    }

    pub fn link(self) -> Self {
        self.variant(BadgeVariant::Link)
    }

    pub fn icon_start(mut self, icon: AppIcon) -> Self {
        self.icon_start = Some(icon);
        self
    }

    pub fn icon_end(mut self, icon: AppIcon) -> Self {
        self.icon_end = Some(icon);
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Styled for Badge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Badge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let fallback_theme = crate::theme::SemanticTheme::from_config(
            &crate::theme::ThemeConfig::default(),
            if gpui_kit::component::Theme::global(cx).mode.is_dark() {
                crate::theme::ThemeMode::Dark
            } else {
                crate::theme::ThemeMode::Light
            },
        );
        let theme = self.theme.as_ref().unwrap_or(&fallback_theme);
        let tokens = PresetTokens::from_theme(theme);

        let (fg, hover_fg, icon_tone) = match self.variant {
            BadgeVariant::Default | BadgeVariant::Outline => (
                tokens.foreground,
                tokens.foreground.opacity(0.70),
                IconTone::Default,
            ),
            BadgeVariant::Secondary | BadgeVariant::Ghost => {
                (tokens.muted_foreground, tokens.foreground, IconTone::Muted)
            }
            BadgeVariant::Destructive => (
                tokens.destructive,
                tokens.destructive.opacity(0.70),
                IconTone::Expense,
            ),
            BadgeVariant::Link => (tokens.foreground, tokens.primary, IconTone::Default),
        };

        let mut base = div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .gap(px(6.0)) // gap-1.5
            .px(px(0.0))
            .py(px(0.0))
            .rounded(PresetRadius::NONE)
            .overflow_hidden()
            .font_family(FONT_FAMILY)
            .text_size(px(10.0)) // text-[0.625rem]
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(fg)
            .preset_ring(self.focused, self.invalid, theme)
            .cx_when(matches!(self.variant, BadgeVariant::Link), |d| {
                d.hover(|s| s.underline().text_color(hover_fg))
            })
            .cx_when(matches!(self.variant, BadgeVariant::Ghost), |d| {
                d.hover(|s| s.text_color(hover_fg))
            })
            .refine_style(&self.style);

        if let Some(icon) = self.icon_start {
            base = base.child(app_icon(icon, IconSize::Xs, icon_tone, theme));
        }
        if let Some(label) = self.label {
            base = base.child(to_preset_uppercase(label.as_ref()));
        }
        base = base.children(self.children);
        if let Some(icon) = self.icon_end {
            base = base.child(app_icon(icon, IconSize::Xs, icon_tone, theme));
        }

        if let Some(id) = self.id {
            let mut interactive = base.id(id);
            if let Some(handler) = self.on_click {
                interactive = interactive
                    .cursor_pointer()
                    .hover(move |s| s.text_color(hover_fg))
                    .on_click(handler);
            }
            interactive.into_any_element()
        } else {
            base.into_any_element()
        }
    }
}

/// Convenience helper constructing a themed `--preset b1oaDsAa` [`Badge`].
pub fn badge(
    label: impl Into<SharedString>,
    variant: BadgeVariant,
    theme: &SemanticTheme,
) -> Badge {
    Badge::new(label).variant(variant).theme(theme)
}

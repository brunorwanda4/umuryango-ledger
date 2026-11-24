//! Button group primitives ported from `web/src/components/ui/button-group.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `ButtonGroup`: `flex w-fit items-stretch` (`Horizontal` | `Vertical`), sharp zero-radius edges
//! - `ButtonGroupText`: `flex items-center gap-2 border border-transparent border-b-input bg-transparent px-2.5 text-xs font-semibold uppercase`
//! - `ButtonGroupSeparator`: `Separator` (`Vertical` default, `self-stretch bg-input`).

pub use gpui_kit::component::button::ButtonGroup as KitButtonGroup;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::separator::{Separator, SeparatorOrientation};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Layout axis matching `buttonGroupVariants.orientation` in `button-group.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Contiguous group of buttons and controls matching `ButtonGroup` in `button-group.tsx`.
#[derive(IntoElement)]
pub struct ButtonGroup {
    orientation: ButtonGroupOrientation,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for ButtonGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self {
            orientation: ButtonGroupOrientation::Horizontal,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn horizontal() -> Self {
        Self::new().orientation(ButtonGroupOrientation::Horizontal)
    }

    pub fn vertical() -> Self {
        Self::new().orientation(ButtonGroupOrientation::Vertical)
    }

    pub fn orientation(mut self, orientation: ButtonGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl Styled for ButtonGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ButtonGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .cx_when(
                matches!(self.orientation, ButtonGroupOrientation::Horizontal),
                |d| d.flex_row(),
            )
            .cx_when(
                matches!(self.orientation, ButtonGroupOrientation::Vertical),
                |d| d.flex_col(),
            )
            .items_stretch()
            .rounded(PresetRadius::NONE)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Inline static label segment inside a [`ButtonGroup`] (`ButtonGroupText` in `button-group.tsx`).
#[derive(IntoElement)]
pub struct ButtonGroupText {
    text: Option<SharedString>,
    icon: Option<AppIcon>,
    outlined: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ButtonGroupText {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            icon: None,
            outlined: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Uses full border (`group-has-[>[data-variant=outline]]/button-group:border-border`).
    pub fn outlined(mut self, outlined: bool) -> Self {
        self.outlined = outlined;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ButtonGroupText {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ButtonGroupText {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ButtonGroupText {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let fallback_theme = SemanticTheme::from_config(
            &ThemeConfig::default(),
            if gpui_kit::component::Theme::global(cx).mode.is_dark() {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        );
        let theme = self.theme.as_ref().unwrap_or(&fallback_theme);
        let tokens = PresetTokens::from_theme(theme);

        let mut base = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .px(px(10.0)) // px-2.5
            .rounded(PresetRadius::NONE)
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground)
            .cx_when(self.outlined, |d| d.border_1().border_color(tokens.border))
            .cx_when(!self.outlined, |d| d.border_b_1().border_color(tokens.input))
            .refine_style(&self.style);

        if let Some(icon) = self.icon {
            base = base.child(app_icon(icon, IconSize::Sm, IconTone::Default, theme));
        }
        if let Some(text) = self.text {
            base = base.child(to_preset_uppercase(text.as_ref()));
        }
        base.children(self.children)
    }
}

/// Hairline divider between adjacent buttons (`ButtonGroupSeparator` in `button-group.tsx`).
#[derive(IntoElement)]
pub struct ButtonGroupSeparator {
    orientation: SeparatorOrientation,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl Default for ButtonGroupSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonGroupSeparator {
    pub fn new() -> Self {
        Self {
            orientation: SeparatorOrientation::Vertical,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ButtonGroupSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ButtonGroupSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut sep = Separator::new().orientation(self.orientation);
        if let Some(ref theme) = self.theme {
            sep = sep.color(PresetTokens::from_theme(theme).input);
        }
        sep.refine_style(&self.style)
    }
}

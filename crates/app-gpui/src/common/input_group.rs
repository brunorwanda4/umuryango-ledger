//! Input group and addon primitives ported from `web/src/components/ui/input-group.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `InputGroup`: `relative flex h-10 w-full min-w-0 items-center rounded-none border-b border-b-input`
//!   (auto-height column when `stacked(true)` or wrapping a `Textarea`)
//! - `InputGroupAddon` (`InlineStart`, `InlineEnd`, `BlockStart`, `BlockEnd`)
//! - `InputGroupButton` (`Xs`, `Sm`, `IconXs`, `IconSm`)
//! - `InputGroupText`, `InputGroupInput`, `InputGroupTextarea`.

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, Entity, FontWeight, IntoElement, ParentElement,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::input::{Input, InputState};
use crate::common::textarea::{Textarea, TextareaState};
use crate::common::utils::{PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Placement of an [`InputGroupAddon`] matching `inputGroupAddonVariants.align`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputGroupAddonAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

/// Compact button dimensions matching `inputGroupButtonVariants.size`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputGroupButtonSize {
    #[default]
    Xs,
    Sm,
    IconXs,
    IconSm,
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

/// Compound underline frame grouping inputs, addons, and inline actions (`InputGroup`).
#[derive(IntoElement)]
pub struct InputGroup {
    stacked: bool,
    focused: bool,
    invalid: bool,
    disabled: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for InputGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl InputGroup {
    pub fn new() -> Self {
        Self {
            stacked: false,
            focused: false,
            invalid: false,
            disabled: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Switches to multi-row vertical layout (`has-[>[data-align=block-start|block-end]]` or `Textarea`).
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.stacked = stacked;
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for InputGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for InputGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for InputGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let border_bottom = if self.invalid {
            tokens.destructive
        } else if self.focused {
            tokens.ring
        } else {
            tokens.input
        };

        div()
            .relative()
            .flex()
            .w_full()
            .min_w_0()
            .rounded(PresetRadius::NONE)
            .border_b_1()
            .border_color(border_bottom)
            .cx_when(!self.stacked, |d| {
                d.flex_row()
                    .items_center()
                    .h(PresetSpacing::CONTROL_DEFAULT)
                    .gap(px(8.0))
            })
            .cx_when(self.stacked, |d| d.flex_col())
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Prefix, suffix, header, or footer slot inside [`InputGroup`] (`InputGroupAddon`).
#[derive(IntoElement)]
pub struct InputGroupAddon {
    align: InputGroupAddonAlign,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for InputGroupAddon {
    fn default() -> Self {
        Self::new()
    }
}

impl InputGroupAddon {
    pub fn new() -> Self {
        Self {
            align: InputGroupAddonAlign::InlineStart,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn align(mut self, align: InputGroupAddonAlign) -> Self {
        self.align = align;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for InputGroupAddon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for InputGroupAddon {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for InputGroupAddon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_block = matches!(
            self.align,
            InputGroupAddonAlign::BlockStart | InputGroupAddonAlign::BlockEnd
        );

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.muted_foreground)
            .cx_when(!is_block, |d| d.justify_center().py(px(8.0)))
            .cx_when(matches!(self.align, InputGroupAddonAlign::BlockStart), |d| {
                d.w_full().justify_start().pt(px(12.0))
            })
            .cx_when(matches!(self.align, InputGroupAddonAlign::BlockEnd), |d| {
                d.w_full().justify_start().pb(px(12.0))
            })
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Compact inline action button inside [`InputGroup`] (`InputGroupButton`).
pub fn input_group_button(
    id: impl Into<ElementId>,
    size: InputGroupButtonSize,
    variant: ButtonVariant,
    theme: &SemanticTheme,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Button {
    let btn = Button::new(id)
        .variant(variant)
        .theme(theme)
        .on_click(on_click);

    match size {
        InputGroupButtonSize::Xs => btn.size(ButtonSize::Xs).h(px(24.0)).px(px(6.0)),
        InputGroupButtonSize::Sm => btn.size(ButtonSize::Sm),
        InputGroupButtonSize::IconXs => btn.size(ButtonSize::IconXs).w(px(24.0)).h(px(24.0)),
        InputGroupButtonSize::IconSm => btn.size(ButtonSize::IconSm).w(px(32.0)).h(px(32.0)),
    }
}

/// Static muted text or icon inside [`InputGroupAddon`] (`InputGroupText`).
#[derive(IntoElement)]
pub struct InputGroupText {
    text: Option<SharedString>,
    icon: Option<AppIcon>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl InputGroupText {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            icon: None,
            theme: None,
            style: StyleRefinement::default(),
        }
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

impl Styled for InputGroupText {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for InputGroupText {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut base = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style);

        if let Some(icon) = self.icon {
            base = base.child(app_icon(icon, IconSize::Sm, IconTone::Muted, &theme));
        }
        if let Some(text) = self.text {
            base = base.child(text);
        }
        base
    }
}

/// Borderless single-line input control slotted inside [`InputGroup`] (`InputGroupInput`).
pub fn input_group_input(state: &Entity<InputState>, theme: &SemanticTheme) -> Input {
    Input::new(state).borderless(true).theme(theme).flex_1()
}

/// Borderless multi-line textarea control slotted inside [`InputGroup`] (`InputGroupTextarea`).
pub fn input_group_textarea(state: &Entity<TextareaState>, theme: &SemanticTheme) -> Textarea {
    Textarea::new(state)
        .borderless(true)
        .theme(theme)
        .flex_1()
        .py(px(10.0))
}

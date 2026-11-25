//! Structured form field primitives ported from `web/src/components/ui/field.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `FieldSet` (`gap-6`), `FieldLegend` (`mb-3 text-xs font-semibold uppercase`), `FieldGroup` (`gap-10`)
//! - `Field` (`Vertical` | `Horizontal` | `Responsive` adapting via `crate::hooks::is_mobile`)
//! - `FieldContent`, `FieldLabel`, `FieldTitle`, `FieldDescription`, `FieldSeparator`, `FieldError`.

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px, relative,
};

use crate::common::label::Label;
use crate::common::separator::Separator;
use crate::common::utils::{PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::hooks::is_mobile;
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

/// Orientation of a [`Field`] matching `fieldVariants.orientation` in `field.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

/// Variant of [`FieldLegend`] matching `variant?: "legend" | "label"` in `field.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldLegendVariant {
    #[default]
    Legend,
    Label,
}

/// Semantic fieldset container (`FieldSet` in `field.tsx`).
#[derive(IntoElement)]
pub struct FieldSet {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for FieldSet {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldSet {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for FieldSet {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for FieldSet {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldSet {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(24.0)) // gap-6
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Uppercase section heading for [`FieldSet`] (`FieldLegend` in `field.tsx`).
#[derive(IntoElement)]
pub struct FieldLegend {
    text: SharedString,
    variant: FieldLegendVariant,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl FieldLegend {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            variant: FieldLegendVariant::Legend,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: FieldLegendVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for FieldLegend {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for FieldLegend {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let _ = self.variant;
        div()
            .mb(px(12.0)) // mb-3
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground)
            .refine_style(&self.style)
            .child(to_preset_uppercase(self.text.as_ref()))
    }
}

/// Vertical stack of form fields (`FieldGroup` in `field.tsx`).
#[derive(IntoElement)]
pub struct FieldGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for FieldGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldGroup {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for FieldGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for FieldGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(40.0)) // gap-10
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Individual form control group (`Field` in `field.tsx`).
#[derive(IntoElement)]
pub struct Field {
    orientation: FieldOrientation,
    invalid: bool,
    disabled: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

impl Field {
    pub fn new() -> Self {
        Self {
            orientation: FieldOrientation::Vertical,
            invalid: false,
            disabled: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn orientation(mut self, orientation: FieldOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn horizontal(self) -> Self {
        self.orientation(FieldOrientation::Horizontal)
    }

    pub fn responsive(self) -> Self {
        self.orientation(FieldOrientation::Responsive)
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

impl Styled for Field {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Field {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Field {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let use_row = match self.orientation {
            FieldOrientation::Vertical => false,
            FieldOrientation::Horizontal => true,
            FieldOrientation::Responsive => !is_mobile(window),
        };

        div()
            .flex()
            .w_full()
            .gap(px(12.0)) // gap-3
            .cx_when(!use_row, |d| d.flex_col())
            .cx_when(use_row, |d| d.flex_row().items_start())
            .cx_when(self.invalid, |d| d.text_color(tokens.destructive))
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Column holding title, description, and error beside a control (`FieldContent`).
#[derive(IntoElement)]
pub struct FieldContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for FieldContent {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for FieldContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for FieldContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_1()
            .flex_col()
            .gap(px(4.0))
            .line_height(relative(1.375))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Label bound to a [`Field`] (`FieldLabel` in `field.tsx`).
pub fn field_slot_label(text: impl Into<SharedString>, theme: &SemanticTheme) -> Label {
    Label::new(text).theme(theme)
}

/// Uppercase title inside [`FieldContent`] (`FieldTitle` in `field.tsx`).
#[derive(IntoElement)]
pub struct FieldTitle {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl FieldTitle {
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

impl Styled for FieldTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for FieldTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground)
            .refine_style(&self.style)
            .child(to_preset_uppercase(self.text.as_ref()))
    }
}

/// Helper text below a control (`FieldDescription` in `field.tsx`).
#[derive(IntoElement)]
pub struct FieldDescription {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl FieldDescription {
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

impl Styled for FieldDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for FieldDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .font_family(FONT_FAMILY)
            .text_sm()
            .font_weight(FontWeight::NORMAL)
            .line_height(relative(1.5))
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Labeled or plain horizontal divider between field groups (`FieldSeparator` in `field.tsx`).
#[derive(IntoElement)]
pub struct FieldSeparator {
    label: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl Default for FieldSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl FieldSeparator {
    pub fn new() -> Self {
        Self {
            label: None,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for FieldSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for FieldSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut sep = Separator::horizontal().theme(&theme);
        if let Some(lbl) = self.label {
            sep = sep.label(lbl);
        }
        div()
            .h(px(20.0))
            .flex()
            .items_center()
            .w_full()
            .refine_style(&self.style)
            .child(sep)
    }
}

/// Destructive validation message list (`FieldError` in `field.tsx`).
#[derive(IntoElement)]
pub struct FieldError {
    messages: Vec<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl FieldError {
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self {
            messages: vec![message.into()],
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn from_messages(messages: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        let mut unique: Vec<SharedString> = Vec::new();
        for msg in messages {
            let s: SharedString = msg.into();
            if !s.is_empty() && !unique.contains(&s) {
                unique.push(s);
            }
        }
        Self {
            messages: unique,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for FieldError {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for FieldError {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        if self.messages.is_empty() {
            return div().into_any_element();
        }

        let mut base = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .font_weight(FontWeight::NORMAL)
            .text_color(tokens.destructive)
            .refine_style(&self.style);

        if self.messages.len() == 1 {
            base = base.child(self.messages[0].clone());
        } else {
            for msg in self.messages {
                base = base.child(div().pl(px(12.0)).child(format!("- {msg}")));
            }
        }

        base.into_any_element()
    }
}

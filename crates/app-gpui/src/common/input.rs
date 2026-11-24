//! Text and password input primitives ported from `web/src/components/ui/input.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Input`: `h-10 w-full min-w-0 border-b border-b-input bg-transparent px-0 py-1 text-sm focus-visible:border-b-ring aria-invalid:border-b-destructive disabled:opacity-50`
//! - Preserves `text_field` and `password_field` labeled compound helpers used across
//!   auth, onboarding, quick-entry, and entity modal screens.

pub use gpui_kit::component::input::{Input as KitInput, InputContentType, InputState};
use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::*;
use gpui_kit::{
    AnyElement, App, ElementId, Entity, Focusable as _, FontWeight, IntoElement, ParentElement,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Underline text input matching `Input` in `web/src/components/ui/input.tsx`.
#[derive(IntoElement)]
pub struct Input {
    id: Option<ElementId>,
    state: Entity<InputState>,
    disabled: bool,
    invalid: bool,
    borderless: bool,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl Input {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            id: None,
            state: state.clone(),
            disabled: false,
            invalid: false,
            borderless: false,
            prefix: None,
            suffix: None,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Removes the bottom underline when nested inside `InputGroup`.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix = Some(prefix.into_any_element());
        self
    }

    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Input {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Input {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
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

        let is_focused = self.state.read(cx).focus_handle(cx).is_focused(window);
        let border_bottom = if self.invalid {
            tokens.destructive
        } else if is_focused {
            tokens.ring
        } else {
            tokens.input
        };

        let mut inner = KitInput::new(&self.state)
            .appearance(false)
            .bordered(false)
            .focus_bordered(false)
            .disabled(self.disabled)
            .w_full()
            .px(px(0.0))
            .py(px(4.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.foreground);

        if let Some(id) = self.id {
            inner = inner.id(id);
        }
        if let Some(prefix) = self.prefix {
            inner = inner.prefix(prefix);
        }
        if let Some(suffix) = self.suffix {
            inner = inner.suffix(suffix);
        }

        div()
            .flex()
            .flex_row()
            .items_center()
            .h(PresetSpacing::CONTROL_DEFAULT) // h-10 (40px)
            .w_full()
            .min_w_0()
            .rounded(PresetRadius::NONE)
            .cx_when(!self.borderless, |d| d.border_b_1().border_color(border_bottom))
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .child(inner)
    }
}

/// Convenience constructor for a themed `--preset b1oaDsAa` [`Input`].
pub fn input_primitive(id: impl Into<ElementId>, state: &Entity<InputState>, theme: &SemanticTheme) -> Input {
    Input::new(state).id(id).theme(theme)
}

/// Single-line text input with a label above it and a blinking cursor when focused.
pub fn text_field(
    id: &'static str,
    label: impl Into<SharedString>,
    state: &Entity<InputState>,
    theme: &SemanticTheme,
) -> impl IntoElement {
    labeled(label, base_input(id, state, theme), theme)
}

/// Password input: masked by default, with an eye button to reveal or hide the text.
///
/// The owning `InputState` should be created with `.masked(true)`.
pub fn password_field(
    id: &'static str,
    label: impl Into<SharedString>,
    state: &Entity<InputState>,
    theme: &SemanticTheme,
    cx: &App,
) -> impl IntoElement {
    let input = base_input(id, state, theme).suffix(mask_toggle(id, state, theme, cx));
    labeled(label, input, theme)
}

fn base_input(id: &'static str, state: &Entity<InputState>, theme: &SemanticTheme) -> KitInput {
    KitInput::new(state)
        .id(id)
        .focus_bordered(true)
        .w_full()
        .bg(theme.bg_canvas)
        .border_color(theme.border_subtle)
        .text_color(theme.text_primary)
        .text_sm()
}

/// Eye button that flips the password field between masked and plain text.
fn mask_toggle(
    id: &'static str,
    state: &Entity<InputState>,
    theme: &SemanticTheme,
    cx: &App,
) -> impl IntoElement {
    let masked = state.read(cx).presentation().is_masked();
    let target = state.clone();
    let hover_bg = theme.bg_elevated;

    div()
        .id(ElementId::Name(format!("{id}-mask-toggle").into()))
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(26.0))
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, window, cx| {
            target.update(cx, |input, cx| input.toggle_masked(window, cx));
            window.refresh();
        })
        .child(app_icon(
            if masked {
                AppIcon::Eye
            } else {
                AppIcon::EyeSlash
            },
            IconSize::Sm,
            if masked {
                IconTone::Muted
            } else {
                IconTone::Default
            },
            theme,
        ))
}

fn labeled(
    label: impl Into<SharedString>,
    input: KitInput,
    theme: &SemanticTheme,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_muted)
                .child(label.into()),
        )
        .child(input)
}

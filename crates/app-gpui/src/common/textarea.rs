//! Multi-line text area primitive ported from `web/src/components/ui/textarea.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! `flex min-h-16 w-full resize-none rounded-none border-b border-b-input bg-transparent px-0 py-3 text-sm focus-visible:border-b-ring disabled:opacity-50 aria-invalid:border-b-destructive`
//! Built on `gpui_kit::component::input::{Textarea, TextareaState}`.

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::input::{Textarea as KitTextarea, TextareaState};
use gpui_kit::{
    App, DefiniteLength, Entity, Focusable as _, FontWeight, IntoElement, ParentElement,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Multi-line underline text input matching `Textarea` in `web/src/components/ui/textarea.tsx`.
#[derive(IntoElement)]
pub struct Textarea {
    state: Entity<TextareaState>,
    disabled: bool,
    readonly: bool,
    invalid: bool,
    borderless: bool,
    height: Option<DefiniteLength>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl Textarea {
    pub fn new(state: &Entity<TextareaState>) -> Self {
        Self {
            state: state.clone(),
            disabled: false,
            readonly: false,
            invalid: false,
            borderless: false,
            height: None,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
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

    pub fn height(mut self, height: impl Into<DefiniteLength>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Textarea {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Textarea {
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

        let mut inner = KitTextarea::new(&self.state)
            .appearance(false)
            .bordered(false)
            .disabled(self.disabled)
            .readonly(self.readonly)
            .w_full()
            .px(px(0.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.foreground);

        if let Some(h) = self.height {
            inner = inner.h(h);
        }

        div()
            .flex()
            .flex_col()
            .min_h(px(64.0)) // min-h-16
            .w_full()
            .py(px(12.0)) // py-3
            .rounded(PresetRadius::NONE)
            .cx_when(!self.borderless, |d| {
                d.border_b_1().border_color(border_bottom)
            })
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .child(inner)
    }
}

/// Convenience helper for a labeled multi-line [`Textarea`].
pub fn textarea_field(
    label: impl Into<SharedString>,
    state: &Entity<TextareaState>,
    theme: &SemanticTheme,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(
            div()
                .font_family(FONT_FAMILY)
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_muted)
                .child(label.into()),
        )
        .child(Textarea::new(state).theme(theme))
}

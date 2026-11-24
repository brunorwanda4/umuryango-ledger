//! Common text and password inputs built on the gpui-kit `Input` component.
//!
//! The `InputState` entity owns focus, the blinking cursor (caret), selection,
//! clipboard, and IME composition. This module layers the app's visual style on
//! top: a label, semantic theme tokens, a brand-colored focus ring, and (for
//! passwords) a show / hide toggle using the app's FontAwesome icons.
//!
//! Caret, focus-ring, and selection colors come from the kit theme, which
//! `SemanticTheme::apply_to_kit` keeps in sync with the app palette.

use gpui_kit::component::input::{Input, InputState};
use gpui_kit::prelude::*;
use gpui_kit::{
    App, ElementId, Entity, FontWeight, IntoElement, ParentElement, SharedString, Styled, div, px,
};

use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::SemanticTheme;

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

fn base_input(id: &'static str, state: &Entity<InputState>, theme: &SemanticTheme) -> Input {
    Input::new(state)
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
    input: Input,
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

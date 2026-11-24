//! One-time passcode (OTP) input primitives ported from `web/src/components/ui/input-otp.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `InputOTP`: interactive OTP input backed by `gpui_kit::component::input::{OtpEvent, OtpState}`
//! - `InputOTPGroup`: `flex items-center gap-1 rounded-none`
//! - `InputOTPSlot`: `relative flex size-10 items-center justify-center border-b border-b-input bg-transparent text-sm data-[active=true]:border-b-ring aria-invalid:border-b-destructive`
//! - `InputOTPSeparator`: `app_icon(AppIcon::Minus, IconSize::Sm, IconTone::Muted, theme)`.

pub use gpui_kit::component::input::{OtpEvent, OtpInput as KitOtpInput, OtpState};
use gpui_kit::component::{Disableable as _, StyledExt as _};
use gpui_kit::{
    AnyElement, App, ElementId, Entity, Focusable as _, InteractiveElement, IntoElement,
    MouseButton, ParentElement, RenderOnce, StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
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

/// Interactive OTP input matching `InputOTP` in `web/src/components/ui/input-otp.tsx`.
///
/// Can either auto-render `groups` of underline slots from an `Entity<OtpState>`
/// or wrap explicit [`InputOTPGroup`] / [`InputOTPSeparator`] children.
#[derive(IntoElement)]
pub struct InputOTP {
    state: Option<Entity<OtpState>>,
    groups: usize,
    disabled: bool,
    invalid: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for InputOTP {
    fn default() -> Self {
        Self::empty()
    }
}

impl InputOTP {
    pub fn new(state: &Entity<OtpState>) -> Self {
        Self {
            state: Some(state.clone()),
            groups: 2,
            disabled: false,
            invalid: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            state: None,
            groups: 1,
            disabled: false,
            invalid: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn groups(mut self, groups: usize) -> Self {
        self.groups = groups.max(1);
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

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for InputOTP {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for InputOTP {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for InputOTP {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);

        if !self.children.is_empty() || self.state.is_none() {
            let mut container = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .preset_disabled(self.disabled)
                .refine_style(&self.style);

            if let Some(ref state) = self.state {
                // Keep hidden KitOtpInput mounted so keyboard events route to OtpState
                container = container.child(
                    div()
                        .size(px(0.0))
                        .overflow_hidden()
                        .child(KitOtpInput::new(state).disabled(self.disabled)),
                );
            }
            return container.children(self.children).into_any_element();
        }

        let state_entity = self.state.expect("checked state is_some above");
        let (val_str, total_len, is_masked, is_focused) = {
            let st = state_entity.read(cx);
            (
                st.value().to_string(),
                st.len(),
                st.is_masked(),
                st.focus_handle(cx).is_focused(window),
            )
        };
        let chars: Vec<char> = val_str.chars().collect();
        let active_idx = chars.len().min(total_len.saturating_sub(1));
        let num_groups = self.groups.min(total_len.max(1));
        let per_group = total_len.div_ceil(num_groups).max(1);

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .preset_disabled(self.disabled)
            .on_mouse_down(MouseButton::Left, {
                let st = state_entity.clone();
                move |_, window, cx| {
                    st.read(cx).focus_handle(cx).focus(window, cx);
                }
            })
            .refine_style(&self.style)
            // Mount 0-sized KitOtpInput to wire native keyboard/focus handling on OtpState
            .child(
                div()
                    .size(px(0.0))
                    .overflow_hidden()
                    .child(KitOtpInput::new(&state_entity).disabled(self.disabled)),
            );

        for g in 0..num_groups {
            if g > 0 {
                row = row.child(InputOTPSeparator::new().theme(&theme));
            }
            let start = g * per_group;
            let end = ((g + 1) * per_group).min(total_len);
            let mut grp = InputOTPGroup::new();
            for idx in start..end {
                let ch = chars.get(idx).copied().map(|c| if is_masked { '*' } else { c });
                let slot_active = is_focused && !self.disabled && idx == active_idx;
                grp = grp.child(
                    InputOTPSlot::new(idx)
                        .character(ch)
                        .active(slot_active)
                        .caret(slot_active && ch.is_none())
                        .invalid(self.invalid)
                        .theme(&theme),
                );
            }
            row = row.child(grp);
        }

        row.into_any_element()
    }
}

/// Horizontal cluster of OTP digit slots (`InputOTPGroup` in `input-otp.tsx`).
#[derive(IntoElement)]
pub struct InputOTPGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for InputOTPGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl InputOTPGroup {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for InputOTPGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for InputOTPGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for InputOTPGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0)) // gap-1
            .rounded(PresetRadius::NONE)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Single `40x40px` character cell (`InputOTPSlot` in `input-otp.tsx`).
#[derive(IntoElement)]
pub struct InputOTPSlot {
    index: usize,
    character: Option<char>,
    active: bool,
    caret: bool,
    invalid: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl InputOTPSlot {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            character: None,
            active: false,
            caret: false,
            invalid: false,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn character(mut self, ch: Option<char>) -> Self {
        self.character = ch;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn caret(mut self, caret: bool) -> Self {
        self.caret = caret;
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
}

impl Styled for InputOTPSlot {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for InputOTPSlot {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let border_bottom = if self.invalid {
            tokens.destructive
        } else if self.active {
            tokens.ring
        } else {
            tokens.input
        };

        let mut cell = div()
            .id(ElementId::NamedInteger("otp-slot".into(), self.index as u64))
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .size(px(40.0)) // size-10
            .rounded(PresetRadius::NONE)
            .border_b_1()
            .border_color(border_bottom)
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.foreground)
            .refine_style(&self.style);

        if let Some(ch) = self.character {
            cell = cell.child(ch.to_string());
        } else if self.caret {
            cell = cell.child(div().h(px(16.0)).w(px(1.0)).bg(tokens.foreground));
        }

        cell
    }
}

/// Dash divider between [`InputOTPGroup`]s (`InputOTPSeparator` in `input-otp.tsx`).
#[derive(IntoElement)]
pub struct InputOTPSeparator {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl Default for InputOTPSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl InputOTPSeparator {
    pub fn new() -> Self {
        Self {
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for InputOTPSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for InputOTPSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        div()
            .flex()
            .items_center()
            .justify_center()
            .refine_style(&self.style)
            .child(app_icon(AppIcon::Minus, IconSize::Sm, IconTone::Muted, &theme))
    }
}

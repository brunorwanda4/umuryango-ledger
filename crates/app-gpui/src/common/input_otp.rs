//! One-time passcode (OTP) input primitives ported from `web/src/components/ui/input-otp.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `InputOTP`: interactive OTP input backed by `gpui_kit::component::input::{OtpEvent, OtpState}`
//! - Supports optional uppercase alphanumeric code entry (e.g. 4-character family invite codes like `HK0N`)
//! - `InputOTPGroup`: `flex items-center gap-1 rounded-none`
//! - `InputOTPSlot`: `relative flex size-10 items-center justify-center border-b border-b-input bg-transparent text-sm data-[active=true]:border-b-ring aria-invalid:border-b-destructive`
//! - `InputOTPSeparator`: `app_icon(AppIcon::Minus, IconSize::Sm, IconTone::Muted, theme)`.

use std::rc::Rc;

pub use gpui_kit::component::input::{OtpEvent, OtpInput as KitOtpInput, OtpState};
use gpui_kit::component::{Disableable as _, StyledExt as _};
use gpui_kit::{
    AnyElement, App, ElementId, Entity, Focusable as _, FontWeight, InteractiveElement,
    IntoElement, KeyDownEvent, MouseButton, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

type OtpChangeHandler = Rc<dyn Fn(&String, &mut Window, &mut App) + 'static>;
type OtpSubmitHandler = Rc<dyn Fn(&String, &mut Window, &mut App) + 'static>;

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

/// Strips non-alphanumeric characters, uppercases all ASCII letters, and truncates to `max_len`.
pub fn sanitize_uppercase_alphanumeric(raw: &str, max_len: usize) -> String {
    raw.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_uppercase())
        .take(max_len)
        .collect()
}

/// Applies a single keyboard event to an uppercase alphanumeric OTP string.
/// Returns `Some(updated)` when the value changed, or `None` if the key is ignored.
pub fn apply_uppercase_alphanumeric_key(
    current: &str,
    key: &str,
    key_char: Option<&str>,
    max_len: usize,
) -> Option<String> {
    let mut chars: Vec<char> = current
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_uppercase())
        .take(max_len)
        .collect();

    if key == "backspace" {
        chars.pop()?;
        return Some(chars.into_iter().collect());
    }

    let candidate = key_char
        .and_then(|s| {
            let mut it = s.chars();
            match (it.next(), it.next()) {
                (Some(c), None) if c.is_ascii_alphanumeric() => Some(c.to_ascii_uppercase()),
                _ => None,
            }
        })
        .or_else(|| {
            let mut it = key.chars();
            match (it.next(), it.next()) {
                (Some(c), None) if c.is_ascii_alphanumeric() => Some(c.to_ascii_uppercase()),
                _ => None,
            }
        })?;

    if chars.len() >= max_len {
        return None;
    }

    chars.push(candidate);
    Some(chars.into_iter().collect())
}

/// Interactive OTP input matching `InputOTP` in `web/src/components/ui/input-otp.tsx`.
///
/// Can either auto-render `groups` of underline slots from an `Entity<OtpState>`
/// or wrap explicit [`InputOTPGroup`] / [`InputOTPSeparator`] children.
#[derive(IntoElement)]
pub struct InputOTP {
    state: Option<Entity<OtpState>>,
    groups: usize,
    uppercase_alphanumeric: bool,
    disabled: bool,
    invalid: bool,
    theme: Option<SemanticTheme>,
    on_change: Option<OtpChangeHandler>,
    on_submit: Option<OtpSubmitHandler>,
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
            uppercase_alphanumeric: false,
            disabled: false,
            invalid: false,
            theme: None,
            on_change: None,
            on_submit: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            state: None,
            groups: 1,
            uppercase_alphanumeric: false,
            disabled: false,
            invalid: false,
            theme: None,
            on_change: None,
            on_submit: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn groups(mut self, groups: usize) -> Self {
        self.groups = groups.max(1);
        self
    }

    /// Enables alphanumeric entry (A-Z, 0-9) where all letters are automatically uppercased,
    /// including pasted clipboard content.
    pub fn uppercase_alphanumeric(mut self, enabled: bool) -> Self {
        self.uppercase_alphanumeric = enabled;
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

    pub fn on_change(mut self, handler: impl Fn(&String, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn on_submit(mut self, handler: impl Fn(&String, &mut Window, &mut App) + 'static) -> Self {
        self.on_submit = Some(Rc::new(handler));
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
        let (val_str, total_len, is_masked, is_focused, blink_visible, focus_handle) = {
            let st = state_entity.read(cx);
            let fh = st.focus_handle(cx);
            (
                st.value().to_string(),
                st.len(),
                st.is_masked(),
                fh.is_focused(window),
                st.cursor_visible(cx),
                fh,
            )
        };
        let chars: Vec<char> = val_str.chars().collect();
        let active_idx = chars.len().min(total_len.saturating_sub(1));
        let num_groups = self.groups.min(total_len.max(1));
        let per_group = total_len.div_ceil(num_groups).max(1);

        let mut row = div()
            .id(("input-otp-root", state_entity.entity_id()))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .cursor_text()
            .preset_disabled(self.disabled)
            .on_mouse_down(MouseButton::Left, {
                let st = state_entity.clone();
                let disabled = self.disabled;
                move |_, window, cx| {
                    if !disabled {
                        st.read(cx).focus_handle(cx).focus(window, cx);
                    }
                }
            })
            .refine_style(&self.style);

        if self.uppercase_alphanumeric {
            row = row.track_focus(&focus_handle);
            if !self.disabled {
                let st = state_entity.clone();
                let on_change = self.on_change.clone();
                let on_submit = self.on_submit.clone();
                row = row.on_key_down(move |event: &KeyDownEvent, window, cx| {
                    let key = event.keystroke.key.as_str();
                    let mods = event.keystroke.modifiers;

                    if key == "enter" {
                        window.prevent_default();
                        cx.stop_propagation();
                        let current = st.read(cx).value().to_string();
                        if let Some(ref submit) = on_submit {
                            submit(&current, window, cx);
                        }
                        return;
                    }

                    if (mods.control || mods.platform) && key.eq_ignore_ascii_case("v") {
                        if let Some(pasted) = cx
                            .read_from_clipboard()
                            .and_then(|item| item.text())
                            .map(|txt| sanitize_uppercase_alphanumeric(&txt, total_len))
                            .filter(|s| !s.is_empty())
                        {
                            window.prevent_default();
                            cx.stop_propagation();
                            st.update(cx, |otp, cx| otp.set_value(pasted.clone(), window, cx));
                            if let Some(ref change) = on_change {
                                change(&pasted, window, cx);
                            }
                        }
                        return;
                    }

                    if mods.control || mods.platform || mods.alt {
                        return;
                    }

                    let current = st.read(cx).value().to_string();
                    if let Some(next) = apply_uppercase_alphanumeric_key(
                        &current,
                        key,
                        event.keystroke.key_char.as_deref(),
                        total_len,
                    ) {
                        window.prevent_default();
                        cx.stop_propagation();
                        st.update(cx, |otp, cx| otp.set_value(next.clone(), window, cx));
                        if let Some(ref change) = on_change {
                            change(&next, window, cx);
                        }
                    }
                });
            }
        } else {
            // Mount 0-sized KitOtpInput to wire native numeric keyboard/focus handling on OtpState
            row = row.child(
                div()
                    .size(px(0.0))
                    .overflow_hidden()
                    .child(KitOtpInput::new(&state_entity).disabled(self.disabled)),
            );
        }

        for g in 0..num_groups {
            if g > 0 {
                row = row.child(InputOTPSeparator::new().theme(&theme));
            }
            let start = g * per_group;
            let end = ((g + 1) * per_group).min(total_len);
            let mut grp = InputOTPGroup::new();
            for idx in start..end {
                let ch = chars.get(idx).copied().map(|c| {
                    if is_masked {
                        '*'
                    } else if self.uppercase_alphanumeric {
                        c.to_ascii_uppercase()
                    } else {
                        c
                    }
                });
                let slot_active = is_focused && !self.disabled && idx == active_idx;
                grp = grp.child(
                    InputOTPSlot::new(idx)
                        .character(ch)
                        .active(slot_active)
                        .caret(slot_active && ch.is_none() && blink_visible)
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

/// Single `44x44px` character cell (`InputOTPSlot` in `input-otp.tsx`).
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
            .id(ElementId::NamedInteger(
                "otp-slot".into(),
                self.index as u64,
            ))
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .size(px(44.0))
            .rounded(PresetRadius::NONE)
            .border_color(border_bottom)
            .font_family(FONT_FAMILY)
            .text_base()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground);

        cell = if self.active {
            cell.border_b_2()
        } else {
            cell.border_b_1()
        };

        cell = cell.refine_style(&self.style);

        if let Some(ch) = self.character {
            cell = cell.child(ch.to_string());
        } else if self.caret {
            cell = cell.child(div().h(px(18.0)).w(px(1.5)).bg(tokens.foreground));
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
            .child(app_icon(
                AppIcon::Minus,
                IconSize::Sm,
                IconTone::Muted,
                &theme,
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_uppercase_alphanumeric_key, sanitize_uppercase_alphanumeric};

    #[test]
    fn uppercase_alphanumeric_converts_lowercase_letters_and_keeps_digits() {
        assert_eq!(
            apply_uppercase_alphanumeric_key("", "h", Some("h"), 4).as_deref(),
            Some("H")
        );
        assert_eq!(
            apply_uppercase_alphanumeric_key("H", "k", Some("k"), 4).as_deref(),
            Some("HK")
        );
        assert_eq!(
            apply_uppercase_alphanumeric_key("HK", "0", Some("0"), 4).as_deref(),
            Some("HK0")
        );
        assert_eq!(
            apply_uppercase_alphanumeric_key("HK0", "n", Some("n"), 4).as_deref(),
            Some("HK0N")
        );
        // Stops at max_len = 4
        assert_eq!(
            apply_uppercase_alphanumeric_key("HK0N", "z", Some("z"), 4),
            None
        );
    }

    #[test]
    fn uppercase_alphanumeric_ignores_symbols_and_handles_backspace() {
        assert_eq!(
            apply_uppercase_alphanumeric_key("HK", "-", Some("-"), 4),
            None
        );
        assert_eq!(
            apply_uppercase_alphanumeric_key("HK", "space", Some(" "), 4),
            None
        );
        assert_eq!(
            apply_uppercase_alphanumeric_key("HK", "left", None, 4),
            None
        );
        assert_eq!(
            apply_uppercase_alphanumeric_key("HK0N", "backspace", None, 4).as_deref(),
            Some("HK0")
        );
        assert_eq!(
            apply_uppercase_alphanumeric_key("", "backspace", None, 4),
            None
        );
    }

    #[test]
    fn sanitize_paste_uppercases_and_truncates() {
        assert_eq!(sanitize_uppercase_alphanumeric("hk-0n!", 4), "HK0N");
        assert_eq!(sanitize_uppercase_alphanumeric("  ab12cd ", 4), "AB12");
    }
}

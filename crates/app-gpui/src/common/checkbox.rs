//! Checkbox primitive ported from `web/src/components/ui/checkbox.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - Box: `size-4.5` (`18px`), `rounded-none`, `border border-input bg-transparent`
//! - Checked: `border-primary bg-primary text-primary-foreground` with `AppIcon::Check` (`14px`)
//! - Indeterminate: `border-primary bg-primary` with `AppIcon::Minus` (`14px`)
//! - Focus & invalid rings + keyboard Space/Enter toggle.

pub use gpui_kit::component::checkbox::Checkbox as KitCheckbox;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    App, ElementId, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::common::label::{Label, LabelMode};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{SemanticTheme, ThemeConfig, ThemeMode};

/// Tri-state check status for [`Checkbox`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckedState {
    #[default]
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckedState {
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Checked | Self::Indeterminate)
    }

    pub const fn to_bool(self) -> bool {
        matches!(self, Self::Checked)
    }
}

impl From<bool> for CheckedState {
    fn from(value: bool) -> Self {
        if value { Self::Checked } else { Self::Unchecked }
    }
}

/// Square `18x18px` checkbox matching `Checkbox` in `web/src/components/ui/checkbox.tsx`.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    state: CheckedState,
    label: Option<SharedString>,
    disabled: bool,
    invalid: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state: CheckedState::Unchecked,
            label: None,
            disabled: false,
            invalid: false,
            focus_handle: None,
            theme: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn checked(mut self, checked: impl Into<CheckedState>) -> Self {
        self.state = checked.into();
        self
    }

    pub fn indeterminate(mut self) -> Self {
        self.state = CheckedState::Indeterminate;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
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

    pub fn track_focus(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl Styled for Checkbox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Checkbox {
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

        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let is_active = self.state.is_active();
        let next_checked = !self.state.to_bool();

        let (bg, border) = if is_active {
            (tokens.primary, tokens.primary)
        } else {
            (tokens.background.opacity(0.0), tokens.input)
        };

        let check_tone = if tokens.is_dark {
            IconTone::Default
        } else {
            IconTone::OnPrimaryButton
        };

        let mut indicator_box = div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .size(px(18.0)) // size-4.5
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(border)
            .bg(bg)
            .preset_ring(is_focused, self.invalid, theme);

        match self.state {
            CheckedState::Checked => {
                indicator_box = indicator_box.child(app_icon(
                    AppIcon::Check,
                    IconSize::Sm,
                    check_tone,
                    theme,
                ));
            }
            CheckedState::Indeterminate => {
                indicator_box = indicator_box.child(app_icon(
                    AppIcon::Minus,
                    IconSize::Sm,
                    check_tone,
                    theme,
                ));
            }
            CheckedState::Unchecked => {}
        }

        let mut root = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .child(indicator_box);

        if let Some(label_text) = self.label {
            root = root.child(
                Label::new(label_text)
                    .mode(LabelMode::Control)
                    .disabled(self.disabled)
                    .theme(theme),
            );
        }

        if let Some(ref fh) = self.focus_handle {
            root = root.track_focus(fh);
        }

        if !self.disabled {
            root = root.cursor_pointer();
            if let Some(handler) = self.on_change {
                let cb = std::rc::Rc::new(handler);
                let click_cb = cb.clone();
                root = root.on_click(move |_, window, cx| {
                    click_cb(&next_checked, window, cx);
                });
                root = root.on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    if key == "space" || key == "enter" {
                        cb(&next_checked, window, cx);
                    }
                });
            }
        }

        root
    }
}

/// Convenience constructor for a labeled controlled [`Checkbox`].
pub fn checkbox_field(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    checked: bool,
    theme: &SemanticTheme,
    on_change: impl Fn(&bool, &mut Window, &mut App) + 'static,
) -> Checkbox {
    Checkbox::new(id)
        .label(label)
        .checked(checked)
        .theme(theme)
        .on_change(on_change)
}

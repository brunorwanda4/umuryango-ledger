//! Toggle switch primitive ported from `web/src/components/ui/switch.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - Sharp zero-radius track & thumb (`rounded-none`)
//! - `SwitchSize::Default`: track `33x18px` (`w-8.25 h-4.5`), thumb `14x14px` (`size-3.5`)
//! - `SwitchSize::Sm`: track `25x14px` (`w-6.25 h-3.5`), thumb `10x10px` (`size-2.5`)
//! - Checked: `border-primary bg-primary`, thumb `bg-background` (`dark:bg-primary-foreground`)
//! - Unchecked: `border-input/50 bg-input`, thumb `bg-background` (`dark:bg-foreground`).

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::switch::Switch as KitSwitch;
use gpui_kit::{
    App, ElementId, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::common::label::{Label, LabelMode};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::theme::{SemanticTheme, ThemeConfig, ThemeMode};

/// Dimensions matching `size = "default" | "sm"` in `web/src/components/ui/switch.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwitchSize {
    #[default]
    Default,
    Sm,
}

impl SwitchSize {
    pub const fn track_size(self) -> (f32, f32) {
        match self {
            Self::Default => (33.0, 18.0), // w-8.25 h-4.5
            Self::Sm => (25.0, 14.0),      // w-6.25 h-3.5
        }
    }

    pub const fn thumb_px(self) -> f32 {
        match self {
            Self::Default => 14.0, // size-3.5
            Self::Sm => 10.0,      // size-2.5
        }
    }
}

/// Binary toggle switch matching `Switch` in `web/src/components/ui/switch.tsx`.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    checked: bool,
    size: SwitchSize,
    label: Option<SharedString>,
    disabled: bool,
    invalid: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            size: SwitchSize::Default,
            label: None,
            disabled: false,
            invalid: false,
            focus_handle: None,
            theme: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    pub fn sm(self) -> Self {
        self.size(SwitchSize::Sm)
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

    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl Styled for Switch {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Switch {
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
        let (track_w, track_h) = self.size.track_size();
        let thumb_sz = self.size.thumb_px();
        let next_checked = !self.checked;

        let (track_bg, track_border, thumb_bg) = if self.checked {
            (
                tokens.primary,
                tokens.primary,
                if tokens.is_dark {
                    tokens.primary_foreground
                } else {
                    tokens.card
                },
            )
        } else {
            (
                if tokens.is_dark {
                    theme.bg_surface
                } else {
                    tokens.secondary
                },
                tokens.input.opacity(0.6),
                if tokens.is_dark {
                    tokens.foreground
                } else {
                    tokens.card
                },
            )
        };

        let thumb = div()
            .flex_shrink_0()
            .size(px(thumb_sz))
            .rounded(PresetRadius::NONE)
            .bg(thumb_bg);

        let track = div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .cx_when(self.checked, |d| d.justify_end())
            .cx_when(!self.checked, |d| d.justify_start())
            .w(px(track_w))
            .h(px(track_h))
            .px(px(1.0))
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(track_border)
            .bg(track_bg)
            .preset_ring(is_focused, self.invalid, theme)
            .child(thumb);

        let mut root = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .child(track);

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

/// Convenience constructor for a labeled controlled [`Switch`].
pub fn switch_field(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    checked: bool,
    theme: &SemanticTheme,
    on_change: impl Fn(&bool, &mut Window, &mut App) + 'static,
) -> Switch {
    Switch::new(id)
        .label(label)
        .checked(checked)
        .theme(theme)
        .on_change(on_change)
}

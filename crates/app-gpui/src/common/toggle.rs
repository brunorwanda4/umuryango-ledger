//! Two-state pressable toggle button ported from `web/src/components/ui/toggle.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - Base: `inline-flex items-center justify-center gap-1.5 rounded-none text-xs font-semibold uppercase hover:bg-muted aria-pressed:bg-muted`
//! - Variants: `Default` (`bg-transparent`), `Outline` (`border border-input bg-transparent`)
//! - Sizes: `Default` (`h-10 min-w-10 px-6`), `Sm` (`h-9 min-w-9 px-4`), `Lg` (`h-11 min-w-11 px-8`).

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::button::Toggle as KitToggle;
use gpui_kit::{
    AnyElement, App, ElementId, FocusHandle, FontWeight, InteractiveElement, IntoElement,
    KeyDownEvent, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Visual border variants matching `toggleVariants.variant` in `toggle.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

/// Dimensions matching `toggleVariants.size` in `toggle.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl ToggleSize {
    pub const fn metrics(self) -> (f32, f32, f32) {
        match self {
            Self::Sm => (36.0, 16.0, 12.0),      // h-9 px-4 icon-edge-3
            Self::Default => (40.0, 24.0, 16.0), // h-10 px-6 icon-edge-4
            Self::Lg => (44.0, 32.0, 20.0),      // h-11 px-8 icon-edge-5
        }
    }
}

/// Pressable toggle button matching `Toggle` in `web/src/components/ui/toggle.tsx`.
#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    pressed: bool,
    label: Option<SharedString>,
    variant: ToggleVariant,
    size: ToggleSize,
    icon_start: Option<AppIcon>,
    icon_end: Option<AppIcon>,
    disabled: bool,
    invalid: bool,
    collapse_start_border: bool,
    collapse_top_border: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            pressed: false,
            label: None,
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            icon_start: None,
            icon_end: None,
            disabled: false,
            invalid: false,
            collapse_start_border: false,
            collapse_top_border: false,
            focus_handle: None,
            theme: None,
            on_change: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn outline(self) -> Self {
        self.variant(ToggleVariant::Outline)
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon_start = Some(icon);
        self
    }

    pub fn icon_start(mut self, icon: AppIcon) -> Self {
        self.icon_start = Some(icon);
        self
    }

    pub fn icon_end(mut self, icon: AppIcon) -> Self {
        self.icon_end = Some(icon);
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

    pub(crate) fn collapse_start_border(mut self, collapse: bool) -> Self {
        self.collapse_start_border = collapse;
        self
    }

    pub(crate) fn collapse_top_border(mut self, collapse: bool) -> Self {
        self.collapse_top_border = collapse;
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

impl Styled for Toggle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Toggle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Toggle {
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
        let next_pressed = !self.pressed;

        let active_bg = if tokens.is_dark {
            theme.bg_elevated.opacity(0.65)
        } else {
            tokens.secondary
        };
        let bg = if self.pressed {
            active_bg
        } else {
            tokens.background.opacity(0.0)
        };

        let (height, base_px, icon_px) = self.size.metrics();
        let is_icon_only = self.label.is_none()
            && self.children.is_empty()
            && (self.icon_start.is_some() ^ self.icon_end.is_some());

        let (pl, pr) = if is_icon_only {
            (0.0, 0.0)
        } else {
            (
                if self.icon_start.is_some() {
                    icon_px
                } else {
                    base_px
                },
                if self.icon_end.is_some() {
                    icon_px
                } else {
                    base_px
                },
            )
        };

        let mut el = div()
            .id(self.id)
            .flex()
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .h(px(height))
            .min_w(px(height))
            .pl(px(pl))
            .pr(px(pr))
            .gap(px(6.0))
            .rounded(PresetRadius::NONE)
            .bg(bg)
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground)
            .whitespace_nowrap()
            .cx_when(matches!(self.variant, ToggleVariant::Outline), |d| {
                let d = d.border_1().border_color(tokens.input);
                let d = if self.collapse_start_border {
                    d.border_l_0()
                } else {
                    d
                };
                if self.collapse_top_border {
                    d.border_t_0()
                } else {
                    d
                }
            })
            .preset_ring(is_focused, self.invalid, theme)
            .preset_disabled(self.disabled)
            .refine_style(&self.style);

        if let Some(ref fh) = self.focus_handle {
            el = el.track_focus(fh);
        }

        if let Some(icon) = self.icon_start {
            el = el.child(app_icon(icon, IconSize::Sm, IconTone::Default, theme));
        }
        if let Some(label) = self.label {
            el = el.child(to_preset_uppercase(label.as_ref()));
        }
        el = el.children(self.children);
        if let Some(icon) = self.icon_end {
            el = el.child(app_icon(icon, IconSize::Sm, IconTone::Default, theme));
        }

        if !self.disabled {
            let hover_bg = active_bg;
            el = el.cursor_pointer().hover(move |s| s.bg(hover_bg));
            if let Some(handler) = self.on_change {
                let cb = std::rc::Rc::new(handler);
                let click_cb = cb.clone();
                el = el.on_click(move |_, window, cx| {
                    click_cb(&next_pressed, window, cx);
                });
                el = el.on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    if key == "space" || key == "enter" {
                        cb(&next_pressed, window, cx);
                    }
                });
            }
        }

        el
    }
}

//! Radio group and circular indicator item ported from `web/src/components/ui/radio-group.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `RadioGroup`: `flex flex-col w-full gap-3` (`12px` spacing, or horizontal axis), keyboard arrow navigation
//! - `RadioGroupItem`: `size-4.5` (`18px`) `rounded-full border border-input bg-transparent`,
//!   checked `border-foreground` with inner `size-2` (`8px`) `rounded-full bg-foreground` dot
//!   (or primary red accent when `.primary_accent(true)`).

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    App, ElementId, FocusHandle, InteractiveElement, IntoElement, KeyDownEvent, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::common::label::{Label, LabelMode};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::theme::{SemanticTheme, ThemeConfig, ThemeMode};

/// Layout axis for [`RadioGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadioGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Mutually exclusive option container matching `RadioGroup` in `radio-group.tsx`.
#[derive(IntoElement)]
pub struct RadioGroup {
    id: ElementId,
    items: Vec<RadioGroupItem>,
    selected_index: Option<usize>,
    orientation: RadioGroupOrientation,
    disabled: bool,
    invalid: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl RadioGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected_index: None,
            orientation: RadioGroupOrientation::Vertical,
            disabled: false,
            invalid: false,
            focus_handle: None,
            theme: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn horizontal(id: impl Into<ElementId>) -> Self {
        Self::new(id).orientation(RadioGroupOrientation::Horizontal)
    }

    pub fn orientation(mut self, orientation: RadioGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_index = index;
        self
    }

    pub fn item(mut self, item: RadioGroupItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = RadioGroupItem>) -> Self {
        self.items.extend(items);
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

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for RadioGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let fallback_theme = SemanticTheme::from_config(
            &ThemeConfig::default(),
            if gpui_kit::component::Theme::global(cx).mode.is_dark() {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        );
        let theme = self.theme.as_ref().unwrap_or(&fallback_theme);
        let count = self.items.len();
        let current = self.selected_index;
        let on_change = self.on_change.clone();

        let mut container = div()
            .id(self.id)
            .flex()
            .w_full()
            .gap(px(12.0)) // gap-3
            .cx_when(
                matches!(self.orientation, RadioGroupOrientation::Vertical),
                |d| d.flex_col(),
            )
            .cx_when(
                matches!(self.orientation, RadioGroupOrientation::Horizontal),
                |d| d.flex_row().items_center().flex_wrap(),
            )
            .refine_style(&self.style);

        if let Some(ref fh) = self.focus_handle {
            container = container.track_focus(fh);
        }

        if !self.disabled && count > 0 {
            if let Some(ref cb) = on_change {
                let key_cb = cb.clone();
                container = container.on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    let next = match key {
                        "down" | "right" => Some(current.map_or(0, |i| (i + 1) % count)),
                        "up" | "left" => Some(current.map_or(0, |i| (i + count - 1) % count)),
                        _ => None,
                    };
                    if let Some(idx) = next {
                        key_cb(&idx, window, cx);
                    }
                });
            }
        }

        for (idx, mut item) in self.items.into_iter().enumerate() {
            let is_checked = self.selected_index == Some(idx);
            let item_disabled = self.disabled || item.disabled;
            let item_invalid = self.invalid || item.invalid;
            item = item
                .checked(is_checked)
                .disabled(item_disabled)
                .invalid(item_invalid)
                .theme(theme);

            if let Some(ref cb) = on_change {
                let item_cb = cb.clone();
                item = item.on_select(move |window, cx| {
                    item_cb(&idx, window, cx);
                });
            }
            container = container.child(item);
        }

        container
    }
}

/// Circular `18x18px` radio button item matching `RadioGroupItem` in `radio-group.tsx`.
#[derive(IntoElement)]
pub struct RadioGroupItem {
    id: ElementId,
    checked: bool,
    label: Option<SharedString>,
    disabled: bool,
    invalid: bool,
    primary_accent: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_select: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl RadioGroupItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            label: None,
            disabled: false,
            invalid: false,
            primary_accent: false,
            focus_handle: None,
            theme: None,
            on_select: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
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

    /// Uses primary `#ba181b` red instead of foreground ink for the checked ring & dot.
    pub fn primary_accent(mut self, primary: bool) -> Self {
        self.primary_accent = primary;
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

    pub fn on_select(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl Styled for RadioGroupItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for RadioGroupItem {
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
        let active_ink = if self.primary_accent {
            tokens.primary
        } else {
            tokens.foreground
        };
        let border = if self.checked {
            active_ink
        } else {
            tokens.input
        };

        let mut circle = div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .size(px(18.0)) // size-4.5
            .rounded(PresetRadius::FULL)
            .border_1()
            .border_color(border)
            .preset_ring(is_focused, self.invalid, theme);

        if self.checked {
            circle = circle.child(
                div()
                    .size(px(8.0)) // size-2
                    .rounded(PresetRadius::FULL)
                    .bg(active_ink),
            );
        }

        let mut root = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .child(circle);

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
            if let Some(handler) = self.on_select {
                let cb = Rc::new(handler);
                let click_cb = cb.clone();
                root = root.on_click(move |_, window, cx| {
                    click_cb(window, cx);
                });
                root = root.on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    if key == "space" || key == "enter" {
                        cb(window, cx);
                    }
                });
            }
        }

        root
    }
}

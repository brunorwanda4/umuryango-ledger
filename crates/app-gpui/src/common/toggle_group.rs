//! Segmented toggle group primitives ported from `web/src/components/ui/toggle-group.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `ToggleGroup`: propagates `ToggleVariant`, `ToggleSize`, `spacing` (default `8px`; `0px` collapses inner outline borders),
//!   and `ToggleGroupOrientation` (`Horizontal` | `Vertical`) across [`ToggleGroupItem`]s
//! - Supports both single-selection and multi-selection callbacks.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled,
    Window, div, px,
};

use crate::common::toggle::{Toggle, ToggleSize, ToggleVariant};
use crate::common::utils::{PresetRadius, StyleComposeExt};
use crate::components::icon::AppIcon;
use crate::theme::{SemanticTheme, ThemeConfig, ThemeMode};

/// Layout axis matching `orientation?: "horizontal" | "vertical"` in `toggle-group.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Item descriptor inside [`ToggleGroup`] (`ToggleGroupItem` in `toggle-group.tsx`).
pub struct ToggleGroupItem {
    id: ElementId,
    label: Option<SharedString>,
    icon_start: Option<AppIcon>,
    icon_end: Option<AppIcon>,
    disabled: bool,
    pressed_override: Option<bool>,
}

impl ToggleGroupItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            icon_start: None,
            icon_end: None,
            disabled: false,
            pressed_override: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
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

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed_override = Some(pressed);
        self
    }
}

/// Group of pressable [`Toggle`] controls (`ToggleGroup` in `web/src/components/ui/toggle-group.tsx`).
#[derive(IntoElement)]
pub struct ToggleGroup {
    items: Vec<ToggleGroupItem>,
    selected_indices: Vec<usize>,
    variant: ToggleVariant,
    size: ToggleSize,
    spacing_px: f32,
    orientation: ToggleGroupOrientation,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_toggle: Option<Rc<dyn Fn(usize, bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Default for ToggleGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ToggleGroup {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected_indices: Vec::new(),
            variant: ToggleVariant::Default,
            size: ToggleSize::Default,
            spacing_px: 8.0, // spacing = 2 (`--spacing(2)` = 8px)
            orientation: ToggleGroupOrientation::Horizontal,
            disabled: false,
            theme: None,
            on_toggle: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, item: ToggleGroupItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = ToggleGroupItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Marks a single active index (mutually exclusive segmented mode).
    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_indices = index.into_iter().collect();
        self
    }

    /// Marks multiple active indices (multi-select toolbar mode).
    pub fn selected_indices(mut self, indices: impl IntoIterator<Item = usize>) -> Self {
        self.selected_indices = indices.into_iter().collect();
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

    /// Gap in pixels between items (`0.0` joins adjacent outline borders).
    pub fn spacing_px(mut self, px_gap: f32) -> Self {
        self.spacing_px = px_gap.max(0.0);
        self
    }

    pub fn joined(self) -> Self {
        self.spacing_px(0.0)
    }

    pub fn orientation(mut self, orientation: ToggleGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn vertical(self) -> Self {
        self.orientation(ToggleGroupOrientation::Vertical)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(usize, bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl Styled for ToggleGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ToggleGroup {
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
        let is_horizontal = matches!(self.orientation, ToggleGroupOrientation::Horizontal);
        let is_joined_outline =
            self.spacing_px == 0.0 && matches!(self.variant, ToggleVariant::Outline);

        let mut container = div()
            .flex()
            .gap(px(self.spacing_px))
            .rounded(PresetRadius::NONE)
            .cx_when(is_horizontal, |d| d.flex_row().items_center())
            .cx_when(!is_horizontal, |d| d.flex_col().items_stretch())
            .refine_style(&self.style);

        for (idx, item) in self.items.into_iter().enumerate() {
            let is_pressed = item
                .pressed_override
                .unwrap_or_else(|| self.selected_indices.contains(&idx));
            let mut toggle = Toggle::new(item.id)
                .pressed(is_pressed)
                .variant(self.variant)
                .size(self.size)
                .disabled(self.disabled || item.disabled)
                .collapse_start_border(is_joined_outline && is_horizontal && idx > 0)
                .collapse_top_border(is_joined_outline && !is_horizontal && idx > 0)
                .theme(theme);

            if let Some(lbl) = item.label {
                toggle = toggle.label(lbl);
            }
            if let Some(ic) = item.icon_start {
                toggle = toggle.icon_start(ic);
            }
            if let Some(ic) = item.icon_end {
                toggle = toggle.icon_end(ic);
            }
            if let Some(ref cb) = self.on_toggle {
                let item_cb = cb.clone();
                toggle = toggle.on_change(move |next_pressed, window, cx| {
                    item_cb(idx, *next_pressed, window, cx);
                });
            }

            container = container.child(toggle);
        }

        container
    }
}

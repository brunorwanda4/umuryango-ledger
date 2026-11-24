//! Form label primitive ported from `web/src/components/ui/label.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - Default field header (`LabelMode::Field`): `flex items-center gap-2 text-xs font-semibold tracking-wide uppercase select-none`
//! - Inline peer control (`LabelMode::Control`, used next to `Checkbox`, `RadioGroupItem`, `Switch`):
//!   `text-sm font-normal tracking-normal normal-case`
//! - Disabled state: `opacity-50`.

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::theme::{FONT_FAMILY, SemanticTheme};

/// Typography mode matching `label.tsx` default vs `peer-data-[slot=checkbox|radio-group-item|switch]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelMode {
    /// Uppercase `text-xs font-semibold` field header (`default` in `base-sera`).
    #[default]
    Field,
    /// Normal-case `text-sm font-normal` inline control label (checkbox / radio / switch).
    Control,
}

/// Accessible form label matching `Label` in `web/src/components/ui/label.tsx`.
#[derive(IntoElement)]
pub struct Label {
    id: Option<ElementId>,
    text: SharedString,
    mode: LabelMode,
    disabled: bool,
    required: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Box<dyn Fn(&gpui_kit::ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            id: None,
            text: text.into(),
            mode: LabelMode::Field,
            disabled: false,
            required: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn mode(mut self, mode: LabelMode) -> Self {
        self.mode = mode;
        self
    }

    /// Switches to inline control styling (`text-sm font-normal normal-case`).
    pub fn control(mut self) -> Self {
        self.mode = LabelMode::Control;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Styled for Label {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Label {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (fg, destructive) = if let Some(ref theme) = self.theme {
            let tokens = PresetTokens::from_theme(theme);
            (tokens.foreground, tokens.destructive)
        } else {
            let kit = gpui_kit::component::Theme::global(cx);
            (kit.foreground, kit.danger)
        };

        let rendered_text: SharedString = match self.mode {
            LabelMode::Field => to_preset_uppercase(self.text.as_ref()).into(),
            LabelMode::Control => self.text,
        };

        let base = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .font_family(FONT_FAMILY)
            .text_color(fg)
            .cx_when(matches!(self.mode, LabelMode::Field), |d| {
                d.text_xs().font_weight(FontWeight::SEMIBOLD)
            })
            .cx_when(matches!(self.mode, LabelMode::Control), |d| {
                d.text_sm().font_weight(FontWeight::NORMAL)
            })
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .child(rendered_text)
            .cx_when(self.required, |d| {
                d.child(div().text_color(destructive).child("*"))
            })
            .children(self.children);

        if let Some(id) = self.id {
            let mut interactive = base.id(id);
            if !self.disabled {
                if let Some(handler) = self.on_click {
                    interactive = interactive.cursor_pointer().on_click(handler);
                }
            }
            interactive.into_any_element()
        } else {
            base.into_any_element()
        }
    }
}

/// Convenience constructor for a themed `--preset b1oaDsAa` uppercase field label.
pub fn field_label(text: impl Into<SharedString>, theme: &SemanticTheme) -> Label {
    Label::new(text).theme(theme)
}

//! Selectable single- and multi-choice card primitives for [`super::Questionnaire`]
//! ported from `web/src/components/ui/questionnaire.tsx`.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ElementId, FocusHandle, FontWeight, InteractiveElement, IntoElement,
    KeyDownEvent, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px, transparent_black,
};

use super::resolve_theme;
use crate::common::utils::{
    PresetRadius, PresetTokens, StyleComposeExt, focus_ring_shadow, to_preset_uppercase,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeMode};

/// Indicator selection mode for [`QuestionnaireChoice`] (`data-type` in `questionnaire.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuestionnaireChoiceType {
    #[default]
    Radio,
    Checkbox,
}

/// Vertical stack of selectable [`QuestionnaireChoice`] cards (`data-slot="questionnaire-choices"`).
#[derive(IntoElement)]
pub struct QuestionnaireChoices {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl QuestionnaireChoices {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for QuestionnaireChoices {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for QuestionnaireChoices {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for QuestionnaireChoices {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireChoices {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(12.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Selectable single- or multi-choice card (`data-slot="questionnaire-choice"`).
#[derive(IntoElement)]
pub struct QuestionnaireChoice {
    id: ElementId,
    choice_type: QuestionnaireChoiceType,
    checked: bool,
    disabled: bool,
    invalid: bool,
    label: Option<SharedString>,
    description: Option<SharedString>,
    shortcut: Option<SharedString>,
    focus_handle: Option<FocusHandle>,
    on_select: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl QuestionnaireChoice {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            choice_type: QuestionnaireChoiceType::Radio,
            checked: false,
            disabled: false,
            invalid: false,
            label: None,
            description: None,
            shortcut: None,
            focus_handle: None,
            on_select: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn choice_type(mut self, choice_type: QuestionnaireChoiceType) -> Self {
        self.choice_type = choice_type;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
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

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn track_focus(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for QuestionnaireChoice {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for QuestionnaireChoice {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireChoice {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_dark = theme.mode == ThemeMode::Dark;

        let card_border = if self.invalid {
            tokens.destructive
        } else if self.checked {
            tokens.primary.opacity(if is_dark { 0.2 } else { 0.3 })
        } else {
            tokens.input
        };

        let card_bg = if self.checked {
            tokens.primary.opacity(if is_dark { 0.1 } else { 0.05 })
        } else {
            transparent_black()
        };

        let indicator = match self.choice_type {
            QuestionnaireChoiceType::Radio => {
                let border = if self.checked {
                    tokens.foreground
                } else {
                    tokens.input
                };
                div()
                    .mt(px(2.0))
                    .flex()
                    .size(px(18.0))
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .rounded(PresetRadius::FULL)
                    .border_1()
                    .border_color(border)
                    .when(self.checked, |d| {
                        d.child(
                            div()
                                .size(px(8.0))
                                .rounded(PresetRadius::FULL)
                                .bg(tokens.foreground),
                        )
                    })
            }
            QuestionnaireChoiceType::Checkbox => {
                let border = if self.checked {
                    tokens.primary
                } else {
                    tokens.input
                };
                let bg = if self.checked {
                    tokens.primary
                } else {
                    transparent_black()
                };
                div()
                    .mt(px(2.0))
                    .flex()
                    .size(px(18.0))
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .rounded(PresetRadius::NONE)
                    .border_1()
                    .border_color(border)
                    .bg(bg)
                    .when(self.checked, |d| {
                        d.child(app_icon(
                            AppIcon::Check,
                            IconSize::Sm,
                            IconTone::OnPrimaryButton,
                            &theme,
                        ))
                    })
            }
        };

        let mut label_col = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap(px(4.0))
            .line_height(px(20.0));
        if let Some(label) = self.label {
            label_col = label_col.child(label);
        }
        if let Some(desc) = self.description {
            label_col = label_col.child(QuestionnaireChoiceDescription::new(desc).theme(&theme));
        }
        label_col = label_col.children(self.children);

        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let next_checked = !self.checked;
        let hover_bg = if self.checked {
            card_bg
        } else {
            tokens.muted.opacity(0.5)
        };

        let mut card = div()
            .id(self.id)
            .relative()
            .flex()
            .min_h(px(44.0))
            .items_start()
            .gap(px(12.0))
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(card_border)
            .bg(card_bg)
            .px(px(16.0))
            .py(px(16.0))
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .text_color(tokens.foreground)
            .preset_disabled(self.disabled)
            .when(is_focused, |d| d.shadow(focus_ring_shadow(&theme)))
            .child(indicator)
            .child(label_col);

        if let Some(shortcut) = self.shortcut {
            card = card.child(
                div()
                    .mt(px(2.0))
                    .ml_auto()
                    .flex()
                    .size(px(20.0))
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .rounded(PresetRadius::NONE)
                    .border_1()
                    .border_color(tokens.input)
                    .bg(tokens.background)
                    .text_size(px(10.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(tokens.muted_foreground)
                    .child(to_preset_uppercase(shortcut.as_ref())),
            );
        }

        if let Some(ref fh) = self.focus_handle {
            card = card.track_focus(fh);
        }

        if !self.disabled {
            card = card.cursor_pointer().hover(move |s| s.bg(hover_bg));
            if let Some(cb) = self.on_select {
                let click_cb = cb.clone();
                card = card.on_click(move |_, win, app| click_cb(&next_checked, win, app));
                card = card.on_key_down(move |ev: &KeyDownEvent, win, app| {
                    let k = ev.keystroke.key.as_str();
                    if k == "enter" || k == "space" {
                        cb(&next_checked, win, app);
                    }
                });
            }
        }

        card.refine_style(&self.style)
    }
}

/// Muted helper text inside a [`QuestionnaireChoice`] (`data-slot="questionnaire-choice-description"`).
#[derive(IntoElement)]
pub struct QuestionnaireChoiceDescription {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl QuestionnaireChoiceDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for QuestionnaireChoiceDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for QuestionnaireChoiceDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireChoiceDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div().text_color(tokens.muted_foreground);
        if let Some(text) = self.text {
            el = el.child(text);
        }
        el.refine_style(&self.style).children(self.children)
    }
}

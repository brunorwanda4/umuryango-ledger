//! Multi-step survey and onboarding questionnaire primitives ported from
//! `web/src/components/ui/questionnaire.tsx`.
//!
//! Choice card primitives live in [`choice`] and are re-exported at this module root
//! so every file stays well below the 1,000-line repository cap.

mod choice;

use std::rc::Rc;

pub use choice::{
    QuestionnaireChoice, QuestionnaireChoiceDescription, QuestionnaireChoiceType,
    QuestionnaireChoices,
};
use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::questionnaire::{
    QuestionnaireAnswer as KitQuestionnaireAnswer, QuestionnaireEvent as KitQuestionnaireEvent,
    QuestionnaireState as KitQuestionnaireState,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, Entity, FocusHandle, FontWeight, IntoElement,
    ParentElement, RenderOnce, SharedString, StyleRefinement, Styled, Window, div, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::input::{Input, InputState};
use crate::common::utils::{PresetTokens, to_preset_uppercase};
use crate::hooks::use_mobile::is_mobile;
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

pub(crate) fn resolve_theme(explicit: Option<&SemanticTheme>, cx: &App) -> SemanticTheme {
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

/// Root container (`data-slot="questionnaire"`).
#[derive(IntoElement)]
pub struct Questionnaire {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Questionnaire {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for Questionnaire {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Questionnaire {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Questionnaire {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Questionnaire {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .gap(px(24.0))
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Editorial uppercase step indicator (`data-slot="questionnaire-progress"`).
#[derive(IntoElement)]
pub struct QuestionnaireProgress {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl QuestionnaireProgress {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn steps(current: usize, total: usize) -> Self {
        Self::new(format!("{current} / {total}"))
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for QuestionnaireProgress {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for QuestionnaireProgress {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .min_h(px(18.0))
            .min_w(px(96.0))
            .font_family(FONT_FAMILY)
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.muted_foreground);

        if let Some(text) = self.text {
            el = el.child(to_preset_uppercase(text.as_ref()));
        }

        el.refine_style(&self.style).children(self.children)
    }
}

/// Question step container (`data-slot="questionnaire-item"`).
#[derive(IntoElement)]
pub struct QuestionnaireItem {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl QuestionnaireItem {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for QuestionnaireItem {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for QuestionnaireItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for QuestionnaireItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(20.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Uppercase question heading (`data-slot="questionnaire-title"`).
#[derive(IntoElement)]
pub struct QuestionnaireTitle {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl QuestionnaireTitle {
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

impl Styled for QuestionnaireTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for QuestionnaireTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .font_family(FONT_FAMILY)
            .text_size(px(12.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground);

        if let Some(text) = self.text {
            el = el.child(to_preset_uppercase(text.as_ref()));
        }

        el.refine_style(&self.style).children(self.children)
    }
}

/// Supporting prompt description (`data-slot="questionnaire-description"`).
#[derive(IntoElement)]
pub struct QuestionnaireDescription {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl QuestionnaireDescription {
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

impl Styled for QuestionnaireDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for QuestionnaireDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .text_color(tokens.muted_foreground);

        if let Some(text) = self.text {
            el = el.child(text);
        }

        el.refine_style(&self.style).children(self.children)
    }
}

/// Free-response underline text field inside a [`QuestionnaireItem`] (`data-slot="questionnaire-input"`).
#[derive(IntoElement)]
pub struct QuestionnaireInput {
    id: Option<ElementId>,
    state: Entity<InputState>,
    disabled: bool,
    invalid: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl QuestionnaireInput {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            id: None,
            state: state.clone(),
            disabled: false,
            invalid: false,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
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

impl Styled for QuestionnaireInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for QuestionnaireInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mobile = is_mobile(window);

        let mut input = Input::new(&self.state)
            .disabled(self.disabled)
            .invalid(self.invalid)
            .theme(&theme)
            .when(mobile, |i| i.min_h(px(44.0)));
        if let Some(id) = self.id {
            input = input.id(id);
        }

        div()
            .relative()
            .w_full()
            .min_w_0()
            .refine_style(&self.style)
            .child(input)
    }
}

/// Validation error message below choices or input (`data-slot="questionnaire-error"`).
#[derive(IntoElement)]
pub struct QuestionnaireError {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl QuestionnaireError {
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

impl Styled for QuestionnaireError {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for QuestionnaireError {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireError {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .mt(px(8.0))
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .text_color(tokens.destructive);
        if let Some(text) = self.text {
            el = el.child(text);
        }
        el.refine_style(&self.style).children(self.children)
    }
}

/// Footer navigation bar (`data-slot="questionnaire-actions"`).
#[derive(IntoElement)]
pub struct QuestionnaireActions {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl QuestionnaireActions {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for QuestionnaireActions {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for QuestionnaireActions {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for QuestionnaireActions {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireActions {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let min_h = if is_mobile(window) {
            px(44.0)
        } else {
            px(40.0)
        };
        div()
            .flex()
            .min_h(min_h)
            .w_full()
            .items_center()
            .gap(px(8.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

macro_rules! questionnaire_nav_button {
    ($name:ident, $default_variant:expr, $align_start:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(IntoElement)]
        pub struct $name {
            id: ElementId,
            label: Option<SharedString>,
            variant: ButtonVariant,
            size: ButtonSize,
            disabled: bool,
            focus_handle: Option<FocusHandle>,
            theme: Option<SemanticTheme>,
            on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
            style: StyleRefinement,
            children: Vec<AnyElement>,
        }

        impl $name {
            pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
                Self {
                    id: id.into(),
                    label: Some(label.into()),
                    variant: $default_variant,
                    size: ButtonSize::Default,
                    disabled: false,
                    focus_handle: None,
                    theme: None,
                    on_click: None,
                    style: StyleRefinement::default(),
                    children: Vec::new(),
                }
            }

            pub fn empty(id: impl Into<ElementId>) -> Self {
                Self {
                    id: id.into(),
                    label: None,
                    variant: $default_variant,
                    size: ButtonSize::Default,
                    disabled: false,
                    focus_handle: None,
                    theme: None,
                    on_click: None,
                    style: StyleRefinement::default(),
                    children: Vec::new(),
                }
            }

            pub fn variant(mut self, variant: ButtonVariant) -> Self {
                self.variant = variant;
                self
            }

            pub fn size(mut self, size: ButtonSize) -> Self {
                self.size = size;
                self
            }

            pub fn disabled(mut self, disabled: bool) -> Self {
                self.disabled = disabled;
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

            pub fn on_click(
                mut self,
                handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
            ) -> Self {
                self.on_click = Some(Rc::new(handler));
                self
            }
        }

        impl Styled for $name {
            fn style(&mut self) -> &mut StyleRefinement {
                &mut self.style
            }
        }

        impl ParentElement for $name {
            fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
                self.children.extend(elements);
            }
        }

        impl RenderOnce for $name {
            fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
                let theme = resolve_theme(self.theme.as_ref(), cx);
                let mobile = is_mobile(window);

                let mut btn = Button::new(self.id)
                    .variant(self.variant)
                    .size(self.size)
                    .disabled(self.disabled)
                    .theme(&theme)
                    .when(mobile, |b| b.min_h(px(44.0)))
                    .when($align_start, |b| b.mr_auto())
                    .when(!$align_start, |b| b.ml_auto());

                if let Some(label) = self.label {
                    btn = btn.label(label);
                }
                if let Some(ref fh) = self.focus_handle {
                    btn = btn.track_focus(fh);
                }
                if let Some(cb) = self.on_click {
                    btn = btn.on_click(move |ev, win, app| cb(ev, win, app));
                }

                btn.children(self.children).refine_style(&self.style)
            }
        }
    };
}

questionnaire_nav_button!(
    QuestionnairePrevious,
    ButtonVariant::Outline,
    true,
    "Step-back action button (`data-slot=\"questionnaire-previous\"`)."
);

questionnaire_nav_button!(
    QuestionnaireSkip,
    ButtonVariant::Outline,
    false,
    "Optional skip-step action button (`data-slot=\"questionnaire-skip\"`)."
);

questionnaire_nav_button!(
    QuestionnaireNext,
    ButtonVariant::Default,
    false,
    "Advance-to-next-step primary button (`data-slot=\"questionnaire-next\"`)."
);

questionnaire_nav_button!(
    QuestionnaireSubmit,
    ButtonVariant::Default,
    false,
    "Final questionnaire submission primary button (`data-slot=\"questionnaire-submit\"`)."
);

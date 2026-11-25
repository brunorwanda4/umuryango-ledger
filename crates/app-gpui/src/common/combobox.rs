//! Filterable combobox and multi-select chip primitives ported from
//! `web/src/components/ui/combobox.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `ComboboxInput`: [`InputGroup`] + [`input_group_input`] + `InlineEnd` trigger (`AppIcon::ChevronDown`)
//!   and optional clear button (`AppIcon::Close`)
//! - `ComboboxContent` & `ComboboxList`: `rounded-none bg-popover shadow-md ring-1 ring-foreground/10`, `p-1.5`
//! - `ComboboxChips` & `ComboboxChip`: `min-h-10 border-b border-b-input`, chip `h-[22px] rounded-none bg-muted px-2 text-xs`
//! - Reuses `SelectItem`, `SelectGroup`, `SelectLabel`, `SelectSeparator`, `SelectValue` per Rule 2.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::combobox::{
    Combobox as KitCombobox, ComboboxChange as KitComboboxChange,
    ComboboxEvent as KitComboboxEvent, ComboboxState as KitComboboxState,
};
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, Entity, FontWeight, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, deferred, div, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::input::InputState;
use crate::common::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButtonSize, input_group_button,
    input_group_input,
};
use crate::common::popover::{PopoverAlign, PopoverSide, apply_anchored_position};
use crate::common::select::{SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectValue};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow};
use crate::components::icon::AppIcon;
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

/// Selected value display (`ComboboxValue` in `combobox.tsx`).
pub type ComboboxValue = SelectValue;

/// Selectable option row (`ComboboxItem` in `combobox.tsx`).
pub type ComboboxItem = SelectItem;

/// Option group (`ComboboxGroup` in `combobox.tsx`).
pub type ComboboxGroup = SelectGroup;

/// Uppercase group heading (`ComboboxLabel` in `combobox.tsx`).
pub type ComboboxLabel = SelectLabel;

/// Thin divider (`ComboboxSeparator` in `combobox.tsx`).
pub type ComboboxSeparator = SelectSeparator;

/// Collection wrapper (`ComboboxCollection` in `combobox.tsx`).
pub type ComboboxCollection = SelectGroup;

/// Dropdown chevron trigger (`ComboboxTrigger` in `combobox.tsx`).
#[derive(IntoElement)]
pub struct ComboboxTrigger {
    id: ElementId,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl ComboboxTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            disabled: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

impl Styled for ComboboxTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ComboboxTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let on_click = self.on_click;

        input_group_button(
            self.id,
            InputGroupButtonSize::IconXs,
            ButtonVariant::Ghost,
            &theme,
            move |ev, window, cx| {
                if let Some(cb) = &on_click {
                    cb(ev, window, cx);
                }
            },
        )
        .icon(AppIcon::ChevronDown)
        .disabled(self.disabled)
        .refine_style(&self.style)
    }
}

/// Clear button (`ComboboxClear` in `combobox.tsx`).
#[derive(IntoElement)]
pub struct ComboboxClear {
    id: ElementId,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl ComboboxClear {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            disabled: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

impl Styled for ComboboxClear {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ComboboxClear {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let on_click = self.on_click;

        input_group_button(
            self.id,
            InputGroupButtonSize::IconXs,
            ButtonVariant::Ghost,
            &theme,
            move |ev, window, cx| {
                if let Some(cb) = &on_click {
                    cb(ev, window, cx);
                }
            },
        )
        .icon(AppIcon::Close)
        .disabled(self.disabled)
        .refine_style(&self.style)
    }
}

/// Underline search input with trailing trigger/clear addons (`ComboboxInput` in `combobox.tsx`).
#[derive(IntoElement)]
pub struct ComboboxInput {
    id: ElementId,
    state: Entity<InputState>,
    disabled: bool,
    invalid: bool,
    show_trigger: bool,
    show_clear: bool,
    theme: Option<SemanticTheme>,
    on_trigger_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_clear_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl ComboboxInput {
    pub fn new(id: impl Into<ElementId>, state: &Entity<InputState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            disabled: false,
            invalid: false,
            show_trigger: true,
            show_clear: false,
            theme: None,
            on_trigger_click: None,
            on_clear_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn show_trigger(mut self, show: bool) -> Self {
        self.show_trigger = show;
        self
    }

    pub fn show_clear(mut self, show: bool) -> Self {
        self.show_clear = show;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_trigger_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_trigger_click = Some(Rc::new(handler));
        self
    }

    pub fn on_clear_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_clear_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for ComboboxInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ComboboxInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let inp = input_group_input(&self.state, &theme).disabled(self.disabled);

        let mut addon = InputGroupAddon::new()
            .align(InputGroupAddonAlign::InlineEnd)
            .theme(&theme);

        if self.show_clear {
            let mut clr =
                ComboboxClear::new(ElementId::Name(format!("{}-combo-clear", self.id).into()))
                    .disabled(self.disabled)
                    .theme(&theme);
            if let Some(cb) = self.on_clear_click {
                clr = clr.on_click(move |ev, window, cx| cb(ev, window, cx));
            }
            addon = addon.child(clr);
        } else if self.show_trigger {
            let mut trig =
                ComboboxTrigger::new(ElementId::Name(format!("{}-combo-trig", self.id).into()))
                    .disabled(self.disabled)
                    .theme(&theme);
            if let Some(cb) = self.on_trigger_click {
                trig = trig.on_click(move |ev, window, cx| cb(ev, window, cx));
            }
            addon = addon.child(trig);
        }

        InputGroup::new()
            .disabled(self.disabled)
            .invalid(self.invalid)
            .theme(&theme)
            .refine_style(&self.style)
            .child(inp)
            .child(addon)
    }
}

/// Floating combobox popup (`ComboboxContent` in `combobox.tsx`).
#[derive(IntoElement)]
pub struct ComboboxContent {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ComboboxContent {
    pub fn new() -> Self {
        Self {
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

impl Default for ComboboxContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ComboboxContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ComboboxContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ComboboxContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .flex_col()
            .min_w(px(180.0))
            .overflow_hidden()
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .font_family(FONT_FAMILY)
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Scrollable list inside [`ComboboxContent`] (`ComboboxList` in `combobox.tsx`).
#[derive(IntoElement)]
pub struct ComboboxList {
    id: ElementId,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ComboboxList {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for ComboboxList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ComboboxList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ComboboxList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .max_h(px(252.0))
            .overflow_y_scroll()
            .p(px(6.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Empty search fallback (`ComboboxEmpty` in `combobox.tsx`).
#[derive(IntoElement)]
pub struct ComboboxEmpty {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ComboboxEmpty {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ComboboxEmpty {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ComboboxEmpty {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .w_full()
            .justify_center()
            .py(px(8.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.muted_foreground)
            .child(self.text)
            .refine_style(&self.style)
    }
}

/// Multi-select chip container (`ComboboxChips` in `combobox.tsx`).
#[derive(IntoElement)]
pub struct ComboboxChips {
    focused: bool,
    invalid: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ComboboxChips {
    pub fn new() -> Self {
        Self {
            focused: false,
            invalid: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
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

impl Default for ComboboxChips {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ComboboxChips {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ComboboxChips {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ComboboxChips {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let border_c = if self.invalid {
            tokens.destructive
        } else if self.focused {
            tokens.ring
        } else {
            tokens.input
        };

        div()
            .flex()
            .min_h(px(40.0))
            .flex_wrap()
            .items_center()
            .gap(px(6.0))
            .rounded(PresetRadius::NONE)
            .border_b_1()
            .border_color(border_c)
            .py(px(6.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Selected tag chip (`ComboboxChip` in `combobox.tsx`).
#[derive(IntoElement)]
pub struct ComboboxChip {
    id: ElementId,
    label: SharedString,
    show_remove: bool,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_remove: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl ComboboxChip {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            show_remove: true,
            disabled: false,
            theme: None,
            on_remove: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn show_remove(mut self, show: bool) -> Self {
        self.show_remove = show;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_remove(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_remove = Some(Rc::new(handler));
        self
    }
}

impl Styled for ComboboxChip {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ComboboxChip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut chip = div()
            .id(self.id.clone())
            .flex()
            .h(px(22.0))
            .items_center()
            .gap(px(4.0))
            .rounded(PresetRadius::NONE)
            .bg(tokens.muted)
            .pl(px(8.0))
            .pr(px(if self.show_remove { 0.0 } else { 8.0 }))
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .whitespace_nowrap()
            .text_color(tokens.foreground)
            .preset_disabled(self.disabled)
            .child(self.label);

        if self.show_remove {
            let mut rm = Button::new(ElementId::Name(format!("{}-rm", self.id).into()))
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::IconXs)
                .icon(AppIcon::Close)
                .disabled(self.disabled)
                .theme(&theme)
                .w(px(20.0))
                .h(px(20.0));
            if let Some(on_rm) = self.on_remove {
                rm = rm.on_click(move |ev, window, cx| on_rm(ev, window, cx));
            }
            chip = chip.child(rm);
        }

        chip.refine_style(&self.style)
    }
}

/// Inline borderless text input inside [`ComboboxChips`] (`ComboboxChipsInput` in `combobox.tsx`).
pub fn combobox_chips_input(
    state: &Entity<InputState>,
    theme: &SemanticTheme,
) -> crate::common::input::Input {
    input_group_input(state, theme).min_w(px(64.0))
}

/// Interactive filterable combobox root (`Combobox` in `combobox.tsx`).
#[derive(IntoElement)]
pub struct Combobox {
    id: ElementId,
    input_state: Entity<InputState>,
    items: Vec<ComboboxItem>,
    value: Option<SharedString>,
    empty_text: SharedString,
    disabled: bool,
    invalid: bool,
    show_clear: bool,
    theme: Option<SemanticTheme>,
    on_value_change: Option<Rc<dyn Fn(&Option<SharedString>, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Combobox {
    pub fn new(id: impl Into<ElementId>, input_state: &Entity<InputState>) -> Self {
        Self {
            id: id.into(),
            input_state: input_state.clone(),
            items: Vec::new(),
            value: None,
            empty_text: "No results".into(),
            disabled: false,
            invalid: false,
            show_clear: false,
            theme: None,
            on_value_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn items(mut self, items: impl IntoIterator<Item = ComboboxItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn value(mut self, value: Option<SharedString>) -> Self {
        self.value = value;
        self
    }

    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
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

    pub fn show_clear(mut self, show: bool) -> Self {
        self.show_clear = show;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_value_change(
        mut self,
        handler: impl Fn(&Option<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Combobox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Combobox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let open_state = window.use_keyed_state(
            ElementId::Name(format!("{}-combo-open", self.id).into()),
            cx,
            |_w, _cx| false,
        );
        let is_open = *open_state.read(cx);
        let query = self.input_state.read(cx).value().to_lowercase();

        let open_trig = open_state.clone();
        let open_clear = open_state.clone();
        let inp_clear = self.input_state.clone();
        let on_change_clear = self.on_value_change.clone();
        let has_val = self.value.as_ref().is_some_and(|v| !v.is_empty());

        let combo_input = ComboboxInput::new(
            ElementId::Name(format!("{}-combo-inp", self.id).into()),
            &self.input_state,
        )
        .disabled(self.disabled)
        .invalid(self.invalid)
        .show_clear(self.show_clear && has_val)
        .theme(&theme)
        .on_trigger_click(move |_ev, _window, cx| {
            open_trig.update(cx, |v, cx| {
                *v = !*v;
                cx.notify();
            });
        })
        .on_clear_click(move |_ev, window, cx| {
            inp_clear.update(cx, |st, cx| {
                st.set_value("", window, cx);
            });
            open_clear.update(cx, |v, cx| {
                *v = false;
                cx.notify();
            });
            if let Some(cb) = &on_change_clear {
                cb(&None, window, cx);
            }
        });

        let open_key = open_state.clone();
        let mut root = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_col()
            .on_key_down(move |ev: &KeyDownEvent, _window, cx| {
                if ev.keystroke.key == "escape" {
                    open_key.update(cx, |v, cx| {
                        *v = false;
                        cx.notify();
                    });
                } else {
                    open_key.update(cx, |v, cx| {
                        if !*v {
                            *v = true;
                            cx.notify();
                        }
                    });
                }
            })
            .refine_style(&self.style)
            .child(combo_input);

        if is_open && !self.disabled {
            let filtered: Vec<ComboboxItem> = self
                .items
                .into_iter()
                .filter(|it| query.is_empty() || it.label.to_lowercase().contains(&query))
                .collect();

            let mut list =
                ComboboxList::new(ElementId::Name(format!("{}-combo-list", self.id).into()));

            if filtered.is_empty() {
                list = list.child(ComboboxEmpty::new(self.empty_text).theme(&theme));
            } else {
                for item in filtered {
                    let is_sel = self.value.as_ref().is_some_and(|v| v == &item.value);
                    let open_sel = open_state.clone();
                    let inp_sel = self.input_state.clone();
                    let on_change_sel = self.on_value_change.clone();
                    let item_lbl = item.label.clone();

                    list = list.child(item.selected(is_sel).theme(&theme).on_select(
                        move |val, window, cx| {
                            inp_sel.update(cx, |st, cx| {
                                st.set_value(item_lbl.clone(), window, cx);
                            });
                            open_sel.update(cx, |v, cx| {
                                *v = false;
                                cx.notify();
                            });
                            if let Some(cb) = &on_change_sel {
                                cb(&Some(val.clone()), window, cx);
                            }
                        },
                    ));
                }
            }

            let open_out = open_state;
            let popup = apply_anchored_position(
                div()
                    .id(ElementId::Name(format!("{}-combo-pos", self.id).into()))
                    .absolute()
                    .w_full(),
                PopoverSide::Bottom,
                6.0,
                PopoverAlign::Start,
                0.0,
            )
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_down_out(move |_ev, _window, cx| {
                open_out.update(cx, |v, cx| {
                    *v = false;
                    cx.notify();
                });
            })
            .child(ComboboxContent::new().theme(&theme).w_full().child(list));

            root = root.child(deferred(popup).with_priority(5));
        }

        root
    }
}

//! Custom rich select dropdown primitives ported from `web/src/components/ui/select.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `SelectTrigger`: `h-10` (`Default`) | `h-9` (`Sm`), `border-b border-b-input bg-transparent px-0 py-2 text-sm`
//!   (`focus-visible:border-b-ring`, `aria-invalid:border-b-destructive`, `AppIcon::ChevronDown`)
//! - `SelectContent`: `min-w-36 rounded-none bg-popover text-popover-foreground shadow-md ring-1 ring-foreground/10`
//! - `SelectGroup`: `p-1.5`
//! - `SelectLabel`: `px-3 py-2 text-xs font-semibold uppercase text-muted-foreground`
//! - `SelectItem`: `py-2 pr-8 pl-3 text-sm` with `AppIcon::Check` indicator at `right-2`
//! - `SelectSeparator`: `-mx-1.5 my-1.5 h-px bg-border/50`
//! - `SelectScrollUpButton` & `SelectScrollDownButton`: `AppIcon::ChevronUp` / `AppIcon::ChevronDown`.

use std::rc::Rc;

pub use gpui_kit::component::select::{
    Select as KitSelect, SelectDelegate as KitSelectDelegate, SelectEvent as KitSelectEvent,
    SelectState as KitSelectState,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, KeyDownEvent, MouseButton, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, deferred, div, px,
};

use crate::common::popover::{PopoverAlign, PopoverSide, apply_anchored_position};
use crate::common::utils::{
    PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow, to_preset_uppercase,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Trigger height preset matching `size = "default" | "sm"` in `select.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectTriggerSize {
    #[default]
    Default,
    Sm,
}

impl SelectTriggerSize {
    pub const fn height_px(self) -> f32 {
        match self {
            Self::Default => 40.0,
            Self::Sm => 36.0,
        }
    }
}

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

/// Selected value or placeholder text slot (`SelectValue` in `select.tsx`).
#[derive(IntoElement)]
pub struct SelectValue {
    text: Option<SharedString>,
    placeholder: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SelectValue {
    pub fn new() -> Self {
        Self {
            text: None,
            placeholder: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn value(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for SelectValue {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SelectValue {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SelectValue {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SelectValue {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_placeholder = self.text.as_ref().is_none_or(|t| t.is_empty());

        let mut el = div()
            .flex()
            .flex_1()
            .items_center()
            .gap(px(6.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(if is_placeholder {
                tokens.muted_foreground
            } else {
                tokens.foreground
            });

        if let Some(val) = self.text.filter(|t| !t.is_empty()) {
            el = el.child(val);
        } else if let Some(ph) = self.placeholder {
            el = el.child(ph);
        }

        el.children(self.children).refine_style(&self.style)
    }
}

/// Underline trigger button (`SelectTrigger` in `select.tsx`).
#[derive(IntoElement)]
pub struct SelectTrigger {
    id: ElementId,
    size: SelectTriggerSize,
    disabled: bool,
    invalid: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SelectTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: SelectTriggerSize::Default,
            disabled: false,
            invalid: false,
            focus_handle: None,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: SelectTriggerSize) -> Self {
        self.size = size;
        self
    }

    pub fn sm(self) -> Self {
        self.size(SelectTriggerSize::Sm)
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

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for SelectTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SelectTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SelectTrigger {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));

        let border_bottom = if self.invalid {
            if tokens.is_dark {
                tokens.destructive.opacity(0.5)
            } else {
                tokens.destructive
            }
        } else if is_focused {
            tokens.ring
        } else {
            tokens.input
        };

        let mut trig = div()
            .id(self.id)
            .flex()
            .h(px(self.size.height_px()))
            .items_center()
            .justify_between()
            .gap(px(6.0))
            .rounded(PresetRadius::NONE)
            .border_b_1()
            .border_color(border_bottom)
            .px(px(0.0))
            .py(px(8.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .whitespace_nowrap()
            .preset_disabled(self.disabled);

        if let Some(fh) = &self.focus_handle {
            trig = trig.track_focus(fh);
        }

        if !self.disabled {
            trig = trig.cursor_pointer();
            if let Some(on_click) = self.on_click {
                trig = trig.on_click(move |ev, window, cx| on_click(ev, window, cx));
            }
        }

        trig.children(self.children)
            .child(app_icon(
                AppIcon::ChevronDown,
                IconSize::Sm,
                IconTone::Muted,
                &theme,
            ))
            .refine_style(&self.style)
    }
}

/// Option group container (`SelectGroup` in `select.tsx`).
#[derive(IntoElement)]
pub struct SelectGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SelectGroup {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SelectGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SelectGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SelectGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SelectGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .p(px(6.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Group heading inside [`SelectGroup`] (`SelectLabel` in `select.tsx`).
#[derive(IntoElement)]
pub struct SelectLabel {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl SelectLabel {
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

impl Styled for SelectLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SelectLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .px(px(12.0))
            .py(px(8.0))
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.muted_foreground)
            .child(to_preset_uppercase(self.text.as_ref()))
            .refine_style(&self.style)
    }
}

/// Selectable option row (`SelectItem` in `select.tsx`).
#[derive(Clone, IntoElement)]
pub struct SelectItem {
    pub(crate) id: ElementId,
    pub(crate) value: SharedString,
    pub(crate) label: SharedString,
    pub(crate) icon: Option<AppIcon>,
    pub(crate) selected: bool,
    pub(crate) highlighted: bool,
    pub(crate) disabled: bool,
    theme: Option<SemanticTheme>,
    on_select: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl SelectItem {
    pub fn new(
        id: impl Into<ElementId>,
        value: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            label: label.into(),
            icon: None,
            selected: false,
            highlighted: false,
            disabled: false,
            theme: None,
            on_select: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
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

    pub fn on_select(
        mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl Styled for SelectItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SelectItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let hover_bg = tokens.accent;
        let hover_fg = tokens.accent_foreground;

        let mut row = div()
            .id(self.id)
            .relative()
            .flex()
            .w_full()
            .items_center()
            .gap(px(10.0))
            .rounded(PresetRadius::NONE)
            .py(px(8.0))
            .pl(px(12.0))
            .pr(px(32.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(if self.highlighted {
                tokens.accent_foreground
            } else {
                tokens.popover_foreground
            })
            .cx_when(self.highlighted, |d| d.bg(tokens.accent))
            .cx_when(!self.disabled, |d| {
                d.cursor_pointer()
                    .hover(move |s| s.bg(hover_bg).text_color(hover_fg))
            })
            .preset_disabled(self.disabled);

        if let Some(icon) = self.icon {
            row = row.child(app_icon(icon, IconSize::Sm, IconTone::Muted, &theme));
        }

        row = row.child(
            div()
                .flex()
                .flex_1()
                .whitespace_nowrap()
                .child(self.label),
        );

        if self.selected {
            row = row.child(
                div()
                    .absolute()
                    .right(px(8.0))
                    .flex()
                    .w(px(16.0))
                    .h(px(16.0))
                    .items_center()
                    .justify_center()
                    .child(app_icon(
                        AppIcon::Check,
                        IconSize::Sm,
                        IconTone::Default,
                        &theme,
                    )),
            );
        }

        if !self.disabled
            && let Some(on_select) = self.on_select
        {
            let value = self.value;
            row = row.on_click(move |_ev, window, cx| on_select(&value, window, cx));
        }

        row.refine_style(&self.style)
    }
}

/// Thin divider inside [`SelectContent`] (`SelectSeparator` in `select.tsx`).
#[derive(IntoElement)]
pub struct SelectSeparator {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl SelectSeparator {
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

impl Default for SelectSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SelectSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SelectSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .mx(px(-6.0))
            .my(px(6.0))
            .h(px(1.0))
            .bg(tokens.border.opacity(0.5))
            .refine_style(&self.style)
    }
}

/// Scroll-up indicator (`SelectScrollUpButton` in `select.tsx`).
#[derive(IntoElement)]
pub struct SelectScrollUpButton {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl SelectScrollUpButton {
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

impl Default for SelectScrollUpButton {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SelectScrollUpButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SelectScrollUpButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .w_full()
            .items_center()
            .justify_center()
            .bg(tokens.popover)
            .py(px(4.0))
            .child(app_icon(
                AppIcon::ChevronUp,
                IconSize::Sm,
                IconTone::Muted,
                &theme,
            ))
            .refine_style(&self.style)
    }
}

/// Scroll-down indicator (`SelectScrollDownButton` in `select.tsx`).
#[derive(IntoElement)]
pub struct SelectScrollDownButton {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl SelectScrollDownButton {
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

impl Default for SelectScrollDownButton {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SelectScrollDownButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SelectScrollDownButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .w_full()
            .items_center()
            .justify_center()
            .bg(tokens.popover)
            .py(px(4.0))
            .child(app_icon(
                AppIcon::ChevronDown,
                IconSize::Sm,
                IconTone::Muted,
                &theme,
            ))
            .refine_style(&self.style)
    }
}

/// Floating option list popup (`SelectContent` in `select.tsx`).
#[derive(IntoElement)]
pub struct SelectContent {
    id: ElementId,
    side: PopoverSide,
    side_offset: f32,
    align: PopoverAlign,
    align_offset: f32,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SelectContent {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            side: PopoverSide::Bottom,
            side_offset: 4.0,
            align: PopoverAlign::Start,
            align_offset: 0.0,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for SelectContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SelectContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SelectContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .min_w(px(144.0))
            .max_h(px(288.0))
            .overflow_y_scroll()
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .p(px(6.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
            .refine_style(&self.style)
            .children(self.children)
    }
}

#[derive(Debug, Clone, Default)]
struct SelectDropdownState {
    open: bool,
    selected_value: Option<SharedString>,
    highlighted_index: usize,
}

/// Interactive rich select root (`Select` in `select.tsx`).
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    items: Vec<SelectItem>,
    value: Option<SharedString>,
    default_value: Option<SharedString>,
    placeholder: Option<SharedString>,
    size: SelectTriggerSize,
    disabled: bool,
    invalid: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_value_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Select {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            value: None,
            default_value: None,
            placeholder: None,
            size: SelectTriggerSize::Default,
            disabled: false,
            invalid: false,
            focus_handle: None,
            theme: None,
            on_value_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn item(mut self, item: SelectItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = SelectItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<SharedString>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn size(mut self, size: SelectTriggerSize) -> Self {
        self.size = size;
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

    pub fn on_value_change(
        mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Select {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let is_controlled = self.value.is_some();
        let init_val = self.default_value.clone();

        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-select-state", self.id).into()),
            cx,
            |_w, _cx| SelectDropdownState {
                open: false,
                selected_value: init_val,
                highlighted_index: 0,
            },
        );

        let snap = state.read(cx).clone();
        let active_val = self.value.or(snap.selected_value);
        let selected_label = active_val.as_ref().and_then(|v| {
            self.items
                .iter()
                .find(|it| &it.value == v)
                .map(|it| it.label.clone())
        });

        let mut val_el = SelectValue::new().theme(&theme);
        if let Some(lbl) = selected_label {
            val_el = val_el.value(lbl);
        }
        if let Some(ph) = self.placeholder {
            val_el = val_el.placeholder(ph);
        }

        let state_trig = state.clone();
        let mut trigger = SelectTrigger::new(ElementId::Name(
            format!("{}-select-trig", self.id).into(),
        ))
        .size(self.size)
        .disabled(self.disabled)
        .invalid(self.invalid)
        .theme(&theme)
        .w_full()
        .child(val_el);

        if let Some(fh) = &self.focus_handle {
            trigger = trigger.track_focus(fh);
        }

        if !self.disabled {
            trigger = trigger.on_click(move |_ev, _window, cx| {
                state_trig.update(cx, |s, cx| {
                    s.open = !s.open;
                    cx.notify();
                });
            });
        }

        let item_count = self.items.len();
        let state_key = state.clone();
        let on_change_key = self.on_value_change.clone();
        let items_for_key = self.items.clone();

        let mut root = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_col()
            .refine_style(&self.style)
            .on_key_down(move |ev: &KeyDownEvent, window, cx| {
                let key = ev.keystroke.key.as_str();
                match key {
                    "escape" => {
                        state_key.update(cx, |s, cx| {
                            s.open = false;
                            cx.notify();
                        });
                    }
                    "down" if item_count > 0 => {
                        state_key.update(cx, |s, cx| {
                            s.open = true;
                            s.highlighted_index = (s.highlighted_index + 1) % item_count;
                            cx.notify();
                        });
                    }
                    "up" if item_count > 0 => {
                        state_key.update(cx, |s, cx| {
                            s.open = true;
                            s.highlighted_index = if s.highlighted_index == 0 {
                                item_count - 1
                            } else {
                                s.highlighted_index - 1
                            };
                            cx.notify();
                        });
                    }
                    "enter" | "space" | " " if item_count > 0 => {
                        let is_open = state_key.read(cx).open;
                        if is_open {
                            let idx = state_key.read(cx).highlighted_index.min(item_count - 1);
                            if let Some(chosen) = items_for_key.get(idx)
                                && !chosen.disabled
                            {
                                let val = chosen.value.clone();
                                state_key.update(cx, |s, cx| {
                                    if !is_controlled {
                                        s.selected_value = Some(val.clone());
                                    }
                                    s.open = false;
                                    cx.notify();
                                });
                                if let Some(cb) = &on_change_key {
                                    cb(&val, window, cx);
                                }
                            }
                        } else {
                            state_key.update(cx, |s, cx| {
                                s.open = true;
                                cx.notify();
                            });
                        }
                    }
                    _ => {}
                }
            })
            .child(trigger);

        if snap.open && !self.disabled {
            let mut content = SelectContent::new(ElementId::Name(
                format!("{}-select-content", self.id).into(),
            ))
            .w_full()
            .theme(&theme);

            for (idx, item) in self.items.into_iter().enumerate() {
                let is_sel = active_val.as_ref().is_some_and(|v| v == &item.value);
                let state_item = state.clone();
                let on_change_item = self.on_value_change.clone();

                let rendered = item
                    .selected(is_sel)
                    .highlighted(idx == snap.highlighted_index)
                    .theme(&theme)
                    .on_select(move |val, window, cx| {
                        state_item.update(cx, |s, cx| {
                            if !is_controlled {
                                s.selected_value = Some(val.clone());
                            }
                            s.highlighted_index = idx;
                            s.open = false;
                            cx.notify();
                        });
                        if let Some(cb) = &on_change_item {
                            cb(val, window, cx);
                        }
                    });
                content = content.child(rendered);
            }

            let state_out = state;
            let popup = apply_anchored_position(
                div()
                    .id(ElementId::Name(format!("{}-select-pos", self.id).into()))
                    .absolute()
                    .w_full(),
                PopoverSide::Bottom,
                4.0,
                PopoverAlign::Start,
                0.0,
            )
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_down_out(move |_ev, _window, cx| {
                state_out.update(cx, |s, cx| {
                    s.open = false;
                    cx.notify();
                });
            })
            .child(content);

            root = root.child(deferred(popup).with_priority(5));
        }

        root
    }
}

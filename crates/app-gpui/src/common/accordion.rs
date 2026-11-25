//! Vertically stacked disclosure accordion ported from `web/src/components/ui/accordion.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Accordion`: `flex w-full flex-col`, supporting `Single` and `Multiple` open items (controlled or uncontrolled)
//! - `AccordionItem`: `not-last:border-b border-border`
//! - `AccordionTrigger`: `flex flex-1 items-start justify-between gap-6 rounded-none py-4 text-left text-sm font-semibold hover:underline`
//!   with `AppIcon::ChevronDown` / `AppIcon::ChevronUp` (`IconSize::Sm`, `IconTone::Muted`)
//! - `AccordionContent`: `pt-0 pb-4 text-sm`.

use std::rc::Rc;

pub use gpui_kit::component::accordion::{
    Accordion as KitAccordion, AccordionItem as KitAccordionItem,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ElementId, FocusHandle, FontWeight, InteractiveElement, IntoElement,
    KeyDownEvent, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Expansion behavior for [`Accordion`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionMode {
    #[default]
    Single,
    Multiple,
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

/// Vertically stacked disclosure group (`Accordion` in `web/src/components/ui/accordion.tsx`).
#[derive(IntoElement)]
pub struct Accordion {
    id: ElementId,
    mode: AccordionMode,
    items: Vec<AccordionItem>,
    open_indices: Option<Vec<usize>>,
    default_open: Vec<usize>,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_change: Option<Rc<dyn Fn(&[usize], &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Accordion {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            mode: AccordionMode::Single,
            items: Vec::new(),
            open_indices: None,
            default_open: Vec::new(),
            disabled: false,
            theme: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn multiple(mut self) -> Self {
        self.mode = AccordionMode::Multiple;
        self
    }

    pub fn mode(mut self, mode: AccordionMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn item(mut self, item: AccordionItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = AccordionItem>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn open_indices(mut self, indices: impl IntoIterator<Item = usize>) -> Self {
        self.open_indices = Some(indices.into_iter().collect());
        self
    }

    pub fn default_open(mut self, indices: impl IntoIterator<Item = usize>) -> Self {
        self.default_open = indices.into_iter().collect();
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

    pub fn on_change(
        mut self,
        handler: impl Fn(&[usize], &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Accordion {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Accordion {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let initial = self.default_open.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| initial);
        let is_controlled = self.open_indices.is_some();
        let active: Vec<usize> = self
            .open_indices
            .clone()
            .unwrap_or_else(|| state.read(cx).clone());

        let total = self.items.len();
        let mode = self.mode;

        let mut col = div()
            .flex()
            .flex_col()
            .w_full()
            .refine_style(&self.style);

        for (idx, mut item) in self.items.into_iter().enumerate() {
            let is_open = item.open_override.unwrap_or_else(|| active.contains(&idx));
            let is_last = idx + 1 == total;
            item.disabled = self.disabled || item.disabled;

            let cur_active = active.clone();
            let st = state.clone();
            let cb = self.on_change.clone();

            let rendered = item.render_section(
                idx,
                is_open,
                !is_last,
                tokens.border,
                &theme,
                move |window, cx| {
                    let mut next = cur_active.clone();
                    if next.contains(&idx) {
                        next.retain(|&x| x != idx);
                    } else {
                        match mode {
                            AccordionMode::Single => {
                                next.clear();
                                next.push(idx);
                            }
                            AccordionMode::Multiple => next.push(idx),
                        }
                    }
                    if !is_controlled {
                        let stored = next.clone();
                        st.update(cx, |v, cx| {
                            *v = stored;
                            cx.notify();
                        });
                    }
                    if let Some(ref handler) = cb {
                        handler(&next, window, cx);
                    }
                },
            );
            col = col.child(rendered);
        }

        col
    }
}

/// Collapsible section inside [`Accordion`] (`AccordionItem` in `accordion.tsx`).
pub struct AccordionItem {
    trigger: AccordionTrigger,
    content: AccordionContent,
    disabled: bool,
    open_override: Option<bool>,
    style: StyleRefinement,
}

impl AccordionItem {
    pub fn new(trigger: AccordionTrigger, content: AccordionContent) -> Self {
        Self {
            trigger,
            content,
            disabled: false,
            open_override: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn simple(
        title: impl Into<SharedString>,
        body: impl IntoElement,
    ) -> Self {
        Self::new(
            AccordionTrigger::new(title),
            AccordionContent::new().child(body),
        )
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open_override = Some(open);
        self
    }

    fn render_section(
        self,
        idx: usize,
        is_open: bool,
        has_bottom_border: bool,
        border_color: gpui_kit::Hsla,
        theme: &SemanticTheme,
        on_toggle: impl Fn(&mut Window, &mut App) + 'static,
    ) -> impl IntoElement {
        let trig = self
            .trigger
            .index(idx)
            .open(is_open)
            .disabled(self.disabled)
            .theme(theme)
            .on_toggle(on_toggle);

        let mut section = div()
            .flex()
            .flex_col()
            .w_full()
            .cx_when(has_bottom_border, |d| {
                d.border_b_1().border_color(border_color)
            })
            .refine_style(&self.style)
            .child(trig);

        if is_open {
            section = section.child(self.content.theme(theme));
        }

        section
    }
}

impl Styled for AccordionItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// Interactive header button toggling an [`AccordionItem`] (`AccordionTrigger` in `accordion.tsx`).
#[derive(IntoElement)]
pub struct AccordionTrigger {
    index: usize,
    title: Option<SharedString>,
    open: bool,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_toggle: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl AccordionTrigger {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            index: 0,
            title: Some(title.into()),
            open: false,
            disabled: false,
            focus_handle: None,
            theme: None,
            on_toggle: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
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

    pub fn on_toggle(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl Styled for AccordionTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AccordionTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AccordionTrigger {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let chevron = if self.open {
            AppIcon::ChevronUp
        } else {
            AppIcon::ChevronDown
        };

        let mut head = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0));

        if let Some(title) = self.title {
            head = head.child(title);
        }
        head = head.children(self.children);

        let mut btn = div()
            .id(ElementId::NamedInteger(
                "accordion-trigger".into(),
                self.index as u64,
            ))
            .flex()
            .flex_row()
            .w_full()
            .items_start()
            .justify_between()
            .gap(px(24.0)) // gap-6
            .py(px(16.0)) // py-4
            .rounded(PresetRadius::NONE)
            .font_family(FONT_FAMILY)
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground)
            .preset_ring(is_focused, false, &theme)
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .child(head)
            .child(
                div()
                    .ml_auto()
                    .pt(px(2.0))
                    .flex_shrink_0()
                    .child(app_icon(chevron, IconSize::Sm, IconTone::Muted, &theme)),
            );

        if let Some(ref fh) = self.focus_handle {
            btn = btn.track_focus(fh);
        }

        if !self.disabled {
            btn = btn.cursor_pointer().hover(|s| s.underline());
            if let Some(cb) = self.on_toggle {
                let click_cb = cb.clone();
                btn = btn.on_click(move |_, window, cx| {
                    click_cb(window, cx);
                });
                btn = btn.on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    if key == "enter" || key == "space" {
                        cb(window, cx);
                    }
                });
            }
        }

        btn
    }
}

/// Revealed panel body inside [`AccordionItem`] (`AccordionContent` in `accordion.tsx`).
#[derive(IntoElement)]
pub struct AccordionContent {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for AccordionContent {
    fn default() -> Self {
        Self::new()
    }
}

impl AccordionContent {
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

impl Styled for AccordionContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AccordionContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AccordionContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .w_full()
            .pt(px(0.0))
            .pb(px(16.0)) // pb-4
            .overflow_hidden()
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.foreground)
            .refine_style(&self.style)
            .children(self.children)
    }
}

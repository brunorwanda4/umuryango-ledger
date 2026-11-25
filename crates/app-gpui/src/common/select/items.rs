//! Option group, label, item, separator, and scroll indicator primitives for [`super::Select`].

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use super::resolve_theme;
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme};

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

        row = row.child(div().flex().flex_1().whitespace_nowrap().child(self.label));

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

/// Thin divider inside `SelectContent` (`SelectSeparator` in `select.tsx`).
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

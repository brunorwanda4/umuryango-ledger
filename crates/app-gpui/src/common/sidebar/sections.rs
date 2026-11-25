//! Structural sections (`SidebarInput`, `SidebarHeader`, `SidebarFooter`,
//! `SidebarSeparator`, `SidebarContent`, `SidebarGroup`, `SidebarGroupLabel`,
//! `SidebarGroupAction`, and `SidebarGroupContent`) for [`super::Sidebar`].

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, Entity, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px,
};

use super::resolve_theme;
use crate::common::input::{Input, InputState};
use crate::common::separator::Separator;
use crate::common::utils::{PresetRadius, PresetTokens, focus_ring_shadow, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme};

/// Compact `32px`-tall filter/search input inside [`SidebarHeader`] (`data-slot="sidebar-input"`).
#[derive(IntoElement)]
pub struct SidebarInput {
    id: Option<ElementId>,
    state: Entity<InputState>,
    disabled: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl SidebarInput {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            id: None,
            state: state.clone(),
            disabled: false,
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

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for SidebarInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut input = Input::new(&self.state)
            .disabled(self.disabled)
            .theme(&theme)
            .h(px(32.0))
            .w_full();
        if let Some(id) = self.id {
            input = input.id(id);
        }
        input.refine_style(&self.style)
    }
}

/// Sticky top section of [`super::Sidebar`] (`data-slot="sidebar-header"`, `flex flex-col gap-2 p-2`).
#[derive(IntoElement)]
pub struct SidebarHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarHeader {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .p(px(8.0))
            .rounded(PresetRadius::NONE)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Sticky bottom section of [`super::Sidebar`] (`data-slot="sidebar-footer"`, `flex flex-col gap-2 p-2`).
#[derive(IntoElement)]
pub struct SidebarFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarFooter {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .p(px(8.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Horizontal divider inside [`super::Sidebar`] (`data-slot="sidebar-separator"`).
#[derive(IntoElement)]
pub struct SidebarSeparator {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl SidebarSeparator {
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

impl Default for SidebarSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        div()
            .mx(px(8.0))
            .refine_style(&self.style)
            .child(Separator::horizontal().theme(&theme))
    }
}

/// Scrollable middle body of [`super::Sidebar`] (`data-slot="sidebar-content"`).
#[derive(IntoElement)]
pub struct SidebarContent {
    id: ElementId,
    icon_collapsed: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarContent {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon_collapsed: false,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon_collapsed(mut self, collapsed: bool) -> Self {
        self.icon_collapsed = collapsed;
        self
    }
}

impl Styled for SidebarContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .gap(px(8.0))
            .rounded(PresetRadius::NONE)
            .when_else(
                self.icon_collapsed,
                |d| d.overflow_hidden(),
                |d| d.overflow_y_scroll(),
            )
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Logical section inside [`SidebarContent`] (`data-slot="sidebar-group"`).
#[derive(IntoElement)]
pub struct SidebarGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarGroup {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .p(px(8.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Editorial uppercase section header (`data-slot="sidebar-group-label"`).
#[derive(IntoElement)]
pub struct SidebarGroupLabel {
    text: Option<SharedString>,
    icon_collapsed: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarGroupLabel {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            icon_collapsed: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon_collapsed(mut self, collapsed: bool) -> Self {
        self.icon_collapsed = collapsed;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for SidebarGroupLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarGroupLabel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarGroupLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.icon_collapsed {
            return div().into_any_element();
        }

        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .flex()
            .h(px(32.0))
            .flex_shrink_0()
            .items_center()
            .gap(px(6.0))
            .rounded(PresetRadius::NONE)
            .px(px(12.0))
            .font_family(FONT_FAMILY)
            .text_size(px(12.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.card_foreground.opacity(0.7));

        if let Some(text) = self.text {
            el = el.child(to_preset_uppercase(text.as_ref()));
        }

        el.refine_style(&self.style)
            .children(self.children)
            .into_any_element()
    }
}

/// Trailing `20x20` icon action inside [`SidebarGroup`] (`data-slot="sidebar-group-action"`).
#[derive(IntoElement)]
pub struct SidebarGroupAction {
    id: ElementId,
    icon: Option<AppIcon>,
    icon_collapsed: bool,
    focus_handle: Option<FocusHandle>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarGroupAction {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            icon_collapsed: false,
            focus_handle: None,
            on_click: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_collapsed(mut self, collapsed: bool) -> Self {
        self.icon_collapsed = collapsed;
        self
    }

    pub fn track_focus(mut self, handle: &FocusHandle) -> Self {
        self.focus_handle = Some(handle.clone());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for SidebarGroupAction {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarGroupAction {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarGroupAction {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.icon_collapsed {
            return div().into_any_element();
        }

        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let hover_bg = tokens.accent;
        let hover_fg = tokens.accent_foreground;

        let mut el = div()
            .id(self.id)
            .absolute()
            .top(px(14.0))
            .right(px(12.0))
            .flex()
            .size(px(20.0))
            .items_center()
            .justify_center()
            .rounded(PresetRadius::NONE)
            .text_color(tokens.card_foreground)
            .cursor_pointer()
            .hover(move |s| s.bg(hover_bg).text_color(hover_fg))
            .when(is_focused, |d| d.shadow(focus_ring_shadow(&theme)));

        if let Some(icon) = self.icon {
            el = el.child(app_icon(icon, IconSize::Sm, IconTone::Muted, &theme));
        }
        if let Some(ref fh) = self.focus_handle {
            el = el.track_focus(fh);
        }
        if let Some(cb) = self.on_click {
            el = el.on_click(move |ev, win, app| cb(ev, win, app));
        }

        el.refine_style(&self.style)
            .children(self.children)
            .into_any_element()
    }
}

/// Content body of a [`SidebarGroup`] (`data-slot="sidebar-group-content"`).
#[derive(IntoElement)]
pub struct SidebarGroupContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarGroupContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarGroupContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarGroupContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarGroupContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarGroupContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

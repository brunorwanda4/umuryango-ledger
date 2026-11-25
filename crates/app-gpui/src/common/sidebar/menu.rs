//! Navigation menu, item, button, action, badge, skeleton, and nested sub-menu primitives
//! for [`super::Sidebar`] ported from `web/src/components/ui/sidebar.tsx`.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, KeyDownEvent, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px, relative, transparent_black,
};

use super::resolve_theme;
use crate::common::skeleton::Skeleton;
use crate::common::tooltip::{TooltipContent, TooltipSide};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, focus_ring_shadow};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::hooks::use_mobile::is_mobile;
use crate::theme::{FONT_FAMILY, SemanticTheme};

/// Visual style of [`SidebarMenuButton`] (`sidebarMenuButtonVariants.variant` in `sidebar.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarMenuButtonVariant {
    #[default]
    Default,
    Outline,
}

/// Height and typography preset of [`SidebarMenuButton`] (`sidebarMenuButtonVariants.size` in `sidebar.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarMenuButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
}

/// Typography preset of [`SidebarMenuSubButton`] (`size = "sm" | "md"` in `sidebar.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarMenuSubButtonSize {
    Sm,
    #[default]
    Md,
}

/// Vertical menu list (`data-slot="sidebar-menu"`, `flex w-full min-w-0 flex-col gap-0.5`).
#[derive(IntoElement)]
pub struct SidebarMenu {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarMenu {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarMenu {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .gap(px(2.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Relative row container inside [`SidebarMenu`] (`data-slot="sidebar-menu-item"`).
#[derive(IntoElement)]
pub struct SidebarMenuItem {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarMenuItem {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarMenuItem {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarMenuItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarMenuItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarMenuItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .relative()
            .flex()
            .flex_col()
            .min_w_0()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Interactive navigation button inside [`SidebarMenuItem`] (`data-slot="sidebar-menu-button"`).
#[derive(IntoElement)]
pub struct SidebarMenuButton {
    id: ElementId,
    variant: SidebarMenuButtonVariant,
    size: SidebarMenuButtonSize,
    is_active: bool,
    disabled: bool,
    icon_collapsed: bool,
    has_action: bool,
    icon: Option<AppIcon>,
    label: Option<SharedString>,
    tooltip: Option<SharedString>,
    focus_handle: Option<FocusHandle>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarMenuButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: SidebarMenuButtonVariant::Default,
            size: SidebarMenuButtonSize::Default,
            is_active: false,
            disabled: false,
            icon_collapsed: false,
            has_action: false,
            icon: None,
            label: None,
            tooltip: None,
            focus_handle: None,
            on_click: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: SidebarMenuButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: SidebarMenuButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Collapses the button to a `32x32` icon tile and enables right-docked tooltip on desktop.
    pub fn icon_collapsed(mut self, collapsed: bool) -> Self {
        self.icon_collapsed = collapsed;
        self
    }

    /// Reserves right padding for a sibling [`SidebarMenuAction`] or [`SidebarMenuBadge`].
    pub fn has_action(mut self, has_action: bool) -> Self {
        self.has_action = has_action;
        self
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
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

impl Styled for SidebarMenuButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarMenuButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarMenuButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let mobile = is_mobile(window);

        let (height, font_size) = match self.size {
            SidebarMenuButtonSize::Default => (px(36.0), px(14.0)),
            SidebarMenuButtonSize::Sm => (px(32.0), px(12.0)),
            SidebarMenuButtonSize::Lg => (px(56.0), px(14.0)),
        };

        let base_bg = if self.is_active {
            tokens.accent
        } else if self.variant == SidebarMenuButtonVariant::Outline {
            tokens.background
        } else {
            transparent_black()
        };

        let base_fg = if self.is_active {
            tokens.accent_foreground
        } else {
            tokens.card_foreground
        };

        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));

        let mut btn = div()
            .id(self.id)
            .flex()
            .items_center()
            .gap(px(8.0))
            .overflow_hidden()
            .rounded(PresetRadius::NONE)
            .bg(base_bg)
            .text_color(base_fg)
            .font_family(FONT_FAMILY)
            .text_size(font_size)
            .when(self.is_active, |d| d.font_weight(FontWeight::MEDIUM))
            .when(self.variant == SidebarMenuButtonVariant::Outline, |d| {
                d.border_1().border_color(tokens.border)
            })
            .when_else(
                self.icon_collapsed,
                |d| {
                    d.size(px(32.0)).justify_center().when_else(
                        self.size == SidebarMenuButtonSize::Lg,
                        |lg| lg.p_0(),
                        |std| std.p(px(8.0)),
                    )
                },
                |d| {
                    d.w_full()
                        .h(height)
                        .pl(px(12.0))
                        .pr(if self.has_action { px(32.0) } else { px(12.0) })
                        .py(px(8.0))
                },
            )
            .preset_disabled(self.disabled)
            .when(is_focused, |d| d.shadow(focus_ring_shadow(&theme)));

        if let Some(icon) = self.icon {
            let tone = if self.is_active {
                IconTone::AccentPrimary
            } else {
                IconTone::Default
            };
            btn = btn.child(app_icon(icon, IconSize::Md, tone, &theme));
        }

        if !self.icon_collapsed {
            if let Some(label) = self.label {
                btn = btn.child(div().min_w_0().truncate().child(label));
            }
            btn = btn.children(self.children);
        } else if self.icon.is_none() {
            btn = btn.children(self.children);
        }

        if let Some(ref fh) = self.focus_handle {
            btn = btn.track_focus(fh);
        }

        if self.icon_collapsed
            && !mobile
            && let Some(tip) = self.tooltip
        {
            let tip_theme = theme.clone();
            btn = btn.tooltip(move |win, app| {
                TooltipContent::new(tip.clone())
                    .side(TooltipSide::Right)
                    .theme(&tip_theme)
                    .build(win, app)
            });
        }

        if !self.disabled {
            let hover_bg = tokens.accent;
            let hover_fg = tokens.accent_foreground;
            btn = btn
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg).text_color(hover_fg));
            if let Some(cb) = self.on_click {
                let click_cb = cb.clone();
                btn = btn.on_click(move |ev, win, app| click_cb(ev, win, app));
                btn = btn.on_key_down(move |ev: &KeyDownEvent, win, app| {
                    let k = ev.keystroke.key.as_str();
                    if k == "enter" || k == "space" {
                        cb(&ClickEvent::default(), win, app);
                    }
                });
            }
        }

        btn.refine_style(&self.style)
    }
}

/// Secondary icon button docked at the trailing edge of [`SidebarMenuItem`] (`data-slot="sidebar-menu-action"`).
#[derive(IntoElement)]
pub struct SidebarMenuAction {
    id: ElementId,
    icon: Option<AppIcon>,
    button_size: SidebarMenuButtonSize,
    show_on_hover: bool,
    icon_collapsed: bool,
    focus_handle: Option<FocusHandle>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarMenuAction {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            button_size: SidebarMenuButtonSize::Default,
            show_on_hover: false,
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

    pub fn button_size(mut self, size: SidebarMenuButtonSize) -> Self {
        self.button_size = size;
        self
    }

    pub fn show_on_hover(mut self, show_on_hover: bool) -> Self {
        self.show_on_hover = show_on_hover;
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

impl Styled for SidebarMenuAction {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarMenuAction {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarMenuAction {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.icon_collapsed {
            return div().into_any_element();
        }

        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let top_offset = match self.button_size {
            SidebarMenuButtonSize::Default => px(8.0),
            SidebarMenuButtonSize::Sm => px(4.0),
            SidebarMenuButtonSize::Lg => px(10.0),
        };
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let hover_bg = tokens.accent;
        let hover_fg = tokens.accent_foreground;

        let mut el = div()
            .id(self.id)
            .absolute()
            .top(top_offset)
            .right(px(4.0))
            .flex()
            .size(px(20.0))
            .items_center()
            .justify_center()
            .rounded(PresetRadius::NONE)
            .text_color(tokens.card_foreground)
            .cursor_pointer()
            .when(self.show_on_hover && !is_mobile(window) && !is_focused, |d| {
                d.opacity(0.65)
            })
            .hover(move |s| s.opacity(1.0).bg(hover_bg).text_color(hover_fg))
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

/// Numeric count badge docked inside [`SidebarMenuItem`] (`data-slot="sidebar-menu-badge"`).
#[derive(IntoElement)]
pub struct SidebarMenuBadge {
    text: Option<SharedString>,
    button_size: SidebarMenuButtonSize,
    icon_collapsed: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarMenuBadge {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            button_size: SidebarMenuButtonSize::Default,
            icon_collapsed: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn button_size(mut self, size: SidebarMenuButtonSize) -> Self {
        self.button_size = size;
        self
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

impl Styled for SidebarMenuBadge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarMenuBadge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarMenuBadge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.icon_collapsed {
            return div().into_any_element();
        }

        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let top_offset = match self.button_size {
            SidebarMenuButtonSize::Default => px(6.0),
            SidebarMenuButtonSize::Sm => px(4.0),
            SidebarMenuButtonSize::Lg => px(10.0),
        };

        let mut el = div()
            .absolute()
            .top(top_offset)
            .right(px(4.0))
            .flex()
            .h(px(20.0))
            .min_w(px(20.0))
            .items_center()
            .justify_center()
            .rounded(PresetRadius::NONE)
            .px(px(4.0))
            .font_family(FONT_FAMILY)
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.card_foreground);

        if let Some(text) = self.text {
            el = el.child(text);
        }

        el.refine_style(&self.style)
            .children(self.children)
            .into_any_element()
    }
}

/// Loading placeholder row inside [`SidebarMenu`] (`data-slot="sidebar-menu-skeleton"`).
#[derive(IntoElement)]
pub struct SidebarMenuSkeleton {
    show_icon: bool,
    width_ratio: f32,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl SidebarMenuSkeleton {
    pub fn new() -> Self {
        Self {
            show_icon: false,
            width_ratio: 0.72,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn show_icon(mut self, show_icon: bool) -> Self {
        self.show_icon = show_icon;
        self
    }

    /// Fractional width (`0.5..=0.9`) for the text skeleton bar.
    pub fn width_ratio(mut self, ratio: f32) -> Self {
        self.width_ratio = ratio.clamp(0.5, 0.9);
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for SidebarMenuSkeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarMenuSkeleton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarMenuSkeleton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);

        div()
            .flex()
            .h(px(32.0))
            .items_center()
            .gap(px(8.0))
            .rounded(PresetRadius::NONE)
            .px(px(8.0))
            .when(self.show_icon, |d| {
                d.child(Skeleton::new().theme(&theme).size(px(14.0)))
            })
            .child(
                Skeleton::new()
                    .theme(&theme)
                    .h(px(16.0))
                    .w(relative(self.width_ratio)),
            )
            .refine_style(&self.style)
    }
}

/// Indented left-bordered nested sub-menu list (`data-slot="sidebar-menu-sub"`).
#[derive(IntoElement)]
pub struct SidebarMenuSub {
    icon_collapsed: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarMenuSub {
    pub fn new() -> Self {
        Self {
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

impl Default for SidebarMenuSub {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarMenuSub {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarMenuSub {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarMenuSub {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.icon_collapsed {
            return div().into_any_element();
        }

        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .mx(px(14.0))
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(4.0))
            .border_l_1()
            .border_color(tokens.border)
            .px(px(10.0))
            .py(px(2.0))
            .refine_style(&self.style)
            .children(self.children)
            .into_any_element()
    }
}

/// Nested sub-menu item container (`data-slot="sidebar-menu-sub-item"`).
#[derive(IntoElement)]
pub struct SidebarMenuSubItem {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarMenuSubItem {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarMenuSubItem {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarMenuSubItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarMenuSubItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarMenuSubItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .relative()
            .min_w_0()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Interactive nested link/button inside [`SidebarMenuSubItem`] (`data-slot="sidebar-menu-sub-button"`).
#[derive(IntoElement)]
pub struct SidebarMenuSubButton {
    id: ElementId,
    size: SidebarMenuSubButtonSize,
    is_active: bool,
    disabled: bool,
    icon_collapsed: bool,
    icon: Option<AppIcon>,
    label: Option<SharedString>,
    focus_handle: Option<FocusHandle>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarMenuSubButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: SidebarMenuSubButtonSize::Md,
            is_active: false,
            disabled: false,
            icon_collapsed: false,
            icon: None,
            label: None,
            focus_handle: None,
            on_click: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: SidebarMenuSubButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon_collapsed(mut self, collapsed: bool) -> Self {
        self.icon_collapsed = collapsed;
        self
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
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

impl Styled for SidebarMenuSubButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarMenuSubButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarMenuSubButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if self.icon_collapsed {
            return div().into_any_element();
        }

        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let font_size = match self.size {
            SidebarMenuSubButtonSize::Sm => px(12.0),
            SidebarMenuSubButtonSize::Md => px(14.0),
        };
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));

        let mut el = div()
            .id(self.id)
            .flex()
            .h(px(28.0))
            .min_w_0()
            .items_center()
            .gap(px(8.0))
            .overflow_hidden()
            .rounded(PresetRadius::NONE)
            .px(px(12.0))
            .bg(if self.is_active {
                tokens.accent
            } else {
                transparent_black()
            })
            .text_color(if self.is_active {
                tokens.accent_foreground
            } else {
                tokens.card_foreground
            })
            .font_family(FONT_FAMILY)
            .text_size(font_size)
            .preset_disabled(self.disabled)
            .when(is_focused, |d| d.shadow(focus_ring_shadow(&theme)));

        if let Some(icon) = self.icon {
            el = el.child(app_icon(icon, IconSize::Sm, IconTone::Default, &theme));
        }
        if let Some(label) = self.label {
            el = el.child(div().min_w_0().truncate().child(label));
        }
        el = el.children(self.children);

        if let Some(ref fh) = self.focus_handle {
            el = el.track_focus(fh);
        }

        if !self.disabled {
            let hover_bg = tokens.accent;
            let hover_fg = tokens.accent_foreground;
            el = el
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg).text_color(hover_fg));
            if let Some(cb) = self.on_click {
                el = el.on_click(move |ev, win, app| cb(ev, win, app));
            }
        }

        el.refine_style(&self.style).into_any_element()
    }
}

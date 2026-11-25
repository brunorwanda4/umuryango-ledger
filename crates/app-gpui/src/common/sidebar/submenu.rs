//! Nested sub-menu primitives (`SidebarMenuSub`, `SidebarMenuSubItem`,
//! `SidebarMenuSubButton`, and `SidebarMenuSubButtonSize`) for [`super::Sidebar`].

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px, transparent_black,
};

use super::resolve_theme;
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, focus_ring_shadow};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme};

/// Typography preset of [`SidebarMenuSubButton`] (`size = "sm" | "md"` in `sidebar.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarMenuSubButtonSize {
    Sm,
    #[default]
    Md,
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

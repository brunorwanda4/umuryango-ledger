//! Menu items, checkbox/radio items, labels, separators, shortcuts, and submenus
//! shared by [`super::DropdownMenu`], `ContextMenu`, and `Menubar`.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement, IntoElement,
    MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, deferred, div, px,
};

use super::resolve_theme;
use crate::common::popover::{PopoverAlign, PopoverSide, apply_anchored_position};
use crate::common::utils::{
    PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow, to_preset_uppercase,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme};

/// Visual tone matching `variant = "default" | "destructive"` in `dropdown-menu.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DropdownMenuItemVariant {
    #[default]
    Default,
    Destructive,
}

/// Group wrapper (`DropdownMenuGroup` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DropdownMenuGroup {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for DropdownMenuGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DropdownMenuGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DropdownMenuGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DropdownMenuGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Radio group wrapper (`DropdownMenuRadioGroup` in `dropdown-menu.tsx`).
pub type DropdownMenuRadioGroup = DropdownMenuGroup;

/// Uppercase section label (`DropdownMenuLabel` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuLabel {
    text: SharedString,
    inset: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl DropdownMenuLabel {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            inset: false,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for DropdownMenuLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DropdownMenuLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .py(px(8.0))
            .pl(px(if self.inset { 38.0 } else { 12.0 }))
            .pr(px(12.0))
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.muted_foreground)
            .child(to_preset_uppercase(self.text.as_ref()))
            .refine_style(&self.style)
    }
}

/// Right-aligned shortcut hint (`DropdownMenuShortcut` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuShortcut {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl DropdownMenuShortcut {
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

impl Styled for DropdownMenuShortcut {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DropdownMenuShortcut {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .ml_auto()
            .font_family(FONT_FAMILY)
            .text_xs()
            .text_color(tokens.muted_foreground)
            .child(self.text)
            .refine_style(&self.style)
    }
}

/// Thin horizontal separator (`DropdownMenuSeparator` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuSeparator {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl DropdownMenuSeparator {
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

impl Default for DropdownMenuSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DropdownMenuSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DropdownMenuSeparator {
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

/// Actionable menu row (`DropdownMenuItem` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuItem {
    id: ElementId,
    label: Option<SharedString>,
    icon: Option<AppIcon>,
    shortcut: Option<SharedString>,
    inset: bool,
    uppercase: bool,
    variant: DropdownMenuItemVariant,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DropdownMenuItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: Some(label.into()),
            icon: None,
            shortcut: None,
            inset: false,
            uppercase: true,
            variant: DropdownMenuItemVariant::Default,
            disabled: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn uppercase(mut self, uppercase: bool) -> Self {
        self.uppercase = uppercase;
        self
    }

    pub fn variant(mut self, variant: DropdownMenuItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.variant = DropdownMenuItemVariant::Destructive;
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

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for DropdownMenuItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DropdownMenuItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DropdownMenuItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_destructive = matches!(self.variant, DropdownMenuItemVariant::Destructive);

        let text_color = if is_destructive {
            tokens.destructive
        } else {
            tokens.popover_foreground
        };
        let hover_bg = if is_destructive {
            tokens.destructive_soft
        } else {
            tokens.accent
        };
        let hover_fg = if is_destructive {
            tokens.destructive
        } else {
            tokens.accent_foreground
        };

        let mut row = div()
            .id(self.id)
            .relative()
            .flex()
            .w_full()
            .items_center()
            .gap(px(10.0))
            .rounded(PresetRadius::NONE)
            .py(px(8.0))
            .pl(px(if self.inset { 38.0 } else { 12.0 }))
            .pr(px(12.0))
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(text_color)
            .cx_when(!self.disabled, |d| {
                d.cursor_pointer()
                    .hover(move |s| s.bg(hover_bg).text_color(hover_fg))
            })
            .preset_disabled(self.disabled);

        if let Some(icon) = self.icon {
            let tone = if is_destructive {
                IconTone::Error
            } else {
                IconTone::Muted
            };
            row = row.child(app_icon(icon, IconSize::Sm, tone, &theme));
        }

        if let Some(lbl) = &self.label {
            let formatted = if self.uppercase {
                to_preset_uppercase(lbl.as_ref())
            } else {
                lbl.to_string()
            };
            row = row.child(formatted);
        }

        row = row.children(self.children);

        if let Some(sc) = self.shortcut {
            row = row.child(DropdownMenuShortcut::new(sc).theme(&theme));
        }

        if !self.disabled
            && let Some(on_click) = self.on_click
        {
            row = row.on_click(move |ev, window, cx| on_click(ev, window, cx));
        }

        row.refine_style(&self.style)
    }
}

/// Checkable menu row (`DropdownMenuCheckboxItem` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuCheckboxItem {
    id: ElementId,
    label: SharedString,
    checked: bool,
    inset: bool,
    indicator_left: bool,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_checked_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl DropdownMenuCheckboxItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            inset: false,
            indicator_left: false,
            disabled: false,
            theme: None,
            on_checked_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn indicator_left(mut self, left: bool) -> Self {
        self.indicator_left = left;
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

    pub fn on_checked_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for DropdownMenuCheckboxItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DropdownMenuCheckboxItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let hover_bg = tokens.accent;
        let hover_fg = tokens.accent_foreground;
        let pl_val = if self.indicator_left || self.inset {
            38.0
        } else {
            12.0
        };
        let pr_val = if self.indicator_left { 12.0 } else { 32.0 };

        let mut row = div()
            .id(self.id)
            .relative()
            .flex()
            .w_full()
            .items_center()
            .gap(px(10.0))
            .rounded(PresetRadius::NONE)
            .py(px(8.0))
            .pl(px(pl_val))
            .pr(px(pr_val))
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.popover_foreground)
            .cx_when(!self.disabled, |d| {
                d.cursor_pointer()
                    .hover(move |s| s.bg(hover_bg).text_color(hover_fg))
            })
            .preset_disabled(self.disabled)
            .child(to_preset_uppercase(self.label.as_ref()));

        if self.checked {
            let mut ind = div()
                .absolute()
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
                ));
            ind = if self.indicator_left {
                ind.left(px(12.0))
            } else {
                ind.right(px(8.0))
            };
            row = row.child(ind);
        }

        if !self.disabled
            && let Some(cb) = self.on_checked_change
        {
            let next = !self.checked;
            row = row.on_click(move |_ev, window, cx| cb(&next, window, cx));
        }

        row.refine_style(&self.style)
    }
}

/// Mutually exclusive radio menu row (`DropdownMenuRadioItem` in `dropdown-menu.tsx`).
pub type DropdownMenuRadioItem = DropdownMenuCheckboxItem;

/// Nested submenu trigger (`DropdownMenuSubTrigger` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuSubTrigger {
    id: ElementId,
    label: SharedString,
    icon: Option<AppIcon>,
    inset: bool,
    open: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl DropdownMenuSubTrigger {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            inset: false,
            open: false,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for DropdownMenuSubTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DropdownMenuSubTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let hover_bg = tokens.accent;
        let hover_fg = tokens.accent_foreground;

        let mut row = div()
            .id(self.id)
            .flex()
            .w_full()
            .items_center()
            .gap(px(8.0))
            .rounded(PresetRadius::NONE)
            .py(px(8.0))
            .pl(px(if self.inset { 38.0 } else { 12.0 }))
            .pr(px(12.0))
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .cx_when(self.open, |d| {
                d.bg(tokens.accent).text_color(tokens.accent_foreground)
            })
            .cx_when(!self.open, |d| d.text_color(tokens.popover_foreground))
            .hover(move |s| s.bg(hover_bg).text_color(hover_fg));

        if let Some(icon) = self.icon {
            row = row.child(app_icon(icon, IconSize::Sm, IconTone::Muted, &theme));
        }

        row.child(to_preset_uppercase(self.label.as_ref()))
            .child(
                div()
                    .ml_auto()
                    .child(app_icon(
                        AppIcon::ChevronRight,
                        IconSize::Sm,
                        IconTone::Muted,
                        &theme,
                    )),
            )
            .refine_style(&self.style)
    }
}

/// Nested submenu popup surface (`DropdownMenuSubContent` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuSubContent {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl DropdownMenuSubContent {
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

impl Default for DropdownMenuSubContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for DropdownMenuSubContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for DropdownMenuSubContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DropdownMenuSubContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .flex_col()
            .min_w(px(144.0))
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .p(px(6.0))
            .font_family(FONT_FAMILY)
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Hover/click-expandable submenu container (`DropdownMenuSub` in `dropdown-menu.tsx`).
#[derive(IntoElement)]
pub struct DropdownMenuSub {
    id: ElementId,
    trigger: DropdownMenuSubTrigger,
    content: DropdownMenuSubContent,
    style: StyleRefinement,
}

impl DropdownMenuSub {
    pub fn new(
        id: impl Into<ElementId>,
        trigger: DropdownMenuSubTrigger,
        content: DropdownMenuSubContent,
    ) -> Self {
        Self {
            id: id.into(),
            trigger,
            content,
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for DropdownMenuSub {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DropdownMenuSub {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-sub-open", self.id).into()),
            cx,
            |_w, _cx| false,
        );
        let is_open = *state.read(cx);
        let state_trig = state.clone();

        let mut root = div()
            .id(self.id.clone())
            .relative()
            .w_full()
            .on_click(move |_ev, _window, cx| {
                state_trig.update(cx, |v, cx| {
                    *v = !*v;
                    cx.notify();
                });
            })
            .refine_style(&self.style)
            .child(self.trigger.open(is_open));

        if is_open {
            let sub_pos = apply_anchored_position(
                div()
                    .id(ElementId::Name(format!("{}-sub-popup", self.id).into()))
                    .absolute(),
                PopoverSide::Right,
                0.0,
                PopoverAlign::Start,
                -3.0,
            )
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .child(self.content);

            root = root.child(deferred(sub_pos).with_priority(6));
        }

        root
    }
}

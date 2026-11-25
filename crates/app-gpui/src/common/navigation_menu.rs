//! Site/section navigation menu primitives ported from `web/src/components/ui/navigation-menu.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `NavigationMenu` & `NavigationMenuList`: horizontal bar (`flex-wrap` on mobile via `is_mobile`)
//! - `NavigationMenuTrigger`: `h-9 rounded-none px-4.5 py-2.5 text-sm font-medium hover:bg-muted data-open:bg-muted/50`
//!   with `AppIcon::ChevronDown` / `AppIcon::ChevronUp`
//! - `NavigationMenuContent` & `NavigationMenuPositioner`: `rounded-none bg-popover p-2.5 pr-3 shadow-md ring-1 ring-foreground/10`
//! - `NavigationMenuLink`: `flex items-center gap-1.5 rounded-none p-3 text-sm hover:bg-muted data-[active=true]:bg-muted/50`
//! - `NavigationMenuIndicator`: `h-1.5` bar with `bg-border` marker.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, KeyDownEvent, MouseButton, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, deferred, div, px,
};

use crate::common::popover::{PopoverAlign, PopoverSide, apply_anchored_position};
use crate::common::utils::{
    PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt, focus_ring_shadow,
    overlay_surface_shadow,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::hooks::use_mobile::is_mobile;
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

/// Reusable trigger style builder matching `navigationMenuTriggerStyle` in `navigation-menu.tsx`.
pub fn navigation_menu_trigger_style(
    open: bool,
    focused: bool,
    disabled: bool,
    theme: &SemanticTheme,
) -> gpui_kit::Div {
    let tokens = PresetTokens::from_theme(theme);
    let hover_bg = tokens.muted;

    div()
        .flex()
        .h(PresetSpacing::CONTROL_SM)
        .items_center()
        .justify_center()
        .gap(px(4.0))
        .rounded(PresetRadius::NONE)
        .px(px(18.0))
        .py(px(10.0))
        .font_family(FONT_FAMILY)
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .text_color(tokens.foreground)
        .cx_when(open, |d| d.bg(tokens.muted.opacity(0.5)))
        .cx_when(!disabled, |d| {
            d.cursor_pointer().hover(move |s| s.bg(hover_bg))
        })
        .cx_when(focused, |d| d.shadow(focus_ring_shadow(theme)))
        .preset_disabled(disabled)
}

/// Root navigation menu container (`NavigationMenu` in `navigation-menu.tsx`).
#[derive(IntoElement)]
pub struct NavigationMenu {
    id: ElementId,
    align: PopoverAlign,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl NavigationMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            align: PopoverAlign::Start,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub const fn popup_align(&self) -> PopoverAlign {
        self.align
    }
}

impl Styled for NavigationMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for NavigationMenu {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for NavigationMenu {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// List of navigation items (`NavigationMenuList` in `navigation-menu.tsx`).
#[derive(IntoElement)]
pub struct NavigationMenuList {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl NavigationMenuList {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for NavigationMenuList {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for NavigationMenuList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for NavigationMenuList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for NavigationMenuList {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mobile = is_mobile(window);

        div()
            .flex()
            .flex_1()
            .items_center()
            .justify_center()
            .gap(px(0.0))
            .cx_when(mobile, |d| d.flex_wrap())
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Trigger button with chevron (`NavigationMenuTrigger` in `navigation-menu.tsx`).
#[derive(IntoElement)]
pub struct NavigationMenuTrigger {
    id: ElementId,
    label: SharedString,
    open: bool,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl NavigationMenuTrigger {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            open: false,
            disabled: false,
            focus_handle: None,
            theme: None,
            style: StyleRefinement::default(),
        }
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
}

impl Styled for NavigationMenuTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NavigationMenuTrigger {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let chevron = if self.open {
            AppIcon::ChevronUp
        } else {
            AppIcon::ChevronDown
        };

        let mut trig =
            navigation_menu_trigger_style(self.open, focused, self.disabled, &theme).id(self.id);

        if let Some(fh) = &self.focus_handle {
            trig = trig.track_focus(fh);
        }

        trig.child(self.label)
            .child(app_icon(chevron, IconSize::Xs, IconTone::Muted, &theme))
            .refine_style(&self.style)
    }
}

/// Anchored popup positioner (`NavigationMenuPositioner` in `navigation-menu.tsx`).
#[derive(IntoElement)]
pub struct NavigationMenuPositioner {
    side: PopoverSide,
    side_offset: f32,
    align: PopoverAlign,
    align_offset: f32,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl NavigationMenuPositioner {
    pub fn new() -> Self {
        Self {
            side: PopoverSide::Bottom,
            side_offset: 8.0,
            align: PopoverAlign::Start,
            align_offset: 0.0,
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
}

impl Default for NavigationMenuPositioner {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for NavigationMenuPositioner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for NavigationMenuPositioner {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for NavigationMenuPositioner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        apply_anchored_position(
            div().absolute().refine_style(&self.style),
            self.side,
            self.side_offset,
            self.align,
            self.align_offset,
        )
        .children(self.children)
    }
}

/// Flyout panel surface (`NavigationMenuContent` in `navigation-menu.tsx`).
#[derive(IntoElement)]
pub struct NavigationMenuContent {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl NavigationMenuContent {
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

impl Default for NavigationMenuContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for NavigationMenuContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for NavigationMenuContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for NavigationMenuContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .flex_col()
            .min_w(px(220.0))
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .p(px(10.0))
            .pr(px(12.0))
            .font_family(FONT_FAMILY)
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Navigation item slot that can pair a trigger and flyout content (`NavigationMenuItem` in `navigation-menu.tsx`).
#[derive(IntoElement)]
pub struct NavigationMenuItem {
    id: ElementId,
    trigger: Option<NavigationMenuTrigger>,
    content: Option<NavigationMenuContent>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl NavigationMenuItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn dropdown(
        mut self,
        trigger: NavigationMenuTrigger,
        content: NavigationMenuContent,
    ) -> Self {
        self.trigger = Some(trigger);
        self.content = Some(content);
        self
    }
}

impl Styled for NavigationMenuItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for NavigationMenuItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for NavigationMenuItem {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if let (Some(trig), Some(content)) = (self.trigger, self.content) {
            let state = window.use_keyed_state(
                ElementId::Name(format!("{}-nav-item-open", self.id).into()),
                cx,
                |_w, _cx| false,
            );
            let is_open = *state.read(cx);
            let state_click = state.clone();

            let mut root = div()
                .id(self.id.clone())
                .relative()
                .flex()
                .items_center()
                .on_click(move |_ev, _window, cx| {
                    state_click.update(cx, |v, cx| {
                        *v = !*v;
                        cx.notify();
                    });
                })
                .refine_style(&self.style)
                .child(trig.open(is_open));

            if is_open {
                let state_out = state.clone();
                let state_esc = state;
                let popup = NavigationMenuPositioner::new().child(
                    div()
                        .id(ElementId::Name(
                            format!("{}-nav-item-popup", self.id).into(),
                        ))
                        .on_mouse_down(MouseButton::Left, |_, _, cx| {
                            cx.stop_propagation();
                        })
                        .on_mouse_down_out(move |_ev, _window, cx| {
                            state_out.update(cx, |v, cx| {
                                *v = false;
                                cx.notify();
                            });
                        })
                        .on_key_down(move |ev: &KeyDownEvent, _window, cx| {
                            if ev.keystroke.key == "escape" {
                                state_esc.update(cx, |v, cx| {
                                    *v = false;
                                    cx.notify();
                                });
                            }
                        })
                        .child(content),
                );
                root = root.child(deferred(popup).with_priority(5));
            }
            root.into_any_element()
        } else {
            div()
                .id(self.id)
                .relative()
                .flex()
                .items_center()
                .refine_style(&self.style)
                .children(self.children)
                .into_any_element()
        }
    }
}

/// Interactive link item (`NavigationMenuLink` in `navigation-menu.tsx`).
#[derive(IntoElement)]
pub struct NavigationMenuLink {
    id: ElementId,
    label: Option<SharedString>,
    description: Option<SharedString>,
    icon: Option<AppIcon>,
    active: bool,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl NavigationMenuLink {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            description: None,
            icon: None,
            active: false,
            disabled: false,
            focus_handle: None,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
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

impl Styled for NavigationMenuLink {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for NavigationMenuLink {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for NavigationMenuLink {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let hover_bg = tokens.muted;

        let mut link = div()
            .id(self.id)
            .flex()
            .items_start()
            .gap(px(8.0))
            .rounded(PresetRadius::NONE)
            .p(px(12.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.foreground)
            .cx_when(self.active, |d| d.bg(tokens.muted.opacity(0.5)))
            .cx_when(!self.disabled, |d| {
                d.cursor_pointer().hover(move |s| s.bg(hover_bg))
            })
            .cx_when(focused, |d| d.shadow(focus_ring_shadow(&theme)))
            .preset_disabled(self.disabled);

        if let Some(fh) = &self.focus_handle {
            link = link.track_focus(fh);
        }

        if let Some(icon) = self.icon {
            link = link.child(
                div()
                    .mt(px(2.0))
                    .child(app_icon(icon, IconSize::Sm, IconTone::Muted, &theme)),
            );
        }

        if self.label.is_some() || self.description.is_some() {
            let mut col = div().flex().flex_col().gap(px(2.0));
            if let Some(lbl) = self.label {
                col = col.child(div().font_weight(FontWeight::MEDIUM).child(lbl));
            }
            if let Some(desc) = self.description {
                col = col.child(div().text_xs().text_color(tokens.muted_foreground).child(desc));
            }
            link = link.child(col);
        }

        link = link.children(self.children).refine_style(&self.style);

        if !self.disabled
            && let Some(on_click) = self.on_click
        {
            link = link.on_click(move |ev, window, cx| on_click(ev, window, cx));
        }

        link
    }
}

/// Active item indicator bar (`NavigationMenuIndicator` in `navigation-menu.tsx`).
#[derive(IntoElement)]
pub struct NavigationMenuIndicator {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl NavigationMenuIndicator {
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

impl Default for NavigationMenuIndicator {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for NavigationMenuIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NavigationMenuIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .h(px(6.0))
            .items_end()
            .justify_center()
            .overflow_hidden()
            .child(
                div()
                    .h(px(4.0))
                    .w(px(12.0))
                    .rounded(PresetRadius::NONE)
                    .bg(tokens.border),
            )
            .refine_style(&self.style)
    }
}

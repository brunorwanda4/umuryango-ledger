//! Responsive application sidebar shell, state provider, collapsible rail, groups,
//! and menu primitives ported from `web/src/components/ui/sidebar.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - Width constants: `SIDEBAR_WIDTH` (`256px`), `SIDEBAR_WIDTH_MOBILE` (`288px`),
//!   `SIDEBAR_WIDTH_ICON` (`48px`), keyboard shortcut `Cmd/Ctrl+B`
//! - Automatically adapts between mobile viewports ([`crate::hooks::use_mobile::is_mobile`],
//!   rendering inside [`Sheet`]) and desktop (`SidebarVariant::Sidebar | Floating | Inset`,
//!   `SidebarCollapsible::Offcanvas | Icon | None`)
//! - Menu primitives live in [`menu`] and are re-exported at this module root so every
//!   file stays well under the 1,000-line repository cap.

mod menu;

use std::rc::Rc;

pub use gpui_kit::component::sidebar::{
    Sidebar as KitSidebar, SidebarCollapsible as KitSidebarCollapsible,
    SidebarFooter as KitSidebarFooter, SidebarGroup as KitSidebarGroup,
    SidebarHeader as KitSidebarHeader, SidebarItem as KitSidebarItem,
    SidebarMenu as KitSidebarMenu, SidebarMenuItem as KitSidebarMenuItem,
    SidebarToggleButton as KitSidebarToggleButton,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, Entity, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, KeyDownEvent, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px,
};
pub use menu::{
    SidebarMenu, SidebarMenuAction, SidebarMenuBadge, SidebarMenuButton, SidebarMenuButtonSize,
    SidebarMenuButtonVariant, SidebarMenuItem, SidebarMenuSkeleton, SidebarMenuSub,
    SidebarMenuSubButton, SidebarMenuSubButtonSize, SidebarMenuSubItem,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::input::{Input, InputState};
use crate::common::separator::Separator;
use crate::common::sheet::{Sheet, SheetContent, SheetSide};
use crate::common::utils::{
    PresetRadius, PresetTokens, card_surface_shadow, focus_ring_shadow, to_preset_uppercase,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::hooks::use_mobile::is_mobile;
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Desktop expanded sidebar width (`16rem` = `256px`).
pub const SIDEBAR_WIDTH: f32 = 256.0;
/// Mobile sheet sidebar width (`18rem` = `288px`).
pub const SIDEBAR_WIDTH_MOBILE: f32 = 288.0;
/// Desktop icon-collapsed sidebar width (`3rem` = `48px`).
pub const SIDEBAR_WIDTH_ICON: f32 = 48.0;
/// Keyboard shortcut key toggled with `Cmd` or `Ctrl` (`"b"`).
pub const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";

/// Expansion state (`"expanded" | "collapsed"` in `sidebar.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarState {
    #[default]
    Expanded,
    Collapsed,
}

/// Docked edge (`side = "left" | "right"` in `sidebar.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarSide {
    #[default]
    Left,
    Right,
}

/// Desktop structural variant (`variant = "sidebar" | "floating" | "inset"` in `sidebar.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

/// Collapse behavior (`collapsible = "offcanvas" | "icon" | "none"` in `sidebar.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

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

/// Reactive sidebar expansion state shared across [`SidebarProvider`], [`Sidebar`],
/// [`SidebarTrigger`], and [`SidebarRail`] (`SidebarContextProps` in `sidebar.tsx`).
#[derive(Debug, Clone)]
pub struct SidebarContextState {
    pub open: bool,
    pub open_mobile: bool,
}

impl Default for SidebarContextState {
    fn default() -> Self {
        Self::new(true)
    }
}

impl SidebarContextState {
    pub const fn new(default_open: bool) -> Self {
        Self {
            open: default_open,
            open_mobile: false,
        }
    }

    pub const fn state(&self) -> SidebarState {
        if self.open {
            SidebarState::Expanded
        } else {
            SidebarState::Collapsed
        }
    }

    pub fn toggle(&mut self, mobile: bool) {
        if mobile {
            self.open_mobile = !self.open_mobile;
        } else {
            self.open = !self.open;
        }
    }
}

/// Retrieves or initializes a keyed [`Entity<SidebarContextState>`] for the current window (`useSidebar`).
pub fn use_sidebar(
    id: impl Into<ElementId>,
    default_open: bool,
    window: &mut Window,
    cx: &mut App,
) -> Entity<SidebarContextState> {
    window.use_keyed_state(id.into(), cx, move |_, _| {
        SidebarContextState::new(default_open)
    })
}

/// Root layout wrapper managing sidebar state and `Cmd/Ctrl+B` keyboard toggling (`data-slot="sidebar-wrapper"`).
#[derive(IntoElement)]
pub struct SidebarProvider {
    id: ElementId,
    state: Option<Entity<SidebarContextState>>,
    default_open: bool,
    inset: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarProvider {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state: None,
            default_open: true,
            inset: false,
            on_open_change: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn state(mut self, state: &Entity<SidebarContextState>) -> Self {
        self.state = Some(state.clone());
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn inset(mut self, inset: bool) -> Self {
        self.inset = inset;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for SidebarProvider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarProvider {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarProvider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let mobile = is_mobile(window);
        let state = self
            .state
            .unwrap_or_else(|| use_sidebar(self.id.clone(), self.default_open, window, cx));
        let on_change = self.on_open_change;

        div()
            .id(ElementId::Name(format!("{}-wrapper", self.id).into()))
            .flex()
            .min_h_full()
            .w_full()
            .font_family(FONT_FAMILY)
            .when(self.inset, |d| d.bg(tokens.card))
            .on_key_down(move |ev: &KeyDownEvent, win, app| {
                let mods = &ev.keystroke.modifiers;
                if ev.keystroke.key.eq_ignore_ascii_case(SIDEBAR_KEYBOARD_SHORTCUT)
                    && (mods.platform || mods.control)
                {
                    let mut next_desktop_open = None;
                    state.update(app, |s, app_cx| {
                        s.toggle(mobile);
                        if !mobile {
                            next_desktop_open = Some(s.open);
                        }
                        app_cx.notify();
                    });
                    if let (Some(next), Some(cb)) = (next_desktop_open, &on_change) {
                        cb(&next, win, app);
                    }
                }
            })
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Responsive desktop/mobile sidebar panel (`data-slot="sidebar"`).
#[derive(IntoElement)]
pub struct Sidebar {
    id: ElementId,
    side: SidebarSide,
    variant: SidebarVariant,
    collapsible: SidebarCollapsible,
    state: Option<Entity<SidebarContextState>>,
    open: Option<bool>,
    open_mobile: Option<bool>,
    on_open_mobile_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Sidebar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            side: SidebarSide::Left,
            variant: SidebarVariant::Sidebar,
            collapsible: SidebarCollapsible::Offcanvas,
            state: None,
            open: None,
            open_mobile: None,
            on_open_mobile_change: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: SidebarSide) -> Self {
        self.side = side;
        self
    }

    pub fn variant(mut self, variant: SidebarVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn collapsible(mut self, collapsible: SidebarCollapsible) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn state(mut self, state: &Entity<SidebarContextState>) -> Self {
        self.state = Some(state.clone());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn open_mobile(mut self, open_mobile: bool) -> Self {
        self.open_mobile = Some(open_mobile);
        self
    }

    pub fn on_open_mobile_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_mobile_change = Some(Rc::new(handler));
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for Sidebar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Sidebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Sidebar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        if self.collapsible == SidebarCollapsible::None {
            return div()
                .flex()
                .flex_col()
                .h_full()
                .w(px(SIDEBAR_WIDTH))
                .flex_shrink_0()
                .bg(tokens.card)
                .text_color(tokens.card_foreground)
                .font_family(FONT_FAMILY)
                .refine_style(&self.style)
                .children(self.children)
                .into_any_element();
        }

        let (desktop_open, mobile_open) = if let Some(ref st) = self.state {
            let snap = st.read(cx);
            (
                self.open.unwrap_or(snap.open),
                self.open_mobile.unwrap_or(snap.open_mobile),
            )
        } else {
            (
                self.open.unwrap_or(true),
                self.open_mobile.unwrap_or(false),
            )
        };

        if is_mobile(window) {
            let sheet_side = match self.side {
                SidebarSide::Left => SheetSide::Left,
                SidebarSide::Right => SheetSide::Right,
            };
            let st_entity = self.state;
            let user_mobile_cb = self.on_open_mobile_change;

            let content = SheetContent::new(ElementId::Name(
                format!("{}-mobile-content", self.id).into(),
            ))
            .side(sheet_side)
            .show_close_button(false)
            .theme(&theme)
            .w(px(SIDEBAR_WIDTH_MOBILE))
            .max_w(px(SIDEBAR_WIDTH_MOBILE))
            .p_0()
            .bg(tokens.card)
            .text_color(tokens.card_foreground)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .children(self.children),
            );

            return Sheet::new(ElementId::Name(format!("{}-mobile-sheet", self.id).into()))
                .open(mobile_open)
                .on_open_change(move |next, win, app| {
                    if let Some(ref st) = st_entity {
                        st.update(app, |s, app_cx| {
                            s.open_mobile = *next;
                            app_cx.notify();
                        });
                    }
                    if let Some(ref cb) = user_mobile_cb {
                        cb(next, win, app);
                    }
                })
                .content(content)
                .into_any_element();
        }

        if !desktop_open && self.collapsible == SidebarCollapsible::Offcanvas {
            return div().w(px(0.0)).flex_shrink_0().into_any_element();
        }

        let is_padded_variant =
            matches!(self.variant, SidebarVariant::Floating | SidebarVariant::Inset);
        let icon_collapsed = !desktop_open && self.collapsible == SidebarCollapsible::Icon;
        let outer_width = if icon_collapsed {
            if is_padded_variant {
                px(SIDEBAR_WIDTH_ICON + 16.0)
            } else {
                px(SIDEBAR_WIDTH_ICON)
            }
        } else {
            px(SIDEBAR_WIDTH)
        };

        let inner = div()
            .flex()
            .flex_col()
            .size_full()
            .bg(tokens.card)
            .text_color(tokens.card_foreground)
            .rounded(PresetRadius::NONE)
            .when(self.variant == SidebarVariant::Floating, |d| {
                d.border_1()
                    .border_color(tokens.border)
                    .shadow(card_surface_shadow(&theme))
            })
            .children(self.children);

        div()
            .relative()
            .flex()
            .flex_col()
            .h_full()
            .w(outer_width)
            .flex_shrink_0()
            .font_family(FONT_FAMILY)
            .when_else(
                is_padded_variant,
                |d| d.p(px(8.0)),
                |d| {
                    d.border_color(tokens.border).when_else(
                        self.side == SidebarSide::Left,
                        |l| l.border_r_1(),
                        |r| r.border_l_1(),
                    )
                },
            )
            .refine_style(&self.style)
            .child(inner)
            .into_any_element()
    }
}

/// Ghost icon button (`AppIcon::PanelLeft`) toggling [`SidebarContextState`] (`data-slot="sidebar-trigger"`).
#[derive(IntoElement)]
pub struct SidebarTrigger {
    id: ElementId,
    state: Option<Entity<SidebarContextState>>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl SidebarTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state: None,
            on_click: None,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn state(mut self, state: &Entity<SidebarContextState>) -> Self {
        self.state = Some(state.clone());
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

impl Styled for SidebarTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarTrigger {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mobile = is_mobile(window);
        let st = self.state;
        let user_cb = self.on_click;

        Button::new(self.id)
            .variant(ButtonVariant::Ghost)
            .size(ButtonSize::IconSm)
            .theme(&theme)
            .child(app_icon(
                AppIcon::PanelLeft,
                IconSize::Md,
                IconTone::Default,
                &theme,
            ))
            .on_click(move |ev, win, app| {
                if let Some(ref cb) = user_cb {
                    cb(ev, win, app);
                }
                if let Some(ref state) = st {
                    state.update(app, |s, app_cx| {
                        s.toggle(mobile);
                        app_cx.notify();
                    });
                }
            })
            .refine_style(&self.style)
    }
}

/// Interactive vertical edge strip toggling [`SidebarContextState`] (`data-slot="sidebar-rail"`).
#[derive(IntoElement)]
pub struct SidebarRail {
    id: ElementId,
    side: SidebarSide,
    state: Option<Entity<SidebarContextState>>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl SidebarRail {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            side: SidebarSide::Left,
            state: None,
            on_click: None,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn side(mut self, side: SidebarSide) -> Self {
        self.side = side;
        self
    }

    pub fn state(mut self, state: &Entity<SidebarContextState>) -> Self {
        self.state = Some(state.clone());
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

impl Styled for SidebarRail {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarRail {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if is_mobile(window) {
            return div().into_any_element();
        }

        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let hover_line = tokens.border;
        let st = self.state;
        let user_cb = self.on_click;

        div()
            .id(self.id)
            .absolute()
            .top_0()
            .bottom_0()
            .w(px(12.0))
            .flex()
            .justify_center()
            .cursor_ew_resize()
            .when_else(
                self.side == SidebarSide::Left,
                |d| d.right(px(-6.0)),
                |d| d.left(px(-6.0)),
            )
            .hover(move |s| s.bg(hover_line.opacity(0.35)))
            .on_click(move |ev, win, app| {
                if let Some(ref cb) = user_cb {
                    cb(ev, win, app);
                }
                if let Some(ref state) = st {
                    state.update(app, |s, app_cx| {
                        s.toggle(false);
                        app_cx.notify();
                    });
                }
            })
            .refine_style(&self.style)
            .into_any_element()
    }
}

/// Primary main-content column beside a [`Sidebar`] (`data-slot="sidebar-inset"`).
#[derive(IntoElement)]
pub struct SidebarInset {
    inset_variant: bool,
    sidebar_collapsed: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl SidebarInset {
    pub fn new() -> Self {
        Self {
            inset_variant: false,
            sidebar_collapsed: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn inset_variant(mut self, inset: bool) -> Self {
        self.inset_variant = inset;
        self
    }

    pub fn sidebar_collapsed(mut self, collapsed: bool) -> Self {
        self.sidebar_collapsed = collapsed;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for SidebarInset {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarInset {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for SidebarInset {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for SidebarInset {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let desktop_inset = self.inset_variant && !is_mobile(window);

        div()
            .relative()
            .flex()
            .flex_col()
            .flex_1()
            .w_full()
            .min_w_0()
            .bg(tokens.background)
            .rounded(PresetRadius::NONE)
            .when(desktop_inset, |d| {
                d.m(px(8.0))
                    .when(!self.sidebar_collapsed, |open_d| open_d.ml_0())
                    .shadow(card_surface_shadow(&theme))
            })
            .refine_style(&self.style)
            .children(self.children)
    }
}

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

/// Sticky top section of [`Sidebar`] (`data-slot="sidebar-header"`, `flex flex-col gap-2 p-2`).
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

/// Sticky bottom section of [`Sidebar`] (`data-slot="sidebar-footer"`, `flex flex-col gap-2 p-2`).
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

/// Horizontal divider inside [`Sidebar`] (`data-slot="sidebar-separator"`, `mx-2 w-auto bg-sidebar-border`).
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

/// Scrollable middle body of [`Sidebar`] (`data-slot="sidebar-content"`).
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

/// Logical section inside [`SidebarContent`] (`data-slot="sidebar-group"`, `relative flex w-full min-w-0 flex-col p-2`).
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

/// Content body of a [`SidebarGroup`] (`data-slot="sidebar-group-content"`, `w-full text-sm`).
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

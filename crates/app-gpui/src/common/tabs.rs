//! Tabbed view switcher primitives ported from `web/src/components/ui/tabs.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Tabs`: `Horizontal` (`flex-col`) or `Vertical` (`flex-row`) orientation, controlled or uncontrolled
//! - `TabsList`: `TabsListVariant::Default` (`bg-muted p-1 h-10`) | `TabsListVariant::Line` (`gap-1 bg-transparent`)
//! - `TabsTrigger`: `px-4 py-1.5 text-xs font-semibold uppercase`, active `bg-background` (`Default`) or `2px` indicator bar (`Line`)
//! - `TabsContent`: `flex-1 text-sm`.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::tab::{Tab as KitTab, TabBar as KitTabBar};
use gpui_kit::{
    AnyElement, App, ElementId, FocusHandle, FontWeight, InteractiveElement, IntoElement,
    KeyDownEvent, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{
    PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt, to_preset_uppercase,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Layout orientation matching `orientation = "horizontal" | "vertical"` in `tabs.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Visual style of [`TabsList`] matching `tabsListVariants.variant` in `tabs.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsListVariant {
    #[default]
    Default,
    Line,
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

/// Tabbed navigation container (`Tabs` in `web/src/components/ui/tabs.tsx`).
#[derive(IntoElement)]
pub struct Tabs {
    id: ElementId,
    orientation: TabsOrientation,
    list_variant: TabsListVariant,
    triggers: Vec<TabsTrigger>,
    panels: Vec<TabsContent>,
    active_index: Option<usize>,
    default_index: usize,
    theme: Option<SemanticTheme>,
    on_change: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Tabs {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            orientation: TabsOrientation::Horizontal,
            list_variant: TabsListVariant::Default,
            triggers: Vec::new(),
            panels: Vec::new(),
            active_index: None,
            default_index: 0,
            theme: None,
            on_change: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn vertical(self) -> Self {
        self.orientation(TabsOrientation::Vertical)
    }

    pub fn list_variant(mut self, variant: TabsListVariant) -> Self {
        self.list_variant = variant;
        self
    }

    pub fn line(self) -> Self {
        self.list_variant(TabsListVariant::Line)
    }

    pub fn tab(mut self, trigger: TabsTrigger, panel: TabsContent) -> Self {
        self.triggers.push(trigger);
        self.panels.push(panel);
        self
    }

    pub fn triggers(mut self, triggers: impl IntoIterator<Item = TabsTrigger>) -> Self {
        self.triggers.extend(triggers);
        self
    }

    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index = Some(index);
        self
    }

    pub fn default_index(mut self, index: usize) -> Self {
        self.default_index = index;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for Tabs {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Tabs {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tabs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let initial = self.default_index;
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| initial);
        let is_controlled = self.active_index.is_some();
        let active = self.active_index.unwrap_or_else(|| *state.read(cx));

        let is_horizontal = matches!(self.orientation, TabsOrientation::Horizontal);
        let mut root = div()
            .flex()
            .gap(px(8.0)) // gap-2
            .cx_when(is_horizontal, |d| d.flex_col())
            .cx_when(!is_horizontal, |d| d.flex_row().items_start())
            .refine_style(&self.style);

        if !self.triggers.is_empty() {
            let count = self.triggers.len();
            let mut list = TabsList::new()
                .variant(self.list_variant)
                .orientation(self.orientation)
                .theme(&theme);

            for (idx, mut trig) in self.triggers.into_iter().enumerate() {
                let st = state.clone();
                let cb = self.on_change.clone();
                trig = trig
                    .index(idx)
                    .active(idx == active)
                    .list_variant(self.list_variant)
                    .orientation(self.orientation)
                    .theme(&theme)
                    .on_select(move |window, cx| {
                        if !is_controlled {
                            st.update(cx, |v, cx| {
                                *v = idx;
                                cx.notify();
                            });
                        }
                        if let Some(ref handler) = cb {
                            handler(&idx, window, cx);
                        }
                    })
                    .on_arrow_step({
                        let st = state.clone();
                        let cb = self.on_change.clone();
                        move |delta, window, cx| {
                            let next = if delta > 0 {
                                (active + 1) % count
                            } else {
                                (active + count - 1) % count
                            };
                            if !is_controlled {
                                st.update(cx, |v, cx| {
                                    *v = next;
                                    cx.notify();
                                });
                            }
                            if let Some(ref handler) = cb {
                                handler(&next, window, cx);
                            }
                        }
                    });
                list = list.child(trig);
            }

            root = root.child(list);

            if let Some(panel) = self.panels.into_iter().nth(active) {
                root = root.child(panel.theme(&theme));
            }
        }

        root.children(self.children)
    }
}

/// Container strip for [`TabsTrigger`] buttons (`TabsList` in `tabs.tsx`).
#[derive(IntoElement)]
pub struct TabsList {
    variant: TabsListVariant,
    orientation: TabsOrientation,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for TabsList {
    fn default() -> Self {
        Self::new()
    }
}

impl TabsList {
    pub fn new() -> Self {
        Self {
            variant: TabsListVariant::Default,
            orientation: TabsOrientation::Horizontal,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: TabsListVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for TabsList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TabsList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TabsList {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_horizontal = matches!(self.orientation, TabsOrientation::Horizontal);
        let is_line = matches!(self.variant, TabsListVariant::Line);

        let bg = if is_line {
            tokens.background.opacity(0.0)
        } else if tokens.is_dark {
            theme.bg_surface
        } else {
            tokens.secondary
        };

        div()
            .flex()
            .p(px(4.0)) // p-1
            .rounded(PresetRadius::NONE)
            .bg(bg)
            .text_color(tokens.muted_foreground)
            .cx_when(is_line, |d| d.gap(px(4.0)))
            .cx_when(is_horizontal, |d| {
                d.flex_row()
                    .items_center()
                    .justify_center()
                    .h(PresetSpacing::CONTROL_DEFAULT)
            })
            .cx_when(!is_horizontal, |d| d.flex_col().items_stretch())
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Selectable tab button (`TabsTrigger` in `tabs.tsx`).
#[derive(IntoElement)]
pub struct TabsTrigger {
    index: usize,
    label: SharedString,
    icon_start: Option<AppIcon>,
    icon_end: Option<AppIcon>,
    active: bool,
    disabled: bool,
    list_variant: TabsListVariant,
    orientation: TabsOrientation,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_select: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_arrow: Option<Rc<dyn Fn(isize, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl TabsTrigger {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            index: 0,
            label: label.into(),
            icon_start: None,
            icon_end: None,
            active: false,
            disabled: false,
            list_variant: TabsListVariant::Default,
            orientation: TabsOrientation::Horizontal,
            focus_handle: None,
            theme: None,
            on_select: None,
            on_arrow: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn icon_start(mut self, icon: AppIcon) -> Self {
        self.icon_start = Some(icon);
        self
    }

    pub fn icon_end(mut self, icon: AppIcon) -> Self {
        self.icon_end = Some(icon);
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

    pub fn list_variant(mut self, variant: TabsListVariant) -> Self {
        self.list_variant = variant;
        self
    }

    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
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

    pub fn on_select(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    pub(crate) fn on_arrow_step(
        mut self,
        handler: impl Fn(isize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_arrow = Some(Rc::new(handler));
        self
    }
}

impl Styled for TabsTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TabsTrigger {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let is_line = matches!(self.list_variant, TabsListVariant::Line);
        let is_horizontal = matches!(self.orientation, TabsOrientation::Horizontal);

        let transparent = tokens.background.opacity(0.0);
        let (bg, border, fg, icon_tone) = if self.active {
            let active_bg = if is_line {
                transparent
            } else if tokens.is_dark {
                theme.bg_elevated.opacity(0.55)
            } else {
                tokens.card
            };
            let active_border = if !is_line && tokens.is_dark {
                tokens.input
            } else {
                transparent
            };
            (
                active_bg,
                active_border,
                tokens.foreground,
                IconTone::Default,
            )
        } else {
            (
                transparent,
                transparent,
                if tokens.is_dark {
                    tokens.muted_foreground
                } else {
                    tokens.foreground.opacity(0.60)
                },
                IconTone::Muted,
            )
        };

        let mut tab = div()
            .id(ElementId::NamedInteger(
                "tab-trigger".into(),
                self.index as u64,
            ))
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .px(px(16.0))
            .py(px(6.0))
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(border)
            .bg(bg)
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(fg)
            .whitespace_nowrap()
            .cx_when(is_horizontal, |d| d.h_full().justify_center())
            .cx_when(!is_horizontal, |d| d.w_full().justify_start().py(px(8.0)))
            .preset_ring(is_focused, false, &theme)
            .preset_disabled(self.disabled)
            .refine_style(&self.style);

        if let Some(icon) = self.icon_start {
            tab = tab.child(app_icon(icon, IconSize::Sm, icon_tone, &theme));
        }
        tab = tab.child(to_preset_uppercase(self.label.as_ref()));
        if let Some(icon) = self.icon_end {
            tab = tab.child(app_icon(icon, IconSize::Sm, icon_tone, &theme));
        }

        if is_line && self.active {
            let indicator = if is_horizontal {
                div()
                    .absolute()
                    .left_0()
                    .right_0()
                    .bottom(px(-5.0))
                    .h(px(2.0))
                    .bg(tokens.foreground)
            } else {
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .right(px(-4.0))
                    .w(px(2.0))
                    .bg(tokens.foreground)
            };
            tab = tab.child(indicator);
        }

        if let Some(ref fh) = self.focus_handle {
            tab = tab.track_focus(fh);
        }

        if !self.disabled {
            let hover_fg = tokens.foreground;
            tab = tab.cursor_pointer().hover(move |s| s.text_color(hover_fg));
            if let Some(cb) = self.on_select {
                let click_cb = cb.clone();
                let arrow_cb = self.on_arrow;
                tab = tab.on_click(move |_, window, cx| {
                    click_cb(window, cx);
                });
                tab = tab.on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    match key {
                        "enter" | "space" => cb(window, cx),
                        "right" | "down" => {
                            if let Some(ref step) = arrow_cb {
                                step(1, window, cx);
                            }
                        }
                        "left" | "up" => {
                            if let Some(ref step) = arrow_cb {
                                step(-1, window, cx);
                            }
                        }
                        _ => {}
                    }
                });
            }
        }

        tab
    }
}

/// Active panel body for a tab (`TabsContent` in `tabs.tsx`).
#[derive(IntoElement)]
pub struct TabsContent {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for TabsContent {
    fn default() -> Self {
        Self::new()
    }
}

impl TabsContent {
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

impl Styled for TabsContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TabsContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TabsContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .flex_1()
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.foreground)
            .refine_style(&self.style)
            .children(self.children)
    }
}

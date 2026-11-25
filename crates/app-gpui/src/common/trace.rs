//! Step-by-step execution trace and collapsible inspection primitives ported from
//! `web/src/components/ui/trace.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Trace`: `flex w-full min-w-0 flex-col gap-3`
//! - `TraceLine`: `flex max-w-full min-w-0 items-center gap-2 text-sm text-muted-foreground`
//!   with `TraceStatus` (`Running`, `Error` -> `text-destructive`, `Pending` -> `opacity-60`)
//! - `TraceIcon`, `TraceTitle` (`min-w-0 truncate`), `TraceDescription` (`shrink-0 text-muted-foreground/70`)
//! - `TraceCollapsible`, `TraceCollapsibleTrigger` (trailing `AppIcon::ChevronRight` / `ChevronDown`),
//!   and `TraceCollapsibleContent` (`mt-3 mb-1 ml-[7px] border-l pl-4 pr-1 text-muted-foreground`)
//!   built directly atop [`crate::common::collapsible`].

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px,
};

use crate::common::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::common::utils::{PresetRadius, PresetTokens, focus_ring_shadow};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Execution status of a [`TraceLine`] or [`TraceCollapsibleTrigger`] (`status` in `trace.tsx`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceStatus {
    Running,
    Error,
    Pending,
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

/// Vertical stack of execution or audit trace rows (`data-slot="trace"`).
#[derive(IntoElement)]
pub struct Trace {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for Trace {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Trace {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Trace {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Trace {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .gap(px(12.0))
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Single inline status row (`data-slot="trace-line"`).
#[derive(IntoElement)]
pub struct TraceLine {
    id: ElementId,
    status: Option<TraceStatus>,
    focus_handle: Option<FocusHandle>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TraceLine {
    pub fn new() -> Self {
        Self {
            id: ElementId::Name("trace-line".into()),
            status: None,
            focus_handle: None,
            on_click: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn status(mut self, status: TraceStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn focus_handle(mut self, handle: &FocusHandle) -> Self {
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

impl Default for TraceLine {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TraceLine {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TraceLine {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TraceLine {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let is_error = self.status == Some(TraceStatus::Error);
        let is_pending = self.status == Some(TraceStatus::Pending);
        let is_running = self.status == Some(TraceStatus::Running);
        let base_color = if is_error {
            tokens.destructive
        } else if is_running {
            tokens.foreground
        } else {
            tokens.muted_foreground
        };
        let hover_color = if is_error {
            tokens.destructive
        } else {
            tokens.foreground
        };
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));

        let mut el = div()
            .id(self.id)
            .flex()
            .max_w_full()
            .min_w_0()
            .items_center()
            .gap(px(8.0))
            .rounded(PresetRadius::NONE)
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .text_color(base_color)
            .when(is_pending, |d| d.opacity(0.6))
            .when(is_focused, |d| d.shadow(focus_ring_shadow(&theme)));

        if let Some(ref fh) = self.focus_handle {
            el = el.track_focus(fh);
        }

        if let Some(cb) = self.on_click {
            el = el
                .cursor_pointer()
                .hover(move |s| s.text_color(hover_color))
                .on_click(move |ev, win, app| cb(ev, win, app));
        }

        el.refine_style(&self.style).children(self.children)
    }
}

/// `16x16` leading/trailing vector icon slot inside a [`TraceLine`] (`data-slot="trace-icon"`).
#[derive(IntoElement)]
pub struct TraceIcon {
    icon: Option<AppIcon>,
    tone: IconTone,
    trailing_compact: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TraceIcon {
    pub fn new() -> Self {
        Self {
            icon: None,
            tone: IconTone::Muted,
            trailing_compact: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn tone(mut self, tone: IconTone) -> Self {
        self.tone = tone;
        self
    }

    /// Tightens left margin when placed immediately after [`TraceTitle`] (`[[data-slot=trace-title]~&]:-ml-1`).
    pub fn trailing_compact(mut self, compact: bool) -> Self {
        self.trailing_compact = compact;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for TraceIcon {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TraceIcon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TraceIcon {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TraceIcon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let mut el = div()
            .flex()
            .size(px(16.0))
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .when(self.trailing_compact, |d| d.ml(px(-4.0)));

        if let Some(icon) = self.icon {
            el = el.child(app_icon(icon, IconSize::Md, self.tone, &theme));
        }

        el.refine_style(&self.style).children(self.children)
    }
}

/// Truncated primary label of a [`TraceLine`] (`data-slot="trace-title"`).
#[derive(IntoElement)]
pub struct TraceTitle {
    text: Option<SharedString>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TraceTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            text: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for TraceTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TraceTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TraceTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div().min_w_0().truncate();
        if let Some(text) = self.text {
            el = el.child(text);
        }
        el.refine_style(&self.style).children(self.children)
    }
}

/// Secondary duration/detail label on a [`TraceLine`] (`data-slot="trace-description"`).
#[derive(IntoElement)]
pub struct TraceDescription {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TraceDescription {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            text: None,
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

impl Styled for TraceDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TraceDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TraceDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .flex_shrink_0()
            .text_color(tokens.muted_foreground.opacity(0.7));
        if let Some(text) = self.text {
            el = el.child(text);
        }
        el.refine_style(&self.style).children(self.children)
    }
}

/// Expandable disclosure step inside a [`Trace`] (`data-slot="trace-collapsible"`).
#[derive(IntoElement)]
pub struct TraceCollapsible {
    id: ElementId,
    open: Option<bool>,
    default_open: bool,
    disabled: bool,
    trigger: Option<TraceCollapsibleTrigger>,
    content: Option<TraceCollapsibleContent>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl TraceCollapsible {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: None,
            default_open: false,
            disabled: false,
            trigger: None,
            content: None,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn trigger(mut self, trigger: TraceCollapsibleTrigger) -> Self {
        self.trigger = Some(trigger);
        self
    }

    pub fn content(mut self, content: TraceCollapsibleContent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for TraceCollapsible {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TraceCollapsible {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let initial = self.default_open;
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, _| initial);
        let is_open = self.open.unwrap_or_else(|| *state.read(cx));

        let mut collapsible = Collapsible::new(self.id)
            .open(is_open)
            .disabled(self.disabled)
            .min_w_0();

        let is_controlled = self.open.is_some();
        let st = state.clone();
        let user_cb = self.on_open_change;
        collapsible = collapsible.on_open_change(move |next_open, win, app| {
            if !is_controlled {
                st.update(app, |v, app_cx| {
                    *v = *next_open;
                    app_cx.notify();
                });
            }
            if let Some(ref cb) = user_cb {
                cb(next_open, win, app);
            }
        });

        if let Some(mut trig) = self.trigger {
            trig.expanded = is_open;
            collapsible = collapsible.trigger(trig.into_collapsible_trigger(window, cx));
        }

        if let Some(panel) = self.content {
            collapsible = collapsible.content(panel.into_collapsible_content(cx));
        }

        collapsible.refine_style(&self.style)
    }
}

/// Trigger header for [`TraceCollapsible`] (`data-slot="trace-collapsible-trigger"`).
pub struct TraceCollapsibleTrigger {
    id: ElementId,
    expanded: bool,
    status: Option<TraceStatus>,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TraceCollapsibleTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            expanded: false,
            status: None,
            disabled: false,
            focus_handle: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    pub fn status(mut self, status: TraceStatus) -> Self {
        self.status = Some(status);
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

    fn into_collapsible_trigger(self, window: &mut Window, cx: &mut App) -> CollapsibleTrigger {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_error = self.status == Some(TraceStatus::Error);
        let text_color = if is_error {
            tokens.destructive
        } else {
            tokens.muted_foreground
        };
        let hover_color = if is_error {
            tokens.destructive
        } else {
            tokens.foreground
        };
        let chevron = if self.expanded {
            AppIcon::ChevronDown
        } else {
            AppIcon::ChevronRight
        };
        let chevron_tone = if is_error {
            IconTone::Error
        } else {
            IconTone::Muted
        };
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));

        let mut trig = CollapsibleTrigger::new(self.id)
            .disabled(self.disabled)
            .max_w_full()
            .min_w_0()
            .rounded(PresetRadius::NONE)
            .when(is_focused, |t| t.shadow(focus_ring_shadow(&theme)));

        if let Some(ref fh) = self.focus_handle {
            trig = trig.track_focus(fh);
        }

        let inner = div()
            .flex()
            .max_w_full()
            .min_w_0()
            .items_center()
            .gap(px(4.0))
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .text_color(text_color)
            .hover(move |s| s.text_color(hover_color))
            .refine_style(&self.style)
            .children(self.children)
            .child(TraceIcon::new().icon(chevron).tone(chevron_tone).theme(&theme));

        trig.child(inner)
    }
}

impl Styled for TraceCollapsibleTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TraceCollapsibleTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

/// Indented left-bordered detail body inside [`TraceCollapsible`] (`data-slot="trace-collapsible-content"`).
pub struct TraceCollapsibleContent {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TraceCollapsibleContent {
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

    fn into_collapsible_content(self, cx: &mut App) -> CollapsibleContent {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let inner = div()
            .mt(px(12.0))
            .mb(px(4.0))
            .ml(px(7.0))
            .min_w_0()
            .border_l_1()
            .border_color(tokens.border)
            .pr(px(4.0))
            .pl(px(16.0))
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .line_height(px(22.0))
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .children(self.children);

        CollapsibleContent::new().min_w_0().child(inner)
    }
}

impl Default for TraceCollapsibleContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TraceCollapsibleContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TraceCollapsibleContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

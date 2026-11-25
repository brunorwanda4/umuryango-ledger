//! Chat and activity bubble primitives ported from `web/src/components/ui/bubble.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `BubbleGroup`: `flex min-w-0 flex-col gap-2`
//! - `Bubble`: `relative flex w-fit max-w-[80%] min-w-0 flex-col gap-1` (`max-w-full` for `Ghost`,
//!   end-aligned when `BubbleAlign::End`)
//! - `BubbleContent`: `rounded-none border px-4 py-3 text-sm leading-relaxed` (`p-0` for `Ghost`)
//!   supporting all 7 variants (`Default`, `Secondary`, `Muted`, `Tinted`, `Outline`, `Ghost`,
//!   `Destructive`) and optional interactive click + focus-ring behavior
//! - `BubbleReactions`: `absolute flex w-fit shrink-0 items-center justify-center gap-1
//!   rounded-none bg-muted px-2 py-0.5 text-sm` anchored at `Top`/`Bottom` and `Start`/`End`.

use std::rc::Rc;

pub use gpui_kit::component::bubble::{
    Bubble as KitBubble, BubbleContent as KitBubbleContent, BubbleGroup as KitBubbleGroup,
    BubbleReactionSide as KitBubbleReactionSide, BubbleReactions as KitBubbleReactions,
    BubbleVariant as KitBubbleVariant,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px, relative, transparent_black,
};

use crate::common::utils::{PresetRadius, PresetTokens, focus_ring_shadow};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Visual tone matching `bubbleVariants.variant` in `bubble.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}

/// Horizontal alignment matching `align` (`"start" | "end"`) in `bubble.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BubbleAlign {
    #[default]
    Start,
    End,
}

/// Vertical anchor side matching `BubbleReactions` `side` (`"top" | "bottom"`) in `bubble.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BubbleReactionSide {
    Top,
    #[default]
    Bottom,
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

/// Vertical stack of related bubbles (`data-slot="bubble-group"`).
#[derive(IntoElement)]
pub struct BubbleGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl BubbleGroup {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for BubbleGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BubbleGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for BubbleGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BubbleGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(8.0))
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Individual bubble wrapper (`data-slot="bubble"`).
#[derive(IntoElement)]
pub struct Bubble {
    variant: BubbleVariant,
    align: BubbleAlign,
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Bubble {
    pub fn new() -> Self {
        Self {
            variant: BubbleVariant::Default,
            align: BubbleAlign::Start,
            text: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: BubbleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn align(mut self, align: BubbleAlign) -> Self {
        self.align = align;
        self
    }

    /// Convenience shorthand to render a styled [`BubbleContent`] carrying `text`
    /// with this bubble's `variant` and `align`.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for Bubble {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Bubble {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Bubble {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Bubble {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_end = self.align == BubbleAlign::End;
        let is_ghost = self.variant == BubbleVariant::Ghost;

        let mut root = div()
            .relative()
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(4.0))
            .font_family(FONT_FAMILY)
            .when_else(
                is_ghost,
                |el| el.max_w_full(),
                |el| el.max_w(relative(0.8)),
            )
            .when(is_end, |el| el.ml_auto().items_end());

        if let Some(text) = self.text {
            let mut content = BubbleContent::new()
                .variant(self.variant)
                .align(self.align)
                .child(text);
            if let Some(ref theme) = self.theme {
                content = content.theme(theme);
            }
            root = root.child(content);
        }

        root.refine_style(&self.style).children(self.children)
    }
}

/// Styled bubble surface (`data-slot="bubble-content"`).
#[derive(IntoElement)]
pub struct BubbleContent {
    id: ElementId,
    variant: BubbleVariant,
    align: BubbleAlign,
    focus_handle: Option<FocusHandle>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl BubbleContent {
    pub fn new() -> Self {
        Self {
            id: ElementId::Name("bubble-content".into()),
            variant: BubbleVariant::Default,
            align: BubbleAlign::Start,
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

    pub fn variant(mut self, variant: BubbleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn align(mut self, align: BubbleAlign) -> Self {
        self.align = align;
        self
    }

    pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
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

impl Default for BubbleContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BubbleContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for BubbleContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BubbleContent {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_dark = theme.mode == ThemeMode::Dark;

        let (bg, fg, border, hover_bg) = match self.variant {
            BubbleVariant::Default => (
                tokens.primary,
                tokens.primary_foreground,
                transparent_black(),
                tokens.primary.opacity(0.82),
            ),
            BubbleVariant::Secondary => (
                tokens.secondary,
                tokens.secondary_foreground,
                transparent_black(),
                tokens.accent,
            ),
            BubbleVariant::Muted => (
                tokens.muted,
                tokens.foreground,
                transparent_black(),
                tokens.accent,
            ),
            BubbleVariant::Tinted => (
                if is_dark {
                    theme.primary_deep.opacity(0.65)
                } else {
                    tokens.primary.opacity(0.12)
                },
                tokens.foreground,
                transparent_black(),
                if is_dark {
                    theme.primary_deep.opacity(0.82)
                } else {
                    tokens.primary.opacity(0.18)
                },
            ),
            BubbleVariant::Outline => (
                tokens.background,
                tokens.foreground,
                tokens.border,
                if is_dark {
                    tokens.input.opacity(0.3)
                } else {
                    tokens.muted
                },
            ),
            BubbleVariant::Ghost => (
                transparent_black(),
                tokens.foreground,
                transparent_black(),
                if is_dark {
                    tokens.muted.opacity(0.5)
                } else {
                    tokens.muted
                },
            ),
            BubbleVariant::Destructive => (
                if is_dark {
                    tokens.destructive.opacity(0.2)
                } else {
                    tokens.destructive.opacity(0.1)
                },
                tokens.destructive,
                transparent_black(),
                if is_dark {
                    tokens.destructive.opacity(0.3)
                } else {
                    tokens.destructive.opacity(0.2)
                },
            ),
        };

        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let is_interactive = self.on_click.is_some();
        let is_ghost = self.variant == BubbleVariant::Ghost;

        let mut el = div()
            .id(self.id)
            .max_w_full()
            .min_w_0()
            .overflow_hidden()
            .rounded(PresetRadius::NONE)
            .bg(bg)
            .text_color(fg)
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .line_height(px(22.0))
            .when_else(
                is_ghost,
                |d| d.p_0(),
                |d| d.border_1().border_color(border).px(px(16.0)).py(px(12.0)),
            )
            .when(self.align == BubbleAlign::End, |d| d.ml_auto())
            .when(is_focused, |d| {
                d.border_1()
                    .border_color(tokens.ring)
                    .shadow(focus_ring_shadow(&theme))
            });

        if let Some(fh) = self.focus_handle {
            el = el.track_focus(&fh);
        }

        if is_interactive {
            el = el.cursor_pointer().hover(move |s| s.bg(hover_bg));
        }

        if let Some(handler) = self.on_click {
            el = el.on_click(move |ev, win, app| handler(ev, win, app));
        }

        el.refine_style(&self.style).children(self.children)
    }
}

/// Floating reaction badge strip anchored to a [`Bubble`] (`data-slot="bubble-reactions"`).
#[derive(IntoElement)]
pub struct BubbleReactions {
    side: BubbleReactionSide,
    align: BubbleAlign,
    button_container: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl BubbleReactions {
    pub fn new() -> Self {
        Self {
            side: BubbleReactionSide::Bottom,
            align: BubbleAlign::End,
            button_container: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: BubbleReactionSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: BubbleAlign) -> Self {
        self.align = align;
        self
    }

    /// Removes inner padding when children are interactive reaction buttons (`has-[button]:p-0`).
    pub fn button_container(mut self, button_container: bool) -> Self {
        self.button_container = button_container;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for BubbleReactions {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BubbleReactions {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for BubbleReactions {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BubbleReactions {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .absolute()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .gap(px(4.0))
            .rounded(PresetRadius::NONE)
            .bg(tokens.muted)
            .text_color(tokens.foreground)
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .border_2()
            .border_color(tokens.card)
            .when_else(
                self.button_container,
                |d| d.p_0(),
                |d| d.px(px(8.0)).py(px(2.0)),
            )
            .when_else(
                self.side == BubbleReactionSide::Top,
                |d| d.top(px(-12.0)),
                |d| d.bottom(px(-12.0)),
            )
            .when_else(
                self.align == BubbleAlign::Start,
                |d| d.left(px(12.0)),
                |d| d.right(px(12.0)),
            )
            .refine_style(&self.style)
            .children(self.children)
    }
}

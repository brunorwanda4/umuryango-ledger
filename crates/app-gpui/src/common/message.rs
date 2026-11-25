//! Structured chat/activity message layout primitives ported from
//! `web/src/components/ui/message.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `MessageGroup`: `flex min-w-0 flex-col gap-2`
//! - `Message`: `relative flex w-full min-w-0 gap-2 text-sm` (`flex-row-reverse` when `End`)
//! - `MessageAvatar`: `min-w-8 shrink-0 self-end overflow-hidden rounded-full bg-muted`
//!   (explicit `PresetRadius::FULL` circular badge; offsets upward when `has_footer` is set)
//! - `MessageContent`: `flex w-full min-w-0 flex-col gap-2.5` (`items-end` when `End`)
//! - `MessageHeader` & `MessageFooter`: `px-4 text-xs font-medium tracking-wide
//!   text-muted-foreground uppercase` (`px-0` in ghost mode; footer right-aligned when `End`).

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::message::{
    Message as KitMessage, MessageAlignment as KitMessageAlignment,
    MessageAvatar as KitMessageAvatar, MessageContent as KitMessageContent,
    MessageFooter as KitMessageFooter, MessageGroup as KitMessageGroup,
    MessageHeader as KitMessageHeader,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, to_preset_uppercase};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Horizontal alignment matching `align` (`"start" | "end"`) in `message.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageAlign {
    #[default]
    Start,
    End,
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

/// Vertical stack of consecutive messages from the same turn (`data-slot="message-group"`).
#[derive(IntoElement)]
pub struct MessageGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageGroup {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for MessageGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageGroup {
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

/// Single message row pairing an optional [`MessageAvatar`] with [`MessageContent`] (`data-slot="message"`).
#[derive(IntoElement)]
pub struct Message {
    align: MessageAlign,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Message {
    pub fn new() -> Self {
        Self {
            align: MessageAlign::Start,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn align(mut self, align: MessageAlign) -> Self {
        self.align = align;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Message {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Message {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Message {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .relative()
            .flex()
            .w_full()
            .min_w_0()
            .gap(px(8.0))
            .font_family(FONT_FAMILY)
            .text_size(px(14.0))
            .text_color(tokens.foreground)
            .when_else(
                self.align == MessageAlign::End,
                |d| d.flex_row_reverse(),
                |d| d.flex_row(),
            )
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Circular sender avatar container anchored to the bottom of a [`Message`] (`data-slot="message-avatar"`).
#[derive(IntoElement)]
pub struct MessageAvatar {
    initials: Option<SharedString>,
    has_footer: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageAvatar {
    pub fn new() -> Self {
        Self {
            initials: None,
            has_footer: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn initials(mut self, initials: impl Into<SharedString>) -> Self {
        self.initials = Some(initials.into());
        self
    }

    /// Offsets the avatar upward by `32px` when the sibling [`MessageContent`] renders a [`MessageFooter`].
    pub fn has_footer(mut self, has_footer: bool) -> Self {
        self.has_footer = has_footer;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for MessageAvatar {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageAvatar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageAvatar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageAvatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .flex()
            .min_w(px(32.0))
            .h(px(32.0))
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .mt_auto()
            .overflow_hidden()
            .rounded(PresetRadius::FULL)
            .bg(tokens.muted)
            .text_color(tokens.muted_foreground)
            .font_family(FONT_FAMILY)
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .when(self.has_footer, |d| d.mb(px(32.0)));

        if let Some(initials) = self.initials {
            el = el.child(to_preset_uppercase(initials.as_ref()));
        }

        el.refine_style(&self.style).children(self.children)
    }
}

/// Main vertical column for headers, bubbles, attachments, and footers (`data-slot="message-content"`).
#[derive(IntoElement)]
pub struct MessageContent {
    align: MessageAlign,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageContent {
    pub fn new() -> Self {
        Self {
            align: MessageAlign::Start,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn align(mut self, align: MessageAlign) -> Self {
        self.align = align;
        self
    }
}

impl Default for MessageContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .min_w_0()
            .gap(px(10.0))
            .font_family(FONT_FAMILY)
            .when(self.align == MessageAlign::End, |d| d.items_end())
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Editorial uppercase sender/meta header above a message (`data-slot="message-header"`).
#[derive(IntoElement)]
pub struct MessageHeader {
    title: Option<SharedString>,
    ghost: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageHeader {
    pub fn new() -> Self {
        Self {
            title: None,
            ghost: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Removes horizontal padding when paired with a ghost bubble (`group-has-data-[variant=ghost]:px-0`).
    pub fn ghost(mut self, ghost: bool) -> Self {
        self.ghost = ghost;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for MessageHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .flex()
            .max_w_full()
            .min_w_0()
            .items_center()
            .gap(px(6.0))
            .when_else(self.ghost, |d| d.px_0(), |d| d.px(px(16.0)))
            .font_family(FONT_FAMILY)
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.muted_foreground);

        if let Some(title) = self.title {
            el = el.child(to_preset_uppercase(title.as_ref()));
        }

        el.refine_style(&self.style).children(self.children)
    }
}

/// Editorial uppercase timestamp/delivery footer below a message (`data-slot="message-footer"`).
#[derive(IntoElement)]
pub struct MessageFooter {
    text: Option<SharedString>,
    align: MessageAlign,
    ghost: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageFooter {
    pub fn new() -> Self {
        Self {
            text: None,
            align: MessageAlign::Start,
            ghost: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn align(mut self, align: MessageAlign) -> Self {
        self.align = align;
        self
    }

    pub fn ghost(mut self, ghost: bool) -> Self {
        self.ghost = ghost;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for MessageFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut el = div()
            .flex()
            .max_w_full()
            .min_w_0()
            .items_center()
            .gap(px(6.0))
            .when_else(self.ghost, |d| d.px_0(), |d| d.px(px(16.0)))
            .when(self.align == MessageAlign::End, |d| d.justify_end())
            .font_family(FONT_FAMILY)
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.muted_foreground);

        if let Some(text) = self.text {
            el = el.child(to_preset_uppercase(text.as_ref()));
        }

        el.refine_style(&self.style).children(self.children)
    }
}

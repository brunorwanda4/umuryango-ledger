//! Code snippet & diff viewer primitives ported from `web/src/components/ui/code-block.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `CodeBlock`: `Default` (`border-input text-foreground`, dark `bg-input/30`) |
//!   `Destructive` (`border-destructive/30 text-destructive`), sizes `Default` (`text-sm`) | `Sm` (`text-xs`)
//! - `CodeBlockHeader`, `CodeBlockTitle`, `CodeBlockActions`, `CodeBlockFooter`
//! - `CodeBlockCopyButton`: copies `value` via `cx.write_to_clipboard` and toggles `AppIcon::Copy` / `AppIcon::Check`
//! - `CodeBlockContent` & `CodeBlockLine`: `line_numbers`, `start_line`, `wrap`, and diff variants
//!   (`Default`, `Added`, `Removed`, `Highlighted`).

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ClipboardItem, ElementId, FontWeight, InteractiveElement,
    IntoElement, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px, relative,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Visual tone matching `codeBlockVariants.variant` in `code-block.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeBlockVariant {
    #[default]
    Default,
    Destructive,
}

/// Density preset matching `size = "default" | "sm"` in `code-block.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeBlockSize {
    #[default]
    Default,
    Sm,
}

/// Diff / highlight variant matching `codeBlockLineVariants.variant` in `code-block.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeBlockLineVariant {
    #[default]
    Default,
    Added,
    Removed,
    Highlighted,
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

/// Root code block container (`CodeBlock` in `code-block.tsx`).
#[derive(IntoElement)]
pub struct CodeBlock {
    id: ElementId,
    variant: CodeBlockVariant,
    size: CodeBlockSize,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CodeBlock {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: CodeBlockVariant::Default,
            size: CodeBlockSize::Default,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: CodeBlockVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn destructive(mut self) -> Self {
        self.variant = CodeBlockVariant::Destructive;
        self
    }

    pub fn size(mut self, size: CodeBlockSize) -> Self {
        self.size = size;
        self
    }

    pub fn sm(mut self) -> Self {
        self.size = CodeBlockSize::Sm;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for CodeBlock {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CodeBlock {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CodeBlock {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_destr = matches!(self.variant, CodeBlockVariant::Destructive);
        let is_sm = matches!(self.size, CodeBlockSize::Sm);

        let border_c = if is_destr {
            tokens
                .destructive
                .opacity(if tokens.is_dark { 0.40 } else { 0.30 })
        } else {
            tokens.input
        };
        let fg_c = if is_destr {
            tokens.destructive
        } else {
            tokens.foreground
        };

        div()
            .id(self.id)
            .relative()
            .flex()
            .w_full()
            .min_w_0()
            .flex_col()
            .overflow_hidden()
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(border_c)
            .cx_when(!is_destr && tokens.is_dark, |d| {
                d.bg(tokens.input.opacity(0.30))
            })
            .font_family(FONT_FAMILY)
            .cx_when(is_sm, |d| d.text_xs())
            .cx_when(!is_sm, |d| d.text_sm())
            .text_color(fg_c)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Header bar inside [`CodeBlock`] (`CodeBlockHeader` in `code-block.tsx`).
#[derive(IntoElement)]
pub struct CodeBlockHeader {
    size: CodeBlockSize,
    variant: CodeBlockVariant,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CodeBlockHeader {
    pub fn new() -> Self {
        Self {
            size: CodeBlockSize::Default,
            variant: CodeBlockVariant::Default,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CodeBlockSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CodeBlockVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for CodeBlockHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CodeBlockHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CodeBlockHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CodeBlockHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let pad = if matches!(self.size, CodeBlockSize::Sm) {
            12.0
        } else {
            16.0
        };
        let text_c = if matches!(self.variant, CodeBlockVariant::Destructive) {
            tokens.destructive
        } else {
            tokens.muted_foreground
        };

        div()
            .flex()
            .min_w_0()
            .flex_shrink_0()
            .items_center()
            .gap(px(8.0))
            .px(px(pad))
            .pt(px(pad))
            .text_color(text_c)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Filename or snippet heading (`CodeBlockTitle` in `code-block.tsx`).
#[derive(IntoElement)]
pub struct CodeBlockTitle {
    text: SharedString,
    icon: Option<AppIcon>,
    size: CodeBlockSize,
    variant: CodeBlockVariant,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl CodeBlockTitle {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            size: CodeBlockSize::Default,
            variant: CodeBlockVariant::Default,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn size(mut self, size: CodeBlockSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: CodeBlockVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for CodeBlockTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CodeBlockTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_sm = matches!(self.size, CodeBlockSize::Sm);
        let is_destr = matches!(self.variant, CodeBlockVariant::Destructive);

        let mut row = div()
            .flex()
            .min_w_0()
            .items_center()
            .gap(px(6.0))
            .font_family(FONT_FAMILY)
            .cx_when(is_sm, |d| d.text_xs())
            .cx_when(!is_sm, |d| d.text_sm())
            .font_weight(FontWeight::MEDIUM)
            .text_color(if is_destr {
                tokens.destructive
            } else {
                tokens.foreground
            });

        if let Some(icon) = self.icon {
            let icon_sz = if is_sm { IconSize::Sm } else { IconSize::Md };
            let tone = if is_destr {
                IconTone::Error
            } else {
                IconTone::Muted
            };
            row = row.child(app_icon(icon, icon_sz, tone, &theme));
        }

        row.child(self.text).refine_style(&self.style)
    }
}

/// Right-aligned action cluster (`CodeBlockActions` in `code-block.tsx`).
#[derive(IntoElement)]
pub struct CodeBlockActions {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CodeBlockActions {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for CodeBlockActions {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CodeBlockActions {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CodeBlockActions {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CodeBlockActions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .ml_auto()
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap(px(4.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Clipboard copy button (`CodeBlockCopyButton` in `code-block.tsx`).
#[derive(IntoElement)]
pub struct CodeBlockCopyButton {
    id: ElementId,
    value: SharedString,
    variant: ButtonVariant,
    size: ButtonSize,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl CodeBlockCopyButton {
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            variant: ButtonVariant::Ghost,
            size: ButtonSize::IconXs,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
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

impl Styled for CodeBlockCopyButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CodeBlockCopyButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-copied", self.id).into()),
            cx,
            |_w, _cx| false,
        );
        let copied = *state.read(cx);
        let icon = if copied {
            AppIcon::Check
        } else {
            AppIcon::Copy
        };

        let val = self.value;
        let on_extra = self.on_click;

        Button::new(self.id)
            .variant(self.variant)
            .size(self.size)
            .icon(icon)
            .theme(&theme)
            .on_click(move |ev, window, cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(val.to_string()));
                state.update(cx, |v, cx| {
                    *v = true;
                    cx.notify();
                });
                if let Some(cb) = &on_extra {
                    cb(ev, window, cx);
                }
            })
            .refine_style(&self.style)
    }
}

/// Individual code or diff line (`CodeBlockLine` in `code-block.tsx`).
#[derive(IntoElement)]
pub struct CodeBlockLine {
    text: SharedString,
    line_number: Option<usize>,
    variant: CodeBlockLineVariant,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl CodeBlockLine {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            line_number: None,
            variant: CodeBlockLineVariant::Default,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn line_number(mut self, line_number: usize) -> Self {
        self.line_number = Some(line_number);
        self
    }

    pub fn variant(mut self, variant: CodeBlockLineVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn added(self) -> Self {
        self.variant(CodeBlockLineVariant::Added)
    }

    pub fn removed(self) -> Self {
        self.variant(CodeBlockLineVariant::Removed)
    }

    pub fn highlighted(self) -> Self {
        self.variant(CodeBlockLineVariant::Highlighted)
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for CodeBlockLine {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CodeBlockLine {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let marker_prefix = match self.variant {
            CodeBlockLineVariant::Added => Some(("+", tokens.muted_foreground)),
            CodeBlockLineVariant::Removed => Some(("-", tokens.destructive)),
            _ => None,
        };

        let mut row = div()
            .flex()
            .w_full()
            .items_start()
            .gap(px(12.0))
            .px(px(16.0))
            .line_height(relative(1.6))
            .cx_when(
                matches!(
                    self.variant,
                    CodeBlockLineVariant::Added | CodeBlockLineVariant::Highlighted
                ),
                |d| d.bg(tokens.muted),
            )
            .cx_when(matches!(self.variant, CodeBlockLineVariant::Removed), |d| {
                d.bg(tokens.destructive_soft)
            });

        if let Some((sym, col)) = marker_prefix {
            row = row.child(div().w(px(10.0)).text_color(col).child(sym));
        }

        if let Some(num) = self.line_number {
            row = row.child(
                div()
                    .min_w(px(24.0))
                    .text_color(tokens.muted_foreground)
                    .child(num.to_string()),
            );
        }

        let display_text: SharedString = if self.text.is_empty() {
            " ".into()
        } else {
            self.text
        };

        row.child(div().min_w_0().flex_1().child(display_text))
            .refine_style(&self.style)
    }
}

/// Scrollable code area (`CodeBlockContent` in `code-block.tsx`).
#[derive(IntoElement)]
pub struct CodeBlockContent {
    id: ElementId,
    code: Option<SharedString>,
    wrap: bool,
    line_numbers: bool,
    start_line: usize,
    size: CodeBlockSize,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CodeBlockContent {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            code: None,
            wrap: false,
            line_numbers: false,
            start_line: 1,
            size: CodeBlockSize::Default,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn code(mut self, code: impl Into<SharedString>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn line_numbers(mut self, line_numbers: bool) -> Self {
        self.line_numbers = line_numbers;
        self
    }

    pub fn start_line(mut self, start_line: usize) -> Self {
        self.start_line = start_line.max(1);
        self
    }

    pub fn size(mut self, size: CodeBlockSize) -> Self {
        self.size = size;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for CodeBlockContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CodeBlockContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CodeBlockContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let pad_v = if matches!(self.size, CodeBlockSize::Sm) {
            12.0
        } else {
            16.0
        };

        let mut area = div()
            .id(self.id)
            .flex()
            .flex_col()
            .min_w_0()
            .py(px(pad_v))
            .cx_when(!self.wrap, |d| d.overflow_x_scroll().whitespace_nowrap());

        if let Some(raw) = self.code {
            let trimmed = raw.strip_suffix('\n').unwrap_or(raw.as_ref());
            for (idx, line_str) in trimmed.split('\n').enumerate() {
                let mut ln = CodeBlockLine::new(line_str.to_string()).theme(&theme);
                if self.line_numbers {
                    ln = ln.line_number(self.start_line + idx);
                }
                area = area.child(ln);
            }
        }

        area.children(self.children).refine_style(&self.style)
    }
}

/// Footer bar inside [`CodeBlock`] (`CodeBlockFooter` in `code-block.tsx`).
#[derive(IntoElement)]
pub struct CodeBlockFooter {
    size: CodeBlockSize,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CodeBlockFooter {
    pub fn new() -> Self {
        Self {
            size: CodeBlockSize::Default,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CodeBlockSize) -> Self {
        self.size = size;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for CodeBlockFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CodeBlockFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CodeBlockFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CodeBlockFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let pad = if matches!(self.size, CodeBlockSize::Sm) {
            12.0
        } else {
            16.0
        };

        div()
            .flex()
            .min_w_0()
            .flex_shrink_0()
            .items_center()
            .gap(px(8.0))
            .px(px(pad))
            .pb(px(pad))
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .children(self.children)
    }
}

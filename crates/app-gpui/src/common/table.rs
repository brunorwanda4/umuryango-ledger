//! Composable data table primitives ported from `web/src/components/ui/table.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Table`: `relative w-full overflow-x-auto` container + `w-full text-sm` table surface
//! - `TableHeader`: `[&_tr]:border-b` header section
//! - `TableBody`: `[&_tr:last-child]:border-0` body rows section
//! - `TableFooter`: `border-t bg-muted/50 font-medium` summary section
//! - `TableRow`: `border-b hover:bg-muted/50 data-[state=selected]:bg-muted`
//! - `TableHead`: `h-12 px-3 text-xs font-medium uppercase text-muted-foreground`
//! - `TableCell`: `p-3 align-middle whitespace-nowrap`
//! - `TableCaption`: `mt-4 text-sm text-muted-foreground`.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::table::{
    Column as KitTableColumn, ColumnSort as KitColumnSort, DataTable as KitDataTable,
    Table as KitTable, TableDelegate as KitTableDelegate, TableState as KitTableState,
};
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, to_preset_uppercase};
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

/// Horizontal cell alignment inside [`TableHead`] and [`TableCell`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Root table container (`Table` in `table.tsx`).
#[derive(IntoElement)]
pub struct Table {
    id: ElementId,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Table {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
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

impl Styled for Table {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Table {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Table {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .id(self.id)
            .relative()
            .w_full()
            .overflow_x_scroll()
            .rounded(PresetRadius::NONE)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .font_family(FONT_FAMILY)
                    .text_sm()
                    .text_color(tokens.foreground)
                    .refine_style(&self.style)
                    .children(self.children),
            )
    }
}

/// Table header group (`TableHeader` in `table.tsx`).
#[derive(IntoElement)]
pub struct TableHeader {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableHeader {
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

impl Default for TableHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .flex_col()
            .w_full()
            .border_b_1()
            .border_color(tokens.border)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Table body group (`TableBody` in `table.tsx`).
#[derive(IntoElement)]
pub struct TableBody {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableBody {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for TableBody {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableBody {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableBody {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableBody {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Table footer summary group (`TableFooter` in `table.tsx`).
#[derive(IntoElement)]
pub struct TableFooter {
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableFooter {
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

impl Default for TableFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .flex_col()
            .w_full()
            .border_t_1()
            .border_color(tokens.border)
            .bg(tokens.muted.opacity(0.5))
            .font_weight(FontWeight::MEDIUM)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Table row (`TableRow` in `table.tsx`).
#[derive(IntoElement)]
pub struct TableRow {
    id: Option<ElementId>,
    selected: bool,
    border_bottom: bool,
    hoverable: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            id: None,
            selected: false,
            border_bottom: true,
            hoverable: true,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn border_bottom(mut self, border_bottom: bool) -> Self {
        self.border_bottom = border_bottom;
        self
    }

    pub fn last(mut self, is_last: bool) -> Self {
        self.border_bottom = !is_last;
        self
    }

    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.hoverable = hoverable;
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

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableRow {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableRow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let hover_bg = tokens.muted.opacity(0.5);

        let base = div()
            .flex()
            .w_full()
            .items_center()
            .cx_when(self.border_bottom, |d| {
                d.border_b_1().border_color(tokens.border)
            })
            .cx_when(self.selected, |d| d.bg(tokens.muted))
            .cx_when(!self.selected && self.hoverable, |d| {
                d.hover(move |s| s.bg(hover_bg))
            })
            .refine_style(&self.style)
            .children(self.children);

        if let Some(id) = self.id {
            let mut stateful = base.id(id);
            if let Some(on_click) = self.on_click {
                stateful = stateful
                    .cursor_pointer()
                    .on_click(move |ev, window, cx| on_click(ev, window, cx));
            }
            stateful.into_any_element()
        } else {
            base.into_any_element()
        }
    }
}

/// Column header cell (`TableHead` in `table.tsx`).
#[derive(IntoElement)]
pub struct TableHead {
    label: Option<SharedString>,
    align: TableAlign,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableHead {
    pub fn new() -> Self {
        Self {
            label: None,
            align: TableAlign::Left,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }

    pub fn align_right(self) -> Self {
        self.align(TableAlign::Right)
    }

    pub fn align_center(self) -> Self {
        self.align(TableAlign::Center)
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for TableHead {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableHead {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableHead {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableHead {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut cell = div()
            .flex()
            .flex_1()
            .min_w_0()
            .h(px(48.0))
            .px(px(12.0))
            .items_center()
            .whitespace_nowrap()
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.muted_foreground);

        cell = match self.align {
            TableAlign::Left => cell.justify_start(),
            TableAlign::Center => cell.justify_center(),
            TableAlign::Right => cell.justify_end(),
        };

        if let Some(label) = &self.label {
            cell = cell.child(to_preset_uppercase(label.as_ref()));
        }

        cell.children(self.children).refine_style(&self.style)
    }
}

/// Data cell (`TableCell` in `table.tsx`).
#[derive(IntoElement)]
pub struct TableCell {
    text: Option<SharedString>,
    align: TableAlign,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableCell {
    pub fn new() -> Self {
        Self {
            text: None,
            align: TableAlign::Left,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }

    pub fn align_right(self) -> Self {
        self.align(TableAlign::Right)
    }

    pub fn align_center(self) -> Self {
        self.align(TableAlign::Center)
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for TableCell {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableCell {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableCell {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableCell {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut cell = div()
            .flex()
            .flex_1()
            .min_w_0()
            .p(px(12.0))
            .items_center()
            .whitespace_nowrap()
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.foreground);

        cell = match self.align {
            TableAlign::Left => cell.justify_start(),
            TableAlign::Center => cell.justify_center(),
            TableAlign::Right => cell.justify_end(),
        };

        if let Some(text) = self.text {
            cell = cell.child(text);
        }

        cell.children(self.children).refine_style(&self.style)
    }
}

/// Descriptive table caption (`TableCaption` in `table.tsx`).
#[derive(IntoElement)]
pub struct TableCaption {
    text: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableCaption {
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

impl Styled for TableCaption {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableCaption {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableCaption {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut cap = div()
            .mt(px(16.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.muted_foreground);

        if let Some(text) = self.text {
            cap = cap.child(text);
        }

        cap.children(self.children).refine_style(&self.style)
    }
}

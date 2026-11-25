//! List row item and sub-slot primitives ported from `web/src/components/ui/item.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Item`: `flex w-full flex-wrap items-center rounded-none border text-sm`
//!   - Variants: `Default`, `Outline`, `Muted`
//!   - Sizes: `Default` (`gap-3.5 px-4 py-3.5`), `Sm` (`gap-3.5 px-3.5 py-3`), `Xs` (`gap-2.5 px-3 py-2.5`)
//! - `ItemGroup`, `ItemSeparator`, `ItemMedia` (`Default` | `Icon` | `Image`), `ItemContent`,
//!   `ItemTitle` (`text-xs font-semibold uppercase`), `ItemDescription`, `ItemActions`,
//!   `ItemHeader`, `ItemFooter`.

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, div, px, relative,
};

use crate::common::separator::Separator;
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Visual border/fill variants matching `itemVariants.variant` in `item.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

/// Padding & gap scale matching `itemVariants.size` in `item.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
    Xs,
}

/// Media container mode matching `itemMediaVariants.variant` in `item.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
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

/// Vertical list of [`Item`] rows (`ItemGroup` in `item.tsx`).
#[derive(IntoElement)]
pub struct ItemGroup {
    size: ItemSize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for ItemGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemGroup {
    pub fn new() -> Self {
        Self {
            size: ItemSize::Default,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }
}

impl Styled for ItemGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ItemGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let gap = match self.size {
            ItemSize::Default => px(16.0), // gap-4
            ItemSize::Sm => px(10.0),      // gap-2.5
            ItemSize::Xs => px(8.0),       // gap-2
        };
        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(gap)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Horizontal divider between [`Item`] rows (`ItemSeparator` in `item.tsx`).
pub fn item_separator(theme: &SemanticTheme) -> Separator {
    Separator::horizontal().theme(theme).my(px(8.0))
}

/// Versatile list item row (`Item` in `web/src/components/ui/item.tsx`).
#[derive(IntoElement)]
pub struct Item {
    id: Option<ElementId>,
    variant: ItemVariant,
    size: ItemSize,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}

impl Item {
    pub fn new() -> Self {
        Self {
            id: None,
            variant: ItemVariant::Default,
            size: ItemSize::Default,
            focus_handle: None,
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

    pub fn variant(mut self, variant: ItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn outline(self) -> Self {
        self.variant(ItemVariant::Outline)
    }

    pub fn muted(self) -> Self {
        self.variant(ItemVariant::Muted)
    }

    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
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
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Styled for Item {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Item {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Item {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));

        let transparent = tokens.background.opacity(0.0);
        let (bg, border) = match self.variant {
            ItemVariant::Default => (transparent, transparent),
            ItemVariant::Outline => (transparent, tokens.border),
            ItemVariant::Muted => (tokens.muted.opacity(0.5), transparent),
        };

        let (gap, pad_x, pad_y) = match self.size {
            ItemSize::Default => (px(14.0), px(16.0), px(14.0)),
            ItemSize::Sm => (px(14.0), px(14.0), px(12.0)),
            ItemSize::Xs => (px(10.0), px(12.0), px(10.0)),
        };

        let mut base = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .w_full()
            .items_center()
            .gap(gap)
            .px(pad_x)
            .py(pad_y)
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(border)
            .bg(bg)
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.foreground)
            .preset_ring(is_focused, false, &theme)
            .refine_style(&self.style)
            .children(self.children);

        if let Some(ref fh) = self.focus_handle {
            base = base.track_focus(fh);
        }

        if let Some(id) = self.id {
            let mut interactive = base.id(id);
            if let Some(handler) = self.on_click {
                let hover_bg = tokens.muted;
                interactive = interactive
                    .cursor_pointer()
                    .hover(move |s| s.bg(hover_bg))
                    .on_click(handler);
            }
            interactive.into_any_element()
        } else {
            base.into_any_element()
        }
    }
}

/// Leading icon, avatar, or thumbnail slot (`ItemMedia` in `item.tsx`).
#[derive(IntoElement)]
pub struct ItemMedia {
    variant: ItemMediaVariant,
    parent_size: ItemSize,
    icon: Option<AppIcon>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for ItemMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemMedia {
    pub fn new() -> Self {
        Self {
            variant: ItemMediaVariant::Default,
            parent_size: ItemSize::Default,
            icon: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn from_icon(icon: AppIcon) -> Self {
        Self::new().variant(ItemMediaVariant::Icon).icon(icon)
    }

    pub fn variant(mut self, variant: ItemMediaVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn parent_size(mut self, size: ItemSize) -> Self {
        self.parent_size = size;
        self
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ItemMedia {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ItemMedia {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemMedia {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let img_dim = match self.parent_size {
            ItemSize::Default => px(40.0), // size-10
            ItemSize::Sm => px(32.0),      // size-8
            ItemSize::Xs => px(24.0),      // size-6
        };

        let mut base = div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .rounded(PresetRadius::NONE)
            .cx_when(matches!(self.variant, ItemMediaVariant::Image), |d| {
                d.size(img_dim).overflow_hidden()
            })
            .refine_style(&self.style);

        if let Some(icon) = self.icon {
            base = base.child(app_icon(icon, IconSize::Md, IconTone::Default, &theme));
        }

        base.children(self.children)
    }
}

/// Primary flexible text stack (`ItemContent` in `item.tsx`).
#[derive(IntoElement)]
pub struct ItemContent {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for ItemContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for ItemContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ItemContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_1()
            .min_w_0()
            .flex_col()
            .gap(px(4.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Uppercase title inside [`ItemContent`] (`ItemTitle` in `item.tsx`).
#[derive(IntoElement)]
pub struct ItemTitle {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ItemTitle {
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

impl Styled for ItemTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ItemTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .truncate()
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(tokens.foreground)
            .refine_style(&self.style)
            .child(to_preset_uppercase(self.text.as_ref()))
    }
}

/// Secondary text inside [`ItemContent`] (`ItemDescription` in `item.tsx`).
#[derive(IntoElement)]
pub struct ItemDescription {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ItemDescription {
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

impl Styled for ItemDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ItemDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        div()
            .font_family(FONT_FAMILY)
            .text_sm()
            .line_height(relative(1.5))
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .child(self.text)
    }
}

/// Trailing button/control slot (`ItemActions` in `item.tsx`).
#[derive(IntoElement)]
pub struct ItemActions {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for ItemActions {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemActions {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for ItemActions {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ItemActions {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemActions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Full-width top row inside [`Item`] (`ItemHeader` in `item.tsx`).
#[derive(IntoElement)]
pub struct ItemHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for ItemHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemHeader {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for ItemHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ItemHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Full-width bottom row inside [`Item`] (`ItemFooter` in `item.tsx`).
#[derive(IntoElement)]
pub struct ItemFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for ItemFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemFooter {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for ItemFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ItemFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

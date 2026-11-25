//! Horizontal application menubar primitives ported from `web/src/components/ui/menubar.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Menubar`: `flex h-10 items-center rounded-none border p-1`
//! - `MenubarTrigger`: `flex items-center rounded-none px-2 py-0.75 text-sm font-medium hover:bg-muted aria-expanded:bg-muted`
//! - `MenubarContent`: `min-w-48 rounded-none bg-popover p-1.5 shadow-md ring-1 ring-foreground/10`
//!   (`align = Start`, `align_offset = -4.0`, `side_offset = 8.0`)
//! - `MenubarCheckboxItem` & `MenubarRadioItem`: left-positioned `AppIcon::Check` at `left-3` (`pl-9.5`)
//! - Reuses `DropdownMenu*` primitives directly per Rule 2 (Strict Component Reuse).

pub use gpui_kit::component::menu::AppMenuBar as KitAppMenuBar;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::common::dropdown_menu::{
    DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuContent, DropdownMenuGroup,
    DropdownMenuItem, DropdownMenuItemVariant, DropdownMenuLabel, DropdownMenuPortal,
    DropdownMenuRadioGroup, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuSub,
    DropdownMenuSubContent, DropdownMenuSubTrigger,
};
use crate::common::popover::PopoverAlign;
use crate::common::utils::{PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt};
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

/// Deferred GPUI overlay layer (`MenubarPortal` in `menubar.tsx`).
pub type MenubarPortal = DropdownMenuPortal;

/// Menu dropdown root inside [`Menubar`] (`MenubarMenu` in `menubar.tsx`).
pub type MenubarMenu = DropdownMenu;

/// Group wrapper (`MenubarGroup` in `menubar.tsx`).
pub type MenubarGroup = DropdownMenuGroup;

/// Thin divider (`MenubarSeparator` in `menubar.tsx`).
pub type MenubarSeparator = DropdownMenuSeparator;

/// Uppercase group heading (`MenubarLabel` in `menubar.tsx`).
pub type MenubarLabel = DropdownMenuLabel;

/// Actionable menubar item (`MenubarItem` in `menubar.tsx`).
pub type MenubarItem = DropdownMenuItem;

/// Visual variant (`Default` | `Destructive`) for [`MenubarItem`].
pub type MenubarItemVariant = DropdownMenuItemVariant;

/// Right-aligned shortcut hint (`MenubarShortcut` in `menubar.tsx`).
pub type MenubarShortcut = DropdownMenuShortcut;

/// Radio group inside [`MenubarContent`] (`MenubarRadioGroup` in `menubar.tsx`).
pub type MenubarRadioGroup = DropdownMenuRadioGroup;

/// Nested submenu container (`MenubarSub` in `menubar.tsx`).
pub type MenubarSub = DropdownMenuSub;

/// Nested submenu trigger (`MenubarSubTrigger` in `menubar.tsx`).
pub type MenubarSubTrigger = DropdownMenuSubTrigger;

/// Nested submenu popup (`MenubarSubContent` in `menubar.tsx`).
pub type MenubarSubContent = DropdownMenuSubContent;

/// Horizontal menubar bar (`Menubar` in `menubar.tsx`).
#[derive(IntoElement)]
pub struct Menubar {
    id: ElementId,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Menubar {
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

impl Styled for Menubar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Menubar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Menubar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .id(self.id)
            .flex()
            .h(PresetSpacing::CONTROL_DEFAULT)
            .items_center()
            .gap(px(4.0))
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(tokens.border)
            .bg(tokens.background)
            .p(px(4.0))
            .font_family(FONT_FAMILY)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Top-level menubar button (`MenubarTrigger` in `menubar.tsx`).
#[derive(IntoElement)]
pub struct MenubarTrigger {
    id: ElementId,
    label: SharedString,
    expanded: bool,
    disabled: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl MenubarTrigger {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            expanded: false,
            disabled: false,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
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

impl Styled for MenubarTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for MenubarTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let hover_bg = tokens.muted;

        div()
            .id(self.id)
            .flex()
            .items_center()
            .rounded(PresetRadius::NONE)
            .px(px(8.0))
            .py(px(3.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(tokens.foreground)
            .cx_when(self.expanded, |d| d.bg(tokens.muted))
            .cx_when(!self.disabled, |d| {
                d.cursor_pointer().hover(move |s| s.bg(hover_bg))
            })
            .preset_disabled(self.disabled)
            .child(self.label)
            .refine_style(&self.style)
    }
}

/// Builds a [`DropdownMenuContent`] pre-configured with `menubar.tsx` offsets
/// (`align = Start`, `align_offset = -4.0`, `side_offset = 8.0`).
pub fn menubar_content() -> DropdownMenuContent {
    DropdownMenuContent::new()
        .align(PopoverAlign::Start)
        .align_offset(-4.0)
        .side_offset(8.0)
}

/// Popup menu surface alias (`MenubarContent` in `menubar.tsx`).
pub type MenubarContent = DropdownMenuContent;

/// Builds a [`DropdownMenuCheckboxItem`] with left-side check indicator (`left-3`, `pl-9.5`)
/// matching `MenubarCheckboxItem` in `menubar.tsx`.
pub fn menubar_checkbox_item(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
) -> DropdownMenuCheckboxItem {
    DropdownMenuCheckboxItem::new(id, label).indicator_left(true)
}

/// Checkable menubar row (`MenubarCheckboxItem` in `menubar.tsx`).
pub type MenubarCheckboxItem = DropdownMenuCheckboxItem;

/// Builds a [`DropdownMenuCheckboxItem`] with left-side radio indicator (`left-3`, `pl-9.5`)
/// matching `MenubarRadioItem` in `menubar.tsx`.
pub fn menubar_radio_item(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
) -> DropdownMenuCheckboxItem {
    DropdownMenuCheckboxItem::new(id, label).indicator_left(true)
}

/// Radio menubar row (`MenubarRadioItem` in `menubar.tsx`).
pub type MenubarRadioItem = DropdownMenuCheckboxItem;

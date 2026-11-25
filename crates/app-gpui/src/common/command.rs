//! Command palette primitives ported from `web/src/components/ui/command.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Command`: `flex size-full flex-col overflow-hidden bg-popover text-popover-foreground`
//! - `CommandDialog`: wraps [`Dialog`] + [`DialogContent`] (`overflow-hidden p-0`, `show_close_button = false`)
//! - `CommandInput`: `p-1` + [`InputGroup`] (`px-3`) + [`input_group_input`] + `AppIcon::Search`
//! - `CommandList`: `max-h-72 overflow-y-scroll`
//! - `CommandEmpty`: `py-6 text-center text-sm`
//! - `CommandGroup`: `p-1.5` with `px-3 py-2 text-xs font-semibold uppercase text-muted-foreground` heading
//! - `CommandItem`: `px-3 py-2 text-sm rounded-none`, `selected` `bg-muted`, `checked` `AppIcon::Check`
//! - `CommandShortcut`: `ml-auto text-xs text-muted-foreground`
//! - `CommandSeparator`: `-mx-1.5 my-1.5 h-px bg-border/50`.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::command::{
    Command as KitCommand, CommandEntry as KitCommandEntry, CommandGroup as KitCommandGroup,
    CommandItem as KitCommandItem, CommandState as KitCommandState,
};
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, Entity, FontWeight, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled,
    Window, div, px,
};

use crate::common::dialog::{Dialog, DialogContent};
use crate::common::dropdown_menu::{DropdownMenuSeparator, DropdownMenuShortcut};
use crate::common::input::InputState;
use crate::common::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, input_group_input,
};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
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

/// Thin divider (`CommandSeparator` in `command.tsx`).
pub type CommandSeparator = DropdownMenuSeparator;

/// Right-aligned shortcut hint (`CommandShortcut` in `command.tsx`).
pub type CommandShortcut = DropdownMenuShortcut;

/// Command palette surface (`Command` in `command.tsx`).
#[derive(IntoElement)]
pub struct Command {
    id: ElementId,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Command {
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

impl Styled for Command {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Command {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Command {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .id(self.id)
            .flex()
            .size_full()
            .flex_col()
            .overflow_hidden()
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .font_family(FONT_FAMILY)
            .text_color(tokens.popover_foreground)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Modal command palette dialog (`CommandDialog` in `command.tsx`).
#[derive(IntoElement)]
pub struct CommandDialog {
    id: ElementId,
    open: Option<bool>,
    default_open: bool,
    show_close_button: bool,
    trigger: Option<AnyElement>,
    theme: Option<SemanticTheme>,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CommandDialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: None,
            default_open: false,
            show_close_button: false,
            trigger: None,
            theme: None,
            on_open_change: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
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

    pub fn show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = show;
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
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

impl Styled for CommandDialog {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CommandDialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CommandDialog {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);

        let content =
            DialogContent::new(ElementId::Name(format!("{}-cmd-content", self.id).into()))
                .show_close_button(self.show_close_button)
                .theme(&theme)
                .overflow_hidden()
                .p(px(0.0))
                .gap(px(0.0))
                .refine_style(&self.style)
                .children(self.children);

        let mut dlg = Dialog::new(self.id)
            .default_open(self.default_open)
            .content(content);

        if let Some(open) = self.open {
            dlg = dlg.open(open);
        }
        if let Some(trig) = self.trigger {
            dlg = dlg.trigger(trig);
        }
        if let Some(on_change) = self.on_open_change {
            dlg = dlg.on_open_change(move |open, window, cx| on_change(open, window, cx));
        }

        dlg
    }
}

/// Underline search input with trailing `AppIcon::Search` (`CommandInput` in `command.tsx`).
#[derive(IntoElement)]
pub struct CommandInput {
    state: Entity<InputState>,
    disabled: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl CommandInput {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            disabled: false,
            theme: None,
            style: StyleRefinement::default(),
        }
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

impl Styled for CommandInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CommandInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let inp = input_group_input(&self.state, &theme)
            .disabled(self.disabled)
            .px(px(8.0));

        div().p(px(4.0)).child(
            InputGroup::new()
                .disabled(self.disabled)
                .theme(&theme)
                .px(px(12.0))
                .refine_style(&self.style)
                .child(inp)
                .child(
                    InputGroupAddon::new()
                        .align(InputGroupAddonAlign::InlineEnd)
                        .theme(&theme)
                        .child(div().opacity(0.5).child(app_icon(
                            AppIcon::Search,
                            IconSize::Sm,
                            IconTone::Muted,
                            &theme,
                        ))),
                ),
        )
    }
}

/// Scrollable list of command groups/items (`CommandList` in `command.tsx`).
#[derive(IntoElement)]
pub struct CommandList {
    id: ElementId,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CommandList {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for CommandList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CommandList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CommandList {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .max_h(px(288.0))
            .overflow_y_scroll()
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Empty-state fallback (`CommandEmpty` in `command.tsx`).
#[derive(IntoElement)]
pub struct CommandEmpty {
    text: SharedString,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl CommandEmpty {
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

impl Styled for CommandEmpty {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CommandEmpty {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .flex()
            .w_full()
            .justify_center()
            .py(px(24.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.muted_foreground)
            .child(self.text)
            .refine_style(&self.style)
    }
}

/// Grouped section of commands (`CommandGroup` in `command.tsx`).
#[derive(IntoElement)]
pub struct CommandGroup {
    heading: Option<SharedString>,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CommandGroup {
    pub fn new() -> Self {
        Self {
            heading: None,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn heading(mut self, heading: impl Into<SharedString>) -> Self {
        self.heading = Some(heading.into());
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Default for CommandGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CommandGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CommandGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CommandGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut grp = div()
            .flex()
            .flex_col()
            .overflow_hidden()
            .p(px(6.0))
            .text_color(tokens.foreground);

        if let Some(h) = self.heading {
            grp = grp.child(
                div()
                    .px(px(12.0))
                    .py(px(8.0))
                    .font_family(FONT_FAMILY)
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(tokens.muted_foreground)
                    .child(to_preset_uppercase(h.as_ref())),
            );
        }

        grp.children(self.children).refine_style(&self.style)
    }
}

/// Selectable command entry (`CommandItem` in `command.tsx`).
#[derive(IntoElement)]
pub struct CommandItem {
    id: ElementId,
    label: Option<SharedString>,
    icon: Option<AppIcon>,
    shortcut: Option<SharedString>,
    selected: bool,
    checked: bool,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_select: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CommandItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: Some(label.into()),
            icon: None,
            shortcut: None,
            selected: false,
            checked: false,
            disabled: false,
            theme: None,
            on_select: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
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

    pub fn on_select(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl Styled for CommandItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CommandItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CommandItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let hover_bg = tokens.muted;

        let mut row = div()
            .id(self.id)
            .relative()
            .flex()
            .w_full()
            .items_center()
            .gap(px(8.0))
            .rounded(PresetRadius::NONE)
            .px(px(12.0))
            .py(px(8.0))
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(tokens.foreground)
            .cx_when(self.selected, |d| d.bg(tokens.muted))
            .cx_when(!self.disabled, |d| {
                d.cursor_pointer().hover(move |s| s.bg(hover_bg))
            })
            .preset_disabled(self.disabled);

        if let Some(icon) = self.icon {
            let tone = if self.selected {
                IconTone::Default
            } else {
                IconTone::Muted
            };
            row = row.child(app_icon(icon, IconSize::Sm, tone, &theme));
        }

        if let Some(lbl) = self.label {
            row = row.child(lbl);
        }

        row = row.children(self.children);

        if let Some(sc) = self.shortcut {
            row = row.child(CommandShortcut::new(sc).theme(&theme));
        } else if self.checked {
            row = row.child(div().ml_auto().child(app_icon(
                AppIcon::Check,
                IconSize::Sm,
                IconTone::Default,
                &theme,
            )));
        }

        if !self.disabled
            && let Some(on_select) = self.on_select
        {
            row = row.on_click(move |ev, window, cx| on_select(ev, window, cx));
        }

        row.refine_style(&self.style)
    }
}

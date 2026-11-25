//! Native-style compact select primitive ported from `web/src/components/ui/native-select.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - Trigger: `h-10` (`data-[size=sm]:h-9`) `w-full rounded-none border-b border-b-input bg-transparent py-2 pr-8 pl-0 text-sm focus-visible:border-b-ring aria-invalid:border-b-destructive`
//! - Trailing chevron: `app_icon(AppIcon::ChevronDown, IconSize::Sm, IconTone::Muted, theme)`
//! - Exports `NativeSelect`, `NativeSelectOption`, and `NativeSelectOptGroup` with native GPUI
//!   floating dropdown overlay (`deferred`) and keyboard navigation.

use std::rc::Rc;

use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    App, ElementId, FocusHandle, FontWeight, InteractiveElement, IntoElement, KeyDownEvent,
    MouseButton, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window, deferred, div, px,
};

use crate::common::utils::{
    PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt, overlay_surface_shadow,
    to_preset_uppercase,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Trigger height matching `size?: "sm" | "default"` in `native-select.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NativeSelectSize {
    #[default]
    Default,
    Sm,
}

/// Selectable item matching `NativeSelectOption` in `native-select.tsx`.
#[derive(Debug, Clone)]
pub struct NativeSelectOption {
    pub value: SharedString,
    pub label: SharedString,
    pub disabled: bool,
}

impl NativeSelectOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Grouped option section matching `NativeSelectOptGroup` in `native-select.tsx`.
#[derive(Debug, Clone)]
pub struct NativeSelectOptGroup {
    pub label: SharedString,
    pub options: Vec<NativeSelectOption>,
}

impl NativeSelectOptGroup {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            options: Vec::new(),
        }
    }

    pub fn option(mut self, option: NativeSelectOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn options(mut self, options: impl IntoIterator<Item = NativeSelectOption>) -> Self {
        self.options.extend(options);
        self
    }
}

#[derive(Debug, Clone)]
enum NativeSelectEntry {
    Option(NativeSelectOption),
    Group(NativeSelectOptGroup),
}

/// Underline select control matching `NativeSelect` in `web/src/components/ui/native-select.tsx`.
#[derive(IntoElement)]
pub struct NativeSelect {
    id: ElementId,
    entries: Vec<NativeSelectEntry>,
    selected_value: Option<SharedString>,
    placeholder: Option<SharedString>,
    size: NativeSelectSize,
    disabled: bool,
    invalid: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl NativeSelect {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            entries: Vec::new(),
            selected_value: None,
            placeholder: None,
            size: NativeSelectSize::Default,
            disabled: false,
            invalid: false,
            focus_handle: None,
            theme: None,
            on_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn option(mut self, option: NativeSelectOption) -> Self {
        self.entries.push(NativeSelectEntry::Option(option));
        self
    }

    pub fn options(mut self, options: impl IntoIterator<Item = NativeSelectOption>) -> Self {
        self.entries
            .extend(options.into_iter().map(NativeSelectEntry::Option));
        self
    }

    pub fn opt_group(mut self, group: NativeSelectOptGroup) -> Self {
        self.entries.push(NativeSelectEntry::Group(group));
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    pub fn selected_opt(mut self, value: Option<SharedString>) -> Self {
        self.selected_value = value;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn size(mut self, size: NativeSelectSize) -> Self {
        self.size = size;
        self
    }

    pub fn sm(self) -> Self {
        self.size(NativeSelectSize::Sm)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
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

    pub fn on_change(
        mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    fn flattened_options(&self) -> Vec<NativeSelectOption> {
        let mut flat = Vec::new();
        for entry in &self.entries {
            match entry {
                NativeSelectEntry::Option(opt) => flat.push(opt.clone()),
                NativeSelectEntry::Group(grp) => flat.extend(grp.options.iter().cloned()),
            }
        }
        flat
    }
}

impl Styled for NativeSelect {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NativeSelect {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let fallback_theme = SemanticTheme::from_config(
            &ThemeConfig::default(),
            if gpui_kit::component::Theme::global(cx).mode.is_dark() {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        );
        let theme = self.theme.clone().unwrap_or(fallback_theme);
        let tokens = PresetTokens::from_theme(&theme);

        let open_state = window.use_keyed_state(self.id.clone(), cx, |_, _| false);
        let is_open = *open_state.read(cx);

        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let border_bottom = if self.invalid {
            tokens.destructive
        } else if is_focused || is_open {
            tokens.ring
        } else {
            tokens.input
        };

        let height = match self.size {
            NativeSelectSize::Default => PresetSpacing::CONTROL_DEFAULT, // 40px
            NativeSelectSize::Sm => PresetSpacing::CONTROL_SM,           // 36px
        };

        let flat = self.flattened_options();
        let active_label = self
            .selected_value
            .as_ref()
            .and_then(|v| flat.iter().find(|o| &o.value == v).map(|o| o.label.clone()));

        let (display_text, text_color) = match active_label {
            Some(lbl) => (lbl, tokens.foreground),
            None => (
                self.placeholder.clone().unwrap_or_default(),
                tokens.muted_foreground,
            ),
        };

        let mut trigger = div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h(height)
            .w_full()
            .min_w_0()
            .py(px(8.0))
            .pl(px(0.0))
            .pr(px(4.0))
            .gap(px(8.0))
            .rounded(PresetRadius::NONE)
            .border_b_1()
            .border_color(border_bottom)
            .font_family(FONT_FAMILY)
            .text_sm()
            .text_color(text_color)
            .child(div().flex_1().truncate().child(display_text))
            .child(app_icon(
                AppIcon::ChevronDown,
                IconSize::Sm,
                IconTone::Muted,
                &theme,
            ));

        if let Some(ref fh) = self.focus_handle {
            trigger = trigger.track_focus(fh);
        }

        if !self.disabled {
            let toggle_state = open_state.clone();
            trigger = trigger.cursor_pointer().on_click(move |_, _, cx| {
                toggle_state.update(cx, |open, cx| {
                    *open = !*open;
                    cx.notify();
                });
            });

            if let Some(ref on_change) = self.on_change {
                let key_cb = on_change.clone();
                let key_state = open_state.clone();
                let enabled_opts: Vec<NativeSelectOption> =
                    flat.into_iter().filter(|o| !o.disabled).collect();
                let cur_val = self.selected_value.clone();
                trigger = trigger.on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    match key {
                        "escape" => {
                            key_state.update(cx, |open, cx| {
                                *open = false;
                                cx.notify();
                            });
                        }
                        "enter" | "space" => {
                            key_state.update(cx, |open, cx| {
                                *open = !*open;
                                cx.notify();
                            });
                        }
                        "down" if !enabled_opts.is_empty() => {
                            let cur_ix = cur_val
                                .as_ref()
                                .and_then(|v| enabled_opts.iter().position(|o| &o.value == v));
                            let next_ix = cur_ix.map_or(0, |i| (i + 1) % enabled_opts.len());
                            key_cb(&enabled_opts[next_ix].value, window, cx);
                        }
                        "up" if !enabled_opts.is_empty() => {
                            let len = enabled_opts.len();
                            let cur_ix = cur_val
                                .as_ref()
                                .and_then(|v| enabled_opts.iter().position(|o| &o.value == v));
                            let prev_ix = cur_ix.map_or(0, |i| (i + len - 1) % len);
                            key_cb(&enabled_opts[prev_ix].value, window, cx);
                        }
                        _ => {}
                    }
                });
            }
        }

        let mut wrapper = div()
            .relative()
            .w_full()
            .preset_disabled(self.disabled)
            .refine_style(&self.style)
            .child(trigger);

        if is_open && !self.disabled {
            let mut menu = div()
                .absolute()
                .top(height + px(4.0))
                .left_0()
                .right_0()
                .min_w(px(160.0))
                .max_h(px(260.0))
                .id(ElementId::Name("native-select-menu".into()))
                .overflow_y_scroll()
                .py(px(4.0))
                .rounded(PresetRadius::NONE)
                .bg(tokens.popover)
                .text_color(tokens.popover_foreground)
                .shadow(overlay_surface_shadow(&theme))
                .on_mouse_down_out({
                    let close_state = open_state.clone();
                    move |_, _, cx| {
                        close_state.update(cx, |open, cx| {
                            *open = false;
                            cx.notify();
                        });
                    }
                });

            let mut item_counter = 0usize;
            for entry in self.entries {
                match entry {
                    NativeSelectEntry::Option(opt) => {
                        menu = menu.child(render_option_row(
                            item_counter,
                            opt,
                            self.selected_value.as_ref(),
                            &open_state,
                            self.on_change.as_ref(),
                            &theme,
                        ));
                        item_counter += 1;
                    }
                    NativeSelectEntry::Group(grp) => {
                        menu = menu.child(
                            div()
                                .px(px(10.0))
                                .pt(px(8.0))
                                .pb(px(4.0))
                                .font_family(FONT_FAMILY)
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(tokens.muted_foreground)
                                .child(to_preset_uppercase(grp.label.as_ref())),
                        );
                        for opt in grp.options {
                            menu = menu.child(render_option_row(
                                item_counter,
                                opt,
                                self.selected_value.as_ref(),
                                &open_state,
                                self.on_change.as_ref(),
                                &theme,
                            ));
                            item_counter += 1;
                        }
                    }
                }
            }

            wrapper = wrapper.child(deferred(menu).with_priority(1));
        }

        wrapper
    }
}

fn render_option_row(
    idx: usize,
    opt: NativeSelectOption,
    selected_value: Option<&SharedString>,
    open_state: &gpui_kit::Entity<bool>,
    on_change: Option<&Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
    theme: &SemanticTheme,
) -> impl IntoElement {
    let tokens = PresetTokens::from_theme(theme);
    let is_selected = selected_value.is_some_and(|v| v == &opt.value);
    let hover_bg = tokens.accent;
    let val = opt.value.clone();

    let mut row = div()
        .id(ElementId::NamedInteger(
            "native-select-opt".into(),
            idx as u64,
        ))
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(px(8.0))
        .px(px(10.0))
        .py(px(6.0))
        .font_family(FONT_FAMILY)
        .text_sm()
        .text_color(tokens.popover_foreground)
        .cx_when(is_selected, |d| {
            d.bg(tokens.muted).font_weight(FontWeight::MEDIUM)
        })
        .preset_disabled(opt.disabled)
        .child(div().flex_1().truncate().child(opt.label));

    if is_selected {
        row = row.child(app_icon(
            AppIcon::Check,
            IconSize::Sm,
            IconTone::AccentPrimary,
            theme,
        ));
    }

    if !opt.disabled {
        let close_state = open_state.clone();
        let cb = on_change.cloned();
        row = row
            .cursor_pointer()
            .hover(move |s| s.bg(hover_bg))
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                close_state.update(cx, |open, cx| {
                    *open = false;
                    cx.notify();
                });
                if let Some(ref handler) = cb {
                    handler(&val, window, cx);
                }
            });
    }

    row
}

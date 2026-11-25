//! Keyboard shortcut badge primitives ported from `web/src/components/ui/kbd.tsx`.
//!
//! Matches `--preset b1oaDsAa`:
//! - `Kbd`: `inline-flex h-5.5 min-w-5.5 items-center justify-center gap-1 rounded-none bg-muted px-1.5 font-sans text-xs font-medium text-muted-foreground`
//! - `KbdGroup`: `inline-flex items-center gap-1`
//! Supports both arbitrary text/children and platform-formatted GPUI [`Keystroke`]s via `gpui_kit::component::kbd::Kbd`.

use gpui_kit::component::StyledExt as _;
use gpui_kit::component::kbd::Kbd as KitKbd;
use gpui_kit::{
    AnyElement, App, FontWeight, IntoElement, Keystroke, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens};
use crate::theme::{FONT_FAMILY, SemanticTheme};

/// Single keyboard key cap matching `Kbd` in `web/src/components/ui/kbd.tsx`.
#[derive(IntoElement)]
pub struct Kbd {
    label: Option<SharedString>,
    on_tooltip: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Kbd {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: Some(label.into()),
            on_tooltip: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            label: None,
            on_tooltip: false,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Formats a GPUI [`Keystroke`] using platform modifier conventions (`Ctrl+K` / `⌘K`).
    pub fn from_keystroke(stroke: &Keystroke) -> Self {
        Self::new(KitKbd::format(stroke))
    }

    /// Parses a keybinding string (e.g. `"ctrl-k"`) and formats it for the active platform.
    pub fn parse_shortcut(shortcut: &'static str) -> Self {
        match Keystroke::parse(shortcut) {
            Ok(stroke) => Self::from_keystroke(&stroke),
            Err(_) => Self::new(shortcut),
        }
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    /// Adapts colors when rendered inside `TooltipContent` (`bg-background/20 text-background`).
    pub fn on_tooltip(mut self, on_tooltip: bool) -> Self {
        self.on_tooltip = on_tooltip;
        self
    }
}

impl Styled for Kbd {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Kbd {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (bg, fg) = if let Some(ref theme) = self.theme {
            let tokens = PresetTokens::from_theme(theme);
            if self.on_tooltip {
                (
                    tokens
                        .background
                        .opacity(if tokens.is_dark { 0.10 } else { 0.20 }),
                    tokens.background,
                )
            } else {
                (
                    if tokens.is_dark {
                        theme.bg_surface
                    } else {
                        tokens.secondary
                    },
                    tokens.muted_foreground,
                )
            }
        } else {
            let kit = gpui_kit::component::Theme::global(cx);
            (kit.muted, kit.muted_foreground)
        };

        let mut base = div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .h(px(22.0)) // h-5.5
            .min_w(px(22.0)) // min-w-5.5
            .px(px(6.0)) // px-1.5
            .gap(px(4.0)) // gap-1
            .items_center()
            .justify_center()
            .rounded(PresetRadius::NONE)
            .bg(bg)
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(fg)
            .refine_style(&self.style);

        if let Some(label) = self.label {
            base = base.child(label);
        }
        base.children(self.children)
    }
}

/// Inline row grouping multiple [`Kbd`] key caps (`KbdGroup` in `kbd.tsx`).
#[derive(IntoElement)]
pub struct KbdGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Default for KbdGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl KbdGroup {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Styled for KbdGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for KbdGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for KbdGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .refine_style(&self.style)
            .children(self.children)
    }
}

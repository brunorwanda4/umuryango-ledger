//! Button primitive ported from `web/src/components/ui/button.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - Base: `inline-flex shrink-0 items-center justify-center rounded-none border text-xs font-semibold tracking-widest whitespace-nowrap uppercase`
//! - Variants: `Default`, `Outline`, `Secondary`, `Ghost`, `Destructive`, `Link`
//! - Sizes: `Default` (`h-10 px-6`), `Xs` (`h-7 px-3`), `Sm` (`h-9 px-4`), `Lg` (`h-11 px-8`),
//!   `Icon` (`size-10`), `IconXs` (`size-7`), `IconSm` (`size-9`), `IconLg` (`size-11`)
//! - Icons rendered strictly via `crate::components::icon::app_icon` (`IconSize::Xs` on `Xs`/`IconXs`, `IconSize::Sm` otherwise).

pub use gpui_kit::component::button::Button as KitButton;
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, KeyDownEvent, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::common::spinner::Spinner;
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, to_preset_uppercase};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Visual variants matching `buttonVariants.variant` in `web/src/components/ui/button.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Secondary,
    Ghost,
    Destructive,
    Link,
}

/// Dimensions matching `buttonVariants.size` in `web/src/components/ui/button.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Xs,
    Sm,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

impl ButtonSize {
    pub const fn is_icon_only(self) -> bool {
        matches!(
            self,
            Self::Icon | Self::IconXs | Self::IconSm | Self::IconLg
        )
    }

    pub const fn height_px(self) -> f32 {
        match self {
            Self::Default | Self::Icon => 40.0,
            Self::Xs | Self::IconXs => 28.0,
            Self::Sm | Self::IconSm => 36.0,
            Self::Lg | Self::IconLg => 44.0,
        }
    }

    pub const fn icon_size(self) -> IconSize {
        match self {
            Self::Xs | Self::IconXs => IconSize::Xs, // 12px (`size-3`)
            _ => IconSize::Sm,                       // 14px (`size-3.5`)
        }
    }
}

/// Interactive button matching `Button` in `web/src/components/ui/button.tsx`.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: Option<SharedString>,
    variant: ButtonVariant,
    size: ButtonSize,
    icon_start: Option<AppIcon>,
    icon_end: Option<AppIcon>,
    disabled: bool,
    loading: bool,
    invalid: bool,
    expanded: bool,
    focus_handle: Option<FocusHandle>,
    theme: Option<SemanticTheme>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            icon_start: None,
            icon_end: None,
            disabled: false,
            loading: false,
            invalid: false,
            expanded: false,
            focus_handle: None,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn outline(self) -> Self {
        self.variant(ButtonVariant::Outline)
    }

    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }

    pub fn ghost(self) -> Self {
        self.variant(ButtonVariant::Ghost)
    }

    pub fn destructive(self) -> Self {
        self.variant(ButtonVariant::Destructive)
    }

    pub fn link(self) -> Self {
        self.variant(ButtonVariant::Link)
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon_start = Some(icon);
        if !self.size.is_icon_only() && self.label.is_none() {
            self.size = ButtonSize::Icon;
        }
        self
    }

    pub fn icon_start(mut self, icon: AppIcon) -> Self {
        self.icon_start = Some(icon);
        self
    }

    pub fn icon_end(mut self, icon: AppIcon) -> Self {
        self.icon_end = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
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

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let fallback_theme = SemanticTheme::from_config(
            &ThemeConfig::default(),
            if gpui_kit::component::Theme::global(cx).mode.is_dark() {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        );
        let theme = self.theme.as_ref().unwrap_or(&fallback_theme);
        let tokens = PresetTokens::from_theme(theme);

        let is_focused = self
            .focus_handle
            .as_ref()
            .is_some_and(|fh| fh.is_focused(window));
        let is_disabled = self.disabled || self.loading;

        let transparent = tokens.background.opacity(0.0);
        let (bg, hover_bg, active_bg, border, fg, icon_tone) = match self.variant {
            ButtonVariant::Default => (
                tokens.primary,
                tokens.primary_hover,
                tokens.primary_pressed,
                transparent,
                tokens.primary_foreground,
                IconTone::OnPrimaryButton,
            ),
            ButtonVariant::Outline => (
                if self.expanded {
                    tokens.muted
                } else {
                    transparent
                },
                tokens.muted,
                tokens.secondary,
                tokens.border,
                tokens.foreground,
                IconTone::Default,
            ),
            ButtonVariant::Secondary => (
                tokens.secondary,
                tokens.secondary_hover,
                tokens.secondary_hover,
                transparent,
                tokens.secondary_foreground,
                IconTone::Default,
            ),
            ButtonVariant::Ghost => (
                if self.expanded {
                    tokens.muted
                } else {
                    transparent
                },
                tokens.muted,
                tokens.secondary,
                transparent,
                tokens.foreground,
                IconTone::Default,
            ),
            ButtonVariant::Destructive => (
                tokens.destructive_soft,
                tokens.destructive_hover,
                tokens.destructive_hover,
                transparent,
                tokens.destructive,
                IconTone::Expense,
            ),
            ButtonVariant::Link => (
                transparent,
                transparent,
                transparent,
                transparent,
                tokens.primary,
                IconTone::AccentPrimary,
            ),
        };

        let height = px(self.size.height_px());
        let icon_sz = self.size.icon_size();

        let mut el = div()
            .id(self.id)
            .flex()
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .rounded(PresetRadius::NONE)
            .border_1()
            .border_color(border)
            .bg(bg)
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(fg)
            .whitespace_nowrap()
            .preset_ring(is_focused, self.invalid, theme);

        if self.size.is_icon_only() {
            el = el.size(height);
        } else {
            let (base_px, icon_edge_px, gap_px) = match self.size {
                ButtonSize::Xs => (12.0, 8.0, 4.0),
                ButtonSize::Sm => (16.0, 12.0, 4.0),
                ButtonSize::Default => (24.0, 16.0, 6.0),
                ButtonSize::Lg => (32.0, 20.0, 6.0),
                _ => (24.0, 16.0, 6.0),
            };
            let pl = if self.icon_start.is_some() || self.loading {
                icon_edge_px
            } else {
                base_px
            };
            let pr = if self.icon_end.is_some() {
                icon_edge_px
            } else {
                base_px
            };
            el = el.h(height).pl(px(pl)).pr(px(pr)).gap(px(gap_px));
        }

        if let Some(ref fh) = self.focus_handle {
            el = el.track_focus(fh);
        }

        if is_disabled {
            el = el.opacity(0.5).cursor_default();
        } else {
            el = el
                .cursor_pointer()
                .hover(move |s| {
                    let s = s.bg(hover_bg);
                    if matches!(self.variant, ButtonVariant::Link) {
                        s.underline()
                    } else {
                        s
                    }
                })
                .active(move |s| s.bg(active_bg).mt(px(1.0)));

            if matches!(self.variant, ButtonVariant::Link) {
                el = el.underline();
            }

            if let Some(handler) = self.on_click {
                let handler = std::rc::Rc::new(handler);
                let click_cb = handler.clone();
                el = el.on_click(move |ev, window, cx| {
                    click_cb(ev, window, cx);
                });
                el = el.on_key_down(move |ev: &KeyDownEvent, window, cx| {
                    let key = ev.keystroke.key.as_str();
                    if key == "enter" || key == "space" {
                        handler(&ClickEvent::default(), window, cx);
                    }
                });
            }
        }

        el = el.refine_style(&self.style);

        if self.loading {
            el = el.child(Spinner::new().size(icon_sz).color(fg));
        } else if let Some(icon) = self.icon_start {
            if matches!(self.variant, ButtonVariant::Default) {
                el = el.child(SpinnerOrIcon::colored(icon, icon_sz, fg, theme));
            } else {
                el = el.child(app_icon(icon, icon_sz, icon_tone, theme));
            }
        }

        if !self.size.is_icon_only() {
            if let Some(label) = self.label {
                el = el.child(to_preset_uppercase(label.as_ref()));
            }
        }

        el = el.children(self.children);

        if !self.size.is_icon_only() {
            if let Some(icon) = self.icon_end {
                if matches!(self.variant, ButtonVariant::Default) {
                    el = el.child(SpinnerOrIcon::colored(icon, icon_sz, fg, theme));
                } else {
                    el = el.child(app_icon(icon, icon_sz, icon_tone, theme));
                }
            }
        }

        el
    }
}

struct SpinnerOrIcon;

impl SpinnerOrIcon {
    fn colored(
        icon: AppIcon,
        size: IconSize,
        _fg: gpui_kit::Hsla,
        theme: &SemanticTheme,
    ) -> impl IntoElement {
        let tone = if theme.mode.is_dark() {
            IconTone::Default
        } else {
            IconTone::OnPrimaryButton
        };
        app_icon(icon, size, tone, theme)
    }
}

/// Convenience helper creating a themed `--preset b1oaDsAa` [`Button`].
pub fn button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    variant: ButtonVariant,
    theme: &SemanticTheme,
) -> Button {
    Button::new(id).label(label).variant(variant).theme(theme)
}

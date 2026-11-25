//! Loading spinner primitive ported from `web/src/components/ui/spinner.tsx`.
//!
//! Matches `--preset b1oaDsAa`: `size-4 animate-spin` (`16px` default) backed by
//! our FontAwesome `AppIcon::SyncPending` (`rotate.svg`) via `gpui_kit` spinner
//! animation and `app_icon` fallback.

use gpui_kit::component::spinner::Spinner as KitSpinner;
use gpui_kit::component::{Icon, Sizable as _, Size, StyledExt as _};
use gpui_kit::{
    App, Hsla, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window, div, px,
};

use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::theme::SemanticTheme;

/// Rotating status indicator matching `Spinner` in `web/src/components/ui/spinner.tsx`.
#[derive(IntoElement)]
pub struct Spinner {
    size: IconSize,
    tone: IconTone,
    color: Option<Hsla>,
    animated: bool,
    style: StyleRefinement,
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            size: IconSize::Md, // 16px (`size-4`)
            tone: IconTone::Default,
            color: None,
            animated: true,
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn tone(mut self, tone: IconTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.color = Some(self.tone.color(theme));
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl Styled for Spinner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut inner = KitSpinner::new()
            .icon(Icon::empty().path(AppIcon::SyncPending.asset_path()))
            .with_size(Size::Size(px(self.size.px())));

        if let Some(color) = self.color {
            inner = inner.color(color);
        }

        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .size(px(self.size.px()))
            .refine_style(&self.style)
            .child(inner)
    }
}

/// Convenience helper rendering a themed `16px` rotating spinner (`size-4 animate-spin`),
/// or a static `app_icon` when `animated` is `false`.
pub fn spinner(size: IconSize, tone: IconTone, theme: &SemanticTheme) -> Spinner {
    Spinner::new().size(size).tone(tone).theme(theme)
}

/// Static icon fallback using `crate::components::icon::app_icon`.
pub fn spinner_static(size: IconSize, tone: IconTone, theme: &SemanticTheme) -> impl IntoElement {
    app_icon(AppIcon::SyncPending, size, tone, theme)
}

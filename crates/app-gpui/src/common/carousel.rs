//! Slide carousel primitives ported from `web/src/components/ui/carousel.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Carousel`: `Horizontal` or `Vertical` slide navigation with keyboard arrow keys
//!   and optional `loop_slides`
//! - `CarouselContent`: `overflow-hidden` viewport
//! - `CarouselItem`: `min-w-0 shrink-0 grow-0 w-full`
//! - `CarouselPrevious` & `CarouselNext`: [`Button`] (`Outline`, `IconSm`) with
//!   `AppIcon::ChevronLeft` / `AppIcon::ChevronRight` (or `ChevronUp` / `ChevronDown` when vertical).

use std::rc::Rc;

pub use gpui_kit::component::carousel::{
    Carousel as KitCarousel, CarouselContent as KitCarouselContent,
    CarouselEvent as KitCarouselEvent, CarouselItem as KitCarouselItem,
    CarouselNext as KitCarouselNext, CarouselPagination as KitCarouselPagination,
    CarouselPrevious as KitCarouselPrevious, CarouselState as KitCarouselState,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, KeyDownEvent,
    ParentElement, RenderOnce, StyleRefinement, Styled, Window, div, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::utils::{PresetRadius, StyleComposeExt};
use crate::components::icon::AppIcon;
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

/// Slide axis matching `orientation = "horizontal" | "vertical"` in `carousel.tsx`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
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

/// Slide viewport (`CarouselContent` in `carousel.tsx`).
#[derive(IntoElement)]
pub struct CarouselContent {
    orientation: CarouselOrientation,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CarouselContent {
    pub fn new() -> Self {
        Self {
            orientation: CarouselOrientation::Horizontal,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl Default for CarouselContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CarouselContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CarouselContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CarouselContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let is_vert = matches!(self.orientation, CarouselOrientation::Vertical);
        div().overflow_hidden().w_full().child(
            div()
                .flex()
                .w_full()
                .cx_when(is_vert, |d| d.flex_col())
                .refine_style(&self.style)
                .children(self.children),
        )
    }
}

/// Individual slide wrapper (`CarouselItem` in `carousel.tsx`).
#[derive(IntoElement)]
pub struct CarouselItem {
    orientation: CarouselOrientation,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CarouselItem {
    pub fn new() -> Self {
        Self {
            orientation: CarouselOrientation::Horizontal,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl Default for CarouselItem {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CarouselItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CarouselItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CarouselItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .min_w_0()
            .flex_shrink_0()
            .w_full()
            .rounded(PresetRadius::NONE)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Previous-slide button (`CarouselPrevious` in `carousel.tsx`).
#[derive(IntoElement)]
pub struct CarouselPrevious {
    id: ElementId,
    orientation: CarouselOrientation,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl CarouselPrevious {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            orientation: CarouselOrientation::Horizontal,
            variant: ButtonVariant::Outline,
            size: ButtonSize::IconSm,
            disabled: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
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

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for CarouselPrevious {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CarouselPrevious {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let icon = match self.orientation {
            CarouselOrientation::Horizontal => AppIcon::ChevronLeft,
            CarouselOrientation::Vertical => AppIcon::ChevronUp,
        };

        let mut btn = Button::new(self.id)
            .variant(self.variant)
            .size(self.size)
            .icon(icon)
            .disabled(self.disabled)
            .theme(&theme);

        if let Some(cb) = self.on_click {
            btn = btn.on_click(move |ev, window, cx| cb(ev, window, cx));
        }

        btn.refine_style(&self.style)
    }
}

/// Next-slide button (`CarouselNext` in `carousel.tsx`).
#[derive(IntoElement)]
pub struct CarouselNext {
    id: ElementId,
    orientation: CarouselOrientation,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl CarouselNext {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            orientation: CarouselOrientation::Horizontal,
            variant: ButtonVariant::Outline,
            size: ButtonSize::IconSm,
            disabled: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
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

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for CarouselNext {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CarouselNext {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let icon = match self.orientation {
            CarouselOrientation::Horizontal => AppIcon::ChevronRight,
            CarouselOrientation::Vertical => AppIcon::ChevronDown,
        };

        let mut btn = Button::new(self.id)
            .variant(self.variant)
            .size(self.size)
            .icon(icon)
            .disabled(self.disabled)
            .theme(&theme);

        if let Some(cb) = self.on_click {
            btn = btn.on_click(move |ev, window, cx| cb(ev, window, cx));
        }

        btn.refine_style(&self.style)
    }
}

/// Interactive slide carousel root (`Carousel` in `carousel.tsx`).
#[derive(IntoElement)]
pub struct Carousel {
    id: ElementId,
    orientation: CarouselOrientation,
    slides: Vec<CarouselItem>,
    active_index: Option<usize>,
    default_index: usize,
    loop_slides: bool,
    show_controls: bool,
    theme: Option<SemanticTheme>,
    on_select: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Carousel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            orientation: CarouselOrientation::Horizontal,
            slides: Vec::new(),
            active_index: None,
            default_index: 0,
            loop_slides: false,
            show_controls: true,
            theme: None,
            on_select: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn slide(mut self, slide: CarouselItem) -> Self {
        self.slides.push(slide);
        self
    }

    pub fn slides(mut self, slides: impl IntoIterator<Item = CarouselItem>) -> Self {
        self.slides.extend(slides);
        self
    }

    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index = Some(index);
        self
    }

    pub fn default_index(mut self, index: usize) -> Self {
        self.default_index = index;
        self
    }

    pub fn loop_slides(mut self, loop_slides: bool) -> Self {
        self.loop_slides = loop_slides;
        self
    }

    pub fn show_controls(mut self, show: bool) -> Self {
        self.show_controls = show;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_select(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl Styled for Carousel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Carousel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Carousel {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let is_controlled = self.active_index.is_some();
        let default_idx = self.default_index;

        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-carousel-idx", self.id).into()),
            cx,
            |_w, _cx| default_idx,
        );

        let count = self.slides.len();
        let current = if count == 0 {
            0
        } else {
            self.active_index
                .unwrap_or_else(|| *state.read(cx))
                .min(count - 1)
        };

        let can_prev = count > 1 && (self.loop_slides || current > 0);
        let can_next = count > 1 && (self.loop_slides || current + 1 < count);

        let calc_prev = move || -> Option<usize> {
            if count <= 1 {
                None
            } else if current > 0 {
                Some(current - 1)
            } else if self.loop_slides {
                Some(count - 1)
            } else {
                None
            }
        };

        let calc_next = move || -> Option<usize> {
            if count <= 1 {
                None
            } else if current + 1 < count {
                Some(current + 1)
            } else if self.loop_slides {
                Some(0)
            } else {
                None
            }
        };

        let state_key = state.clone();
        let on_sel_key = self.on_select.clone();
        let is_vert = matches!(self.orientation, CarouselOrientation::Vertical);

        let mut root = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .cx_when(is_vert, |d| d.flex_col())
            .items_center()
            .gap(px(12.0))
            .font_family(FONT_FAMILY)
            .on_key_down(move |ev: &KeyDownEvent, window, cx| {
                let k = ev.keystroke.key.as_str();
                let target = if (!is_vert && k == "left") || (is_vert && k == "up") {
                    calc_prev()
                } else if (!is_vert && k == "right") || (is_vert && k == "down") {
                    calc_next()
                } else {
                    None
                };
                if let Some(next_idx) = target {
                    if !is_controlled {
                        state_key.update(cx, |v, cx| {
                            *v = next_idx;
                            cx.notify();
                        });
                    }
                    if let Some(cb) = &on_sel_key {
                        cb(&next_idx, window, cx);
                    }
                }
            })
            .refine_style(&self.style);

        if count > 0 {
            if self.show_controls {
                let state_prev = state.clone();
                let on_sel_prev = self.on_select.clone();
                root = root.child(
                    CarouselPrevious::new(ElementId::Name(
                        format!("{}-carousel-prev", self.id).into(),
                    ))
                    .orientation(self.orientation)
                    .disabled(!can_prev)
                    .theme(&theme)
                    .on_click(move |_ev, window, cx| {
                        if let Some(next_idx) = calc_prev() {
                            if !is_controlled {
                                state_prev.update(cx, |v, cx| {
                                    *v = next_idx;
                                    cx.notify();
                                });
                            }
                            if let Some(cb) = &on_sel_prev {
                                cb(&next_idx, window, cx);
                            }
                        }
                    }),
                );
            }

            let mut slides = self.slides;
            let active_slide = slides
                .drain(..)
                .nth(current)
                .map(|s| s.orientation(self.orientation));

            let mut viewport = CarouselContent::new()
                .orientation(self.orientation)
                .flex_1();
            if let Some(s) = active_slide {
                viewport = viewport.child(s);
            }
            root = root.child(viewport);

            if self.show_controls {
                let state_next = state;
                let on_sel_next = self.on_select;
                root = root.child(
                    CarouselNext::new(ElementId::Name(
                        format!("{}-carousel-next", self.id).into(),
                    ))
                    .orientation(self.orientation)
                    .disabled(!can_next)
                    .theme(&theme)
                    .on_click(move |_ev, window, cx| {
                        if let Some(next_idx) = calc_next() {
                            if !is_controlled {
                                state_next.update(cx, |v, cx| {
                                    *v = next_idx;
                                    cx.notify();
                                });
                            }
                            if let Some(cb) = &on_sel_next {
                                cb(&next_idx, window, cx);
                            }
                        }
                    }),
                );
            }
        }

        root.children(self.children)
    }
}

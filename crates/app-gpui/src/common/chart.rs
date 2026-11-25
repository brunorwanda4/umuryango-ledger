//! Chart container, series config, tooltip, legend, and `gpui-kit` chart primitives
//! ported from `web/src/components/ui/chart.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`) and Rule 2 + Rule 3:
//! - Re-exports `AreaChart`, `BarChart`, `LineChart`, `PieChart`, `RadarChart`,
//!   `CandlestickChart`, and `SankeyChart` from `gpui_kit::component::chart`
//! - Resolves `--chart-1..5` strictly from Umuryango Ledger's 10-color Red-Primary palette
//! - `ChartContainer`: `flex w-full min-h-[200px] justify-center text-xs`
//! - `ChartTooltipContent`: `min-w-32 gap-1.5 rounded-none bg-popover px-2.5 py-1.5 text-xs shadow-sm ring-1 ring-foreground/10`
//! - `ChartLegendContent`: `flex items-center justify-center gap-4` (`pt-3` bottom / `pb-3` top).

use std::collections::BTreeMap;

use gpui_kit::component::StyledExt as _;
pub use gpui_kit::component::chart::{
    AreaChart, BarChart, CandlestickChart, LineChart, PieChart, RadarChart, RadarLabel,
    SankeyChart, SankeyLabel,
};
use gpui_kit::{
    AnyElement, App, ElementId, FontWeight, Hsla, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, px,
};

use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt, overlay_surface_shadow};
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

/// Resolves `--chart-1` through `--chart-5` strictly from the 10-color Red-Primary palette.
pub fn preset_chart_color(series_index: usize, theme: &SemanticTheme) -> Hsla {
    let tokens = PresetTokens::from_theme(theme);
    match series_index % 5 {
        0 => tokens.primary,          // #ba181b
        1 => tokens.primary_hover,    // #e5383b
        2 => tokens.primary_pressed,  // #a4161a
        3 => tokens.primary_deep,     // #660708
        _ => tokens.muted_foreground, // #b1a7a6
    }
}

/// Metadata for a single chart data series inside [`ChartConfig`].
#[derive(Debug, Clone)]
pub struct ChartSeriesConfig {
    pub label: SharedString,
    pub icon: Option<AppIcon>,
    pub light_color: Option<Hsla>,
    pub dark_color: Option<Hsla>,
}

impl ChartSeriesConfig {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            light_color: None,
            dark_color: None,
        }
    }

    pub fn icon(mut self, icon: AppIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.light_color = Some(color);
        self.dark_color = Some(color);
        self
    }

    pub fn themed_color(mut self, light: Hsla, dark: Hsla) -> Self {
        self.light_color = Some(light);
        self.dark_color = Some(dark);
        self
    }

    pub fn resolve_color(&self, fallback_index: usize, theme: &SemanticTheme) -> Hsla {
        let is_dark = matches!(theme.mode, ThemeMode::Dark);
        let explicit = if is_dark {
            self.dark_color.or(self.light_color)
        } else {
            self.light_color.or(self.dark_color)
        };
        explicit.unwrap_or_else(|| preset_chart_color(fallback_index, theme))
    }
}

/// Series dictionary (`ChartConfig` in `chart.tsx`).
#[derive(Debug, Clone, Default)]
pub struct ChartConfig {
    series: BTreeMap<SharedString, ChartSeriesConfig>,
}

impl ChartConfig {
    pub fn new() -> Self {
        Self {
            series: BTreeMap::new(),
        }
    }

    pub fn insert(mut self, key: impl Into<SharedString>, series: ChartSeriesConfig) -> Self {
        self.series.insert(key.into(), series);
        self
    }

    pub fn get(&self, key: &str) -> Option<&ChartSeriesConfig> {
        self.series.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SharedString, &ChartSeriesConfig)> {
        self.series.iter()
    }
}

/// Semantic style carrier (`ChartStyle` in `chart.tsx`).
#[derive(Debug, Clone, Default)]
pub struct ChartStyle {
    pub config: ChartConfig,
}

impl ChartStyle {
    pub fn new(config: ChartConfig) -> Self {
        Self { config }
    }
}

/// Responsive chart frame (`ChartContainer` in `chart.tsx`).
#[derive(IntoElement)]
pub struct ChartContainer {
    id: ElementId,
    config: ChartConfig,
    min_height: f32,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl ChartContainer {
    pub fn new(id: impl Into<ElementId>, config: ChartConfig) -> Self {
        Self {
            id: id.into(),
            config,
            min_height: 200.0,
            theme: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn min_height(mut self, height_px: f32) -> Self {
        self.min_height = height_px;
        self
    }

    pub fn config(&self) -> &ChartConfig {
        &self.config
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ChartContainer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for ChartContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ChartContainer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        div()
            .id(self.id)
            .flex()
            .w_full()
            .min_h(px(self.min_height))
            .justify_center()
            .rounded(PresetRadius::NONE)
            .font_family(FONT_FAMILY)
            .text_xs()
            .text_color(tokens.muted_foreground)
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Swatch shape inside [`ChartTooltipContent`] (`indicator = "dot" | "line" | "dashed"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartTooltipIndicator {
    #[default]
    Dot,
    Line,
    Dashed,
}

/// Single series row inside [`ChartTooltipContent`].
#[derive(Debug, Clone)]
pub struct ChartTooltipDatum {
    pub key: SharedString,
    pub value_label: SharedString,
    pub color: Option<Hsla>,
}

impl ChartTooltipDatum {
    pub fn new(key: impl Into<SharedString>, value_label: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            value_label: value_label.into(),
            color: None,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

/// Floating tooltip card for chart data points (`ChartTooltipContent` in `chart.tsx`).
#[derive(IntoElement)]
pub struct ChartTooltipContent {
    label: Option<SharedString>,
    config: ChartConfig,
    payload: Vec<ChartTooltipDatum>,
    indicator: ChartTooltipIndicator,
    hide_label: bool,
    hide_indicator: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ChartTooltipContent {
    pub fn new(config: ChartConfig) -> Self {
        Self {
            label: None,
            config,
            payload: Vec::new(),
            indicator: ChartTooltipIndicator::Dot,
            hide_label: false,
            hide_indicator: false,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn datum(mut self, datum: ChartTooltipDatum) -> Self {
        self.payload.push(datum);
        self
    }

    pub fn payload(mut self, items: impl IntoIterator<Item = ChartTooltipDatum>) -> Self {
        self.payload.extend(items);
        self
    }

    pub fn indicator(mut self, indicator: ChartTooltipIndicator) -> Self {
        self.indicator = indicator;
        self
    }

    pub fn hide_label(mut self, hide: bool) -> Self {
        self.hide_label = hide;
        self
    }

    pub fn hide_indicator(mut self, hide: bool) -> Self {
        self.hide_indicator = hide;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ChartTooltipContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ChartTooltipContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut card = div()
            .flex()
            .flex_col()
            .min_w(px(128.0))
            .gap(px(6.0))
            .rounded(PresetRadius::NONE)
            .bg(tokens.popover)
            .px(px(10.0))
            .py(px(6.0))
            .font_family(FONT_FAMILY)
            .text_xs()
            .text_color(tokens.popover_foreground)
            .shadow(overlay_surface_shadow(&theme));

        if !self.hide_label
            && let Some(lbl) = self.label
        {
            card = card.child(div().font_weight(FontWeight::MEDIUM).child(lbl));
        }

        let mut rows = div().flex().flex_col().gap(px(6.0));
        for (idx, item) in self.payload.into_iter().enumerate() {
            let cfg_entry = self.config.get(item.key.as_ref());
            let series_label = cfg_entry
                .map(|c| c.label.clone())
                .unwrap_or_else(|| item.key.clone());
            let swatch_color = item.color.unwrap_or_else(|| {
                cfg_entry
                    .map(|c| c.resolve_color(idx, &theme))
                    .unwrap_or_else(|| preset_chart_color(idx, &theme))
            });

            let mut row = div().flex().w_full().items_center().gap(px(8.0));

            if let Some(icon) = cfg_entry.and_then(|c| c.icon) {
                row = row.child(app_icon(icon, IconSize::Xs, IconTone::Muted, &theme));
            } else if !self.hide_indicator {
                let swatch = match self.indicator {
                    ChartTooltipIndicator::Dot => div()
                        .flex_shrink_0()
                        .w(px(10.0))
                        .h(px(10.0))
                        .rounded(PresetRadius::NONE)
                        .bg(swatch_color),
                    ChartTooltipIndicator::Line => div()
                        .flex_shrink_0()
                        .w(px(4.0))
                        .h(px(12.0))
                        .rounded(PresetRadius::NONE)
                        .bg(swatch_color),
                    ChartTooltipIndicator::Dashed => div()
                        .flex_shrink_0()
                        .w(px(4.0))
                        .h(px(12.0))
                        .border_l_1()
                        .border_dashed()
                        .border_color(swatch_color),
                };
                row = row.child(swatch);
            }

            row = row.child(
                div()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_between()
                    .gap(px(12.0))
                    .child(
                        div()
                            .text_color(tokens.muted_foreground)
                            .child(series_label),
                    )
                    .child(
                        div()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(tokens.foreground)
                            .child(item.value_label),
                    ),
            );
            rows = rows.child(row);
        }

        card.child(rows).refine_style(&self.style)
    }
}

/// Alias matching `ChartTooltip` in `chart.tsx`.
pub type ChartTooltip = ChartTooltipContent;

/// Vertical placement of [`ChartLegendContent`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartLegendAlign {
    Top,
    #[default]
    Bottom,
}

/// Legend row rendering all series in [`ChartConfig`] (`ChartLegendContent` in `chart.tsx`).
#[derive(IntoElement)]
pub struct ChartLegendContent {
    config: ChartConfig,
    vertical_align: ChartLegendAlign,
    hide_icon: bool,
    theme: Option<SemanticTheme>,
    style: StyleRefinement,
}

impl ChartLegendContent {
    pub fn new(config: ChartConfig) -> Self {
        Self {
            config,
            vertical_align: ChartLegendAlign::Bottom,
            hide_icon: false,
            theme: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn vertical_align(mut self, align: ChartLegendAlign) -> Self {
        self.vertical_align = align;
        self
    }

    pub fn hide_icon(mut self, hide: bool) -> Self {
        self.hide_icon = hide;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl Styled for ChartLegendContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ChartLegendContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let mut bar = div()
            .flex()
            .flex_wrap()
            .items_center()
            .justify_center()
            .gap(px(16.0))
            .font_family(FONT_FAMILY)
            .text_xs()
            .text_color(tokens.foreground)
            .cx_when(matches!(self.vertical_align, ChartLegendAlign::Top), |d| {
                d.pb(px(12.0))
            })
            .cx_when(
                matches!(self.vertical_align, ChartLegendAlign::Bottom),
                |d| d.pt(px(12.0)),
            );

        for (idx, (_key, series)) in self.config.iter().enumerate() {
            let color = series.resolve_color(idx, &theme);
            let mut entry = div().flex().items_center().gap(px(6.0));
            if let Some(icon) = series.icon.filter(|_| !self.hide_icon) {
                entry = entry.child(app_icon(icon, IconSize::Xs, IconTone::Muted, &theme));
            } else {
                entry = entry.child(
                    div()
                        .flex_shrink_0()
                        .w(px(8.0))
                        .h(px(8.0))
                        .rounded(PresetRadius::NONE)
                        .bg(color),
                );
            }
            entry = entry.child(series.label.clone());
            bar = bar.child(entry);
        }

        bar.refine_style(&self.style)
    }
}

/// Alias matching `ChartLegend` in `chart.tsx`.
pub type ChartLegend = ChartLegendContent;

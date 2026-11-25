//! Month grid calendar and day button primitives ported from
//! `web/src/components/ui/calendar.tsx`.
//!
//! Matches `--preset b1oaDsAa` (`base-sera`):
//! - `Calendar`: `bg-background p-3`, `--cell-radius: 0`, `--cell-size: 32px` (`size-8`),
//!   month navigation (`AppIcon::ChevronLeft` / `AppIcon::ChevronRight`), 7-column weekday header,
//!   outside-month days (`show_outside_days = true`), single and range selection
//! - `CalendarDayButton`: `size-8 rounded-none`, `selected` / `range_start` / `range_end` in
//!   `bg-primary text-primary-foreground`, `range_middle` / `today` in `bg-muted text-foreground`.

use std::rc::Rc;

use chrono::{Datelike, Duration as ChronoDuration, NaiveDate, Utc};

pub use gpui_kit::component::calendar::{
    Calendar as KitCalendar, CalendarEvent as KitCalendarEvent, CalendarState as KitCalendarState,
    Date as KitCalendarDate, Matcher as KitCalendarMatcher,
};
pub use gpui_kit::component::date_picker::{
    DatePicker as KitDatePicker, DatePickerEvent as KitDatePickerEvent,
    DatePickerState as KitDatePickerState,
};
use gpui_kit::component::StyledExt as _;
use gpui_kit::{
    App, ClickEvent, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div, px,
};

use crate::common::button::{Button, ButtonSize, ButtonVariant};
use crate::common::utils::{PresetRadius, PresetTokens, StyleComposeExt};
use crate::components::icon::AppIcon;
use crate::theme::{FONT_FAMILY, SemanticTheme, ThemeConfig, ThemeMode};

const CELL_SIZE_PX: f32 = 32.0;

const DEFAULT_MONTHS_EN: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const DEFAULT_WEEKDAYS_EN: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// Selection value supported by [`Calendar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarSelection {
    None,
    Single(NaiveDate),
    Range {
        start: NaiveDate,
        end: Option<NaiveDate>,
    },
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

fn shift_month(year: i32, month: u32, delta: i32) -> (i32, u32) {
    let total = year * 12 + (month as i32 - 1) + delta;
    let next_year = total.div_euclid(12);
    let next_month = (total.rem_euclid(12) + 1) as u32;
    (next_year, next_month)
}

/// Interactive day cell button (`CalendarDayButton` in `calendar.tsx`).
#[derive(IntoElement)]
pub struct CalendarDayButton {
    id: ElementId,
    date: NaiveDate,
    selected_single: bool,
    range_start: bool,
    range_end: bool,
    range_middle: bool,
    is_today: bool,
    outside_month: bool,
    disabled: bool,
    theme: Option<SemanticTheme>,
    on_click: Option<Rc<dyn Fn(&NaiveDate, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl CalendarDayButton {
    pub fn new(id: impl Into<ElementId>, date: NaiveDate) -> Self {
        Self {
            id: id.into(),
            date,
            selected_single: false,
            range_start: false,
            range_end: false,
            range_middle: false,
            is_today: false,
            outside_month: false,
            disabled: false,
            theme: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn selected_single(mut self, selected: bool) -> Self {
        self.selected_single = selected;
        self
    }

    pub fn range_start(mut self, start: bool) -> Self {
        self.range_start = start;
        self
    }

    pub fn range_end(mut self, end: bool) -> Self {
        self.range_end = end;
        self
    }

    pub fn range_middle(mut self, middle: bool) -> Self {
        self.range_middle = middle;
        self
    }

    pub fn today(mut self, is_today: bool) -> Self {
        self.is_today = is_today;
        self
    }

    pub fn outside_month(mut self, outside: bool) -> Self {
        self.outside_month = outside;
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
        handler: impl Fn(&NaiveDate, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for CalendarDayButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for CalendarDayButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);

        let is_primary = self.selected_single || self.range_start || self.range_end;
        let is_muted_bg = !is_primary && (self.range_middle || self.is_today);
        let hover_bg = if is_primary {
            tokens.primary_hover
        } else {
            tokens.muted
        };

        let text_color = if is_primary {
            tokens.primary_foreground
        } else if self.outside_month {
            tokens.muted_foreground
        } else {
            tokens.foreground
        };

        let mut cell = div()
            .id(self.id)
            .flex()
            .w(px(CELL_SIZE_PX))
            .h(px(CELL_SIZE_PX))
            .items_center()
            .justify_center()
            .rounded(PresetRadius::NONE)
            .font_family(FONT_FAMILY)
            .text_xs()
            .font_weight(FontWeight::NORMAL)
            .text_color(text_color)
            .cx_when(is_primary, |d| d.bg(tokens.primary))
            .cx_when(is_muted_bg, |d| d.bg(tokens.muted))
            .cx_when(!self.disabled, |d| {
                d.cursor_pointer().hover(move |s| s.bg(hover_bg))
            })
            .preset_disabled(self.disabled)
            .child(self.date.day().to_string());

        if !self.disabled
            && let Some(cb) = self.on_click
        {
            let date = self.date;
            cell = cell.on_click(move |_ev: &ClickEvent, window, cx| cb(&date, window, cx));
        }

        cell.refine_style(&self.style)
    }
}

#[derive(Debug, Clone, Copy)]
struct CalendarRuntimeState {
    year: i32,
    month: u32,
    selection: CalendarSelection,
}

/// Interactive month calendar (`Calendar` in `calendar.tsx`).
#[derive(IntoElement)]
pub struct Calendar {
    id: ElementId,
    selection: Option<CalendarSelection>,
    default_selection: CalendarSelection,
    range_mode: bool,
    initial_month: Option<(i32, u32)>,
    show_outside_days: bool,
    button_variant: ButtonVariant,
    month_names: Option<[SharedString; 12]>,
    weekday_labels: Option<[SharedString; 7]>,
    min_date: Option<NaiveDate>,
    max_date: Option<NaiveDate>,
    transparent_surface: bool,
    theme: Option<SemanticTheme>,
    on_select: Option<Rc<dyn Fn(&CalendarSelection, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl Calendar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selection: None,
            default_selection: CalendarSelection::None,
            range_mode: false,
            initial_month: None,
            show_outside_days: true,
            button_variant: ButtonVariant::Ghost,
            month_names: None,
            weekday_labels: None,
            min_date: None,
            max_date: None,
            transparent_surface: false,
            theme: None,
            on_select: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn selected_date(mut self, date: Option<NaiveDate>) -> Self {
        self.selection = Some(match date {
            Some(d) => CalendarSelection::Single(d),
            None => CalendarSelection::None,
        });
        self
    }

    pub fn selection(mut self, selection: CalendarSelection) -> Self {
        if matches!(selection, CalendarSelection::Range { .. }) {
            self.range_mode = true;
        }
        self.selection = Some(selection);
        self
    }

    pub fn default_selection(mut self, selection: CalendarSelection) -> Self {
        if matches!(selection, CalendarSelection::Range { .. }) {
            self.range_mode = true;
        }
        self.default_selection = selection;
        self
    }

    pub fn range_mode(mut self, range_mode: bool) -> Self {
        self.range_mode = range_mode;
        self
    }

    pub fn initial_month(mut self, year: i32, month: u32) -> Self {
        self.initial_month = Some((year, month.clamp(1, 12)));
        self
    }

    pub fn show_outside_days(mut self, show: bool) -> Self {
        self.show_outside_days = show;
        self
    }

    pub fn button_variant(mut self, variant: ButtonVariant) -> Self {
        self.button_variant = variant;
        self
    }

    pub fn month_names(mut self, names: [SharedString; 12]) -> Self {
        self.month_names = Some(names);
        self
    }

    pub fn weekday_labels(mut self, labels: [SharedString; 7]) -> Self {
        self.weekday_labels = Some(labels);
        self
    }

    pub fn min_date(mut self, min_date: NaiveDate) -> Self {
        self.min_date = Some(min_date);
        self
    }

    pub fn max_date(mut self, max_date: NaiveDate) -> Self {
        self.max_date = Some(max_date);
        self
    }

    pub fn transparent_surface(mut self, transparent: bool) -> Self {
        self.transparent_surface = transparent;
        self
    }

    pub fn theme(mut self, theme: &SemanticTheme) -> Self {
        self.theme = Some(theme.clone());
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&CalendarSelection, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl Styled for Calendar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Calendar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = resolve_theme(self.theme.as_ref(), cx);
        let tokens = PresetTokens::from_theme(&theme);
        let today = Utc::now().date_naive();

        let (init_y, init_m) = self.initial_month.unwrap_or_else(|| match self.selection.or(Some(self.default_selection)) {
            Some(CalendarSelection::Single(d)) => (d.year(), d.month()),
            Some(CalendarSelection::Range { start, .. }) => (start.year(), start.month()),
            _ => (today.year(), today.month()),
        });

        let is_controlled = self.selection.is_some();
        let default_sel = self.default_selection;
        let state = window.use_keyed_state(
            ElementId::Name(format!("{}-cal-state", self.id).into()),
            cx,
            |_w, _cx| CalendarRuntimeState {
                year: init_y,
                month: init_m,
                selection: default_sel,
            },
        );

        let snap = *state.read(cx);
        let active_sel = self.selection.unwrap_or(snap.selection);
        let year = snap.year;
        let month = snap.month;

        let month_label = match &self.month_names {
            Some(custom) => format!("{} {year}", custom[(month as usize - 1) % 12]),
            None => format!("{} {year}", DEFAULT_MONTHS_EN[(month as usize - 1) % 12]),
        };

        let state_prev = state.clone();
        let state_next = state.clone();

        let header = div()
            .flex()
            .h(px(CELL_SIZE_PX))
            .w_full()
            .items_center()
            .justify_between()
            .gap(px(4.0))
            .child(
                Button::new(ElementId::Name(format!("{}-cal-prev", self.id).into()))
                    .variant(self.button_variant)
                    .size(ButtonSize::IconSm)
                    .icon(AppIcon::ChevronLeft)
                    .theme(&theme)
                    .w(px(CELL_SIZE_PX))
                    .h(px(CELL_SIZE_PX))
                    .on_click(move |_ev, _window, cx| {
                        state_prev.update(cx, |s, cx| {
                            let (ny, nm) = shift_month(s.year, s.month, -1);
                            s.year = ny;
                            s.month = nm;
                            cx.notify();
                        });
                    }),
            )
            .child(
                div()
                    .font_family(FONT_FAMILY)
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(tokens.foreground)
                    .child(month_label),
            )
            .child(
                Button::new(ElementId::Name(format!("{}-cal-next", self.id).into()))
                    .variant(self.button_variant)
                    .size(ButtonSize::IconSm)
                    .icon(AppIcon::ChevronRight)
                    .theme(&theme)
                    .w(px(CELL_SIZE_PX))
                    .h(px(CELL_SIZE_PX))
                    .on_click(move |_ev, _window, cx| {
                        state_next.update(cx, |s, cx| {
                            let (ny, nm) = shift_month(s.year, s.month, 1);
                            s.year = ny;
                            s.month = nm;
                            cx.notify();
                        });
                    }),
            );

        let mut weekday_row = div().flex().w_full();
        for idx in 0..7 {
            let lbl: SharedString = match &self.weekday_labels {
                Some(custom) => custom[idx].clone(),
                None => DEFAULT_WEEKDAYS_EN[idx].into(),
            };
            weekday_row = weekday_row.child(
                div()
                    .flex()
                    .w(px(CELL_SIZE_PX))
                    .h(px(24.0))
                    .items_center()
                    .justify_center()
                    .font_family(FONT_FAMILY)
                    .text_xs()
                    .text_color(tokens.muted_foreground)
                    .child(lbl),
            );
        }

        let first_of_month =
            NaiveDate::from_ymd_opt(year, month, 1).unwrap_or_else(|| today);
        let leading_days = first_of_month.weekday().num_days_from_sunday() as i64;
        let grid_start = first_of_month - ChronoDuration::days(leading_days);

        let mut grid = div().flex().flex_col().gap(px(4.0));
        for week_idx in 0..6i64 {
            let mut week_row = div().flex().w_full();
            for day_idx in 0..7i64 {
                let d = grid_start + ChronoDuration::days(week_idx * 7 + day_idx);
                let is_outside = d.month() != month || d.year() != year;

                if is_outside && !self.show_outside_days {
                    week_row = week_row.child(div().w(px(CELL_SIZE_PX)).h(px(CELL_SIZE_PX)));
                    continue;
                }

                let is_disabled = self.min_date.is_some_and(|min| d < min)
                    || self.max_date.is_some_and(|max| d > max);

                let (sel_single, r_start, r_end, r_mid) = match active_sel {
                    CalendarSelection::None => (false, false, false, false),
                    CalendarSelection::Single(sel) => (d == sel, false, false, false),
                    CalendarSelection::Range { start, end } => match end {
                        None => (false, d == start, false, false),
                        Some(end_d) => {
                            let lo = start.min(end_d);
                            let hi = start.max(end_d);
                            (false, d == lo, d == hi, d > lo && d < hi)
                        }
                    },
                };

                let state_day = state.clone();
                let on_sel_day = self.on_select.clone();
                let range_mode = self.range_mode;

                let btn = CalendarDayButton::new(
                    ElementId::Name(format!("{}-day-{}", self.id, d).into()),
                    d,
                )
                .selected_single(sel_single)
                .range_start(r_start)
                .range_end(r_end)
                .range_middle(r_mid)
                .today(d == today)
                .outside_month(is_outside)
                .disabled(is_disabled)
                .theme(&theme)
                .on_click(move |clicked, window, cx| {
                    let next_sel = if range_mode {
                        match active_sel {
                            CalendarSelection::Range { start, end: None } => {
                                if *clicked < start {
                                    CalendarSelection::Range {
                                        start: *clicked,
                                        end: Some(start),
                                    }
                                } else {
                                    CalendarSelection::Range {
                                        start,
                                        end: Some(*clicked),
                                    }
                                }
                            }
                            _ => CalendarSelection::Range {
                                start: *clicked,
                                end: None,
                            },
                        }
                    } else {
                        CalendarSelection::Single(*clicked)
                    };

                    state_day.update(cx, |s, cx| {
                        s.year = clicked.year();
                        s.month = clicked.month();
                        if !is_controlled {
                            s.selection = next_sel;
                        }
                        cx.notify();
                    });
                    if let Some(cb) = &on_sel_day {
                        cb(&next_sel, window, cx);
                    }
                });

                week_row = week_row.child(btn);
            }
            grid = grid.child(week_row);
        }

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(12.0))
            .rounded(PresetRadius::NONE)
            .cx_when(!self.transparent_surface, |d| d.bg(tokens.background))
            .p(px(12.0))
            .font_family(FONT_FAMILY)
            .child(header)
            .child(div().flex().flex_col().gap(px(4.0)).child(weekday_row).child(grid))
            .refine_style(&self.style)
    }
}

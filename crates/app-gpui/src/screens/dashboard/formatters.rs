//! Responsive layout tiers, time-period filters, and exact RWF integer formatters
//! for the Umuryango Ledger Dashboard.

use chrono::{DateTime, Datelike, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use umuryango_core_domain::models::TransactionType;
use umuryango_i18n::Locale;

use crate::state::AppState;

/// Width of the collapsible desktop sidebar in [`crate::components::sidebar::Sidebar`].
pub const DESKTOP_SIDEBAR_WIDTH_PX: f32 = 240.0;

/// Responsive layout tier computed from available dashboard content width
/// (viewport width minus active sidebar and horizontal safe-area insets).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardLayoutTier {
    /// `>= 1060px`: 5-col KPIs, 2:1 cash-flow split, 3-col bento row, 1:2 activity row.
    Wide,
    /// `680px .. 1060px`: 3-col KPIs, stacked cash-flow + 2-col subcards, 2-col bento + hero, stacked activity.
    Medium,
    /// `420px .. 680px`: 2-col KPIs, single-column cards, 2-col mini-metrics.
    Compact,
    /// `< 420px`: 1-col KPIs and single-column stacked cards for narrow phones.
    NarrowMobile,
}

impl DashboardLayoutTier {
    #[inline]
    pub fn from_content_width(width_px: f32) -> Self {
        if width_px >= 1060.0 {
            Self::Wide
        } else if width_px >= 680.0 {
            Self::Medium
        } else if width_px >= 420.0 {
            Self::Compact
        } else {
            Self::NarrowMobile
        }
    }

    #[inline]
    pub const fn kpi_columns(self) -> u16 {
        match self {
            Self::Wide => 5,
            Self::Medium => 3,
            Self::Compact => 2,
            Self::NarrowMobile => 1,
        }
    }

    #[inline]
    pub const fn outer_padding_px(self) -> f32 {
        match self {
            Self::Wide => 24.0,
            Self::Medium => 20.0,
            Self::Compact | Self::NarrowMobile => 14.0,
        }
    }

    #[inline]
    pub const fn grid_gap_px(self) -> f32 {
        match self {
            Self::Wide | Self::Medium => 16.0,
            Self::Compact | Self::NarrowMobile => 12.0,
        }
    }
}

/// Time window filter shared by the Cash Flow Updates and Recent Custody Activity cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardPeriod {
    CurrentMonth,
    PreviousMonth,
    ThisYear,
    AllTime,
}

impl DashboardPeriod {
    pub const ALL: [Self; 4] = [
        Self::CurrentMonth,
        Self::PreviousMonth,
        Self::ThisYear,
        Self::AllTime,
    ];

    /// Resolves the active period, defaulting to `CurrentMonth` when current-month
    /// transactions exist (or the ledger is brand new), and `AllTime` when historical
    /// transactions exist only in earlier months.
    pub fn resolve(state: &AppState, now: DateTime<Utc>) -> Self {
        if let Some(explicit) = state.dashboard_period {
            return explicit;
        }
        if state.transactions.is_empty() {
            return Self::CurrentMonth;
        }
        let has_current = state
            .transactions
            .iter()
            .any(|tx| tx.tx_date.year() == now.year() && tx.tx_date.month() == now.month());
        if has_current {
            Self::CurrentMonth
        } else {
            Self::AllTime
        }
    }

    pub fn matches(self, tx_date: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        let (prev_year, prev_month) = previous_year_month(now.year(), now.month());
        match self {
            Self::CurrentMonth => tx_date.year() == now.year() && tx_date.month() == now.month(),
            Self::PreviousMonth => tx_date.year() == prev_year && tx_date.month() == prev_month,
            Self::ThisYear => tx_date.year() == now.year(),
            Self::AllTime => true,
        }
    }

    pub fn label(self, locale: Locale, now: DateTime<Utc>) -> String {
        match self {
            Self::CurrentMonth => {
                format!("{} {}", month_full_name(locale, now.month()), now.year())
            }
            Self::PreviousMonth => {
                let (py, pm) = previous_year_month(now.year(), now.month());
                format!("{} {}", month_full_name(locale, pm), py)
            }
            Self::ThisYear => umuryango_i18n::t(locale, "dashboard.period_this_year")
                .replace("{year}", &now.year().to_string()),
            Self::AllTime => umuryango_i18n::t(locale, "dashboard.period_all_time"),
        }
    }
}

pub fn previous_year_month(year: i32, month: u32) -> (i32, u32) {
    if month <= 1 {
        (year - 1, 12)
    } else {
        (year, month - 1)
    }
}

pub fn month_full_name(locale: Locale, month: u32) -> String {
    let clamped = month.clamp(1, 12);
    umuryango_i18n::t(locale, &format!("dashboard.month_{}", clamped))
}

pub fn month_short_name(locale: Locale, month: u32) -> String {
    let clamped = month.clamp(1, 12);
    umuryango_i18n::t(locale, &format!("dashboard.month_short_{}", clamped))
}

/// Formats an integer/decimal RWF value with thousands separators and no fractional noise
/// (e.g. `1450000.00` -> `"1,450,000 RWF"`).
pub fn format_rwf(amount: Decimal) -> String {
    let rounded = amount.round().to_i128().unwrap_or(0);
    format!("{} RWF", format_grouped_i128(rounded))
}

/// Formats a transaction amount with explicit `+` / `-` sign for inflows and expenses.
pub fn format_signed_rwf(tx_type: TransactionType, amount: Decimal) -> String {
    let rounded = amount.abs().round().to_i128().unwrap_or(0);
    let grouped = format_grouped_i128(rounded);
    match tx_type {
        TransactionType::Income => format!("+{} RWF", grouped),
        TransactionType::Expense => format!("-{} RWF", grouped),
        TransactionType::Handover => format!("{} RWF", grouped),
    }
}

/// Formats y-axis values compactly (`+200k`, `+100k`, `0`, `-100k`, `-200k`).
pub fn format_compact_axis(val: f64) -> String {
    let rounded = val.round() as i64;
    if rounded == 0 {
        return "0".to_string();
    }
    let sign = if rounded > 0 { "+" } else { "-" };
    let abs = rounded.unsigned_abs();
    if abs >= 1_000_000 {
        let millions = abs as f64 / 1_000_000.0;
        if (millions - millions.round()).abs() < 0.05 {
            format!("{}{}M", sign, millions.round() as u64)
        } else {
            format!("{}{:.1}M", sign, millions)
        }
    } else if abs >= 1_000 {
        format!("{}{}k", sign, abs / 1_000)
    } else {
        format!("{}{}", sign, abs)
    }
}

pub fn format_grouped_i128(value: i128) -> String {
    let is_neg = value < 0;
    let digits = value.unsigned_abs().to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3 + 1);
    if is_neg {
        out.push('-');
    }
    for (idx, ch) in digits.chars().enumerate() {
        if idx > 0 && (digits.len() - idx).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

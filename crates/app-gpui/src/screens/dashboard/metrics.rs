//! Pure financial aggregations and analytical snapshot builders for the Umuryango Ledger Dashboard.

use chrono::{DateTime, Datelike, Duration, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use umuryango_core_domain::models::{
    FamilyRole, LoanStatus, ProjectStatus, Transaction, TransactionType, WalletChannel,
};
use umuryango_i18n::{Locale, Translator, terms};

use crate::common::initials_from_name;
use crate::state::AppState;

pub use super::formatters::{
    DESKTOP_SIDEBAR_WIDTH_PX, DashboardLayoutTier, DashboardPeriod, format_compact_axis,
    format_grouped_i128, format_rwf, format_signed_rwf, month_full_name, month_short_name,
    previous_year_month,
};

/// Single datum for the diverging Cash Flow Updates `BarChart`.
#[derive(Debug, Clone)]
pub struct DivergingBarDatum {
    pub band: String,
    pub value: f64,
    pub is_inflow: bool,
    pub is_anchor: bool,
}

/// Single datum for monthly or mini bar charts.
#[derive(Debug, Clone)]
pub struct SimpleBarDatum {
    pub band: String,
    pub value: f64,
    pub highlighted: bool,
}

/// Single point for sparkline and weekly `AreaChart`s.
#[derive(Debug, Clone)]
pub struct SparkPoint {
    pub label: String,
    pub value: f64,
}

/// Active project or personal savings plan displayed in the crimson hero card.
#[derive(Debug, Clone)]
pub struct HeroGoalItem {
    pub title: String,
    pub amount_label: String,
    pub pct: i32,
    pub pct_f32: f32,
}

/// Monogram avatar chip displayed in the Mother's Report Ready card.
#[derive(Debug, Clone)]
pub struct MemberChip {
    pub initials: String,
    pub primary_accent: bool,
}

/// Highlight item in the Weekly Activity card.
#[derive(Debug, Clone)]
pub struct WeeklyHighlight {
    pub subtitle: String,
    pub badge_text: String,
}

/// Pre-resolved row for the Recent Custody Activity `Table`.
#[derive(Debug, Clone)]
pub struct RecentActivityRow {
    pub initials: String,
    pub actor_name: String,
    pub actor_role_label: String,
    pub category_note: String,
    pub tx_type: TransactionType,
    pub type_badge_label: String,
    pub amount_label: String,
}

/// Complete immutable analytical snapshot driving all 10 dashboard cards.
#[derive(Debug, Clone)]
pub struct DashboardSnapshot {
    pub is_family: bool,
    pub active_period: DashboardPeriod,
    pub active_period_label: String,
    pub current_year: i32,
    pub previous_year: i32,
    pub report_date_label: String,

    // Row 1 KPIs
    pub total_custody: Decimal,
    pub period_inflow: Decimal,
    pub period_expense: Decimal,
    pub active_projects_or_plans_count: usize,
    pub active_loans_count: usize,
    pub members_or_wallets_count: usize,

    // Row 2: Cash Flow + Yearly Breakup + Monthly Net Savings
    pub cashflow_bars: Vec<DivergingBarDatum>,
    pub cashflow_band_labels: Vec<String>,
    pub yearly_current_total: Decimal,
    pub yearly_prev_total: Decimal,
    pub yearly_yoy_pct: i32,
    pub yearly_share_pct: i32,
    pub net_savings: Decimal,
    pub net_savings_mom_pct: i32,
    pub savings_sparkline: Vec<SparkPoint>,
    pub savings_spark_max: f64,

    // Row 3: Monthly Custody Flow + Wallets/Handovers + Mother's Report + Hero Projects
    pub monthly_flow_bars: Vec<SimpleBarDatum>,
    pub pool_custody_amount: Decimal,
    pub cash_custody_amount: Decimal,
    pub total_wallets_count: usize,
    pub active_wallets_count: usize,
    pub wallet_sparkline: Vec<SparkPoint>,
    pub wallet_spark_max: f64,
    pub handovers_count: usize,
    pub handovers_delta_pct: i32,
    pub handover_bars: Vec<SimpleBarDatum>,
    pub member_chips: Vec<MemberChip>,
    pub hero_goals: Vec<HeroGoalItem>,

    // Row 4: Weekly Activity + Recent Custody Activity
    pub weekly_points: Vec<SparkPoint>,
    pub weekly_max: f64,
    pub top_inflow: WeeklyHighlight,
    pub top_expense: WeeklyHighlight,
    pub latest_handover: WeeklyHighlight,
    pub recent_rows: Vec<RecentActivityRow>,
}

impl DashboardSnapshot {
    pub fn compute(state: &AppState, now: DateTime<Utc>) -> Self {
        let locale = state.locale;
        let is_family = state.active_scope.is_family();
        let active_period = DashboardPeriod::resolve(state, now);
        let active_period_label = active_period.label(locale, now);
        let current_year = now.year();
        let previous_year = current_year - 1;
        let report_date_label = format!(
            "{} {} {}",
            now.day(),
            month_full_name(locale, now.month()),
            current_year
        );

        let total_custody = state.total_balance();

        let period_txs: Vec<&Transaction> = state
            .transactions
            .iter()
            .filter(|tx| active_period.matches(tx.tx_date, now))
            .collect();

        let period_inflow: Decimal = period_txs
            .iter()
            .filter(|tx| tx.tx_type == TransactionType::Income)
            .map(|tx| tx.amount)
            .sum();
        let period_expense: Decimal = period_txs
            .iter()
            .filter(|tx| tx.tx_type == TransactionType::Expense)
            .map(|tx| tx.amount)
            .sum();

        let active_projects_or_plans_count = if is_family {
            state
                .projects
                .iter()
                .filter(|p| p.status.is_active())
                .count()
        } else {
            state.future_plans.iter().filter(|p| !p.is_achieved).count()
        };

        let active_loans_count = state
            .loans
            .iter()
            .filter(|l| l.status == LoanStatus::Active)
            .count();

        let active_wallets_count = state.wallets.iter().filter(|w| w.is_active).count();
        let members_or_wallets_count = if is_family {
            let active_members = state.family_members.iter().filter(|m| m.is_active).count();
            active_members.max(usize::from(state.is_authenticated))
        } else {
            active_wallets_count
        };

        // Diverging Cash Flow bars (6 recent anchor days ending at latest tx in period or `now`)
        let anchor_date = period_txs
            .iter()
            .map(|tx| tx.tx_date.date_naive())
            .max()
            .unwrap_or_else(|| now.date_naive());
        let mut cashflow_band_labels = Vec::with_capacity(6);
        let mut raw_pairs: Vec<(String, f64, f64)> = Vec::with_capacity(6);
        let mut max_abs_flow: f64 = 0.0;

        for offset in (0..6).rev() {
            let day = anchor_date - Duration::days(offset);
            let label = format!("{:02}/{:02}", day.day(), day.month());
            let mut day_in = Decimal::ZERO;
            let mut day_out = Decimal::ZERO;
            for tx in &period_txs {
                if tx.tx_date.date_naive() == day {
                    match tx.tx_type {
                        TransactionType::Income => day_in += tx.amount,
                        TransactionType::Expense => day_out += tx.amount,
                        TransactionType::Handover => {}
                    }
                }
            }
            let in_f = day_in.to_f64().unwrap_or(0.0).max(0.0);
            let out_f = day_out.to_f64().unwrap_or(0.0).max(0.0);
            max_abs_flow = max_abs_flow.max(in_f).max(out_f);
            cashflow_band_labels.push(label.clone());
            raw_pairs.push((label, in_f, out_f));
        }

        let nice_bound = nice_symmetric_bound(max_abs_flow);
        let mut cashflow_bars = Vec::with_capacity(14);
        if let Some((first_band, _, _)) = raw_pairs.first() {
            cashflow_bars.push(DivergingBarDatum {
                band: first_band.clone(),
                value: nice_bound,
                is_inflow: true,
                is_anchor: true,
            });
            cashflow_bars.push(DivergingBarDatum {
                band: first_band.clone(),
                value: -nice_bound,
                is_inflow: false,
                is_anchor: true,
            });
        }
        for (band, in_f, out_f) in raw_pairs {
            cashflow_bars.push(DivergingBarDatum {
                band: band.clone(),
                value: in_f,
                is_inflow: true,
                is_anchor: false,
            });
            cashflow_bars.push(DivergingBarDatum {
                band,
                value: -out_f,
                is_inflow: false,
                is_anchor: false,
            });
        }

        // Yearly breakup (current year vs previous year inflow + activity volume)
        let yearly_current_total: Decimal = state
            .transactions
            .iter()
            .filter(|tx| {
                tx.tx_date.year() == current_year && tx.tx_type != TransactionType::Handover
            })
            .map(|tx| tx.amount)
            .sum();
        let yearly_prev_total: Decimal = state
            .transactions
            .iter()
            .filter(|tx| {
                tx.tx_date.year() == previous_year && tx.tx_type != TransactionType::Handover
            })
            .map(|tx| tx.amount)
            .sum();
        let combined_years = yearly_current_total + yearly_prev_total;
        let yearly_share_pct = if combined_years > Decimal::ZERO {
            ((yearly_current_total * Decimal::from(100)) / combined_years)
                .round()
                .to_i32()
                .unwrap_or(0)
                .clamp(0, 100)
        } else if total_custody > Decimal::ZERO {
            100
        } else {
            0
        };
        let yearly_yoy_pct = compute_delta_pct(yearly_current_total, yearly_prev_total);

        // Monthly Net Savings + MoM delta
        let net_savings = period_inflow - period_expense;
        let (prev_y, prev_m) = previous_year_month(now.year(), now.month());
        let prev_net: Decimal = state
            .transactions
            .iter()
            .filter(|tx| tx.tx_date.year() == prev_y && tx.tx_date.month() == prev_m)
            .fold(Decimal::ZERO, |acc, tx| match tx.tx_type {
                TransactionType::Income => acc + tx.amount,
                TransactionType::Expense => acc - tx.amount,
                TransactionType::Handover => acc,
            });
        let net_savings_mom_pct = compute_delta_pct(net_savings, prev_net);

        // 6-month custody flow & savings sparkline
        let mut monthly_flow_bars = Vec::with_capacity(6);
        let mut savings_sparkline = Vec::with_capacity(6);
        let mut ym = (now.year(), now.month());
        let mut six_months = Vec::with_capacity(6);
        for _ in 0..6 {
            six_months.push(ym);
            ym = previous_year_month(ym.0, ym.1);
        }
        six_months.reverse();

        for (idx, (y, m)) in six_months.iter().copied().enumerate() {
            let mut m_in = Decimal::ZERO;
            let mut m_out = Decimal::ZERO;
            for tx in &state.transactions {
                if tx.tx_date.year() == y && tx.tx_date.month() == m {
                    match tx.tx_type {
                        TransactionType::Income => m_in += tx.amount,
                        TransactionType::Expense => m_out += tx.amount,
                        TransactionType::Handover => {}
                    }
                }
            }
            let vol = (m_in + m_out).to_f64().unwrap_or(0.0).max(0.0);
            let net = (m_in - m_out).to_f64().unwrap_or(0.0);
            let short_lbl = month_short_name(locale, m);
            monthly_flow_bars.push(SimpleBarDatum {
                band: short_lbl.clone(),
                value: vol,
                highlighted: idx + 1 == six_months.len(),
            });
            savings_sparkline.push(SparkPoint {
                label: short_lbl,
                value: net.max(0.0),
            });
        }
        let savings_spark_max = ensure_sparkline_variance(&mut savings_sparkline);

        // Custody pool split: Bank + MobileMoney vs CashInHand (exact sum == total_custody)
        let mut pool_custody_amount = Decimal::ZERO;
        let mut cash_custody_amount = Decimal::ZERO;
        for w in &state.wallets {
            match w.channel {
                WalletChannel::Bank | WalletChannel::MobileMoney => {
                    pool_custody_amount += w.balance;
                }
                WalletChannel::CashInHand => {
                    cash_custody_amount += w.balance;
                }
            }
        }

        // Wallet sparkline (wallet balances across active wallets, padded to >= 5 points)
        let mut wallet_sparkline: Vec<SparkPoint> = state
            .wallets
            .iter()
            .enumerate()
            .map(|(i, w)| SparkPoint {
                label: format!("W{}", i + 1),
                value: w.balance.to_f64().unwrap_or(0.0).max(0.0),
            })
            .collect();
        while wallet_sparkline.len() < 5 {
            let n = wallet_sparkline.len() + 1;
            wallet_sparkline.push(SparkPoint {
                label: format!("W{}", n),
                value: 0.0,
            });
        }
        let wallet_spark_max = ensure_sparkline_variance(&mut wallet_sparkline);

        // Handover count, MoM delta, and 7-day handover mini-bars
        let handovers_count = state
            .transactions
            .iter()
            .filter(|tx| tx.tx_type == TransactionType::Handover)
            .count();
        let curr_m_handovers = Decimal::from(
            state
                .transactions
                .iter()
                .filter(|tx| {
                    tx.tx_type == TransactionType::Handover
                        && tx.tx_date.year() == now.year()
                        && tx.tx_date.month() == now.month()
                })
                .count() as i64,
        );
        let prev_m_handovers = Decimal::from(
            state
                .transactions
                .iter()
                .filter(|tx| {
                    tx.tx_type == TransactionType::Handover
                        && tx.tx_date.year() == prev_y
                        && tx.tx_date.month() == prev_m
                })
                .count() as i64,
        );
        let handovers_delta_pct = compute_delta_pct(curr_m_handovers, prev_m_handovers);

        let mut handover_bars = Vec::with_capacity(7);
        for idx in 0..7 {
            let day = anchor_date - Duration::days(6 - idx as i64);
            let amt: Decimal = state
                .transactions
                .iter()
                .filter(|tx| {
                    tx.tx_type == TransactionType::Handover && tx.tx_date.date_naive() == day
                })
                .map(|tx| tx.amount)
                .sum();
            handover_bars.push(SimpleBarDatum {
                band: format!("H{}", idx + 1),
                value: amt.to_f64().unwrap_or(0.0).max(0.0),
                highlighted: idx % 2 == 1 || idx == 6,
            });
        }

        // Member monogram chips for Mother's Report card
        let mut member_chips: Vec<MemberChip> = if is_family && !state.family_members.is_empty() {
            state
                .family_members
                .iter()
                .filter(|m| m.is_active)
                .take(5)
                .enumerate()
                .map(|(idx, m)| MemberChip {
                    initials: initials_from_name(&m.full_name),
                    primary_accent: idx != 2,
                })
                .collect()
        } else if !state.user_name.trim().is_empty() {
            vec![MemberChip {
                initials: initials_from_name(&state.user_name),
                primary_accent: true,
            }]
        } else {
            Vec::new()
        };
        if member_chips.is_empty() {
            for (idx, w) in state.wallets.iter().take(4).enumerate() {
                member_chips.push(MemberChip {
                    initials: initials_from_name(&w.name),
                    primary_accent: idx % 2 == 0,
                });
            }
        }

        // Hero projects (Family scope) or Future Plans (Personal scope)
        let hero_goals: Vec<HeroGoalItem> = if is_family {
            state
                .projects
                .iter()
                .filter(|p| p.status != ProjectStatus::Completed)
                .take(2)
                .map(|p| {
                    let pct_dec = if p.target_amount > Decimal::ZERO {
                        ((p.allocated_amount * Decimal::from(100)) / p.target_amount)
                            .round()
                            .clamp(Decimal::ZERO, Decimal::from(100))
                    } else {
                        Decimal::ZERO
                    };
                    let pct = pct_dec.to_i32().unwrap_or(0);
                    HeroGoalItem {
                        title: p.name.clone(),
                        amount_label: format_rwf(p.allocated_amount),
                        pct,
                        pct_f32: pct as f32,
                    }
                })
                .collect()
        } else {
            let pool_ref = total_custody.max(Decimal::ZERO);
            state
                .future_plans
                .iter()
                .take(2)
                .map(|plan| {
                    let pct = if plan.is_achieved {
                        100
                    } else if plan.estimated_cost > Decimal::ZERO {
                        ((pool_ref * Decimal::from(100)) / plan.estimated_cost)
                            .round()
                            .to_i32()
                            .unwrap_or(0)
                            .clamp(0, 100)
                    } else {
                        0
                    };
                    HeroGoalItem {
                        title: plan.title.clone(),
                        amount_label: format_rwf(plan.estimated_cost),
                        pct,
                        pct_f32: pct as f32,
                    }
                })
                .collect()
        };

        // Weekly Activity 7-day curve + Top Inflow / Top Expense / Latest Handover
        let mut weekly_points = Vec::with_capacity(7);
        for idx in 0..7 {
            let day = anchor_date - Duration::days(6 - idx as i64);
            let spent: Decimal = state
                .transactions
                .iter()
                .filter(|tx| {
                    tx.tx_date.date_naive() == day && tx.tx_type != TransactionType::Handover
                })
                .map(|tx| tx.amount)
                .sum();
            weekly_points.push(SparkPoint {
                label: format!("{:02}/{:02}", day.day(), day.month()),
                value: spent.to_f64().unwrap_or(0.0).max(0.0),
            });
        }
        let weekly_max = ensure_sparkline_variance(&mut weekly_points);

        let empty_sub = umuryango_i18n::t(locale, "dashboard.no_entry_yet");
        let top_inflow = period_txs
            .iter()
            .filter(|tx| tx.tx_type == TransactionType::Income)
            .max_by(|a, b| a.amount.cmp(&b.amount))
            .map(|tx| WeeklyHighlight {
                subtitle: tx_short_subtitle(state, tx),
                badge_text: format_signed_rwf(TransactionType::Income, tx.amount),
            })
            .unwrap_or_else(|| WeeklyHighlight {
                subtitle: empty_sub.clone(),
                badge_text: "0 RWF".to_string(),
            });

        let top_expense = period_txs
            .iter()
            .filter(|tx| tx.tx_type == TransactionType::Expense)
            .max_by(|a, b| a.amount.cmp(&b.amount))
            .map(|tx| WeeklyHighlight {
                subtitle: tx_short_subtitle(state, tx),
                badge_text: format_signed_rwf(TransactionType::Expense, tx.amount),
            })
            .unwrap_or_else(|| WeeklyHighlight {
                subtitle: empty_sub.clone(),
                badge_text: "0 RWF".to_string(),
            });

        let latest_handover = period_txs
            .iter()
            .copied()
            .find(|tx| tx.tx_type == TransactionType::Handover)
            .or_else(|| {
                state
                    .transactions
                    .iter()
                    .find(|tx| tx.tx_type == TransactionType::Handover)
            })
            .map(|tx| WeeklyHighlight {
                subtitle: handover_subtitle(state, tx),
                badge_text: format_signed_rwf(TransactionType::Handover, tx.amount),
            })
            .unwrap_or_else(|| WeeklyHighlight {
                subtitle: empty_sub,
                badge_text: "0 RWF".to_string(),
            });

        // Recent Custody Activity rows (up to 5 matching active period)
        let recent_rows = period_txs
            .into_iter()
            .take(5)
            .map(|tx| build_recent_row(state, tx))
            .collect();

        Self {
            is_family,
            active_period,
            active_period_label,
            current_year,
            previous_year,
            report_date_label,
            total_custody,
            period_inflow,
            period_expense,
            active_projects_or_plans_count,
            active_loans_count,
            members_or_wallets_count,
            cashflow_bars,
            cashflow_band_labels,
            yearly_current_total,
            yearly_prev_total,
            yearly_yoy_pct,
            yearly_share_pct,
            net_savings,
            net_savings_mom_pct,
            savings_sparkline,
            savings_spark_max,
            monthly_flow_bars,
            pool_custody_amount,
            cash_custody_amount,
            total_wallets_count: state.wallets.len(),
            active_wallets_count,
            wallet_sparkline,
            wallet_spark_max,
            handovers_count,
            handovers_delta_pct,
            handover_bars,
            member_chips,
            hero_goals,
            weekly_points,
            weekly_max,
            top_inflow,
            top_expense,
            latest_handover,
            recent_rows,
        }
    }
}

fn nice_symmetric_bound(max_val: f64) -> f64 {
    if max_val <= 0.0 {
        return 200_000.0;
    }
    let steps = [
        10_000.0,
        20_000.0,
        50_000.0,
        100_000.0,
        200_000.0,
        500_000.0,
        1_000_000.0,
        2_000_000.0,
        5_000_000.0,
        10_000_000.0,
    ];
    for step in steps {
        if max_val <= step {
            return step;
        }
    }
    (max_val / 5_000_000.0).ceil() * 5_000_000.0
}

fn compute_delta_pct(current: Decimal, previous: Decimal) -> i32 {
    if previous.abs() > Decimal::ZERO {
        (((current - previous) * Decimal::from(100)) / previous.abs())
            .round()
            .to_i32()
            .unwrap_or(0)
            .clamp(-999, 999)
    } else if current > Decimal::ZERO {
        100
    } else if current < Decimal::ZERO {
        -100
    } else {
        0
    }
}

fn ensure_sparkline_variance(points: &mut [SparkPoint]) -> f64 {
    let max = points.iter().map(|p| p.value).fold(0.0_f64, f64::max);
    if max <= 0.0 {
        if let Some(last) = points.last_mut() {
            last.value = 0.001;
        }
        1.0
    } else {
        (max * 1.15).max(1.0)
    }
}

fn tx_short_subtitle(state: &AppState, tx: &Transaction) -> String {
    if !tx.category.trim().is_empty() {
        tx.category.clone()
    } else if let Some(desc) = tx.description.as_ref().filter(|d| !d.trim().is_empty()) {
        desc.clone()
    } else {
        umuryango_i18n::t(state.locale, "transactions.general")
    }
}

fn handover_subtitle(state: &AppState, tx: &Transaction) -> String {
    let from_w = state
        .wallets
        .iter()
        .find(|w| w.id == tx.source_wallet_id)
        .and_then(|w| {
            state
                .custodian_name_for_wallet(w)
                .or_else(|| Some(w.name.clone()))
        });
    let to_w = tx
        .destination_wallet_id
        .and_then(|id| state.wallets.iter().find(|w| w.id == id))
        .and_then(|w| {
            state
                .custodian_name_for_wallet(w)
                .or_else(|| Some(w.name.clone()))
        });

    if let (Some(from), Some(to)) = (from_w, to_w) {
        let from_short = from.split_whitespace().next().unwrap_or(&from);
        let to_short = to.split_whitespace().next().unwrap_or(&to);
        umuryango_i18n::t(state.locale, "dashboard.handover_from_to")
            .replace("{from}", from_short)
            .replace("{to}", to_short)
    } else {
        tx_short_subtitle(state, tx)
    }
}

fn role_label(locale: Locale, role: FamilyRole) -> String {
    match role {
        FamilyRole::Head => umuryango_i18n::t(locale, "members.role_head"),
        FamilyRole::Custodian => umuryango_i18n::t(locale, "members.role_custodian"),
        FamilyRole::Member => umuryango_i18n::t(locale, "members.role_member"),
    }
}

fn build_recent_row(state: &AppState, tx: &Transaction) -> RecentActivityRow {
    let locale = state.locale;
    let member = state
        .family_members
        .iter()
        .find(|m| m.user_id == tx.authorized_by);

    let actor_name = member
        .map(|m| m.full_name.clone())
        .or_else(|| state.actor_name(tx.authorized_by))
        .or_else(|| {
            state
                .wallets
                .iter()
                .find(|w| w.id == tx.source_wallet_id)
                .map(|w| w.name.clone())
        })
        .unwrap_or_else(|| umuryango_i18n::t(locale, "wallets.unassigned_custodian"));

    let actor_role_label = if state.active_scope.is_family() {
        let role = member.map(|m| m.role).unwrap_or_else(|| {
            if tx.authorized_by == state.user_id {
                state.active_role
            } else {
                FamilyRole::Member
            }
        });
        role_label(locale, role)
    } else {
        Translator::t(locale, terms::PERSONAL_SCOPE, None)
    };

    let cat = if tx.category.trim().is_empty() {
        umuryango_i18n::t(locale, "transactions.general")
    } else {
        tx.category.clone()
    };
    let category_note = match tx.description.as_deref().map(str::trim) {
        Some(desc) if !desc.is_empty() && desc != cat => format!("{} — {}", cat, desc),
        _ => cat,
    };

    let type_badge_label = match tx.tx_type {
        TransactionType::Income => Translator::t(locale, terms::INCOME, None),
        TransactionType::Expense => Translator::t(locale, terms::EXPENSE, None),
        TransactionType::Handover => umuryango_i18n::t(locale, "dashboard.handovers_label"),
    };

    RecentActivityRow {
        initials: initials_from_name(&actor_name),
        actor_name,
        actor_role_label,
        category_note,
        tx_type: tx.tx_type,
        type_badge_label,
        amount_label: format_signed_rwf(tx.tx_type, tx.amount),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeZone};
    use umuryango_core_domain::models::Wallet;
    use umuryango_core_domain::scope::LedgerScope;
    use uuid::Uuid;

    fn utc_ymd(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        let date = NaiveDate::from_ymd_opt(year, month, day).expect("valid test date");
        Utc.from_utc_datetime(&date.and_hms_opt(12, 0, 0).expect("valid test time"))
    }

    #[test]
    fn rwf_formatters_group_thousands_without_float_drift() {
        assert_eq!(format_rwf(Decimal::new(145_000_000, 2)), "1,450,000 RWF");
        assert_eq!(format_rwf(Decimal::ZERO), "0 RWF");
        assert_eq!(
            format_signed_rwf(TransactionType::Income, Decimal::from(350_000)),
            "+350,000 RWF"
        );
        assert_eq!(
            format_signed_rwf(TransactionType::Expense, Decimal::from(84_500)),
            "-84,500 RWF"
        );
        assert_eq!(
            format_signed_rwf(TransactionType::Handover, Decimal::from(50_000)),
            "50,000 RWF"
        );
        assert_eq!(format_compact_axis(200_000.0), "+200k");
        assert_eq!(format_compact_axis(0.0), "0");
        assert_eq!(format_compact_axis(-100_000.0), "-100k");
    }

    #[test]
    fn layout_tiers_adapt_to_content_width() {
        assert_eq!(
            DashboardLayoutTier::from_content_width(1200.0),
            DashboardLayoutTier::Wide
        );
        assert_eq!(DashboardLayoutTier::Wide.kpi_columns(), 5);
        assert_eq!(
            DashboardLayoutTier::from_content_width(820.0),
            DashboardLayoutTier::Medium
        );
        assert_eq!(DashboardLayoutTier::Medium.kpi_columns(), 3);
        assert_eq!(
            DashboardLayoutTier::from_content_width(500.0),
            DashboardLayoutTier::Compact
        );
        assert_eq!(DashboardLayoutTier::Compact.kpi_columns(), 2);
        assert_eq!(
            DashboardLayoutTier::from_content_width(360.0),
            DashboardLayoutTier::NarrowMobile
        );
        assert_eq!(DashboardLayoutTier::NarrowMobile.kpi_columns(), 1);
    }

    #[test]
    fn snapshot_custody_split_and_savings_preserve_exact_invariants() {
        let family_id = Uuid::new_v4();
        let actor_id = Uuid::new_v4();
        let scope = LedgerScope::Family { family_id };
        let now = utc_ymd(2026, 11, 21);

        let momo_id = Uuid::new_v4();
        let cash_id = Uuid::new_v4();
        let mut state = AppState {
            user_id: actor_id,
            user_name: "Bruno Rwanda".to_string(),
            active_scope: scope,
            locale: Locale::En,
            is_authenticated: true,
            ..Default::default()
        };
        state.wallets.push(Wallet {
            id: momo_id,
            scope,
            channel: WalletChannel::MobileMoney,
            name: "MTN MoMo".to_string(),
            account_number: None,
            currency: "RWF".to_string(),
            balance: Decimal::from(980_000),
            custodian_id: Some(actor_id),
            is_active: true,
            created_at: now,
            updated_at: now,
        });
        state.wallets.push(Wallet {
            id: cash_id,
            scope,
            channel: WalletChannel::CashInHand,
            name: "Household Cash".to_string(),
            account_number: None,
            currency: "RWF".to_string(),
            balance: Decimal::from(470_000),
            custodian_id: Some(actor_id),
            is_active: true,
            created_at: now,
            updated_at: now,
        });
        state.transactions.push(Transaction {
            id: Uuid::new_v4(),
            scope,
            source_wallet_id: momo_id,
            destination_wallet_id: None,
            tx_type: TransactionType::Income,
            amount: Decimal::from(620_000),
            currency: "RWF".to_string(),
            category: "Maman Contribution".to_string(),
            description: None,
            authorized_by: actor_id,
            paid_to: None,
            tx_date: now,
            created_at: now,
        });
        state.transactions.push(Transaction {
            id: Uuid::new_v4(),
            scope,
            source_wallet_id: cash_id,
            destination_wallet_id: None,
            tx_type: TransactionType::Expense,
            amount: Decimal::from(285_000),
            currency: "RWF".to_string(),
            category: "Household Groceries".to_string(),
            description: None,
            authorized_by: actor_id,
            paid_to: None,
            tx_date: now,
            created_at: now,
        });

        let snap = DashboardSnapshot::compute(&state, now);
        assert_eq!(snap.total_custody, Decimal::from(1_450_000));
        assert_eq!(
            snap.pool_custody_amount + snap.cash_custody_amount,
            snap.total_custody
        );
        assert_eq!(snap.period_inflow, Decimal::from(620_000));
        assert_eq!(snap.period_expense, Decimal::from(285_000));
        assert_eq!(snap.net_savings, Decimal::from(335_000));
        assert_eq!(snap.active_period_label, "November 2026");
    }
}

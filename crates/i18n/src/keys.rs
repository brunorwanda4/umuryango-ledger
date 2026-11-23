use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(example = "rw")]
pub enum Locale {
    #[default]
    #[serde(rename = "rw")]
    Rw,
    #[serde(rename = "en")]
    En,
}

impl Locale {
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::Rw => "rw",
            Locale::En => "en",
        }
    }

    pub fn from_header(header: Option<&str>) -> Self {
        header
            .map(|h| Self::from_accept_language(h))
            .unwrap_or(Locale::Rw)
    }

    pub fn from_accept_language(header: &str) -> Self {
        let mut best_locale = Locale::Rw;
        let mut max_q = -1.0;

        for part in header.split(',') {
            let mut items = part.trim().split(';');
            let tag = items.next().unwrap_or("").trim().to_lowercase();
            let mut q = 1.0;

            if let Some(q_part) = items.next() {
                let q_part = q_part.trim();
                if q_part.starts_with("q=") {
                    if let Ok(val) = q_part[2..].parse::<f32>() {
                        q = val;
                    }
                }
            }

            if tag.starts_with("rw") {
                if q > max_q {
                    max_q = q;
                    best_locale = Locale::Rw;
                }
            } else if tag.starts_with("en") {
                if q > max_q {
                    max_q = q;
                    best_locale = Locale::En;
                }
            }
        }

        if max_q >= 0.0 {
            best_locale
        } else {
            Locale::Rw
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Locale {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean = s.trim().to_lowercase();
        if clean.starts_with("en") {
            Ok(Locale::En)
        } else if clean.starts_with("rw") {
            Ok(Locale::Rw)
        } else {
            Ok(Locale::Rw)
        }
    }
}

pub mod terms {
    pub const APP_NAME: &str = "app-name";
    pub const TAGLINE: &str = "tagline";

    pub const NAV_DASHBOARD: &str = "nav-dashboard";
    pub const NAV_TRANSACTIONS: &str = "nav-transactions";
    pub const NAV_WALLETS: &str = "nav-wallets";
    pub const NAV_PROJECTS: &str = "nav-projects";
    pub const NAV_LOANS: &str = "nav-loans";
    pub const NAV_PLANS: &str = "nav-plans";
    pub const NAV_REPORTS: &str = "nav-reports";
    pub const NAV_MEMBERS: &str = "nav-members";

    pub const PERSONAL_SCOPE: &str = "personal-scope";
    pub const FAMILY_SCOPE: &str = "family-scope";
    pub const ROLE_HEAD: &str = "role-head";
    pub const ROLE_CUSTODIAN: &str = "role-custodian";
    pub const ROLE_MEMBER: &str = "role-member";

    pub const INCOME: &str = "income";
    pub const EXPENSE: &str = "expense";
    pub const HANDOVER: &str = "handover";
    pub const CUSTODIAN: &str = "custodian";
    pub const OPENING_BALANCE: &str = "opening-balance";
    pub const CLOSING_BALANCE: &str = "closing-balance";
    pub const HELD_BY: &str = "held-by";
    pub const AUTHORIZED_BY: &str = "authorized-by";
    pub const PAID_TO: &str = "paid-to";

    pub const CHANNEL_BANK: &str = "channel-bank";
    pub const CHANNEL_MOMO: &str = "channel-momo";
    pub const CHANNEL_CASH: &str = "channel-cash";

    pub const MOTHERS_REPORT_TITLE: &str = "mothers-report-title";
    pub const TOTAL_INCOME: &str = "total-income";
    pub const TOTAL_EXPENSES: &str = "total-expenses";
    pub const CASH_CUSTODY_BREAKDOWN: &str = "cash-custody-breakdown";
    pub const ACTIVE_PROJECTS: &str = "active-projects";
    pub const LOANS_SUMMARY: &str = "loans-summary";
    pub const FUTURE_PLANS: &str = "future-plans";

    pub const NOTIF_INCOME_ADDED: &str = "notif-income-added";
    pub const NOTIF_EXPENSE_LOGGED: &str = "notif-expense-logged";
    pub const NOTIF_HANDOVER_COMPLETED: &str = "notif-handover-completed";
    pub const NOTIF_PROJECT_LOAN_UPDATED: &str = "notif-project-loan-updated";
    pub const NOTIF_APP_UPDATE_AVAILABLE: &str = "notif-app-update-available";
    pub const NOTIF_APP_UPDATE_MANDATORY: &str = "notif-app-update-mandatory";

    pub const UPDATE_READY_BANNER: &str = "update-ready-banner";
    pub const UPDATE_DOWNLOADING: &str = "update-downloading";
    pub const UPDATE_MANDATORY_TITLE: &str = "update-mandatory-title";
    pub const UPDATE_RESTART_NOW: &str = "update-restart-now";

    pub const ERR_INSUFFICIENT_CUSTODY: &str = "error-insufficient-custody";
    pub const ERR_UNAUTHORIZED_ROLE: &str = "error-unauthorized-role";
    pub const ERR_INVALID_HANDOVER: &str = "error-invalid-handover";
    pub const ERR_SCOPE_MISMATCH: &str = "error-scope-mismatch";
    pub const ERR_INVALID_AMOUNT: &str = "error-invalid-amount";
    pub const ERR_NOT_FOUND: &str = "error-not-found";
    pub const ERR_VALIDATION: &str = "error-validation";
}

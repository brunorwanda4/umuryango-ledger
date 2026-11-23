use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::scope::LedgerScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoanDirection {
    Lent,
    Borrowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoanStatus {
    Active,
    Repaid,
    Defaulted,
}

impl LoanStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Loan {
    pub id: Uuid,
    pub scope: LedgerScope,
    pub counterparty_name: String,
    pub principal_amount: Decimal,
    pub balance_remaining: Decimal,
    pub interest_rate: Decimal,
    pub due_date: Option<NaiveDate>,
    pub direction: LoanDirection,
    pub status: LoanStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

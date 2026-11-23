use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::scope::LedgerScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(example = "lent")]
pub enum LoanDirection {
    Lent,
    Borrowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(example = "active")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Loan {
    #[schema(example = "5f210e30-b3b4-4b52-9ef1-8c46059d2902")]
    pub id: Uuid,
    pub scope: LedgerScope,
    #[schema(example = "Uncle Jean")]
    pub counterparty_name: String,
    #[schema(example = "1000000.00")]
    pub principal_amount: Decimal,
    #[schema(example = "400000.00")]
    pub balance_remaining: Decimal,
    #[schema(example = "0.00")]
    pub interest_rate: Decimal,
    #[schema(example = "2026-12-01")]
    pub due_date: Option<NaiveDate>,
    pub direction: LoanDirection,
    pub status: LoanStatus,
    #[schema(example = "Harvest bridge financing")]
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

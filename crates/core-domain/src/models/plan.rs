use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::scope::LedgerScope;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuturePlan {
    pub id: Uuid,
    pub scope: LedgerScope,
    pub title: String,
    pub description: Option<String>,
    pub estimated_cost: Decimal,
    pub target_date: Option<NaiveDate>,
    pub is_achieved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

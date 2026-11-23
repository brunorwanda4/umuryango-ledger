use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::scope::LedgerScope;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct FuturePlan {
    #[schema(example = "7e14a229-3df2-42ec-a077-e83cb2b041e8")]
    pub id: Uuid,
    pub scope: LedgerScope,
    #[schema(example = "Solar Power Installation")]
    pub title: String,
    #[schema(example = "Equip rural home with 5kW solar backup system")]
    pub description: Option<String>,
    #[schema(example = "3200000.00")]
    pub estimated_cost: Decimal,
    #[schema(example = "2027-06-30")]
    pub target_date: Option<NaiveDate>,
    #[schema(example = false)]
    pub is_achieved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

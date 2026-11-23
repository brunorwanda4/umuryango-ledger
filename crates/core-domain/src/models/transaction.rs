use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::scope::LedgerScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(example = "income")]
pub enum TransactionType {
    Income,
    Expense,
    Handover,
}

impl TransactionType {
    pub fn is_income(&self) -> bool {
        matches!(self, Self::Income)
    }

    pub fn is_expense(&self) -> bool {
        matches!(self, Self::Expense)
    }

    pub fn is_handover(&self) -> bool {
        matches!(self, Self::Handover)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Transaction {
    #[schema(example = "b24c18d3-5592-4933-bf9a-32863b7e4128")]
    pub id: Uuid,
    pub scope: LedgerScope,
    #[schema(example = "6cb3b9a5-1d42-43bb-a5a1-778912efc401")]
    pub source_wallet_id: Uuid,
    #[schema(example = "8fd4a102-3c12-4fbc-b530-9941a7741ef0")]
    pub destination_wallet_id: Option<Uuid>,
    pub tx_type: TransactionType,
    #[schema(example = "50000.00")]
    pub amount: Decimal,
    #[schema(example = "RWF")]
    pub currency: String,
    #[schema(example = "Construction Materials")]
    pub category: String,
    #[schema(example = "50 bags of cement delivered to Bugesera site")]
    pub description: Option<String>,
    #[schema(example = "415d18d4-5487-432a-bb3b-31d7f6b96e47")]
    pub authorized_by: Uuid,
    #[schema(example = "Cimerwa Depot")]
    pub paid_to: Option<String>,
    pub tx_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::scope::LedgerScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub scope: LedgerScope,
    pub source_wallet_id: Uuid,
    pub destination_wallet_id: Option<Uuid>,
    pub tx_type: TransactionType,
    pub amount: Decimal,
    pub currency: String,
    pub category: String,
    pub description: Option<String>,
    pub authorized_by: Uuid,
    pub paid_to: Option<String>,
    pub tx_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

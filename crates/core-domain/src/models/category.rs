use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::scope::LedgerScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(example = "expense")]
pub enum CategoryKind {
    Income,
    #[default]
    Expense,
}

impl CategoryKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Income => "income",
            Self::Expense => "expense",
        }
    }

    pub fn from_db_str(raw: &str) -> Self {
        if raw.eq_ignore_ascii_case("income") {
            Self::Income
        } else {
            Self::Expense
        }
    }

    pub const fn is_income(&self) -> bool {
        matches!(self, Self::Income)
    }

    pub const fn is_expense(&self) -> bool {
        matches!(self, Self::Expense)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct MoneyCategory {
    #[schema(example = "9c12b4a0-41d2-4d77-8a11-77e8912bc109")]
    pub id: Uuid,
    pub scope: LedgerScope,
    pub kind: CategoryKind,
    #[schema(example = "School Fees")]
    pub name: String,
    pub created_at: DateTime<Utc>,
}

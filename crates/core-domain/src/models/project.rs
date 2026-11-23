use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Completed,
    Paused,
}

impl ProjectStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Planning | Self::InProgress)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Planning => "planning",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Paused => "paused",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub family_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub target_amount: Decimal,
    pub allocated_amount: Decimal,
    pub status: ProjectStatus,
    pub start_date: Option<NaiveDate>,
    pub target_end_date: Option<NaiveDate>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
#[schema(example = "planning")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Project {
    #[schema(example = "3c3a4411-9a71-44bb-9972-e19c30114a90")]
    pub id: Uuid,
    #[schema(example = "9a756613-263a-4467-9d7a-11239c04901f")]
    pub family_id: Uuid,
    #[schema(example = "Bugesera Family House")]
    pub name: String,
    #[schema(example = "Building boundary wall and foundation")]
    pub description: Option<String>,
    #[schema(example = "15000000.00")]
    pub target_amount: Decimal,
    #[schema(example = "6500000.00")]
    pub allocated_amount: Decimal,
    pub status: ProjectStatus,
    #[schema(example = "2026-01-15")]
    pub start_date: Option<NaiveDate>,
    #[schema(example = "2026-12-31")]
    pub target_end_date: Option<NaiveDate>,
    #[schema(example = "415d18d4-5487-432a-bb3b-31d7f6b96e47")]
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

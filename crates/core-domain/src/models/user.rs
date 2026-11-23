use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use umuryango_i18n::Locale;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[schema(example = "415d18d4-5487-432a-bb3b-31d7f6b96e47")]
    pub id: Uuid,
    #[schema(example = "Bruno Hapiman")]
    pub full_name: String,
    #[schema(example = "bruno@example.rw")]
    pub email: Option<String>,
    #[schema(example = "+250788123456")]
    pub phone_number: Option<String>,
    pub preferred_locale: Locale,
    #[schema(example = true)]
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

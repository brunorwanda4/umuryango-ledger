use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use umuryango_i18n::Locale;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub preferred_locale: Locale,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

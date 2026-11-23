use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FamilyRole {
    Head,
    Custodian,
    Member,
}

impl FamilyRole {
    pub fn can_transact(&self) -> bool {
        matches!(self, FamilyRole::Head | FamilyRole::Custodian)
    }

    pub fn can_authorize(&self) -> bool {
        matches!(self, FamilyRole::Head)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Family {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub currency: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyMember {
    pub id: Uuid,
    pub family_id: Uuid,
    pub user_id: Uuid,
    pub role: FamilyRole,
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

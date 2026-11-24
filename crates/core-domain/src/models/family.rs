use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
#[schema(example = "head")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Family {
    #[schema(example = "9a756613-263a-4467-9d7a-11239c04901f")]
    pub id: Uuid,
    #[schema(example = "Umuryango w'Abahire")]
    pub name: String,
    #[schema(example = "Rwanda & Diaspora joint custody ledger")]
    pub description: Option<String>,
    #[schema(example = "RWF")]
    pub currency: String,
    #[schema(example = "HK0N")]
    pub invite_code: Option<String>,
    #[schema(example = "415d18d4-5487-432a-bb3b-31d7f6b96e47")]
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct FamilyWithRole {
    #[schema(example = "9a756613-263a-4467-9d7a-11239c04901f")]
    pub id: Uuid,
    #[schema(example = "Umuryango w'Abahire")]
    pub name: String,
    #[schema(example = "Rwanda & Diaspora joint custody ledger")]
    pub description: Option<String>,
    #[schema(example = "RWF")]
    pub currency: String,
    pub role: FamilyRole,
    #[schema(example = "HK0N")]
    pub invite_code: Option<String>,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct FamilyMember {
    #[schema(example = "a11e8605-7f41-45ec-b91b-68936ef11d51")]
    pub id: Uuid,
    #[schema(example = "9a756613-263a-4467-9d7a-11239c04901f")]
    pub family_id: Uuid,
    #[schema(example = "415d18d4-5487-432a-bb3b-31d7f6b96e47")]
    pub user_id: Uuid,
    pub role: FamilyRole,
    #[schema(example = true)]
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct FamilyMemberView {
    #[schema(example = "a11e8605-7f41-45ec-b91b-68936ef11d51")]
    pub id: Uuid,
    #[schema(example = "9a756613-263a-4467-9d7a-11239c04901f")]
    pub family_id: Uuid,
    #[schema(example = "415d18d4-5487-432a-bb3b-31d7f6b96e47")]
    pub user_id: Uuid,
    #[schema(example = "Bruno Hapiman")]
    pub full_name: String,
    #[schema(example = "+250788123456")]
    pub phone: String,
    #[schema(example = "Papa")]
    pub display_name: Option<String>,
    pub role: FamilyRole,
    #[schema(example = true)]
    pub is_active: bool,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct PendingFamilyInvite {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "9a756613-263a-4467-9d7a-11239c04901f")]
    pub family_id: Uuid,
    #[schema(example = "Umuryango w'Abahire")]
    pub family_name: String,
    #[schema(example = "Bruno Hapiman")]
    pub invited_by_name: String,
    #[schema(example = "+250788123456")]
    pub invited_phone: String,
    pub role: FamilyRole,
    #[schema(example = "PENDING")]
    pub status: String,
    pub created_at: DateTime<Utc>,
}

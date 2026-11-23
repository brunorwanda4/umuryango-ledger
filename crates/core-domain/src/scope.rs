use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LedgerScope {
    Personal {
        #[schema(example = "415d18d4-5487-432a-bb3b-31d7f6b96e47")]
        user_id: Uuid,
    },
    Family {
        #[schema(example = "9a756613-263a-4467-9d7a-11239c04901f")]
        family_id: Uuid,
    },
}

impl LedgerScope {
    pub fn personal(user_id: Uuid) -> Self {
        Self::Personal { user_id }
    }

    pub fn family(family_id: Uuid) -> Self {
        Self::Family { family_id }
    }

    pub fn is_personal(&self) -> bool {
        matches!(self, Self::Personal { .. })
    }

    pub fn is_family(&self) -> bool {
        matches!(self, Self::Family { .. })
    }

    pub fn user_id(&self) -> Option<Uuid> {
        match self {
            Self::Personal { user_id } => Some(*user_id),
            Self::Family { .. } => None,
        }
    }

    pub fn family_id(&self) -> Option<Uuid> {
        match self {
            Self::Personal { .. } => None,
            Self::Family { family_id } => Some(*family_id),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Personal { .. } => "personal",
            Self::Family { .. } => "family",
        }
    }
}

impl fmt::Display for LedgerScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Personal { user_id } => write!(f, "personal:{}", user_id),
            Self::Family { family_id } => write!(f, "family:{}", family_id),
        }
    }
}

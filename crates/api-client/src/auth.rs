use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSession {
    pub token: Option<String>,
    pub user_id: Option<uuid::Uuid>,
}

impl AuthSession {
    pub fn new(token: String, user_id: uuid::Uuid) -> Self {
        Self {
            token: Some(token),
            user_id: Some(user_id),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }
}

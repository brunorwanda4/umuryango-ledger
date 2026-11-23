use sqlx::PgPool;
use std::sync::Arc;

use crate::realtime::FamilyHub;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub public_web_url: String,
    pub family_hub: Arc<FamilyHub>,
}

impl AppState {
    pub fn new(db: PgPool, jwt_secret: String, public_web_url: String) -> Self {
        Self {
            db,
            jwt_secret,
            public_web_url,
            family_hub: Arc::new(FamilyHub::new()),
        }
    }
}

use actix_web::{FromRequest, HttpRequest, dev::Payload, error::ErrorBadRequest};
use futures::future::{Ready, err, ok};
use umuryango_core_domain::scope::LedgerScope;
use uuid::Uuid;

#[allow(dead_code)]
pub struct RequestScope(pub LedgerScope);

impl FromRequest for RequestScope {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let scope_str = req
            .headers()
            .get("X-Financial-Scope")
            .and_then(|val| val.to_str().ok())
            .unwrap_or("family");

        let user_id = req
            .headers()
            .get("X-User-Id")
            .and_then(|val| val.to_str().ok())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        let family_id = req
            .headers()
            .get("X-Family-Id")
            .and_then(|val| val.to_str().ok())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        match scope_str.trim().to_lowercase().as_str() {
            "personal" => ok(RequestScope(LedgerScope::Personal { user_id })),
            "family" => ok(RequestScope(LedgerScope::Family { family_id })),
            other => err(ErrorBadRequest(format!(
                "Unknown financial scope: {}",
                other
            ))),
        }
    }
}

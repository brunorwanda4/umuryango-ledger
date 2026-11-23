use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use std::str::FromStr;
use umuryango_i18n::Locale;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestLocale(pub Locale);

impl FromRequest for RequestLocale {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // 1. Check query parameter `?lang=rw|en`
        let query_str = req.query_string();
        for param in query_str.split('&') {
            if let Some(val) = param.strip_prefix("lang=") {
                if let Ok(loc) = Locale::from_str(val) {
                    return ok(RequestLocale(loc));
                }
            }
        }

        // 2. Check Accept-Language HTTP header
        let header_val = req
            .headers()
            .get("Accept-Language")
            .and_then(|h| h.to_str().ok());

        let locale = Locale::from_header(header_val);
        ok(RequestLocale(locale))
    }
}

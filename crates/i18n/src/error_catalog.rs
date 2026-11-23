use fluent_bundle::FluentArgs;
use serde::{Deserialize, Serialize};

use crate::keys::Locale;
use crate::Translator;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedError {
    pub code: String,
    pub message: String,
}

pub trait LocalizableError {
    fn error_code(&self) -> &'static str;
    fn to_fluent_args(&self) -> Option<FluentArgs<'_>> {
        None
    }
}

pub fn localize_domain_error<E: LocalizableError>(locale: Locale, err: &E) -> LocalizedError {
    let code = err.error_code();
    let args = err.to_fluent_args();
    let message = Translator::t(locale, code, args.as_ref());
    LocalizedError {
        code: code.to_string(),
        message,
    }
}

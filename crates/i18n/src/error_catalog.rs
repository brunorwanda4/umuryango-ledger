use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::keys::Locale;
use crate::loader::lookup_and_format;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct LocalizedError {
    #[schema(example = "errors.insufficient_custody")]
    pub code: String,
    #[schema(example = "Konti 'MoMo' ifite 5,000 RWF gusa; ntihagije ngo ukoreshe 20,000 RWF.")]
    pub message: String,
}

pub trait LocalizableError {
    fn error_code(&self) -> &'static str;
    fn error_args(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

pub fn localize_domain_error<E: LocalizableError>(locale: Locale, error: &E) -> LocalizedError {
    let raw_args = error.error_args();
    let borrowed_args: Vec<(&str, &str)> = raw_args.iter().map(|(k, v)| (*k, v.as_str())).collect();

    let message = lookup_and_format(locale, error.error_code(), &borrowed_args);
    LocalizedError {
        code: error.error_code().to_string(),
        message,
    }
}

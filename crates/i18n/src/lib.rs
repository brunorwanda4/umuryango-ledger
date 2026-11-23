use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

pub mod keys;
pub use keys::{Locale, terms};

pub static EN_FTL: &str = include_str!("../../../locales/en.ftl");
pub static RW_FTL: &str = include_str!("../../../locales/rw.ftl");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedError {
    pub code: String,
    pub message: String,
}

pub trait LocalizableError {
    fn error_code(&self) -> &'static str;
    fn to_fluent_args(&self) -> Option<FluentArgs<'static>>;
}

pub fn localize_domain_error<E: LocalizableError>(locale: Locale, error: &E) -> LocalizedError {
    let args = error.to_fluent_args();
    let message = Translator::t(locale, error.error_code(), args.as_ref());
    LocalizedError {
        code: error.error_code().to_string(),
        message,
    }
}

thread_local! {
    static BUNDLES: RefCell<HashMap<Locale, FluentBundle<FluentResource>>> = RefCell::new({
        let mut map = HashMap::new();

        // English bundle
        let en_lang: LanguageIdentifier = "en".parse().expect("valid en locale");
        let mut en_bundle = FluentBundle::new(vec![en_lang]);
        en_bundle.set_use_isolating(false);
        let en_res = FluentResource::try_new(EN_FTL.to_string())
            .expect("Failed to parse en.ftl at compile time");
        en_bundle
            .add_resource(en_res)
            .expect("Failed to add en resource");
        map.insert(Locale::En, en_bundle);

        // Kinyarwanda bundle
        let rw_lang: LanguageIdentifier = "rw".parse().expect("valid rw locale");
        let mut rw_bundle = FluentBundle::new(vec![rw_lang]);
        rw_bundle.set_use_isolating(false);
        let rw_res = FluentResource::try_new(RW_FTL.to_string())
            .expect("Failed to parse rw.ftl at compile time");
        rw_bundle
            .add_resource(rw_res)
            .expect("Failed to add rw resource");
        map.insert(Locale::Rw, rw_bundle);

        map
    });
}

#[derive(Debug, Clone, Default)]
pub struct Localizer;

impl Localizer {
    pub fn new() -> Self {
        Self
    }

    pub fn translate(&self, locale: Locale, key: &str, args: Option<&FluentArgs>) -> String {
        Translator::t(locale, key, args)
    }
}

pub struct Translator;

impl Translator {
    /// Formats a localized string for the specified locale and key, with optional arguments
    pub fn t(locale: Locale, key: &str, args: Option<&FluentArgs>) -> String {
        BUNDLES.with(|cell| {
            let map = cell.borrow();
            let bundle = map.get(&locale).or_else(|| map.get(&Locale::Rw));

            if let Some(b) = bundle {
                if let Some(msg) = b.get_message(key) {
                    if let Some(pattern) = msg.value() {
                        let mut errors = vec![];
                        let val = b.format_pattern(pattern, args, &mut errors);
                        return val.to_string();
                    }
                }
            }

            key.to_string()
        })
    }

    /// Formats domain error code to human-readable message in requested locale
    pub fn localize_error(locale: Locale, error_code: &str, args: Option<&FluentArgs>) -> String {
        let key = format!("error-{}", error_code.replace('_', "-").to_lowercase());
        Self::t(locale, &key, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fluent_bundle::FluentValue;

    #[test]
    fn test_locale_from_header() {
        assert_eq!(
            Locale::from_header(Some("en-US,en;q=0.9,rw;q=0.8")),
            Locale::En
        );
        assert_eq!(
            Locale::from_header(Some("rw-RW,rw;q=0.9,en;q=0.8")),
            Locale::Rw
        );
        assert_eq!(Locale::from_header(Some("")), Locale::Rw);
        assert_eq!(Locale::from_header(None), Locale::Rw);
    }

    #[test]
    fn test_ledger_terms_translation() {
        let en_scope = Translator::t(Locale::En, terms::PERSONAL_SCOPE, None);
        assert_eq!(en_scope, "Personal Finances");

        let rw_scope = Translator::t(Locale::Rw, terms::PERSONAL_SCOPE, None);
        assert_eq!(rw_scope, "Imari Yanjye Bwite");

        let en_handover = Translator::t(Locale::En, terms::HANDOVER, None);
        assert_eq!(en_handover, "Custody Handover");

        let rw_handover = Translator::t(Locale::Rw, terms::HANDOVER, None);
        assert_eq!(rw_handover, "Ihererekanya ry'Umutungo");
    }

    #[test]
    fn test_insufficient_custody_error_interpolation() {
        let mut args = FluentArgs::new();
        args.set("wallet", FluentValue::from("BK Main"));
        args.set("available", FluentValue::from("10,000"));
        args.set("requested", FluentValue::from("50,000"));

        let en_err = Translator::t(Locale::En, terms::ERR_INSUFFICIENT_CUSTODY, Some(&args));
        assert_eq!(
            en_err,
            "Wallet 'BK Main' only holds 10,000 RWF, which is not enough for 50,000 RWF."
        );

        let rw_err = Translator::t(Locale::Rw, terms::ERR_INSUFFICIENT_CUSTODY, Some(&args));
        assert_eq!(
            rw_err,
            "Konti 'BK Main' ifite 10,000 RWF gusa; ntihagije ngo ukoreshe 50,000 RWF."
        );
    }

    #[test]
    fn test_localize_domain_error() {
        let rw = Translator::localize_error(Locale::Rw, "UNAUTHORIZED_ROLE", None);
        assert_eq!(
            rw,
            "Inshingano zawe muri uyu muryango ntizikwemerera gukora iki gikorwa."
        );

        let en = Translator::localize_error(Locale::En, "UNAUTHORIZED_ROLE", None);
        assert_eq!(
            en,
            "Your role in this family does not allow performing this action."
        );
    }
}

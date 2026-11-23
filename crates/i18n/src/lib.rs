pub mod error_catalog;
pub mod keys;

pub use error_catalog::{localize_domain_error, LocalizableError, LocalizedError};
pub use keys::{terms, Locale};

use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource};
use std::collections::HashMap;
use std::sync::OnceLock;
use unic_langid::LanguageIdentifier;

static GLOBAL_TRANSLATOR: OnceLock<Translator> = OnceLock::new();

pub struct Translator {
    bundles: HashMap<Locale, FluentBundle<FluentResource>>,
}

// Backward compatibility alias
pub type Localizer = Translator;

impl Default for Translator {
    fn default() -> Self {
        Self::new()
    }
}

impl Translator {
    pub fn new() -> Self {
        let mut bundles = HashMap::new();

        // English Bundle
        let en_source = include_str!("../../../locales/en.ftl");
        if let Ok(bundle) = Self::create_bundle("en", en_source) {
            bundles.insert(Locale::En, bundle);
        }

        // Kinyarwanda Bundle
        let rw_source = include_str!("../../../locales/rw.ftl");
        if let Ok(bundle) = Self::create_bundle("rw", rw_source) {
            bundles.insert(Locale::Rw, bundle);
        }

        Self { bundles }
    }

    pub fn global() -> &'static Translator {
        GLOBAL_TRANSLATOR.get_or_init(Translator::new)
    }

    /// Global translation helper
    pub fn t(locale: Locale, key: &str, args: Option<&FluentArgs>) -> String {
        Self::global().translate(locale, key, args)
    }

    fn create_bundle(
        lang: &str,
        source: &'static str,
    ) -> Result<FluentBundle<FluentResource>, String> {
        let lang_id: LanguageIdentifier = lang.parse().map_err(|e| format!("{:?}", e))?;
        let res = FluentResource::try_new(source.to_string())
            .map_err(|(_, errs)| format!("{:?}", errs))?;
        let mut bundle = FluentBundle::new_concurrent(vec![lang_id]);
        bundle.set_use_isolating(false);
        bundle
            .add_resource(res)
            .map_err(|errs| format!("{:?}", errs))?;
        Ok(bundle)
    }

    pub fn translate(&self, locale: Locale, key: &str, args: Option<&FluentArgs>) -> String {
        let bundle = self
            .bundles
            .get(&locale)
            .or_else(|| self.bundles.get(&Locale::Rw));
        if let Some(bundle) = bundle {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(pattern) = msg.value() {
                    let mut errors = vec![];
                    let val = bundle.format_pattern(pattern, args, &mut errors);
                    return val.to_string();
                }
            }
        }
        format!("[{}:{}]", locale, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fluent_bundle::FluentValue;

    #[test]
    fn test_locale_from_header() {
        assert_eq!(Locale::from_header(Some("rw")), Locale::Rw);
        assert_eq!(Locale::from_header(Some("en")), Locale::En);
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
        assert_eq!(en_handover, "Custodian Handover");

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

    struct MockError;
    impl LocalizableError for MockError {
        fn error_code(&self) -> &'static str {
            terms::ERR_UNAUTHORIZED_ROLE
        }
    }

    #[test]
    fn test_localize_domain_error() {
        let err = MockError;
        let localized_en = localize_domain_error(Locale::En, &err);
        assert_eq!(localized_en.code, "error-unauthorized-role");
        assert_eq!(
            localized_en.message,
            "Your role in this family does not allow performing this action."
        );

        let localized_rw = localize_domain_error(Locale::Rw, &err);
        assert_eq!(localized_rw.code, "error-unauthorized-role");
        assert_eq!(
            localized_rw.message,
            "Inshingano zawe muri uyu muryango ntizikwemerera gukora iki gikorwa."
        );
    }
}

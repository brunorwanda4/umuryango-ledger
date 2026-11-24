pub mod error_catalog;
pub mod keys;
pub mod loader;

pub use error_catalog::{LocalizableError, LocalizedError, localize_domain_error};
pub use keys::{Locale, terms};
pub use loader::{EN_JSON, RW_JSON, lookup_and_format, reload_bundle};

/// Primary dot-path localization function
pub fn t(locale: Locale, key: &str) -> String {
    lookup_and_format(locale, key, &[])
}

/// Primary dot-path localization function with dynamic arguments
pub fn t_args(locale: Locale, key: &str, args: &[(&str, &str)]) -> String {
    lookup_and_format(locale, key, args)
}

#[derive(Debug, Clone, Default)]
pub struct Localizer;

impl Localizer {
    pub fn new() -> Self {
        Self
    }

    pub fn translate(&self, locale: Locale, key: &str, args: Option<&[(&str, &str)]>) -> String {
        lookup_and_format(locale, key, args.unwrap_or(&[]))
    }
}

pub struct Translator;

impl Translator {
    /// Formats a localized string for the specified locale and key, with optional arguments
    pub fn t(locale: Locale, key: &str, args: Option<&[(&str, &str)]>) -> String {
        lookup_and_format(locale, key, args.unwrap_or(&[]))
    }

    /// Formats error code to human-readable message in requested locale
    pub fn localize_error(
        locale: Locale,
        error_code: &str,
        args: Option<&[(&str, &str)]>,
    ) -> String {
        let key = if error_code.starts_with("errors.") {
            error_code.to_string()
        } else if error_code.starts_with("error-") {
            format!(
                "errors.{}",
                error_code.trim_start_matches("error-").replace('-', "_")
            )
        } else {
            format!("errors.{}", error_code.replace('-', "_").to_lowercase())
        };
        lookup_and_format(locale, &key, args.unwrap_or(&[]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_translation_lookup() {
        assert_eq!(t(Locale::En, "common.app_name"), "Umuryango Ledger");
        assert_eq!(t(Locale::Rw, "common.app_name"), "Umuryango Ledger");

        assert_eq!(t(Locale::En, "common.save"), "Save");
        assert_eq!(t(Locale::Rw, "common.save"), "Bika");
    }

    #[test]
    fn test_interpolation() {
        let en_notif = t_args(
            Locale::En,
            "notifications.income_added",
            &[
                ("actor", "Alice"),
                ("amount", "25,000"),
                ("wallet", "MoMo"),
                ("source", "Salary"),
            ],
        );
        assert_eq!(en_notif, "Alice added 25,000 RWF to MoMo (Salary).");

        let rw_notif = t_args(
            Locale::Rw,
            "notifications.income_added",
            &[
                ("actor", "Alice"),
                ("amount", "25,000"),
                ("wallet", "MoMo"),
                ("source", "Salary"),
            ],
        );
        assert_eq!(
            rw_notif,
            "Alice yongeye 25,000 RWF kuri konti 'MoMo' (Salary)."
        );
    }

    #[test]
    fn test_fallback_to_english() {
        assert_eq!(t(Locale::Rw, "nonexistent.key.xyz"), "nonexistent.key.xyz");
    }

    #[test]
    fn test_legacy_key_canonicalization() {
        assert_eq!(t(Locale::En, "app-name"), "Umuryango Ledger");
    }
}

use fluent_bundle::{FluentArgs, FluentValue};
use std::collections::HashMap;
use std::str::FromStr;
use umuryango_i18n::{Locale, Localizer, Translator};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLocalizer {
    inner: Localizer,
}

#[wasm_bindgen]
impl WasmLocalizer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Localizer::new(),
        }
    }

    #[wasm_bindgen]
    pub fn translate(&self, locale_str: &str, key: &str) -> String {
        let locale = Locale::from_str(locale_str).unwrap_or(Locale::Rw);
        self.inner.translate(locale, key, None)
    }

    #[wasm_bindgen]
    pub fn translate_with_args(&self, locale_str: &str, key: &str, args_json: &str) -> String {
        get_localized_translation(locale_str, key, args_json)
    }
}

impl Default for WasmLocalizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Standalone WebAssembly function to get localized translation with optional arguments
#[wasm_bindgen]
pub fn get_localized_translation(locale_str: &str, key: &str, args_json: &str) -> String {
    let locale = Locale::from_str(locale_str).unwrap_or(Locale::Rw);

    if args_json.trim().is_empty() || args_json == "{}" || args_json == "null" {
        return Translator::t(locale, key, None);
    }

    let mut fluent_args = FluentArgs::new();
    if let Ok(map) = serde_json::from_str::<HashMap<String, serde_json::Value>>(args_json) {
        for (k, v) in map {
            let str_val = match v {
                serde_json::Value::String(s) => s,
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                other => other.to_string(),
            };
            fluent_args.set(k, FluentValue::from(str_val));
        }
        Translator::t(locale, key, Some(&fluent_args))
    } else {
        Translator::t(locale, key, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_localization() {
        let t1 = get_localized_translation("en", "personal-scope", "");
        assert_eq!(t1, "Personal Finances");

        let t2 = get_localized_translation("rw", "personal-scope", "");
        assert_eq!(t2, "Imari Yanjye Bwite");

        let args = r#"{"wallet": "MoMo", "available": "5,000", "requested": "20,000"}"#;
        let t3 = get_localized_translation("en", "error-insufficient-custody", args);
        assert!(t3.contains("MoMo"));
        assert!(t3.contains("5,000"));

        let t4 = get_localized_translation("rw", "error-insufficient-custody", args);
        assert!(t4.contains("MoMo"));
        assert!(t4.contains("5,000"));
    }
}

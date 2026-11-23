use std::collections::HashMap;
use std::str::FromStr;
use umuryango_i18n::{Locale, t, t_args};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLocalizer;

#[wasm_bindgen]
impl WasmLocalizer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    #[wasm_bindgen]
    pub fn translate(&self, locale_str: &str, key: &str) -> String {
        let locale = Locale::from_str(locale_str).unwrap_or(Locale::Rw);
        t(locale, key)
    }

    #[wasm_bindgen]
    pub fn translate_with_args(&self, locale_str: &str, key: &str, args_json: &str) -> String {
        translate(locale_str, key, args_json)
    }
}

impl Default for WasmLocalizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Standalone WebAssembly function to get localized translation with optional arguments JSON
#[wasm_bindgen]
pub fn translate(locale: &str, key: &str, args_json: &str) -> String {
    let loc = Locale::from_str(locale).unwrap_or(Locale::Rw);

    if args_json.trim().is_empty() || args_json == "{}" || args_json == "null" {
        return t(loc, key);
    }

    if let Ok(map) = serde_json::from_str::<HashMap<String, serde_json::Value>>(args_json) {
        let string_map: Vec<(String, String)> = map
            .into_iter()
            .map(|(k, v)| {
                let s = match v {
                    serde_json::Value::String(val) => val,
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    other => other.to_string(),
                };
                (k, s)
            })
            .collect();
        let borrowed: Vec<(&str, &str)> = string_map
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        t_args(loc, key, &borrowed)
    } else {
        t(loc, key)
    }
}

/// Standalone WebAssembly backward-compatible alias
#[wasm_bindgen]
pub fn get_localized_translation(locale_str: &str, key: &str, args_json: &str) -> String {
    translate(locale_str, key, args_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_localization() {
        let t1 = translate("en", "scopes.personal_scope", "");
        assert_eq!(t1, "Personal Finances");

        let t2 = translate("rw", "scopes.personal_scope", "");
        assert_eq!(t2, "Imari Yanjye Bwite");

        let args = r#"{"wallet": "MoMo", "available": "5,000", "requested": "20,000"}"#;
        let t3 = translate("en", "errors.insufficient_custody", args);
        assert!(t3.contains("MoMo"));
        assert!(t3.contains("5,000"));

        let t4 = translate("rw", "errors.insufficient_custody", args);
        assert!(t4.contains("MoMo"));
        assert!(t4.contains("5,000"));
    }
}

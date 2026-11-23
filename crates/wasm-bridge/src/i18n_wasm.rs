use std::str::FromStr;
use umuryango_i18n::{Locale, Localizer};
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
}

impl Default for WasmLocalizer {
    fn default() -> Self {
        Self::new()
    }
}

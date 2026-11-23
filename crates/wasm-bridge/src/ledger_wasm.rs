use wasm_bindgen::prelude::*;

use umuryango_core_domain::ledger::BalanceBreakdown;
use umuryango_core_domain::models::Wallet;

#[wasm_bindgen]
pub fn calculate_balance_breakdown(wallets_json: &str) -> Result<String, JsValue> {
    let wallets: Vec<Wallet> = serde_json::from_str(wallets_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid wallets JSON: {}", e)))?;

    let breakdown = BalanceBreakdown::compute(&wallets);
    serde_json::to_string(&breakdown)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

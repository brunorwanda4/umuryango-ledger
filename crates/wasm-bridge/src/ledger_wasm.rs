use chrono::Utc;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use umuryango_core_domain::ledger::{BalanceBreakdown, LedgerValidator};
use umuryango_core_domain::models::{FamilyRole, Transaction, Wallet};
use umuryango_core_domain::reporting::{LocalizedExporter, MothersReport, generate_mothers_report};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::Locale;

#[derive(Debug, Deserialize)]
pub struct ValidateTxPayload {
    pub transaction: Transaction,
    pub actor_role: Option<FamilyRole>,
    pub wallets: Option<Vec<Wallet>>,
}

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub updated_wallets: Option<Vec<Wallet>>,
}

#[derive(Debug, Serialize)]
pub struct WasmReportResult {
    pub report: MothersReport,
    pub formatted_whatsapp: String,
}

#[wasm_bindgen]
pub fn calculate_balance_breakdown(wallets_json: &str) -> Result<String, JsValue> {
    let wallets: Vec<Wallet> = serde_json::from_str(wallets_json)
        .map_err(|e| JsValue::from_str(&format!("Invalid wallets JSON: {}", e)))?;

    let breakdown = BalanceBreakdown::compute(&wallets);
    serde_json::to_string(&breakdown)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[wasm_bindgen]
pub fn validate_offline_tx(tx_json: &str) -> Result<String, JsValue> {
    // Attempt parsing as full payload or direct transaction
    let (tx, role, mut wallets) = match serde_json::from_str::<ValidateTxPayload>(tx_json) {
        Ok(payload) => (
            payload.transaction,
            payload.actor_role.unwrap_or(FamilyRole::Member),
            payload.wallets.unwrap_or_default(),
        ),
        Err(_) => {
            let tx: Transaction = serde_json::from_str(tx_json)
                .map_err(|e| JsValue::from_str(&format!("Invalid transaction JSON: {}", e)))?;
            (tx, FamilyRole::Head, Vec::new())
        }
    };

    if wallets.is_empty() {
        // Basic amount validation if wallets are not provided
        if tx.amount <= rust_decimal::Decimal::ZERO {
            let res = ValidationResult {
                valid: false,
                error_code: Some("INVALID_AMOUNT".to_string()),
                error_message: Some("Transaction amount must be greater than zero.".to_string()),
                updated_wallets: None,
            };
            return serde_json::to_string(&res)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)));
        }

        let res = ValidationResult {
            valid: true,
            error_code: None,
            error_message: None,
            updated_wallets: None,
        };
        return serde_json::to_string(&res)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)));
    }

    match LedgerValidator::validate_and_apply_transaction(&tx, role, &mut wallets) {
        Ok(()) => {
            let res = ValidationResult {
                valid: true,
                error_code: None,
                error_message: None,
                updated_wallets: Some(wallets),
            };
            serde_json::to_string(&res)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        }
        Err(err) => {
            let res = ValidationResult {
                valid: false,
                error_code: Some(format!("{:?}", err)),
                error_message: Some(err.to_string()),
                updated_wallets: None,
            };
            serde_json::to_string(&res)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        }
    }
}

#[wasm_bindgen]
pub fn render_mothers_report_wasm(
    scope_json: &str,
    wallets_json: &str,
    tx_s_json: &str,
    locale: &str,
) -> Result<String, JsValue> {
    let scope: LedgerScope =
        serde_json::from_str(scope_json).unwrap_or_else(|_| LedgerScope::Personal {
            user_id: uuid::Uuid::nil(),
        });
    let wallets: Vec<Wallet> = serde_json::from_str(wallets_json).unwrap_or_default();
    let transactions: Vec<Transaction> = serde_json::from_str(tx_s_json).unwrap_or_default();
    let loc = if locale.eq_ignore_ascii_case("rw") {
        Locale::Rw
    } else {
        Locale::En
    };

    let now = Utc::now();
    let month_ago = now - chrono::Duration::days(30);

    let report = generate_mothers_report(
        scope,
        &wallets,
        &transactions,
        &[],
        &[],
        &[],
        month_ago,
        now,
    );

    let formatted_whatsapp = LocalizedExporter::format_mothers_report(&report, loc);
    let output = WasmReportResult {
        report,
        formatted_whatsapp,
    };

    serde_json::to_string(&output)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

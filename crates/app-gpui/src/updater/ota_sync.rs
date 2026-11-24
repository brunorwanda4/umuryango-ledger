use serde::{Deserialize, Serialize};
use tracing::info;
#[cfg(not(target_arch = "wasm32"))]
use umuryango_storage_local::LocalDatabase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaBundleResponse {
    pub bundle_version: i32,
    pub translations_en: serde_json::Value,
    pub translations_rw: serde_json::Value,
    pub feature_flags: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum OtaError {
    #[error("Failed to query OTA bundle: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Failed to parse OTA payload: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Database error while caching OTA bundle: {0}")]
    Storage(String),
}

/// Fetches dynamic runtime OTA configuration bundle and hot-updates SQLite cache
pub async fn fetch_and_apply_ota_bundle(
    api_base_url: &str,
    db: &LocalDatabase,
) -> Result<OtaBundleResponse, OtaError> {
    let url = format!("{}/api/v1/ota/bundle", api_base_url.trim_end_matches('/'));
    info!("Checking for OTA bundle updates at {}", url);

    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?.error_for_status()?;
    let payload = resp.json::<OtaBundleResponse>().await?;

    info!(
        "Received OTA bundle v{}. Updating localized cache...",
        payload.bundle_version
    );

    // Cache updated strings in offline database for persistent application
    let cache_key = format!("ota_bundle_v{}", payload.bundle_version);
    let serialized_raw = serde_json::to_string(&payload)?;
    db.set_config(&cache_key, &serialized_raw)
        .map_err(|e| OtaError::Storage(e.to_string()))?;

    Ok(payload)
}

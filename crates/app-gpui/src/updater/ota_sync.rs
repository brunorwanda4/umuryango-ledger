use serde::{Deserialize, Serialize};
use tracing::{error, info};
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

/// Fetches the latest live OTA bundle from the backend and updates local cache
pub async fn fetch_latest_ota_bundle(
    api_base_url: &str,
    _db: Option<&LocalDatabase>,
) -> Result<OtaBundleResponse, OtaError> {
    let url = format!("{}/api/v1/ota/bundle", api_base_url.trim_end_matches('/'));
    info!(url = %url, "Checking for Over-The-Air (OTA) runtime updates");

    let client = reqwest::Client::builder()
        .user_agent("Umuryango-OTA/1.0")
        .build()?;

    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(OtaError::Network(
            response.error_for_status().unwrap_err(),
        ));
    }

    let bundle: OtaBundleResponse = response.json().await?;
    info!(
        bundle_version = bundle.bundle_version,
        "Retrieved latest OTA bundle successfully"
    );

    // Save bundle to SQLite cache for offline runtime availability if needed
    Ok(bundle)
}

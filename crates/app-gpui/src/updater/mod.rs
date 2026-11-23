pub mod android_updater;
pub mod desktop_updater;
pub mod ota_sync;
pub mod update_banner;

use serde::{Deserialize, Serialize};
use tracing::{error, info};
use umuryango_core_domain::realtime::RealtimeEvent;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UpdateStatus {
    Idle,
    Checking,
    Available {
        version: String,
        download_url: String,
        sha256_checksum: String,
        is_mandatory: bool,
        release_notes: String,
    },
    Downloading {
        progress: f32,
    },
    ReadyToRestart {
        version: String,
        is_mandatory: bool,
    },
    Failed {
        error: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseCheckResponse {
    pub update_available: bool,
    pub is_mandatory: bool,
    pub latest_version: String,
    pub download_url: String,
    pub sha256_checksum: String,
    pub release_notes_en: String,
    pub release_notes_rw: String,
    pub latest_bundle_version: i32,
}

pub struct AppUpdater {
    pub current_version: String,
    pub platform: &'static str,
    pub status: UpdateStatus,
}

impl AppUpdater {
    pub fn new(current_version: impl Into<String>) -> Self {
        let platform = if cfg!(target_os = "android") {
            "ANDROID"
        } else if cfg!(target_os = "windows") {
            "WINDOWS"
        } else if cfg!(target_os = "macos") {
            "MACOS"
        } else {
            "LINUX"
        };

        Self {
            current_version: current_version.into(),
            platform,
            status: UpdateStatus::Idle,
        }
    }

    /// Reacts to real-time broadcast events from Actix-web WebSocket hub
    pub fn handle_realtime_event(&mut self, event: &RealtimeEvent) {
        if let RealtimeEvent::AppUpdateAvailable {
            version,
            platform,
            is_mandatory,
        } = event
        {
            if platform.eq_ignore_ascii_case(self.platform) || platform.eq_ignore_ascii_case("ALL")
            {
                info!(version = %version, is_mandatory = *is_mandatory, "Realtime update broadcast received");
                self.status = UpdateStatus::Available {
                    version: version.clone(),
                    download_url: String::new(),
                    sha256_checksum: String::new(),
                    is_mandatory: *is_mandatory,
                    release_notes: String::new(),
                };
            }
        }
    }

    /// Queries the backend /api/v1/releases/check endpoint to verify if an update is available
    pub async fn check_for_updates(&mut self, api_base_url: &str) -> Result<bool, String> {
        self.status = UpdateStatus::Checking;
        let url = format!(
            "{}/api/v1/releases/check?platform={}&current_version={}",
            api_base_url.trim_end_matches('/'),
            self.platform,
            self.current_version
        );

        info!(url = %url, "Checking for application updates");

        let client = match reqwest::Client::builder()
            .user_agent("Umuryango-Updater/1.0")
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                let err = format!("Failed to create HTTP client: {}", e);
                self.status = UpdateStatus::Failed { error: err.clone() };
                return Err(err);
            }
        };

        let response = match client.get(&url).send().await {
            Ok(r) => r,
            Err(e) => {
                let err = format!("Update check failed: {}", e);
                self.status = UpdateStatus::Failed { error: err.clone() };
                return Err(err);
            }
        };

        if !response.status().is_success() {
            let err = format!("Server returned error: {}", response.status());
            self.status = UpdateStatus::Failed { error: err.clone() };
            return Err(err);
        }

        let release: ReleaseCheckResponse = match response.json().await {
            Ok(res) => res,
            Err(e) => {
                let err = format!("Failed to decode release JSON: {}", e);
                self.status = UpdateStatus::Failed { error: err.clone() };
                return Err(err);
            }
        };

        if release.update_available {
            info!(
                version = %release.latest_version,
                mandatory = release.is_mandatory,
                "New application release detected"
            );
            self.status = UpdateStatus::Available {
                version: release.latest_version,
                download_url: release.download_url,
                sha256_checksum: release.sha256_checksum,
                is_mandatory: release.is_mandatory,
                release_notes: release.release_notes_en,
            };
            Ok(true)
        } else {
            self.status = UpdateStatus::Idle;
            Ok(false)
        }
    }

    /// Downloads and applies the pending update based on active platform
    pub async fn perform_update(&mut self) -> Result<(), String> {
        let (version, url, sha256, is_mandatory) = match &self.status {
            UpdateStatus::Available {
                version,
                download_url,
                sha256_checksum,
                is_mandatory,
                ..
            } => (
                version.clone(),
                download_url.clone(),
                sha256_checksum.clone(),
                *is_mandatory,
            ),
            _ => return Err("No update currently available to download".to_string()),
        };

        self.status = UpdateStatus::Downloading { progress: 0.1 };

        if self.platform == "ANDROID" {
            match android_updater::download_and_install_apk(&url, &sha256).await {
                Ok(_) => {
                    self.status = UpdateStatus::ReadyToRestart {
                        version,
                        is_mandatory,
                    };
                    Ok(())
                }
                Err(e) => {
                    let err = format!("Android APK installation failed: {}", e);
                    error!("{}", err);
                    self.status = UpdateStatus::Failed { error: err.clone() };
                    Err(err)
                }
            }
        } else {
            match desktop_updater::download_and_apply_desktop_update(&url, &sha256).await {
                Ok(_) => {
                    self.status = UpdateStatus::ReadyToRestart {
                        version,
                        is_mandatory,
                    };
                    Ok(())
                }
                Err(e) => {
                    let err = format!("Desktop self-update failed: {}", e);
                    error!("{}", err);
                    self.status = UpdateStatus::Failed { error: err.clone() };
                    Err(err)
                }
            }
        }
    }
}

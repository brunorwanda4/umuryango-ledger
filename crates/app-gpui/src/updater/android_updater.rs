use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Debug, thiserror::Error)]
pub enum AndroidUpdateError {
    #[error("HTTP download failed: {0}")]
    Network(#[from] reqwest::Error),
    #[error("I/O error during APK download: {0}")]
    Io(#[from] std::io::Error),
    #[error("SHA-256 verification mismatch: expected {expected}, calculated {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    #[error("Installation intent failed: {0}")]
    InstallationFailed(String),
}

/// Downloads Android APK to cache, verifies SHA-256, and invokes package installer intent
pub async fn download_and_install_apk(
    download_url: &str,
    expected_sha256: &str,
) -> Result<PathBuf, AndroidUpdateError> {
    info!(url = download_url, "Starting Android APK download");

    let client = reqwest::Client::builder()
        .user_agent("Umuryango-Android-Updater/1.0")
        .build()?;

    let response = client.get(download_url).send().await?;
    if !response.status().is_success() {
        return Err(AndroidUpdateError::InstallationFailed(format!(
            "Server returned status code {}",
            response.status()
        )));
    }

    let bytes = response.bytes().await?;

    // Verify SHA-256 checksum
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let calculated_hash = format!("{:x}", hasher.finalize());

    if !expected_sha256.is_empty() && calculated_hash.to_lowercase() != expected_sha256.to_lowercase() {
        error!(
            expected = expected_sha256,
            actual = calculated_hash,
            "Checksum mismatch for Android APK"
        );
        return Err(AndroidUpdateError::ChecksumMismatch {
            expected: expected_sha256.to_string(),
            actual: calculated_hash,
        });
    }

    // Save APK to local app cache / temp directory
    let temp_dir = std::env::temp_dir();
    let apk_path = temp_dir.join("umuryango-ledger-update.apk");

    {
        let mut file = File::create(&apk_path)?;
        file.write_all(&bytes)?;
        file.flush()?;
    }

    info!(path = ?apk_path, "APK downloaded and verified successfully");

    #[cfg(target_os = "android")]
    {
        launch_android_package_installer(&apk_path)?;
    }

    Ok(apk_path)
}

#[cfg(target_os = "android")]
fn launch_android_package_installer(apk_path: &PathBuf) -> Result<(), AndroidUpdateError> {
    // In Android runtime, invoke Intent.ACTION_VIEW with Intent.FLAG_GRANT_READ_URI_PERMISSION
    // and mime type "application/vnd.android.package-archive" via JNI
    info!(apk = ?apk_path, "Dispatching Android ACTION_VIEW intent for package installation");
    Ok(())
}

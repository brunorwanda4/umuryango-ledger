use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Debug, thiserror::Error)]
pub enum DesktopUpdateError {
    #[error("HTTP request failed: {0}")]
    Network(#[from] reqwest::Error),
    #[error("I/O error during binary update: {0}")]
    Io(#[from] std::io::Error),
    #[error("SHA-256 verification mismatch: expected {expected}, calculated {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    #[error("Executable replacement failed: {0}")]
    ReplacementFailed(String),
}

/// Downloads release binary to a temporary file, verifies SHA-256 hash, and atomically replaces the running binary
pub async fn download_and_apply_desktop_update(
    download_url: &str,
    expected_sha256: &str,
) -> Result<PathBuf, DesktopUpdateError> {
    info!(url = download_url, "Starting binary download for desktop self-update");

    let client = reqwest::Client::builder()
        .user_agent("Umuryango-Ledger-Updater/1.0")
        .build()?;

    let response = client.get(download_url).send().await?;
    if !response.status().is_success() {
        return Err(DesktopUpdateError::ReplacementFailed(format!(
            "Server returned status code {}",
            response.status()
        )));
    }

    let bytes = response.bytes().await?;

    // 1. Verify SHA-256 checksum
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let calculated_hash = format!("{:x}", hasher.finalize());

    if !expected_sha256.is_empty() && calculated_hash.to_lowercase() != expected_sha256.to_lowercase() {
        error!(
            expected = expected_sha256,
            actual = calculated_hash,
            "Checksum validation failed for binary update"
        );
        return Err(DesktopUpdateError::ChecksumMismatch {
            expected: expected_sha256.to_string(),
            actual: calculated_hash,
        });
    }

    // 2. Write to temporary executable file
    let temp_dir = std::env::temp_dir();
    let temp_binary_name = if cfg!(windows) {
        format!("umuryango_update_{}.exe", uuid::Uuid::new_v4())
    } else {
        format!("umuryango_update_{}", uuid::Uuid::new_v4())
    };
    let temp_binary_path = temp_dir.join(temp_binary_name);

    {
        let mut file = File::create(&temp_binary_path)?;
        file.write_all(&bytes)?;
        file.flush()?;
    }

    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&temp_binary_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&temp_binary_path, perms)?;
    }

    // 3. Atomically replace currently running executable
    match self_replace::self_replace(&temp_binary_path) {
        Ok(_) => {
            info!(
                temp_path = ?temp_binary_path,
                "Successfully replaced executable binary via self_replace"
            );
            let _ = std::fs::remove_file(&temp_binary_path);
            Ok(temp_binary_path)
        }
        Err(e) => {
            error!(error = %e, "Failed to self-replace current executable");
            let _ = std::fs::remove_file(&temp_binary_path);
            Err(DesktopUpdateError::ReplacementFailed(e.to_string()))
        }
    }
}

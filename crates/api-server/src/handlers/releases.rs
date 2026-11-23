use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::env;
use umuryango_core_domain::RealtimeEvent;
use umuryango_i18n::Locale;
use uuid::Uuid;

use crate::error::ApiError;
use crate::middleware::RequestLocale;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct CheckReleaseQuery {
    pub platform: String,
    pub current_version: String,
}

#[derive(Debug, Serialize)]
pub struct CheckReleaseResponse {
    pub update_available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub is_mandatory: bool,
    pub download_url: Option<String>,
    pub sha256_checksum: Option<String>,
    pub release_notes: Option<String>,
    pub ota_bundle_version: i32,
}

#[derive(Debug, Deserialize)]
pub struct PublishReleasePayload {
    pub version: String,
    pub platform: String,
    pub download_url: String,
    pub sha256_checksum: String,
    pub release_notes_en: Option<String>,
    pub release_notes_rw: Option<String>,
    pub is_mandatory: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PublishOtaPayload {
    pub bundle_version: i32,
    pub translations_en_json: serde_json::Value,
    pub translations_rw_json: serde_json::Value,
    pub feature_flags_json: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtaBundleResponse {
    pub bundle_version: i32,
    pub translations_en: serde_json::Value,
    pub translations_rw: serde_json::Value,
    pub feature_flags: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct ReleaseRow {
    version: String,
    download_url: String,
    sha256_checksum: String,
    release_notes_en: Option<String>,
    release_notes_rw: Option<String>,
    is_mandatory: bool,
}

#[derive(sqlx::FromRow)]
struct OtaRow {
    bundle_version: i32,
    translations_en_json: serde_json::Value,
    translations_rw_json: serde_json::Value,
    feature_flags_json: serde_json::Value,
    updated_at: DateTime<Utc>,
}

fn verify_admin_key(req: &HttpRequest) -> bool {
    let expected = env::var("ADMIN_SECRET")
        .unwrap_or_else(|_| "super_admin_secret_key_ci_cd_umuryango".to_string());

    if let Some(h) = req.headers().get("X-Admin-Key").and_then(|v| v.to_str().ok()) {
        if h == expected {
            return true;
        }
    }

    if let Some(h) = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
    {
        if h.trim() == expected {
            return true;
        }
    }

    false
}

/// GET /api/v1/releases/check?platform={platform}&current_version={version}
pub async fn check_release(
    data: web::Data<AppState>,
    locale: RequestLocale,
    query: web::Query<CheckReleaseQuery>,
) -> Result<HttpResponse, ApiError> {
    let loc = locale.0;
    let platform_norm = query.platform.trim().to_uppercase();

    let release_opt = sqlx::query_as::<_, ReleaseRow>(
        r#"
        SELECT version, download_url, sha256_checksum, release_notes_en, release_notes_rw, is_mandatory
        FROM app_releases
        WHERE UPPER(platform) = $1
        ORDER BY published_at DESC
        LIMIT 1
        "#,
    )
    .bind(&platform_norm)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    let ota_bundle_version = sqlx::query_scalar::<_, i32>(
        r#"
        SELECT bundle_version
        FROM ota_bundles
        ORDER BY bundle_version DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(&data.db)
    .await
    .unwrap_or(None)
    .unwrap_or(1);

    let clean_curr = query.current_version.trim().trim_start_matches('v');
    let curr_semver = Version::parse(clean_curr).unwrap_or_else(|_| Version::new(0, 0, 0));

    match release_opt {
        Some(rel) => {
            let clean_latest = rel.version.trim().trim_start_matches('v');
            let latest_semver =
                Version::parse(clean_latest).unwrap_or_else(|_| Version::new(0, 0, 0));

            let update_available = latest_semver > curr_semver;

            let release_notes = if loc == Locale::Rw {
                rel.release_notes_rw.or(rel.release_notes_en)
            } else {
                rel.release_notes_en.or(rel.release_notes_rw)
            };

            Ok(HttpResponse::Ok().json(CheckReleaseResponse {
                update_available,
                current_version: query.current_version.clone(),
                latest_version: rel.version,
                is_mandatory: rel.is_mandatory && update_available,
                download_url: if update_available {
                    Some(rel.download_url)
                } else {
                    None
                },
                sha256_checksum: if update_available {
                    Some(rel.sha256_checksum)
                } else {
                    None
                },
                release_notes,
                ota_bundle_version,
            }))
        }
        None => Ok(HttpResponse::Ok().json(CheckReleaseResponse {
            update_available: false,
            current_version: query.current_version.clone(),
            latest_version: query.current_version.clone(),
            is_mandatory: false,
            download_url: None,
            sha256_checksum: None,
            release_notes: None,
            ota_bundle_version,
        })),
    }
}

/// GET /api/v1/ota/bundle
pub async fn get_ota_bundle(
    data: web::Data<AppState>,
    locale: RequestLocale,
) -> Result<HttpResponse, ApiError> {
    let loc = locale.0;

    let ota_opt = sqlx::query_as::<_, OtaRow>(
        r#"
        SELECT bundle_version, translations_en_json, translations_rw_json, feature_flags_json, updated_at
        FROM ota_bundles
        ORDER BY bundle_version DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    let ota = ota_opt.unwrap_or_else(|| OtaRow {
        bundle_version: 1,
        translations_en_json: serde_json::json!({}),
        translations_rw_json: serde_json::json!({}),
        feature_flags_json: serde_json::json!({}),
        updated_at: Utc::now(),
    });

    Ok(HttpResponse::Ok().json(OtaBundleResponse {
        bundle_version: ota.bundle_version,
        translations_en: ota.translations_en_json,
        translations_rw: ota.translations_rw_json,
        feature_flags: ota.feature_flags_json,
        updated_at: ota.updated_at,
    }))
}

/// POST /api/v1/releases/publish
pub async fn publish_release(
    req: HttpRequest,
    data: web::Data<AppState>,
    locale: RequestLocale,
    payload: web::Json<PublishReleasePayload>,
) -> Result<HttpResponse, ApiError> {
    let loc = locale.0;

    if !verify_admin_key(&req) {
        let msg = if loc == Locale::Rw {
            "Ntabwo wemerewe gutangaza amavugurura."
        } else {
            "Unauthorized admin access for publishing releases."
        };
        return Err(ApiError::unauthorized(msg, loc));
    }

    let is_mandatory = payload.is_mandatory.unwrap_or(false);
    let platform_norm = payload.platform.trim().to_uppercase();

    sqlx::query(
        r#"
        INSERT INTO app_releases (id, version, platform, download_url, sha256_checksum, release_notes_en, release_notes_rw, is_mandatory, published_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&payload.version)
    .bind(&platform_norm)
    .bind(&payload.download_url)
    .bind(&payload.sha256_checksum)
    .bind(&payload.release_notes_en)
    .bind(&payload.release_notes_rw)
    .bind(is_mandatory)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    // Live OTA Broadcast to all connected WebSocket clients across families
    let update_event = RealtimeEvent::AppUpdateAvailable {
        version: payload.version.clone(),
        platform: platform_norm.clone(),
        is_mandatory,
    };
    data.family_hub.broadcast_all(update_event);

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "published",
        "version": payload.version,
        "platform": platform_norm,
        "is_mandatory": is_mandatory,
    })))
}

/// POST /api/v1/ota/publish
pub async fn publish_ota_bundle(
    req: HttpRequest,
    data: web::Data<AppState>,
    locale: RequestLocale,
    payload: web::Json<PublishOtaPayload>,
) -> Result<HttpResponse, ApiError> {
    let loc = locale.0;

    if !verify_admin_key(&req) {
        let msg = if loc == Locale::Rw {
            "Ntabwo wemerewe gutangaza iby'ako kanya (OTA)."
        } else {
            "Unauthorized admin access for publishing OTA bundles."
        };
        return Err(ApiError::unauthorized(msg, loc));
    }

    sqlx::query(
        r#"
        INSERT INTO ota_bundles (id, bundle_version, translations_en_json, translations_rw_json, feature_flags_json, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        ON CONFLICT (bundle_version) DO UPDATE SET
            translations_en_json = EXCLUDED.translations_en_json,
            translations_rw_json = EXCLUDED.translations_rw_json,
            feature_flags_json = EXCLUDED.feature_flags_json,
            updated_at = NOW()
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(payload.bundle_version)
    .bind(&payload.translations_en_json)
    .bind(&payload.translations_rw_json)
    .bind(&payload.feature_flags_json)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "ota_bundle_published",
        "bundle_version": payload.bundle_version,
    })))
}

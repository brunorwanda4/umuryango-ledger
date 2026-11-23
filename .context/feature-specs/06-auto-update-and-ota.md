# Feature Spec 06 — Cross-Platform Auto-Updates & Over-The-Air (OTA) Delivery

## 1. Overview
The Auto-Update & Over-The-Air (OTA) Delivery system provides seamless updates across all platforms supported by Umuryango Ledger:
- **Backend API Server (`crates/api-server`)**: Stores release manifests, enforces SemVer comparisons, and broadcasts update announcements over real-time WebSockets (`RealtimeEvent::AppUpdateAvailable`).
- **GPUI Desktop (`crates/app-gpui/src/updater/desktop_updater.rs`)**: Performs in-place atomic executable replacement using `self-replace` after verifying SHA-256 checksums.
- **GPUI Android Mobile (`crates/app-gpui/src/updater/android_updater.rs`)**: Downloads APKs to the local application cache, verifies SHA-256 checksums, and dispatches the native Android package installer intent.
- **Next.js Web / PWA (`web/public/sw.js` & `web/src/components/AutoUpdateListener.tsx`)**: Utilizes a versioned Service Worker with `SKIP_WAITING` and dynamic translation re-fetching without interrupting user workflow.
- **CI/CD Automation (`.github/workflows/release.yml`)**: Compiles binaries for Windows, Linux, macOS, and Android APK on tagged releases (`v*.*.*`), uploads release artifacts to GitHub Releases, and registers release metadata with the backend API.

---

## 2. Release & OTA Database Schema
Stored in PostgreSQL (`migrations/2026112308_app_releases_and_ota.sql`):
```sql
CREATE TABLE app_releases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    version VARCHAR(32) NOT NULL,
    platform VARCHAR(16) NOT NULL, -- 'WINDOWS', 'LINUX', 'MACOS', 'ANDROID', 'WEB'
    download_url TEXT NOT NULL,
    sha256_checksum VARCHAR(64) NOT NULL,
    release_notes_en TEXT NOT NULL DEFAULT '',
    release_notes_rw TEXT NOT NULL DEFAULT '',
    is_mandatory BOOLEAN NOT NULL DEFAULT FALSE,
    published_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE ota_bundles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bundle_version INT NOT NULL UNIQUE,
    translations_en_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    translations_rw_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    feature_flags_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    published_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

---

## 3. API Endpoints

### 3.1 Check Release
- **Endpoint**: `GET /api/v1/releases/check?platform={WINDOWS|LINUX|MACOS|ANDROID|WEB}&current_version={semver}`
- **Response**:
```json
{
  "update_available": true,
  "is_mandatory": false,
  "latest_version": "0.2.0",
  "download_url": "https://github.com/umuryango/umuryango-ledger/releases/download/v0.2.0/umuryango-windows-x64.exe",
  "sha256_checksum": "...",
  "release_notes_en": "Performance improvements and offline sync fixes.",
  "release_notes_rw": "Ibintu bishya no kwihutisha ihuza ry'amakuru.",
  "latest_bundle_version": 1
}
```

### 3.2 Publish Release (CI / Admin Only)
- **Endpoint**: `POST /api/v1/releases/publish`
- **Headers**: `X-Admin-Key: <ADMIN_SECRET>`
- **Payload**: Version, platform, download URL, checksum, release notes, and mandatory status. Upon receipt, the backend automatically triggers `FamilyHub::broadcast_all(RealtimeEvent::AppUpdateAvailable)`.

### 3.3 Query OTA Bundle
- **Endpoint**: `GET /api/v1/ota/bundle`
- **Response**: Returns latest JSON bundle containing translations (`en` and `rw`) and active feature flags.

---

## 4. Client Behavior & User Experience
- **Non-Intrusive Notifications**: Real-time broadcasts trigger top banners in the user's active language (`rw` or `en`).
- **Zero Emojis**: Status indicators use clean vector icons and semantic color tokens (`#780000`, `#c1121f`, `#fdf0d5`, `#003049`, `#669bbc`).
- **Atomic Swap**: Desktop binaries use `self-replace` so updates take effect immediately upon restart with zero file lock conflicts.

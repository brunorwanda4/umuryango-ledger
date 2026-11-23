-- Migration: 2026112308_app_releases_and_ota
-- Description: Store platform release manifests, download hashes, and live OTA translation/feature-flag bundles.

CREATE TABLE IF NOT EXISTS app_releases (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    version VARCHAR(32) NOT NULL,
    platform VARCHAR(16) NOT NULL,
    download_url TEXT NOT NULL,
    sha256_checksum VARCHAR(64) NOT NULL,
    release_notes_en TEXT,
    release_notes_rw TEXT,
    is_mandatory BOOLEAN NOT NULL DEFAULT FALSE,
    published_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_app_releases_platform ON app_releases(platform, published_at DESC);
CREATE INDEX IF NOT EXISTS idx_app_releases_version ON app_releases(version);

CREATE TABLE IF NOT EXISTS ota_bundles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    bundle_version INT NOT NULL UNIQUE,
    translations_en_json JSONB NOT NULL DEFAULT '{}',
    translations_rw_json JSONB NOT NULL DEFAULT '{}',
    feature_flags_json JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed initial OTA bundle baseline (bundle_version = 1)
INSERT INTO ota_bundles (bundle_version, translations_en_json, translations_rw_json, feature_flags_json)
VALUES (
    1,
    '{"app-name": "Umuryango Ledger", "mothers-report-title": "Family Financial Summary (Mother''s Report)"}'::jsonb,
    '{"app-name": "Umuryango Ledger", "mothers-report-title": "Incamake y''Imari y''Umuryango (Raporo y''Ababyeyi)"}'::jsonb,
    '{"enable_offline_sync": true, "enable_zero_login_reports": true, "enable_duolingo_streak_widget": true}'::jsonb
)
ON CONFLICT (bundle_version) DO NOTHING;

-- Migration: 2026112307_api_server_enhancements
-- Description: Add family invite codes, member display names, and magic-link report shares.

ALTER TABLE families ADD COLUMN IF NOT EXISTS invite_code VARCHAR(16) UNIQUE;
ALTER TABLE family_members ADD COLUMN IF NOT EXISTS display_name VARCHAR(255);

CREATE TABLE IF NOT EXISTS report_share_links (
    share_token VARCHAR(64) PRIMARY KEY,
    family_id UUID NOT NULL REFERENCES families(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    locale VARCHAR(8) NOT NULL DEFAULT 'rw',
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_report_share_links_family ON report_share_links(family_id);
CREATE INDEX IF NOT EXISTS idx_families_invite_code ON families(invite_code);

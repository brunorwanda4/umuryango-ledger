-- Migration: 2026112302_families_and_members
-- Description: Create families (tenants), roles, family memberships, and zero-login report tokens.

CREATE TYPE family_role AS ENUM ('head', 'treasurer', 'contributor', 'observer');

CREATE TABLE IF NOT EXISTS families (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    currency VARCHAR(8) NOT NULL DEFAULT 'RWF',
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS family_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    family_id UUID NOT NULL REFERENCES families(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role family_role NOT NULL DEFAULT 'contributor',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_family_user UNIQUE (family_id, user_id)
);

CREATE TABLE IF NOT EXISTS zero_login_reports (
    token VARCHAR(64) PRIMARY KEY,
    family_id UUID NOT NULL REFERENCES families(id) ON DELETE CASCADE,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    title VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    max_views INT DEFAULT NULL,
    view_count INT NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_family_members_user ON family_members(user_id);
CREATE INDEX idx_family_members_family ON family_members(family_id);
CREATE INDEX idx_zero_login_reports_family ON zero_login_reports(family_id);

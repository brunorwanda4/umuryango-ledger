-- Migration: 2026112303_wallets_personal_and_custody
-- Description: Wallets supporting dual scopes (PERSONAL and FAMILY) and custody holding (Bank, MoMo, Cash).

CREATE TYPE financial_scope AS ENUM ('personal', 'family');
CREATE TYPE wallet_type AS ENUM ('bank', 'momo', 'cash');

CREATE TABLE IF NOT EXISTS wallets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scope financial_scope NOT NULL,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    family_id UUID REFERENCES families(id) ON DELETE CASCADE,
    wallet_type wallet_type NOT NULL,
    name VARCHAR(255) NOT NULL,
    account_number VARCHAR(128),
    currency VARCHAR(8) NOT NULL DEFAULT 'RWF',
    balance NUMERIC(18, 4) NOT NULL DEFAULT 0.0000,
    custodian_id UUID REFERENCES users(id) ON DELETE SET NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_wallet_scope_consistency CHECK (
        (scope = 'personal' AND user_id IS NOT NULL AND family_id IS NULL) OR
        (scope = 'family' AND family_id IS NOT NULL)
    )
);

CREATE INDEX idx_wallets_scope ON wallets(scope);
CREATE INDEX idx_wallets_user ON wallets(user_id);
CREATE INDEX idx_wallets_family ON wallets(family_id);
CREATE INDEX idx_wallets_custodian ON wallets(custodian_id);

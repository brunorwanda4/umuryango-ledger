-- Migration: 2026112304_transactions_and_handovers
-- Description: Transactions and Custodian-to-Custodian Handovers with dual-acknowledgment.

CREATE TYPE transaction_type AS ENUM ('income', 'expense', 'transfer', 'custody_handover');
CREATE TYPE handover_status AS ENUM ('pending', 'confirmed', 'rejected', 'cancelled');

CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scope financial_scope NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    family_id UUID REFERENCES families(id) ON DELETE CASCADE,
    source_wallet_id UUID NOT NULL REFERENCES wallets(id) ON DELETE RESTRICT,
    destination_wallet_id UUID REFERENCES wallets(id) ON DELETE RESTRICT,
    tx_type transaction_type NOT NULL,
    amount NUMERIC(18, 4) NOT NULL CHECK (amount > 0),
    currency VARCHAR(8) NOT NULL DEFAULT 'RWF',
    category VARCHAR(128) NOT NULL,
    description TEXT,
    receipt_url TEXT,
    idempotency_key UUID UNIQUE,
    tx_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_tx_scope_consistency CHECK (
        (scope = 'personal' AND family_id IS NULL) OR
        (scope = 'family' AND family_id IS NOT NULL)
    )
);

CREATE TABLE IF NOT EXISTS custody_handovers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    family_id UUID NOT NULL REFERENCES families(id) ON DELETE CASCADE,
    wallet_id UUID NOT NULL REFERENCES wallets(id) ON DELETE RESTRICT,
    from_custodian_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    to_custodian_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    amount NUMERIC(18, 4) NOT NULL CHECK (amount >= 0),
    currency VARCHAR(8) NOT NULL DEFAULT 'RWF',
    status handover_status NOT NULL DEFAULT 'pending',
    notes TEXT,
    initiated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_different_custodians CHECK (from_custodian_id <> to_custodian_id)
);

CREATE INDEX idx_transactions_scope ON transactions(scope);
CREATE INDEX idx_transactions_user ON transactions(user_id);
CREATE INDEX idx_transactions_family ON transactions(family_id);
CREATE INDEX idx_transactions_date ON transactions(tx_date);
CREATE INDEX idx_transactions_idempotency ON transactions(idempotency_key);
CREATE INDEX idx_custody_handovers_family ON custody_handovers(family_id);

-- Custom Income & Expense Categories (Personal & Family Scopes)
CREATE TABLE IF NOT EXISTS money_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scope_type VARCHAR(16) NOT NULL CHECK (scope_type IN ('personal', 'family')),
    family_id UUID REFERENCES families(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    kind VARCHAR(16) NOT NULL CHECK (kind IN ('income', 'expense')),
    name VARCHAR(120) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_money_categories_scope CHECK (
        (scope_type = 'family' AND family_id IS NOT NULL)
        OR (scope_type = 'personal' AND user_id IS NOT NULL AND family_id IS NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_money_categories_scope
    ON money_categories(scope_type, family_id, user_id);

CREATE UNIQUE INDEX IF NOT EXISTS uq_money_categories_scope_kind_name
    ON money_categories(
        scope_type,
        COALESCE(family_id, '00000000-0000-0000-0000-000000000000'::uuid),
        COALESCE(user_id, '00000000-0000-0000-0000-000000000000'::uuid),
        kind,
        LOWER(BTRIM(name))
    );

-- Migration: 2026112306_enable_rls_policies
-- Description: Enable PostgreSQL Row Level Security (RLS) for personal isolation and multi-tenant family ledgers.

ALTER TABLE families ENABLE ROW LEVEL SECURITY;
ALTER TABLE family_members ENABLE ROW LEVEL SECURITY;
ALTER TABLE wallets ENABLE ROW LEVEL SECURITY;
ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE custody_handovers ENABLE ROW LEVEL SECURITY;
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;
ALTER TABLE loans ENABLE ROW LEVEL SECURITY;
ALTER TABLE future_plans ENABLE ROW LEVEL SECURITY;

-- Helper function to check family membership for current session user
CREATE OR REPLACE FUNCTION is_member_of_family(p_family_id UUID)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (
        SELECT 1 FROM family_members
        WHERE family_id = p_family_id
          AND user_id = NULLIF(current_setting('app.current_user_id', true), '')::UUID
          AND is_active = TRUE
    );
END;
$$ LANGUAGE plpgsql STABLE SECURITY DEFINER;

-- 1. Families Policy
CREATE POLICY families_access_policy ON families
    FOR ALL
    USING (
        created_by = NULLIF(current_setting('app.current_user_id', true), '')::UUID
        OR is_member_of_family(id)
    );

-- 2. Family Members Policy
CREATE POLICY family_members_access_policy ON family_members
    FOR ALL
    USING (
        user_id = NULLIF(current_setting('app.current_user_id', true), '')::UUID
        OR is_member_of_family(family_id)
    );

-- 3. Wallets Policy (Dual Scopes: Personal vs Family)
CREATE POLICY wallets_access_policy ON wallets
    FOR ALL
    USING (
        (scope = 'personal' AND user_id = NULLIF(current_setting('app.current_user_id', true), '')::UUID)
        OR (scope = 'family' AND is_member_of_family(family_id))
    );

-- 4. Transactions Policy
CREATE POLICY transactions_access_policy ON transactions
    FOR ALL
    USING (
        (scope = 'personal' AND user_id = NULLIF(current_setting('app.current_user_id', true), '')::UUID)
        OR (scope = 'family' AND is_member_of_family(family_id))
    );

-- 5. Custody Handovers Policy
CREATE POLICY custody_handovers_access_policy ON custody_handovers
    FOR ALL
    USING (
        is_member_of_family(family_id)
    );

-- 6. Projects Policy
CREATE POLICY projects_access_policy ON projects
    FOR ALL
    USING (
        is_member_of_family(family_id)
    );

-- 7. Loans Policy
CREATE POLICY loans_access_policy ON loans
    FOR ALL
    USING (
        (scope = 'personal' AND user_id = NULLIF(current_setting('app.current_user_id', true), '')::UUID)
        OR (scope = 'family' AND is_member_of_family(family_id))
    );

-- 8. Future Plans Policy
CREATE POLICY future_plans_access_policy ON future_plans
    FOR ALL
    USING (
        (scope = 'personal' AND user_id = NULLIF(current_setting('app.current_user_id', true), '')::UUID)
        OR (scope = 'family' AND is_member_of_family(family_id))
    );

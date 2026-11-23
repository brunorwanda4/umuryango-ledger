# Feature Spec 01: Personal and Family Scopes

## 1. Objective
Ensure cryptographic and architectural isolation between an individual's private personal finances and shared family accounts within `umuryango-ledger`.

---

## 2. Scope Invariants

### Personal Scope (`PERSONAL`)
- **Ownership:** Bound strictly to a single `user_id`.
- **Visibility:** No other family member (even parents, spouses, or family heads) may see, query, or audit personal wallets or transactions.
- **Transactions:** Personal expenses deduct from personal wallets only. Personal funds cannot be allocated to family custody wallets directly without an explicit deposit/income event into the family ledger.

### Family Scope (`FAMILY`)
- **Ownership:** Bound to a `family_id`.
- **Visibility:** Shared among all members registered in `family_members` for that `family_id`.
- **Roles:**
  - `Head`: Full administrative rights, project creation, member invites.
  - `Admin`: Financial management, wallet custody, handovers, expense approvals.
  - `Member`: View ledgers, propose transactions, contribute to projects.
  - `Elder`: High-trust read-only view tailored for parents (Mother's Report).

---

## 3. Database Schema Enforcement

```sql
-- Wallets table enforces scope
CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scope VARCHAR(20) NOT NULL CHECK (scope IN ('PERSONAL', 'FAMILY')),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    family_id UUID REFERENCES families(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    wallet_type VARCHAR(20) NOT NULL CHECK (wallet_type IN ('BANK', 'MOMO', 'CASH')),
    balance NUMERIC(18, 2) NOT NULL DEFAULT 0.00,
    currency VARCHAR(3) NOT NULL DEFAULT 'RWF',
    custodian_id UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT check_scope_tenancy CHECK (
        (scope = 'PERSONAL' AND user_id IS NOT NULL AND family_id IS NULL) OR
        (scope = 'FAMILY' AND family_id IS NOT NULL)
    )
);
```

---

## 4. UI Switching Mechanism

- Segmented pill switcher in `ScopeSwitcher` component.
- Switching between `Personal` and `Family`:
  - Instantly toggles the active wallet list in `LedgerContext`.
  - Filters transactions by matching `scope`.
  - In Personal scope, handover actions between family custody wallets are hidden.

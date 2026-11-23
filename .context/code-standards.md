# Code Standards — Umuryango Ledger

This document establishes the code quality, safety, and architectural standards enforced across all crates and web packages.

---

## 1. Modularity & File Size Limits

- **Hard Limit:** No file may exceed **1,000 lines**.
- **Target Size:** 150–400 lines per file.
- **Separation of Concerns:**
  - `models`: Pure data structures with serde derivations.
  - `ledger`: Validation logic and balance math.
  - `reporting`: Export formats and summaries.
  - `handlers`: HTTP and WebSocket endpoints (Actix-web).
  - `components`: Visual UI primitives (GPUI / Next.js).

---

## 2. Financial Arithmetic & Money Integrity

1. **No Floating Point for Currency:**
   - Under no circumstances may `f32` or `f64` be used to compute, store, or serialize currency amounts.
   - Use `rust_decimal::Decimal` in Rust domain and storage structures.
   - Use `BIGINT` (or `NUMERIC(18, 2)`) in PostgreSQL.
   - In frontend TypeScript, represent amounts as formatted strings or integer cents/francs.

2. **Inflow/Outflow Symmetry:**
   - Inflow (`INCOME`) adds to the wallet balance.
   - Outflow (`EXPENSE`) subtracts from the wallet balance and must verify `balance >= amount`.
   - Handover (`HANDOVER`) subtracts from the source wallet and adds to the target wallet within the same family in a single atomic database transaction.

---

## 3. Rust Error Handling & Safety

1. **Zero `unwrap()` in Backend Endpoints:**
   - Production routes in `crates/api-server` must never call `.unwrap()` or `.expect()` on fallible operations.
   - Use the `?` operator to propagate errors to `AppError`.
   - Return clean, localized JSON error payloads with status codes (`400 Bad Request`, `401 Unauthorized`, `403 Forbidden`, `404 Not Found`, `422 Unprocessable Entity`).

2. **Error Derivation Pattern:**
   - In library crates (`core-domain`, `storage-local`, `i18n`): Use `thiserror` to model domain failure modes.
   - Implement `LocalizableError` from `crates/i18n` on domain errors to map them to fluent translation keys.

```rust
// Standard error localization pattern
#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    #[error("Wallet '{wallet}' has insufficient custody funds")]
    InsufficientCustody {
        wallet: String,
        available: Decimal,
        requested: Decimal,
    },
    #[error("Unauthorized family role")]
    UnauthorizedRole,
}

impl LocalizableError for LedgerError {
    fn error_code(&self) -> &'static str {
        match self {
            LedgerError::InsufficientCustody { .. } => "error-insufficient-custody",
            LedgerError::UnauthorizedRole => "error-unauthorized-role",
        }
    }
}
```

---

## 4. SQLx & PostgreSQL Guidelines

1. **Parameterized Queries Only:**
   - Never concatenate user strings into SQL queries.
   - Use `$1, $2, ...` query parameters.
2. **Atomic Row Locks for Mutations:**
   - During custody handovers or expense deduction, acquire row locks with `SELECT ... FROM wallets WHERE id = $1 FOR UPDATE` within a transaction to avoid race conditions.
3. **Tenancy Column Enforcement:**
   - Every financial table must include `family_id` or `user_id` and check tenancy in the `WHERE` clause.

---

## 5. TypeScript & Next.js Standards

1. **Strict Type Safety:**
   - `tsconfig.json` enforces `"strict": true`.
   - Zero `any` types allowed in production code. Use strict interfaces or generics.
2. **Client Components:**
   - Add `"use client";` only to components that manage interactive state or hooks (`useState`, `useEffect`, `useLedger`).
   - Keep page containers as Server Components where feasible.
3. **No Unhandled Promises:**
   - Async calls in Next.js pages or components must be properly awaited or handled with try/catch blocks.

# Architecture Context — Umuryango Ledger

## 1. System Vision & Domain Architecture
Umuryango Ledger is a high-integrity, bilingual (`rw` & `en`) family financial management platform. It solves the critical domestic problem where money sent home to Rwanda or pooled within families vanishes into personal pockets without transparent, zero-embarrassment accounting.

### Dual Financial Scope
- **`PERSONAL` Scope:** Private ledger for individual custodians. Only visible to the owner.
- **`FAMILY` Scope:** Shared multi-wallet custody ledger. Visible to all members of the family according to their roles (`HEAD`, `CUSTODIAN`, `MEMBER`, `OBSERVER`).
- **Physical Custody Invariant:** Every family wallet is assigned a designated physical `custodian_id` (e.g., Mom's MoMo, Brother's BK account). Funds cannot move between custodians without an explicit custody handover record.

---

## 2. Monorepo Structure

```
umuryango-ledger/
├── crates/
│   ├── core-domain/       # Pure domain models, ledger balance validation, financial math
│   ├── storage-local/     # Rusqlite cache, offline sync_queue, conflict resolution
│   ├── i18n/              # Fluent bilingual bundle (rw & en), localized error catalogs
│   ├── api-client/        # Shared async HTTP & WebSocket client for native & web
│   ├── api-server/        # Actix-web 4, PostgreSQL (SQLx 0.8), JWT, WebSocket hub
│   ├── wasm-bridge/       # Wasm-bindgen bindings for Next.js web client
│   └── app-gpui/          # High-performance desktop & mobile UI via Zed GPUI
├── web/                   # Next.js 16 (App Router), React 19, Tailwind v4, Bun 1.4+
├── locales/               # Bilingual Fluent translations (rw.ftl & en.ftl)
└── migrations/            # Production PostgreSQL SQLx database migrations
```

---

## 3. Crate Responsibilities & Technologies

### `crates/core-domain` (Domain Engine)
- Zero I/O, zero network dependencies.
- Models: `User`, `Family`, `Wallet`, `Transaction`, `Project`, `Loan`, `FuturePlan`.
- Mathematical guarantees: `rust_decimal::Decimal` with 4-decimal precision (e.g. `0.0000 RWF`). No floating-point math anywhere.
- Ledger validation invariants: validates double-entry consistency and enforces custody constraints before persistence.
- Implements `utoipa::ToSchema` across all models for OpenAPI 3.1 code generation.

### `crates/i18n` (Bilingual Localization)
- Uses Mozilla Project Fluent (`fluent-bundle`).
- Reads `locales/en.ftl` and `locales/rw.ftl`.
- Provides static interpolation helper: `Translator::t(locale, key, args)`.
- Defines domain error translation catalog (`LocalizableError`, `LocalizedError`).
- Implements `utoipa::ToSchema` for `Locale` and `LocalizedError`.

### `crates/storage-local` (Offline SQLite)
- Embeds SQLite via `rusqlite` bundled with UUID and chrono features.
- Maintains cache of `wallets`, `transactions`, `projects`, `loans`, and `sync_queue`.
- Stores `user_theme_prefs` for persistent light/dark and custom theme configuration.
- Guarantees idempotent offline mutation replay with unique `idempotency_key` UUIDs.

### `crates/api-server` (Backend Actix-web & PostgreSQL)
- Connection pooling with `sqlx::PgPool`.
- Password hashing using Argon2id (`argon2` crate).
- Stateless JWT authentication via `jsonwebtoken`.
- Real-time `FamilyHub`: `DashMap<Uuid, broadcast::Sender<RealtimeEvent>>` keyed by `family_id`.
- Zero-login magic link verification with cryptographic hash tokens.
- Interactive Swagger UI and OpenAPI 3.1 documentation via `utoipa 6.0` and `utoipa-swagger-ui 10.0`.
- Mounted at `/swagger-ui/` referencing `/api-docs/openapi.json`.

### `web/` (Next.js 16 Web Portal on Bun)
- Driven exclusively by Bun (`bun run dev`, `bun run build`, `bun test`, `bunx`).
- Native Turbopack compilation with WebAssembly support for local verification.
- Bilingual dynamic routing (`/[locale]/dashboard`, `/[locale]/transactions`, `/[locale]/projects`, `/[locale]/report/[token]`).
- Clean separation between Personal and Family financial scopes with live real-time sync.

---

## 4. PostgreSQL Security & Isolation
- All tables (`wallets`, `transactions`, `projects`, `loans`, `family_members`, `custody_handovers`) carry `tenant_id` (`family_id` or `user_id`).
- Row-Level Security (RLS) is enabled across all tables in production migrations.
- Personal transactions have `scope = 'PERSONAL'` and `user_id = auth_uid()`.
- Family transactions have `scope = 'FAMILY'` and check family membership via `family_members`.

---

## 5. Real-Time Broadcast Architecture
- WebSocket endpoint: `GET /api/v1/ws?token=<jwt>`.
- When a transaction or handover is committed in PostgreSQL, the server emits an event onto the family's broadcast channel.
- Connected clients (GPUI desktop/mobile and Next.js web) receive typed events:
  - `IncomeAdded`
  - `ExpenseLogged`
  - `HandoverCompleted`
  - `ProjectOrLoanUpdated`
- Clients immediately update local balances and display bilingual toast alerts.

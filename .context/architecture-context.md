# Architecture Context — Umuryango Ledger

## 1. Monorepo Structure

```
umuryango-ledger/
├── AGENTS.md                   # Agent master instructions & 8 core rules
├── .context/                   # System design, standards & progress tracking
├── Cargo.toml                  # Workspace manifest (edition = "2024", resolver = "3")
├── locales/                    # Fluent bilingual catalog (en.ftl, rw.ftl)
├── migrations/                 # PostgreSQL migrations (SQLx with RLS)
├── assets/                     # Master branding and multi-platform icon assets
├── crates/
│   ├── i18n/                   # Bilingual translator & domain error catalog
│   ├── core-domain/            # Pure business logic, models, ledger math, reports
│   ├── storage-local/          # Offline SQLite cache & mutation sync queue
│   ├── api-client/             # Typed HTTP & WebSocket client for Rust apps
│   ├── api-server/             # Actix-web backend, SQLx PostgreSQL, auth, WS hub
│   ├── wasm-bridge/            # WebAssembly bindings for browser calculations
│   └── app-gpui/               # Desktop & mobile GPUI native application
└── web/                        # Next.js 16 App Router (Bun 1.4+, React 19, Tailwind CSS v4)
    ├── public/                 # Static web assets, manifests, icons
    ├── src/
    │   ├── app/                # Root layout, globals.css & bilingual routes (/[locale]/...)
    │   ├── components/         # Shared web UI primitives, modals & charts
    │   └── lib/                # LedgerContext, API clients, WASM bridge loaders
    ├── postcss.config.js       # @tailwindcss/postcss pipeline
    ├── next.config.ts          # Turbopack & WebAssembly configuration
    ├── tsconfig.json           # TypeScript configuration with @types/bun
    ├── bun.lock                # Bun lockfile (exclusive package manager)
    └── package.json            # Web dependencies & bun scripts
```

---

## 2. Framework & Library Versions

| Layer | Framework / Library | Version | Notes |
| :--- | :--- | :--- | :--- |
| **Rust Toolchain** | Rust Edition | `2024` | Resolver `3` across all workspace crates |
| **Error Handling** | `thiserror` | `2.0` | Deriving domain errors in core & local storage |
| **JS/TS Runtime** | **Bun** | `1.4.4` | Exclusive runtime, package manager, and test runner |
| **Web Framework** | Next.js | `16.4.6` | App Router with Turbopack native builds (`bun --bun next`) |
| **Frontend UI** | React / React DOM | `19.3.1` | Native Server & Client components (`use()` hook) |
| **Styling** | Tailwind CSS | `4.3.5` | CSS-first `@theme` design tokens via `@tailwindcss/postcss` |
| **Icons** | `lucide-react` | `1.77.0` | Professional vector icons only, zero emojis |
| **Language** | TypeScript | `7.0.2` | Full strict mode typing with `@types/bun` |

---

## 3. Layer Responsibilities & Dependencies

```mermaid
graph TD
    UI_GPUI[crates/app-gpui] --> Client[crates/api-client]
    UI_GPUI --> Storage[crates/storage-local]
    UI_GPUI --> Core[crates/core-domain]
    UI_GPUI --> I18N[crates/i18n]

    Web[web/ Next.js 16 on Bun] --> WASM[crates/wasm-bridge]
    WASM --> Core
    WASM --> I18N

    Server[crates/api-server] --> Core
    Server --> I18N
    Server --> DB[(PostgreSQL 16)]

    Client --> Server
    Storage --> Core
```

### `crates/core-domain` (Pure Domain Layer)
- Strictly zero network or database dependencies.
- Contains all mathematical assertions:
  - Total balance computation.
  - Dual-scope integrity (`PERSONAL` cannot reference family wallets; `FAMILY` requires family tenancy).
  - Custodian handovers (atomic source subtraction and target addition without changing total family net worth).
  - Validation of transaction allowances based on `FamilyRole` (`Head`, `Admin`, `Member`, `Elder`).

### `crates/i18n` (Bilingual Localization)
- Uses Mozilla Project Fluent (`fluent-bundle`).
- Reads `locales/en.ftl` and `locales/rw.ftl`.
- Provides static interpolation helper: `Translator::t(locale, key, args)`.
- Defines domain error translation catalog (`LocalizableError`, `LocalizedError`).

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

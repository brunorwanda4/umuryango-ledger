# Architecture Context — Umuryango Ledger

## 1. System Vision & Architecture Paradigm
`umuryango-ledger` is built on a **Local-First, High-Trust Monorepo** designed for Rwandan family financial accountability. It bridges modern digital finance (Mobile Money / MTN MoMo, Airtel Money, Bank of Kigali) with traditional, elder-friendly custody principles.

Every financial unit is tied to a specific human or account custodian. No money exists in the abstract; transactions are either inflows, outflows, or custody handovers.

---

## 2. Monorepo Structure

```
umuryango-ledger/
├── crates/
│   ├── core-domain/       # Pure domain models, ledger balance validation, financial math
│   ├── storage-local/     # Rusqlite cache, offline sync_queue, conflict resolution
│   ├── i18n/              # Centralized JSON bilingual engine (rw & en), OnceLock dot-path lookup
│   ├── api-client/        # Shared async HTTP & WebSocket client for native & web
│   ├── api-server/        # Actix-web 4, PostgreSQL (SQLx 0.8), JWT, WebSocket hub
│   ├── wasm-bridge/       # Wasm-bindgen bindings for Next.js web client
│   └── app-gpui/          # High-performance desktop & mobile UI via Zed GPUI
├── web/                   # Next.js 16 (App Router), React 19, Tailwind v4, Bun 1.4+
├── locales/               # Bilingual structured JSON translations (rw.json & en.json)
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
- Centralized structured JSON single source of truth (`locales/en.json` and `locales/rw.json`).
- Compile-time embedded bundles via `include_str!` parsed into a thread-safe `OnceLock<HashMap<Locale, serde_json::Value>>`.
- Dot-path key lookup (`t(locale, "dashboard.title")`) with automatic fallback to English.
- Dynamic runtime string interpolation (`t_args(locale, "...", &[("key", "val")])`).
- Supports runtime hot-reloading for Over-The-Air (OTA) translation bundles.
- Defines domain error translation catalog (`LocalizableError`, `LocalizedError`).
- Implements `utoipa::ToSchema` for `Locale` and `LocalizedError`.

### `crates/storage-local` (Offline SQLite)
- Embeds SQLite via `rusqlite` bundled with UUID and chrono features.
- Maintains cache of `wallets`, `transactions`, `projects`, `loans`, and `sync_queue`.
- Stores `user_theme_prefs` for persistent light/dark and custom theme configuration.
- Guarantees idempotent offline mutation replay with unique `idempotency_key` UUIDs.

### `crates/api-server` (Backend Actix-web & PostgreSQL)
- Connection pooling with `sqlx::PgPool`.
- Row-Level Security (RLS) enforcement ensuring complete data isolation between families.
- Atomic custody handovers requiring dual-party agreement or Head-of-Family authorization.
- Real-time notification hub with WebSockets (`actix-ws`) and actor-based broadcast queues.
- Generates zero-login Mother's Report read tokens for elder accessibility.
- Automatic self-update registry and OTA binary/bundle distribution endpoints (`/api/v1/releases`, `/api/v1/ota`).
- Mounted OpenAPI 3.1 & interactive Swagger UI at `/swagger-ui/` via `utoipa 6.0`.

### `crates/wasm-bridge` (WebAssembly Bridge)
- Compiles domain math, ledger validation, and Mother's Report generator to `wasm32-unknown-unknown`.
- Exposes clean JavaScript/TypeScript interfaces via `wasm-bindgen` for the Next.js frontend.
- Provides `translate(locale, key, args_json)` backed by `crates/i18n`.

### `crates/app-gpui` (Native Desktop & Mobile UI)
- Zed GPUI GPU-accelerated immediate-mode graphics with `gpui-kit` reusable primitives and charts.
- Strictly adheres to the customizable 5-color token system and Archivo typography.
- Offline-first UX: optimistic UI updates with background synchronization indicators.
- Android safe area handling (querying status bar, navigation bar, cutouts, and IME height).
- Native cross-platform updater engine (`crates/app-gpui/src/updater/`).
- Theme-aware FontAwesome SVG vector icon catalog (`app_icon`, `AppIcon`).

### `web/` (Next.js Web Portal)
- Next.js 16 (App Router) with Turbopack, React 19, Tailwind CSS v4, Bun 1.4+.
- Directly imports `locales/en.json` and `locales/rw.json` via `@/lib/i18n` with typed `useTranslation` and `t()`.
- Public, mobile-optimized, zero-login view for family elders (`/report/[token]`).
- Print-to-PDF styles and WhatsApp sharing formatters.
- Client-side WASM integration for instant balance verification.
- Fully wired PWA Service Worker (`web/public/sw.js`) and live `AutoUpdateListener`.
- Theme-aware FontAwesome vector SVG icons (`<AppIcon />`).

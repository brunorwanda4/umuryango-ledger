# Progress Tracker & Roadmap — Umuryango Ledger

This document tracks completed milestones, in-progress tasks, and upcoming roadmap items across all components of `umuryango-ledger`.

> **AGENT INSTRUCTION:** On every commit, append a new entry to the Commit Log below with the commit hash, summary, and verification status.

---

## 1. Component Implementation Status

| Component | Status | Details |
| :--- | :--- | :--- |
| **`crates/i18n`** | **DONE** | Centralized JSON translation engine (`locales/en.json`, `locales/rw.json`) single source of truth shared across all layers. Compile-time embedded `OnceLock` fast dot-path lookup with fallback to English, dynamic `{placeholder}` interpolation, runtime hot-reloading for OTA bundles, domain error catalog, and automated 100% key parity & variable match tests. Derives `utoipa::ToSchema` for `Locale` and `LocalizedError`. Rust 2024 edition. |
| **`crates/core-domain`** | **DONE** | Pure domain models, mathematical assertions, dual scopes (`PERSONAL` / `FAMILY`), custody handovers, Mother's Report generator, `RealtimeEvent::AppUpdateAvailable`, shared domain types (`SyncStatus`, `UserThemePrefs`). Fully compatible with `wasm32-unknown-unknown` and native targets. Rust 2024 edition. |
| **`crates/storage-local`** | **DONE** | Bundled SQLite database (`umuryango_offline.db`), cache repos, idempotent mutation `sync_queue`, theme persistence, `app_config` key-value persistence. Re-exports domain sync types. Rust 2024 edition, thiserror 2.0. |
| **`crates/api-client`** | **DONE** | Typed asynchronous HTTP client and WebSocket streaming client for Rust native apps. Rust 2024 edition. |
| **`crates/api-server`** | **DONE** | Actix-web 4 backend, Argon2id auth, JWT middleware, atomic SQLx handovers, zero-login magic links, `FamilyHub` WebSocket broadcasting, release manifests (`/api/v1/releases`), OTA bundle serving (`/api/v1/ota`), phone-based auth, 4-char family invite codes, phone pre-invites, and interactive Swagger UI / OpenAPI 3.1 documentation via `utoipa 6.0` and `utoipa-swagger-ui 10.0` at `/swagger-ui/`. Rust 2024 edition. |
| **`crates/wasm-bridge`** | **DONE** | `cdylib` WASM export with `validate_offline_tx`, `get_localized_translation`, `translate`, `calculate_balance_breakdown`, `render_mothers_report_wasm`. Backed directly by `crates/i18n`. Rust 2024 edition. |
| **`crates/app-gpui`** | **DONE** | Unified application UI across desktop, mobile, and browser WebAssembly (`wasm32-unknown-unknown`). 100% of the interactive app screens (Auth, Onboarding, Dashboard, Transactions, Reports, Projects, Loans, Members, Settings) live here. Exposes `mount_gpui_app` entry point for WebAssembly `<canvas id="gpui-canvas">`. Rust 2024 edition. |
| **`web/`** | **DONE** | Public visitor landing page (`/` or `/[locale]`), Mom's zero-login report link (`/report/[token]`), and full-screen (`100vw x 100vh`) GPUI WebAssembly host page (`/app`). All React app pages removed so 100% of the app UI is powered by GPUI via WASM. Powered exclusively by **Bun 1.4+** (`bun.lock`, `bun --bun`). |
| **`migrations/`** | **DONE** | PostgreSQL migrations with schema for users, families, wallets, transactions, handovers, projects, loans, magic tokens, `app_releases`, `ota_bundles`, and `family_phone_invites`. |
| **`.github/workflows`** | **DONE** | Automated cross-platform release workflow (`release.yml`) compiling Windows, Linux, macOS, and Android APK binaries with SHA-256 verification and automatic API release registration. |
| **`crates/api-server/tests`** | **DONE** | End-to-end integration tests (`e2e_offline_sync.rs`) with simulated network dropout proxy (`FaultyNetworkProxy`), verifying chronological offline sync, mid-sync dropout idempotency, custody rollback, batch recovery, and live WebSocket broadcast in `rw` and `en`. |

---

## 2. Commit Log

| Date | Commit Hash | Scope / Title | Verification Passed |
| :--- | :--- | :--- | :--- |
| **2026-11-23** | `91e231d` | `chore(repo): initial commit` | Scaffold verified |
| **2026-11-23** | `4b30e46` | `feat(domain): implement core domain and bilingual i18n` | `cargo test -p umuryango-i18n -p umuryango-core-domain` |
| **2026-11-23** | `4f32631` | `feat(api): implement actix-web backend with bilingual error handling` | `cargo check -p umuryango-api-server` |
| **2026-11-23** | `83dc617` | `feat(gpui, storage): implement offline sqlite storage and GPUI app` | `cargo test -p umuryango-storage-local -p app-gpui` |
| **2026-11-23** | `68e826b` | `feat(realtime): add websocket broadcast hub and family notifications` | `cargo test --workspace` |
| **2026-11-23** | `e8bac59` | `feat(branding): generate multi-platform icon assets from master emblem` | Multi-size icon generation script |
| **2026-11-23** | `534706d` | `feat(web, wasm-bridge): implement wasm bridge and complete Next.js bilingual web app` | `npm run build` & `cargo test --workspace` |
| **2026-11-23** | `554862e` | `docs(context): add AGENTS.md and project context guidelines` | Line count and markdown checks |
| **2026-11-23** | `1d8ec68` | `feat(mobile): add Android and mobile safe area insets support` | `cargo test --workspace` & `npm run build` |
| **2026-11-23** | `39cb7f2` | `chore(stack): upgrade to latest stable frameworks across Rust and Web` | `cargo test --workspace` (23 passed) & `bun run build` (Turbopack, Next 16, React 19) |
| **2026-11-23** | `eaa362c` | `chore(runtime): configure Bun as default JS/TS runtime and package manager` | `bun run typecheck`, `bun test`, `bun run build` |
| **2026-11-23** | `f72fd8d` | `test(sync): add E2E offline SQLite sync and network dropout integration tests` | `cargo test --workspace` (28 passed) & `bun run typecheck`, `bun test` |
| **2026-11-23** | `3ce4f57` | `feat(updater): add cross-platform auto-updates and OTA system` | `cargo test --workspace` (34 passed), `bun run typecheck`, `bun test`, `bun run build` |
| **2026-11-23** | `f40dc79` | `feat(icons): add theme-aware FontAwesome icon system across GPUI and Web` | `cargo check -p app-gpui`, `cargo test -p app-gpui` (8 passed), `bun run typecheck` |
| **2026-11-23** | `dca1559` | `feat(api): add interactive Swagger UI and OpenAPI 3.1 documentation with utoipa 6` | `cargo check -p umuryango-api-server` |
| **2026-11-23** | `3329ed5` | `fix(scripts): filter watchTargets with existsSync in dev-native` | `cargo test --workspace`, `bun run typecheck`, `bun test`, `bun run build` |
| **2026-11-23** | `8388ebb` | `fix(dev): resolve endless rebuild loop and port 8080 permission conflict` | `bun run typecheck`, `cargo check --workspace`, verified `bun run dev:all --no-web` startup |
| **2026-11-23** | `d9516da` | `fix(gpui): restore GPUI window event loop and implement app_config in storage-local` | `cargo check --workspace`, `cargo test -p umuryango-core-domain -p umuryango-storage-local`, `bun run dev:all --no-web` verified |
| **2026-11-23** | `3dfb00d` | `fix(gpui): wire interactive click listeners across all views, controls, and quick entry dialog` | `cargo check --workspace`, `cargo test -p umuryango-app`, `bun run --cwd web typecheck` |
| **2026-11-23** | `95e9bf1` | `refactor(i18n): centralize all translations into shared en.json and rw.json` | `cargo test --workspace`, `bun test`, `bun run typecheck`, `bun run build` |
| **2026-11-24** | `f1b313f` | `arch(wasm): serve GPUI WASM app on /app and keep Next.js for public visitors` | `cargo check -p umuryango-app --no-default-features --target wasm32-unknown-unknown`, `cargo check -p umuryango-app`, `bun run --cwd web build` |
| **2026-11-24** | `c737311` | `fix(app): eliminate compiler warnings across workspace and deduplicate app-gpui binary` | `cargo check --tests --all-targets` (0 warnings), `cargo test --workspace` (34 passed), `bun run typecheck`, `bun test`, `bun run build` |
| **2026-11-24** | `0fc8522` | `style(theme): update default Light/Dark palette to 10-color red primary theme` | `cargo check -p app-gpui`, `cargo test -p umuryango-storage-local -p umuryango-core-domain -p umuryango-i18n -p umuryango-app`, `bun run build` |
| **2026-11-24** | `66bd970` | `feat(gpui): add reusable input and radio components and wire interactive auth form` | Tests skipped per user request |
| **2026-11-24** | `c6aa642` | `feat(gpui): embed AppAssets icon source and refine auth password inputs and rw label` | Tests skipped per user request |
| **2026-11-25** | `eb71ca3` | `feat(gpui, api): wire live API bridge, interactive entity modals, and family member endpoints` | Tests skipped per user request |
| **2026-11-25** | `59db18c` | `feat(web): initialize shadcn UI setup and add button primitive` | Tests skipped per user request |

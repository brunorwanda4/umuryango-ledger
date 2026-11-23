# Progress Tracker & Roadmap — Umuryango Ledger

This document tracks completed milestones, in-progress tasks, and upcoming roadmap items across all components of `umuryango-ledger`.

> **AGENT INSTRUCTION:** On every commit, append a new entry to the Commit Log below with the commit hash, summary, and verification status.

---

## 1. Component Implementation Status

| Component | Status | Details |
| :--- | :--- | :--- |
| **`crates/i18n`** | **DONE** | Complete bilingual Fluent bundles (`rw.ftl`, `en.ftl`), `Translator::t()`, `Localizer`, and domain error translation catalog. Derives `utoipa::ToSchema` for `Locale` and `LocalizedError`. Rust 2024 edition. |
| **`crates/core-domain`** | **DONE** | Pure domain models, mathematical assertions, dual scopes (`PERSONAL` / `FAMILY`), custody handovers, Mother's Report generator, `RealtimeEvent::AppUpdateAvailable`. Derives `utoipa::ToSchema` on all models and enums. Rust 2024 edition. |
| **`crates/storage-local`** | **DONE** | Bundled SQLite database (`umuryango_offline.db`), cache repos, idempotent mutation `sync_queue`, theme persistence, `app_config` key-value persistence. Rust 2024 edition, thiserror 2.0. |
| **`crates/api-client`** | **DONE** | Typed asynchronous HTTP client and WebSocket streaming client for Rust native apps. Rust 2024 edition. |
| **`crates/api-server`** | **DONE** | Actix-web 4 backend, Argon2id auth, JWT middleware, atomic SQLx handovers, zero-login magic links, `FamilyHub` WebSocket broadcasting, release manifests (`/api/v1/releases`), OTA bundle serving (`/api/v1/ota`), and interactive Swagger UI / OpenAPI 3.1 documentation via `utoipa 6.0` and `utoipa-swagger-ui 10.0` at `/swagger-ui/`. Rust 2024 edition. |
| **`crates/wasm-bridge`** | **DONE** | `cdylib` WASM export with `validate_offline_tx`, `get_localized_translation`, `calculate_balance_breakdown`, `render_mothers_report_wasm`. Rust 2024 edition. |
| **`crates/app-gpui`** | **DONE** | Native desktop & mobile app using `gpui` + `gpui-kit`, Archivo typography, light/dark modes, offline sync, Android/mobile safe area insets, cross-platform self-updating engine (`crates/app-gpui/src/updater/`), and unified FontAwesome SVG icon system (`AppIcon`, zero emojis). Rust 2024 edition. |
| **`web/`** | **DONE** | Native Next.js 16 (16.4.6) with Turbopack, React 19 (19.3.1), Tailwind CSS v4 (4.3.5) with CSS-first design tokens, zero emojis, Archivo font, `LedgerContext`, live WebSocket toasts, interactive modals, Mother's Report, mobile safe-area insets, versioned PWA Service Worker (`/sw.js`), live `AutoUpdateListener`, and unified FontAwesome vector icons (`<AppIcon />`). Powered exclusively by **Bun 1.4+** (`bun.lock`, `bun --bun`). |
| **`migrations/`** | **DONE** | PostgreSQL migrations with schema for users, families, wallets, transactions, handovers, projects, loans, magic tokens, `app_releases`, and `ota_bundles`. |
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

---

## 3. Upcoming Milestones & Roadmap

- [x] **Auto-Update & Over-the-Air Delivery:** Cross-platform self-updating system across Actix-web, GPUI Desktop (atomic `self-replace`), GPUI Android (`APK installer intent`), Next.js Web / PWA (`sw.js` + `AutoUpdateListener`), and GitHub Actions release CI/CD.
- [x] **E2E Automated Sync Flow:** End-to-end integration tests between offline SQLite `sync_queue` and live Actix-web server under simulated network dropouts.
- [x] **Unified Vector Icon System:** Local FontAwesome Free Solid SVG catalog replacing all emojis across GPUI and Web with dynamic Light/Dark mode palette tokens.
- [x] **Interactive OpenAPI 3.1 & Swagger UI:** Complete OpenAPI 3.1 schema generation and interactive Swagger UI mounted at `/swagger-ui/` using native `utoipa 6.0` and `utoipa-swagger-ui 10.0`.
- [ ] **SMS / USSD Reporting Fallback:** Basic USSD / SMS report dispatch for family elders without smartphone internet.
- [ ] **Export to Excel / CSV:** One-click CSV and Excel financial report export in Kinyarwanda and English.
- [ ] **Multi-Currency Support:** Support for USD, EUR, and KES with daily converted totals alongside primary RWF balances for diaspora senders.

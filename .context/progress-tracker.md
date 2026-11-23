# Progress Tracker & Roadmap — Umuryango Ledger

This document tracks completed milestones, in-progress tasks, and upcoming roadmap items across all components of `umuryango-ledger`.

> **AGENT INSTRUCTION:** On every commit, append a new entry to the Commit Log below with the commit hash, summary, and verification status.

---

## 1. Component Implementation Status

| Component | Status | Details |
| :--- | :---: | :--- |
| **`crates/i18n`** | **DONE** | Complete bilingual Fluent bundles (`rw.ftl`, `en.ftl`), `Translator::t()`, and domain error translation catalog. |
| **`crates/core-domain`** | **DONE** | Pure domain models, mathematical assertions, dual scopes (`PERSONAL` / `FAMILY`), custody handovers, Mother's Report generator. |
| **`crates/storage-local`** | **DONE** | Bundled SQLite database (`umuryango_offline.db`), cache repos, idempotent mutation `sync_queue`, theme persistence. |
| **`crates/api-client`** | **DONE** | Typed asynchronous HTTP client and WebSocket streaming client for Rust native apps. |
| **`crates/api-server`** | **DONE** | Actix-web 4 backend, Argon2id auth, JWT middleware, atomic SQLx handovers, zero-login magic links, `FamilyHub` WebSocket broadcasting. |
| **`crates/wasm-bridge`** | **DONE** | `cdylib` WASM export with `validate_offline_tx`, `get_localized_translation`, `calculate_balance_breakdown`, `render_mothers_report_wasm`. |
| **`crates/app-gpui`** | **DONE** | Native desktop & mobile app using `gpui` + `gpui-kit`, Archivo typography, light/dark modes, offline sync, icons pipeline. |
| **`web/`** | **DONE** | Next.js 15 App Router, TypeScript, Tailwind tokens, Archivo font, `LedgerContext`, live WebSocket toasts, interactive modals, Mother's Report. |
| **`migrations/`** | **DONE** | PostgreSQL migrations with schema for users, families, wallets, transactions, handovers, projects, loans, and magic tokens. |

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

---

## 3. Upcoming Milestones & Roadmap

- [ ] **E2E Automated Sync Flow:** End-to-end integration tests between offline SQLite `sync_queue` and live Actix-web server under simulated network dropouts.
- [ ] **SMS / USSD Reporting Fallback:** Basic USSD / SMS report dispatch for family elders without smartphone internet.
- [ ] **Export to Excel / CSV:** One-click CSV and Excel financial report export in Kinyarwanda and English.
- [ ] **Multi-Currency Support:** Support for USD, EUR, and KES with daily converted totals alongside primary RWF balances for diaspora senders.

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
| **2026-11-25** | `13d9781` | `feat(web): add all shadcn UI components` | `bun run typecheck`, `bun test` (4 passed), `bun run build` |
| **2026-11-24** | `dd258be` | `feat(ui): add GPUI hooks and preset b1oaDsAa style utilities` | `cargo check -p umuryango-app` |
| **2026-11-24** | `56d4401` | `feat(ui): port shadcn direction to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `28dfc32` | `feat(ui): port shadcn separator to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `37e1622` | `feat(ui): port shadcn skeleton to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `4b11501` | `feat(ui): port shadcn spinner to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `c529373` | `feat(ui): port shadcn aspect-ratio to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `a0893c8` | `feat(ui): port shadcn kbd to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `d12803e` | `feat(ui): port shadcn label to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `24db7e7` | `feat(ui): port shadcn badge to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `03f9367` | `feat(ui): port shadcn button to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `8bca0af` | `feat(ui): port shadcn button-group to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `8b4896d` | `feat(ui): port shadcn card to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `b036043` | `feat(ui): port shadcn input to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `f970174` | `feat(ui): port shadcn textarea to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `6f4e58a` | `feat(ui): port shadcn input-group to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `4388da4` | `feat(ui): port shadcn input-otp to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `24fbfc8` | `feat(ui): port shadcn checkbox to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `2246257` | `feat(ui): port shadcn switch to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `39e7527` | `feat(ui): port shadcn radio-group to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `a610686` | `feat(ui): port shadcn slider to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `a19b5ac` | `feat(ui): port shadcn progress to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `a3e28bb` | `feat(ui): port shadcn native-select to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `2399de4` | `feat(ui): port shadcn field to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `944e9e0` | `feat(ui): port shadcn avatar to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `a49e5ad` | `feat(ui): port shadcn alert to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `979d208` | `feat(ui): port shadcn empty to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `03e25f4` | `feat(ui): port shadcn item to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `8a05b9b` | `feat(ui): port shadcn marker to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `f50c999` | `feat(ui): port shadcn toggle to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `6dd2aa2` | `feat(ui): port shadcn toggle-group to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `2d1eccc` | `feat(ui): port shadcn collapsible to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `6d4d80e` | `feat(ui): port shadcn accordion to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `d77a58a` | `feat(ui): port shadcn tabs to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `bd56b21` | `feat(ui): port shadcn breadcrumb to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `6645ac4` | `feat(ui): port shadcn pagination to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `4798089` | `feat(ui): port shadcn scroll-area to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `d816761` | `feat(ui): port shadcn resizable to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `65a27fe` | `feat(ui): port shadcn table to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `e2b8b48` | `feat(ui): port shadcn tooltip to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `2694309` | `feat(ui): port shadcn popover to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `f076a06` | `feat(ui): port shadcn hover-card to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `5bd8009` | `feat(ui): port shadcn dialog to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `8aafc90` | `feat(ui): port shadcn alert-dialog to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `99d93d9` | `feat(ui): port shadcn sheet to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `68b8c94` | `feat(ui): port shadcn drawer to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `7876e88` | `feat(ui): port shadcn select to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `67c3cd3` | `refactor(ui): split select into focused submodules under 1000-line limit` | `cargo check -p umuryango-app` |
| **2026-11-24** | `345b5c1` | `feat(ui): port shadcn dropdown-menu to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `b5081f8` | `feat(ui): port shadcn context-menu to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `d47c73d` | `feat(ui): port shadcn menubar to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `20179d4` | `feat(ui): port shadcn navigation-menu to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `b20f97c` | `feat(ui): port shadcn command to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `1aaeb30` | `feat(ui): port shadcn combobox to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `9c55f27` | `feat(ui): port shadcn toast to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `9193682` | `feat(ui): port shadcn carousel to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `e7f259c` | `feat(ui): port shadcn calendar to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `517fc31` | `feat(ui): port shadcn chart to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `28d6839` | `feat(ui): port shadcn code-block to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-24** | `f7f39e4` | `feat(ui): port shadcn attachment to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-25** | `de1e242` | `feat(ui): port shadcn bubble to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-25** | `1408392` | `feat(ui): port shadcn message to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-25** | `711a3ed` | `feat(ui): port shadcn message-scroller to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-25** | `c440e3a` | `feat(ui): port shadcn trace to GPUI common` | `cargo check -p umuryango-app` |
| **2026-11-25** | `d23283f` | `feat(ui): port shadcn questionnaire to GPUI common` | `cargo check -p umuryango-app` |

---

## 3. shadcn/ui (`--preset b1oaDsAa` / `base-sera`) → GPUI `common/` Porting Checklist

### A. Hooks & Style Utilities
- [x] `web/src/hooks/use-mobile.ts` → `crates/app-gpui/src/hooks/use_mobile.rs` (`Breakpoint::Mobile | Tablet | Desktop`, `is_mobile(window)`, `observe_viewport`)
- [x] `web/src/lib/utils.ts` → `crates/app-gpui/src/common/utils.rs` (`PresetRadius`, `PresetSpacing`, `PresetTokens`, `StyleComposeExt`, focus/invalid/surface shadow builders)
- [x] `web/src/hooks/use-highlighted-code.ts` & `web/src/lib/highlighter.tsx` → **Skipped (Documented):** Zero installed components in `web/src/components/ui/` (including `code-block.tsx`) and zero Umuryango Ledger views import or depend on Shiki syntax highlighting; omitted per Rule 1 (Zero Dead Code).

### B. All 63 Components (`web/src/components/ui/*.tsx` → `crates/app-gpui/src/common/*.rs`)
- [x] 01. `direction.tsx` → `direction.rs`
- [x] 02. `separator.tsx` → `separator.rs`
- [x] 03. `skeleton.tsx` → `skeleton.rs`
- [x] 04. `spinner.tsx` → `spinner.rs`
- [x] 05. `aspect-ratio.tsx` → `aspect_ratio.rs`
- [x] 06. `kbd.tsx` → `kbd.rs`
- [x] 07. `label.tsx` → `label.rs`
- [x] 08. `badge.tsx` → `badge.rs`
- [x] 09. `button.tsx` → `button.rs`
- [x] 10. `button-group.tsx` → `button_group.rs`
- [x] 11. `card.tsx` → `card.rs`
- [x] 12. `input.tsx` → `input.rs`
- [x] 13. `textarea.tsx` → `textarea.rs`
- [x] 14. `input-group.tsx` → `input_group.rs`
- [x] 15. `input-otp.tsx` → `input_otp.rs`
- [x] 16. `checkbox.tsx` → `checkbox.rs`
- [x] 17. `switch.tsx` → `switch.rs`
- [x] 18. `radio-group.tsx` → `radio_group.rs`
- [x] 19. `slider.tsx` → `slider.rs`
- [x] 20. `progress.tsx` → `progress.rs`
- [x] 21. `native-select.tsx` → `native_select.rs`
- [x] 22. `field.tsx` → `field.rs`
- [x] 23. `avatar.tsx` → `avatar.rs`
- [x] 24. `alert.tsx` → `alert.rs`
- [x] 25. `empty.tsx` → `empty.rs`
- [x] 26. `item.tsx` → `item.rs`
- [x] 27. `marker.tsx` → `marker.rs`
- [x] 28. `toggle.tsx` → `toggle.rs`
- [x] 29. `toggle-group.tsx` → `toggle_group.rs`
- [x] 30. `collapsible.tsx` → `collapsible.rs`
- [x] 31. `accordion.tsx` → `accordion.rs`
- [x] 32. `tabs.tsx` → `tabs.rs`
- [x] 33. `breadcrumb.tsx` → `breadcrumb.rs`
- [x] 34. `pagination.tsx` → `pagination.rs`
- [x] 35. `scroll-area.tsx` → `scroll_area.rs`
- [x] 36. `resizable.tsx` → `resizable.rs`
- [x] 37. `table.tsx` → `table.rs`
- [x] 38. `tooltip.tsx` → `tooltip.rs`
- [x] 39. `popover.tsx` → `popover.rs`
- [x] 40. `hover-card.tsx` → `hover_card.rs`
- [x] 41. `dialog.tsx` → `dialog.rs`
- [x] 42. `alert-dialog.tsx` → `alert_dialog.rs`
- [x] 43. `sheet.tsx` → `sheet.rs`
- [x] 44. `drawer.tsx` → `drawer.rs`
- [x] 45. `select.tsx` → `select/mod.rs`
- [x] 46. `dropdown-menu.tsx` → `dropdown_menu/mod.rs`
- [x] 47. `context-menu.tsx` → `context_menu.rs`
- [x] 48. `menubar.tsx` → `menubar.rs`
- [x] 49. `navigation-menu.tsx` → `navigation_menu.rs`
- [x] 50. `command.tsx` → `command.rs`
- [x] 51. `combobox.tsx` → `combobox.rs`
- [x] 52. `toast.tsx` → `toast.rs`
- [x] 53. `carousel.tsx` → `carousel.rs`
- [x] 54. `calendar.tsx` → `calendar.rs`
- [x] 55. `chart.tsx` → `chart.rs`
- [x] 56. `code-block.tsx` → `code_block.rs`
- [x] 57. `attachment.tsx` → `attachment.rs`
- [x] 58. `bubble.tsx` → `bubble.rs`
- [x] 59. `message.tsx` → `message.rs`
- [x] 60. `message-scroller.tsx` → `message_scroller.rs`
- [x] 61. `trace.tsx` → `trace.rs`
- [x] 62. `questionnaire.tsx` → `questionnaire.rs`
- [x] 63. `sidebar.tsx` → `sidebar/mod.rs`


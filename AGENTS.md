# AGENTS.md — AI Agent Operating Instructions

> **MANDATORY READING:** Every AI agent operating in this repository must read this document and the referenced context files in `.context/` before planning, creating, modifying, or refactoring any code.

---

## 1. Required Context Reading Order
Before performing any task, examine:
1. [`.context/project-overview.md`](file:///.context/project-overview.md) — Domain problem, custody model, and dual scopes (`PERSONAL` vs `FAMILY`).
2. [`.context/ai-workflow-rules.md`](file:///.context/ai-workflow-rules.md) — Non-negotiable operating protocol, verification steps, and commit guidelines.
3. [`.context/code-standards.md`](file:///.context/code-standards.md) — Rust, SQLx, TypeScript, Bun runtime, and architectural hard limits (<1,000 lines per file).
4. [`.context/progress-tracker.md`](file:///.context/progress-tracker.md) — Current implementation status, pending roadmap, and log of completed commits.
5. Specific feature specs in [`.context/feature-specs/`](file:///.context/feature-specs/) corresponding to the active task.

---

## 2. Nine Non-Negotiable Core Rules

### Rule 1: Less Is More (Zero Bloat & Zero Dead UI)
- Never create placeholder buttons, disabled links that do nothing, or filler text.
- Every button, dropdown, input, and modal must be 100% wired, tested, and functional.
- Eliminate decorative fluff and redundant labels. High density, high clarity, and purposeful interaction only.

### Rule 2: Strict Component Reuse
- **Rust GPUI (`crates/app-gpui`):** Use shared components and charts from `gpui-kit`. Never duplicate one-off cards, inputs, or button primitives.
- **Next.js Web (`web/src/components/`):** Reuse shared UI primitives and existing layouts. Never duplicate one-off modals, cards, or form inputs.

### Rule 3: Strict Color Discipline (Customizable 5-Color System)
Never hardcode arbitrary hex colors in stylesheets or component code. Use only semantic tokens initialized from the 5-color brand palette:
- Primary Dark: `#003049`
- Accent Cream: `#fdf0d5`
- Primary Crimson: `#780000`
- Active Crimson: `#c1121f`
- Steel Blue: `#669bbc`
Plus semantic status tokens: `error`, `success`, `warning`, `info`.

### Rule 4: Professional Icons Only (Zero Emojis)
- **Zero Emojis:** Never use emojis in production UI headers, buttons, cards, forms, or system log messages.
- **Vector Icons Only:**
  - Rust GPUI: SVG vector icons via `gpui-kit` / SVG renderers.
  - Next.js: Lucide icons from `lucide-react`.

### Rule 5: 1,000-Line Hard File Limit
- No file anywhere in the repository may exceed 1,000 lines. Target 150–400 lines per file.
- Break down monolithic files into focused, pure, reusable modules and helper functions immediately.

### Rule 6: Mandatory Git Commits Per Change
- Every single logical change or feature must pass checks (`cargo check`, `cargo test`, `bun run typecheck`, `bun test`, `bun run build`).
- Commit immediately with a clean Conventional Commit message (e.g., `feat(wallets): add custody handover validation`).
- Immediately update [`.context/progress-tracker.md`](file:///.context/progress-tracker.md) with the commit hash and summary.

### Rule 7: Bilingual First (`rw` & `en`)
- Centralized, structured JSON translation files (`locales/rw.json` and `locales/en.json`) serve as the **single source of truth** shared across all layers (`crates/i18n`, `crates/core-domain`, `crates/api-server`, `crates/wasm-bridge`, `crates/app-gpui`, and `web/`).
- Every user-facing UI text, report label, WebSocket notification, and backend Actix-web API error must exist in both Kinyarwanda (`locales/rw.json`) and English (`locales/en.json`).
- Never hardcode unlocalized English or Kinyarwanda strings in templates, UI components, or backend error responses.
- Adding a new language only requires adding a new `locales/<lang>.json` file and registering its `Locale` variant.

### Rule 8: Bun as Exclusive JS/TS Runtime & Package Manager
- **Bun Only:** Exclusively use `bun` and `bunx` across `web/` and all JavaScript/TypeScript tooling, package management, script running, and testing.
- **Never Use Legacy Tools:** NEVER run `npm`, `npx`, `yarn`, or `pnpm`. Do not create or commit `package-lock.json`, `yarn.lock`, or `pnpm-lock.yaml`.
- **Workflow Commands:**
  - Package Management: `bun install`, `bun add <pkg>`, `bun add -d <pkg>`, `bun remove <pkg>`.
  - Development & Build: `bun run dev` (executing `bun --bun next dev`), `bun run build` (executing `bun --bun next build`).
  - Type-checking & Testing: `bun run typecheck` (`bunx tsc --noEmit`), `bun test`.
  - Utility & Scripts: Use native `Bun` APIs and runtime execution for automation scripts.

### Rule 9: 100% App UI Built Only in GPUI (Rust) & Served via WASM at `/app`
- **Zero App UI in React/Next.js:** Never build Login, Register, Onboarding, Dashboard, Transactions, Projects, Loans, or Settings pages in React/Next.js. Every app screen lives strictly in `crates/app-gpui` and compiles to WebAssembly (`wasm32-unknown-unknown`) for the browser.
- **Next.js (`web/`) is strictly for public visitors and reports:**
  - `/` (or `/[locale]`): public visitor landing page with an "Open App" button linking directly to `/app`.
  - `/report/[token]` (or `/[locale]/report/[token]`): Mom's zero-login web report link.
  - `/app`: full-screen (`100vw x 100vh`) GPUI WebAssembly host page hosting `<canvas id="gpui-canvas">`.

---

## 3. Technology Stack Reference

| Layer | Technologies |
| :--- | :--- |
| **Backend API** | Rust 2024 (edition 2024, resolver 3), Actix-web 4, SQLx 0.8 (PostgreSQL 16), Argon2id, JWT, `actix-ws` |
| **Desktop / Mobile UI & Web App** | Rust, `gpui`, `gpui-kit` components and charts, Android safe area insets, WebAssembly (`wasm32-unknown-unknown`) |
| **Offline Storage** | SQLite (`rusqlite` bundled on native, in-memory facade on web), optimistic sync queue |
| **Public Web & Report Portal**| Next.js 16 (App Router), Bun 1.4+, React 19, Tailwind CSS v4, WebAssembly bridge |
| **Localization** | Centralized JSON translation bundles (`locales/en.json`, `locales/rw.json`), `crates/i18n` with OnceLock fast dot-path lookup & fallback, `@/lib/i18n` in Next.js |
| **Typography** | `Archivo` Google Font family |

---

## 4. Context Directory Index
- [`.context/project-overview.md`](file:///.context/project-overview.md) — Core problem, user personas, custody model.
- [`.context/architecture-context.md`](file:///.context/architecture-context.md) — Monorepo layout, database RLS, offline sync, GPUI WASM web architecture.
- [`.context/ai-workflow-rules.md`](file:///.context/ai-workflow-rules.md) — Agent operating steps and verification standards.
- [`.context/code-standards.md`](file:///.context/code-standards.md) — Coding conventions, integer money math, safety.
- [`.context/rust-actix-rebuild.md`](file:///.context/rust-actix-rebuild.md) — Actix-web handler patterns, middleware, WS hub.
- [`.context/typescript-safety.md`](file:///.context/typescript-safety.md) — Next.js TypeScript, Bun runtime, WASM bridge, client API.
- [`.context/ui-context.md`](file:///.context/ui-context.md) — Design tokens, themes, component catalogs.
- [`.context/ui-clarity.md`](file:///.context/ui-clarity.md) — Radical simplicity, zero dead UI, visual scan paths.
- [`.context/ui-depth.md`](file:///.context/ui-depth.md) — Visual hierarchy, elevation, responsive layout transitions.
- [`.context/apple-design.md`](file:///.context/apple-design.md) — Craft, spacing, smooth transitions, ergonomics.
- [`.context/ux-psychology.md`](file:///.context/ux-psychology.md) — Trust, reducing cognitive load for family elders.
- [`.context/progress-tracker.md`](file:///.context/progress-tracker.md) — Task list and commit log.
- [`.context/feature-specs/`](file:///.context/feature-specs/) — Technical specifications for key features.

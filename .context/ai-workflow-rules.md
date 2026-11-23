# AI Agent Workflow Rules — Umuryango Ledger

This document establishes the mandatory protocol that every AI agent must execute sequentially on every task.

---

## 1. Pre-Task Preparation Protocol
Before generating or modifying any code:
1. **Read Progress State:** Open [`.context/progress-tracker.md`](file:///.context/progress-tracker.md) to understand current milestones, what was completed in preceding commits, and what is currently pending.
2. **Consult Feature Specifications:** Open the relevant document in [`.context/feature-specs/`](file:///.context/feature-specs/) to understand the mathematical formulas, database columns, and validation invariants.
3. **Audit Existing Components:** Scan existing components in `crates/app-gpui/src/components/` and `web/src/components/`. If a button, card, modal, or input already exists, reuse or extend it. Never build duplicate one-off primitives.
4. **Use Bun Exclusively for JS/TS:** Always invoke `bun`, `bunx`, and `bun test` when touching `web/` or writing scripts. Never invoke `npm`, `npx`, `yarn`, or `pnpm`.

---

## 2. Code Authoring Protocol

### A. The 1,000-Line Hard Limit
- **Absolute Ceiling:** No file in the repository may exceed 1,000 lines under any circumstance.
- **Recommended Range:** Aim for 150–400 lines per file.
- **Immediate Refactoring:** If a file approaches 800 lines, extract helper functions, pure domain submodules, or subcomponents before adding new code.

### B. Integer Financial Arithmetic
- Never use IEEE 754 floating-point numbers (`f32`, `f64`, or JavaScript `number` without precision guards) for currency math.
- In Rust: Use `rust_decimal::Decimal` or integer units (`i64` in cents / whole RWF).
- In SQL: Use `BIGINT` or `NUMERIC(18, 2)`.

### C. Bilingual First
- Every new user-facing string or API error must be declared in both `locales/en.ftl` and `locales/rw.ftl`.
- Add test assertions checking both languages for new errors or report sections.

### D. Zero-Bloat and Zero-Emoji Policy
- Do not render placeholder buttons that do not trigger real state mutations.
- Every button, input, toggle, and link must be fully functional.
- Zero emojis in UI or logs; use vector icons (`lucide-react` in web, `gpui-kit` / SVG in GPUI).

### E. Bun Runtime & Lockfile Discipline
- The web application and any Node/TypeScript tool runs exclusively on **Bun** (`1.4+`).
- Only `web/bun.lock` is allowed. Never commit `package-lock.json`, `yarn.lock`, or `pnpm-lock.yaml`.
- All development and production commands must be run via Bun: `bun run dev`, `bun run build`, `bun test`, `bun run typecheck`.

---

## 3. Verification Protocol
Before proposing changes or creating a commit, run the required verification commands:

1. **Rust Workspace Check:**
   ```bash
   cargo check --workspace
   ```
2. **Rust Workspace Tests:**
   ```bash
   cargo test --workspace
   ```
3. **Web Application Typecheck, Unit Tests & Production Build (Bun):**
   ```bash
   cd web && bun run typecheck && bun test && bun run build
   ```

Do not proceed to commit if any test fails or any compilation error is emitted.

---

## 4. Commit & Logging Protocol
1. **Atomic Commits:** Make focused, logical commits for each completed unit of work.
2. **Conventional Commit Message:** Follow the conventional commit format:
   - `feat(<crate/area>): <description>`
   - `fix(<crate/area>): <description>`
   - `refactor(<crate/area>): <description>`
   - `docs(<crate/area>): <description>`
   - `test(<crate/area>): <description>`
   - `chore(<crate/area>): <description>`
3. **Update Progress Tracker:** Immediately add a line to [`.context/progress-tracker.md`](file:///.context/progress-tracker.md) documenting:
   - Commit hash
   - Features or fixes included
   - Verification commands passed

---

## 5. Reporting Integrity Guidelines
- Report only observed facts from tool outputs.
- Never state that a build, test, or file creation succeeded unless the command returned exit code 0.
- If blocked by permissions or unexpected environment issues, state the exact error clearly.

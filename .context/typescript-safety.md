# TypeScript & Web Standards — Umuryango Ledger

This document defines TypeScript safety rules, component patterns, Bun runtime standards, and WASM bridge integration standards for `web/`.

---

## 1. Strict Typing Principles

1. **Zero `any`:**
   - The use of `any` is forbidden. Use explicit interfaces, discriminated unions, or generic parameters.
   - For unknown payload shapes, use `unknown` and validate with type guards or schema validators.

2. **DTO Synchronization with Rust `core-domain`:**
   - Frontend interfaces must match Rust models:
     - `CustodyWallet` -> `umuryango_core_domain::models::Wallet`
     - `LedgerTransaction` -> `umuryango_core_domain::models::Transaction`
     - `MothersReport` -> `umuryango_core_domain::reporting::MothersReport`
     - `ScopeType` (`"personal" | "family"`) -> `umuryango_core_domain::scope::LedgerScope`

---

## 2. Safe WebAssembly Bridge Loading

The browser can execute pure Rust business rules compiled to WASM via `crates/wasm-bridge`.

```typescript
// Safe async bridge loader with fallback
export async function loadWasmBridge() {
  try {
    const wasm = await import("@/../wasm-pkg/umuryango_wasm_bridge.js" as any);
    return wasm;
  } catch (err) {
    console.warn("WASM bridge module not loaded. Using TypeScript fallback.", err);
    return null;
  }
}
```

- When WASM is available, delegate heavy computations (balance recalculations, offline transaction validation) to WASM.
- Always provide a deterministic TypeScript fallback so server-side rendering (SSR) and environments without WASM support execute without crashing.

---

## 3. UI Icon Standards (Zero Emojis)

- **Rule:** Never use raw emojis in UI buttons, tables, badges, or headers.
- **Icon Source:** Use `lucide-react` exclusively.
- **Standard Mapping:**
  - Inflow / Income: `<ArrowUpRight className="text-emerald-600" />`
  - Outflow / Expense: `<ArrowDownRight className="text-brand-crimson" />`
  - Handover / Custody Transfer: `<Handshake className="text-brand-steel-blue" />`
  - Bank Account: `<Building2 />`
  - Mobile Money: `<Smartphone />`
  - Cash in Hand: `<Banknote />`
  - Family Scope: `<Landmark />`
  - Personal Scope: `<Lock />`
  - Zero-Login Report: `<ShieldCheck />`

---

## 4. State Management with `LedgerContext`

- Central client state is provided via `web/src/lib/LedgerContext.tsx`.
- Consumers use the custom hook: `const { scope, wallets, transactions, kpis } = useLedger()`.
- Optimistic updates:
  - When user records an expense or handover in `TransactionModal` or `HandoverModal`, balances update optimistically in state while mutating backend or syncing offline.
  - An automatic toast notification is emitted in the active locale (`rw` or `en`).

---

## 5. Bun Runtime & Tooling Mandate

- **Exclusive Tooling:** Always use `bun` and `bunx`. Never invoke `npm`, `npx`, `yarn`, or `pnpm`.
- **Lockfile Integrity:** `web/bun.lock` is the single source of truth for dependencies.
- **Commands:**
  - `cd web && bun install` — Install dependencies.
  - `cd web && bun run dev` — Launch Next.js dev server (`bun --bun next dev`).
  - `cd web && bun run build` — Compile optimized production build (`bun --bun next build`).
  - `cd web && bun run typecheck` — Run TypeScript typecheck (`bunx tsc --noEmit`).
  - `cd web && bun test` — Execute fast unit tests using `bun:test`.

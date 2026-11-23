# TypeScript & Web Standards — Umuryango Ledger

This document defines TypeScript safety rules, component patterns, and WASM bridge integration standards for `web/`.

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
  - Inflow / Income: `<ArrowDownLeft className="text-emerald-600" />`
  - Outflow / Expense: `<ArrowUpRight className="text-brand-crimson" />`
  - Handover / Custody Transfer: `<ArrowLeftRight className="text-brand-steel-blue" />`
  - Bank Account: `<Landmark />`
  - Mobile Money: `<Smartphone />`
  - Cash in Hand: `<Banknote />`
  - Family Scope: `<Users />`
  - Personal Scope: `<Lock />`
  - Zero-Login Report: `<FileText />` or `<ShieldCheck />`

---

## 4. State Management with `LedgerContext`

- Central client state is provided via `web/src/lib/LedgerContext.tsx`.
- Consumers use the custom hook: `const { scope, wallets, transactions, kpis } = useLedger()`.
- Optimistic updates:
  - When user records an expense or handover in `TransactionModal` or `HandoverModal`, balances update optimistically in state while mutating backend or syncing offline.
  - An automatic toast notification is emitted in the active locale (`rw` or `en`).

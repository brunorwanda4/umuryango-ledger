# Project Overview — Umuryango Ledger

## 1. Executive Summary
`umuryango-ledger` is an offline-first, bilingual (`rw` / `en`) family financial accountability platform and personal ledger. It solves the friction, distrust, and cognitive load common in Rwandan and East African extended families when handling shared money.

In typical families, one or two members hold pooled family funds across bank accounts (e.g., Bank of Kigali), mobile wallets (MTN MoMo), or physical cash envelopes. They buy groceries at the local market, pay construction workers, and distribute funds for medical emergencies or school fees. Traditionally, accounting is done on loose paper or ad-hoc WhatsApp messages, causing confusion, misattribution of expenses, and tension between parents, custodians, and diaspora contributors.

`umuryango-ledger` provides:
1. **Custodial Accountability:** Every franc belongs to a specific custody wallet (Bank, MoMo, Cash in Hand) assigned to a designated custodian.
2. **Custody Handovers vs. Expenses:** Clear mathematical and visual distinction between spending money and handing over cash from one custodian to another.
3. **Mother's Summary Report (Zero-Login):** Elder-friendly, password-free tokenized summary accessible via web and shareable to WhatsApp.
4. **Dual Scopes (`PERSONAL` vs. `FAMILY`):** Strict cryptographic and database separation ensuring private personal money is never visible on the shared family ledger.
5. **Multi-Tenant Architecture:** Independent family ledgers with multi-tenant onboarding.

---

## 2. Key Personas & Use Cases

### Persona A: Mama / Family Elder ("The Decision Maker")
- **Profile:** Non-technical, values transparency, family unity, and trust. Needs to know the answer to three questions in 5 seconds:
  1. *How much family money came in?*
  2. *What was spent on legitimate family needs?*
  3. *Who holds the remaining cash right now?*
- **Solution:** Receives a zero-login tokenized web link via WhatsApp. Sees an uncluttered view formatted in Kinyarwanda or English with clear custody breakdowns and project progress.

### Persona B: Active Custodian ("The Market Shopper")
- **Profile:** Son or daughter living with parents or managing family errands. Frequently makes quick payments at outdoor markets where internet connectivity is spotty.
- **Solution:** Uses the GPUI desktop or mobile app to log an expense in 5 seconds offline. Mutations queue locally in SQLite and automatically sync when connected.

### Persona C: Diaspora / Contributing Sibling ("The Funder")
- **Profile:** Lives abroad or works in Kigali, sends money for parents' health insurance (*Mutuelle de Santé*), school tuition, or homestead renovations.
- **Solution:** Receives real-time WebSocket notifications upon money arrival and can monitor project progress bars and loan repayment checklists.

### Persona D: Self / Individual ("Personal Finances")
- **Profile:** Any user wanting to manage their private bank accounts and pocket expenses inside the same app without exposing personal finances to family scrutiny.
- **Solution:** Toggles between `PERSONAL` and `FAMILY` scopes with a single click. Row-Level Security (RLS) guarantees complete data isolation.

---

## 3. Core Domain Concepts

| Concept | Description |
| :--- | :--- |
| **Custody Wallet** | An account (Bank, MoMo, or Cash) held by a named custodian within a family. |
| **Dual Scope** | `PERSONAL` (private to user) vs `FAMILY` (shared ledger visible to authorized members). |
| **Handover** | Transfer of physical or digital funds between two custodians. Does not change total family net worth. |
| **Transaction** | Inflow (Income) or Outflow (Expense). Deducts or increments net balance and requires category & authorized actor. |
| **Mother's Report** | A pre-computed financial summary designed for family elders, accessible via zero-login magic tokens. |
| **Family Project** | Goal-oriented pooled fund with target amount, allocated contributions, and progress tracking. |
| **Family Loan** | Internal or external debt tracking with due date, debtor/creditor, and repayment history. |

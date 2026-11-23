# Feature Spec 03: Mother's Report & Zero-Login Magic Links

## 1. Objective
Provide a simplified, elder-friendly financial summary view that can be opened without passwords or accounts via secure tokenized links, optimized for viewing on mobile and sharing via WhatsApp.

---

## 2. Core Structure of the Mother's Report

The Mother's Report synthesizes complex ledger entries into four high-clarity sections:

1. **Spotlight Overview:**
   - Total Family Balance (Net Worth)
   - Monthly Inflow (Money Added)
   - Monthly Outflow (Legitimate Expenses)
2. **Where the Money Is (Cash Custody Breakdown):**
   - Exact balances held by each person:
     - Bank Deposits (Papa)
     - MoMo Balance (Mama)
     - Cash in Hand (Mama & Papa)
3. **Active Family Projects:**
   - List of ongoing goals (e.g., House Renovation, School Fees).
   - Target vs. Collected progress bar.
4. **Debts & Liabilities:**
   - Outstanding loans and deadlines.

---

## 3. Zero-Login Token Architecture

Elders should not be forced through password forms or SMS verification codes to check family transparency.

```
Family Admin -> Generate Magic Token -> Public URL -> Mom clicks link on WhatsApp -> Renders Report
```

### Security & Privacy Controls
- **Cryptographic Token:** A 32-byte secure random string generated using `rand::thread_rng()`.
- **Hashed in Database:** Stored in PostgreSQL as `token_hash = sha256(token)`.
- **Permissions:** Read-only access to pre-computed family summaries. No ability to mutate balances, create transactions, or access personal scopes.
- **Expiry & Revocation:** Family Admins can set expiration dates (e.g., 30 days) or revoke tokens immediately from their dashboard.

---

## 4. WhatsApp Sharing & Print Optimizations

### Formatted WhatsApp Text Export
Tapping *"Sangiza kuri WhatsApp"* encodes a readable summary message:
```text
*Raporo y'Imari y'Umuryango Mugisha & Keza*
📅 Itariki: 23 Ugushyingo 2026
🏛️ Umuryango: Umuryango Mugisha & Keza

💰 *Umutungo Wose:* 10,000,000 RWF
📱 MoMo: 1,450,000 RWF
💵 Mu Ntoki (Cash): 350,000 RWF
🏦 Muri Banki: 8,200,000 RWF
⚠️ Amadeni: 150,000 RWF

Reba incamake yose hano: https://ledger.umuryango.rw/rw/report/<token>
```

### Print-Ready Stylesheet
`@media print` rules in `web/src/app/globals.css` strip away headers, footers, and buttons, producing a crisp, clean A4 page suitable for printing or saving to PDF.

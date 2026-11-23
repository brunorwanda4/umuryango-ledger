# Feature Spec 05: Bilingual Localization & Real-Time Notifications

## 1. Objective
Deliver native, culturally authentic localization in Kinyarwanda (`rw`) and English (`en`) across backend API errors, native desktop/mobile screens, and web reports, synchronized in real time via WebSockets.

---

## 2. Fluent Catalog Architecture

All localization keys are stored in Mozilla Project Fluent format in:
- `locales/en.ftl`
- `locales/rw.ftl`

### Core Domain Terminology Map

| Key | English (`en`) | Kinyarwanda (`rw`) |
| :--- | :--- | :--- |
| `personal-scope` | Personal Finances | Imari Yanjye Bwite |
| `family-scope` | Family Ledger | Igitabo cy'Umuryango |
| `income` | Inflow / Income | Ibyinjira |
| `expense` | Outflow / Expense | Ibisohoka |
| `handover` | Custodian Handover | Ihererekanya ry'Umutungo |
| `custodian` | Custodian | Umubikizi |
| `held-by` | Held by | Uwabitse |
| `authorized-by` | Authorized by | Uwemereye |
| `paid-to` | Paid to | Uwahawe |
| `mothers-report-title` | Mother's Financial Summary | Raporo y'Ababyeyi n'Abakambwe |

### Interpolated Error Catalog
Errors provide dynamic context parameters:
- `error-insufficient-custody`:
  - `en`: "Wallet '{wallet}' only holds {available} RWF, which is not enough for {requested} RWF."
  - `rw`: "Konti '{wallet}' ifite {available} RWF gusa; ntihagije ngo ukoreshe {requested} RWF."
- `error-invalid-handover`:
  - `en`: "Handover must be between two different custody wallets within the same family."
  - `rw`: "Guhererekanya amafaranga bigomba gukorwa hagati ya konti ebyiri zitandukanye z'umuryango umwe."

---

## 3. Real-Time WebSocket Notifications

### WebSocket Protocol
- Connected URL: `GET /api/v1/ws?token=<jwt>`
- Broadcast Hub: `FamilyHub` in `crates/api-server`.

### Event Payloads & Bilingual Formatting

1. **`IncomeAdded`:**
   - `rw`: "Amafaranga Mashya Yakiriwe: {amount} RWF yinjijwe kuri {wallet_name} na {added_by_name}."
   - `en`: "New Inflow: {amount} RWF added to {wallet_name} by {added_by_name}."
2. **`ExpenseLogged`:**
   - `rw`: "Amafaranga Yakoreshejwe: {amount} RWF kuri {wallet_name} ({category}) na {logged_by_name}."
   - `en`: "Expense Logged: {amount} RWF on {wallet_name} ({category}) by {logged_by_name}."
3. **`HandoverCompleted`:**
   - `rw`: "Ihererekanya Ryemejwe: {amount} RWF kuva kuri {from_wallet_name} yageze kuri {to_wallet_name}."
   - `en`: "Handover Confirmed: {amount} RWF transferred from {from_wallet_name} to {to_wallet_name}."

### Toast Rendering
- On native GPUI: Banner appears at the top right of the application window.
- On Next.js Web: `ToastContainer` renders a floating alert in the active locale, updating cached wallet balances in real time.

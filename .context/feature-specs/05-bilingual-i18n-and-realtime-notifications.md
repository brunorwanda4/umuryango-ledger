# Feature Spec 05: Bilingual Localization & Real-Time Notifications

## 1. Objective
Deliver native, culturally authentic localization in Kinyarwanda (`rw`) and English (`en`) across backend API errors, native desktop/mobile screens, and web reports, synchronized in real time via WebSockets. Centralized structured JSON bundles serve as the single source of truth for all crates and the web client.

---

## 2. Centralized Structured JSON Architecture

All translations are maintained in two identically structured root JSON files:
- `locales/en.json` (English)
- `locales/rw.json` (Kinyarwanda)

### Single Source of Truth Across Layers
1. **`crates/i18n`:** Compile-time embedded via `include_str!`, parsed into a thread-safe `OnceLock<HashMap<Locale, serde_json::Value>>`, with dot-path resolution (`t(locale, "dashboard.title")`), parameter interpolation (`t_args(locale, "...", &[("key", "val")])`), fallback to English, and runtime hot-reloading for OTA bundles.
2. **`crates/wasm-bridge`:** Exposes `translate(locale, key, args_json)` backed by `crates/i18n`.
3. **`web/`:** Directly imports `locales/en.json` and `locales/rw.json` via `@/lib/i18n`, providing type-safe `t(locale, key, params)` and `useTranslation(locale)`.
4. **`crates/app-gpui`:** Invokes `umuryango_i18n::t` across all screens and components.
5. **`crates/api-server`:** Emits bilingual `LocalizedError` models with localized HTTP error bodies.

### Adding a New Language
To add a new language in the future:
1. Create `locales/<lang_code>.json` with the identical JSON structure.
2. Register the new variant in `enum Locale` in `crates/i18n/src/keys.rs`.
3. Add compile-time embedding in `crates/i18n/src/loader.rs`.
4. Automated parity tests (`crates/i18n/tests/locale_parity.rs`) will immediately verify 100% key parity and placeholder match.

### Namespaces
- `"meta"`: Locale metadata (`code`, `name`, `direction`).
- `"common"`: App name, core actions (save, cancel, delete, edit, confirm), network status (`online`, `offline`, `pending_sync`, `synced`).
- `"auth"`: Sign in, registration, form labels, success toasts.
- `"onboarding"`: Welcome flow, family creation, member invitations, join code.
- `"scopes"`: Personal money (`personal_money`) vs Family ledger (`family_ledger`).
- `"dashboard"`: KPI titles, breakdown headers, charts, recent activity.
- `"wallets"`: Channels (`channel_bank`, `channel_momo`, `channel_cash`), custodians, add wallet.
- `"transactions"`: Types (`income`, `expense`, `handover`), ledger table, modal labels.
- `"report"`: Mother's report title, balance summary, custody breakdown, WhatsApp sharing preview, link copy feedback.
- `"projects"`: Project tabs, demo projects, progress labels, new project.
- `"loans"`: Lending, borrowing, repayments, due date tags.
- `"future_plans"`: Long-term savings goals, targets, fund allocation.
- `"members"`: Family members, roles (`role_head`, `role_custodian`, `role_member`).
- `"settings"`: Theme settings, color palette tokens, typography, about.
- `"notifications"`: Realtime financial event templates with `{actor}`, `{amount}`, `{wallet}`.
- `"updater"`: Self-updating banners, version readiness, download and restart prompts.
- `"errors"`: Domain validation, authorization, custody constraints, HTTP error messages.

---

## 3. Real-Time WebSocket Notifications

### WebSocket Protocol
- Connected URL: `GET /api/v1/ws?token=<jwt>`
- Broadcast Hub: `FamilyHub` in `crates/api-server`.

### Event Payloads & Bilingual Formatting

1. **`IncomeAdded`:**
   - `rw`: `"{actor} yongeye {amount} RWF kuri konti '{wallet}' ({source})."`
   - `en`: `"{actor} added {amount} RWF to {wallet} ({source})."`

2. **`ExpenseLogged`:**
   - `rw`: `"{actor} yakoresheje {amount} RWF kuri konti '{wallet}' ({category})."`
   - `en`: `"{actor} recorded an expense of {amount} RWF from {wallet} ({category})."`

3. **`HandoverCompleted`:**
   - `rw`: `"{actor} yahererekanyije {amount} RWF kuva kuri {from_wallet} ijya kuri {to_wallet}."`
   - `en`: `"{actor} transferred {amount} RWF from {from_wallet} to {to_wallet}."`

4. **`AppUpdateAvailable`:**
   - Broadcast to all connected clients when a new desktop/mobile/web version or OTA translation bundle is published.

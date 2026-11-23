# Umuryango Ledger 🇷🇼

> **Multi-tenant family & personal finance accountability platform** with native English and Kinyarwanda localization, custody management, offline sync, and zero-login reporting.

---

## 🌟 Highlights

- **Dual Financial Scopes:** Strictly isolate private `PERSONAL` accounts from collaborative multi-tenant `FAMILY` ledgers.
- **Custody Management & Handovers:** Transparent tracking across Bank accounts, Mobile Money (`MoMo`), and Cash custodians. Supports formal Custodian-to-Custodian handovers with dual acknowledgment.
- **Bilingual By Design (`en` & `rw`):** Complete localization in English and Kinyarwanda across user interfaces, report summaries, and backend REST API error responses (via `Accept-Language: rw | en`).
- **Zero-Login Report Portal:** Secure, tokenized web links allowing non-technical family members (e.g., parents/elders) to review family financial health and project status without passwords.
- **Multi-Platform Architecture:**
  - **Backend:** High-performance Rust (`actix-web`, `sqlx`, PostgreSQL with RLS).
  - **Desktop / Mobile:** Rust `gpui` client with local SQLite offline cache and sync queue.
  - **Web & Zero-Login Portal:** Next.js (App Router, TypeScript) + WebAssembly bridge (`wasm-bindgen`).

---

## 📁 Repository Structure

```text
umuryango-ledger/
├── Cargo.toml                              # Workspace configuration (resolver = "2")
├── .env.example                            # Sample configuration variables
├── docker-compose.yml                      # PostgreSQL 16 + MinIO storage setup
├── README.md                               # Project documentation
│
├── locales/                                # Fluent i18n translation files
│   ├── en.ftl                              # English translations & error catalog
│   └── rw.ftl                              # Kinyarwanda translations & error catalog
│
├── migrations/                             # PostgreSQL schema migrations (SQLx)
│   ├── 2026112301_users_and_locales.sql
│   ├── 2026112302_families_and_members.sql
│   ├── 2026112303_wallets_personal_and_custody.sql
│   ├── 2026112304_transactions_and_handovers.sql
│   ├── 2026112305_projects_loans_future_plans.sql
│   └── 2026112306_enable_rls_policies.sql
│
├── crates/                                 # Rust modular crates
│   ├── i18n/                              # Fluent-based localized catalog & message resolution
│   ├── core-domain/                       # Financial models, double-entry validation, balance calculation
│   ├── storage-local/                     # SQLite offline storage & sync queue for clients
│   ├── api-client/                        # Cross-platform HTTP/WebSocket API client
│   ├── api-server/                        # Actix-web REST API server with RLS & bilingual error middleware
│   ├── wasm-bridge/                       # WebAssembly interface for browser-side calculations
│   └── app-gpui/                          # Native desktop/mobile app powered by Zed's GPUI
│
└── web/                                    # Next.js App Router bilingual web application
    ├── package.json
    ├── next.config.ts
    ├── tsconfig.json
    └── src/
        ├── app/[locale]/                  # Locale-routed pages (en/rw)
        ├── components/                    # Bilingual components (e.g. MothersReportCard, ScopeSwitcher)
        └── lib/                           # WebAssembly bindings and API client
```

---

## 🚀 Getting Started

### 1. Prerequisites
- **Rust Toolchain:** `rustc 1.80+` / `cargo`
- **Node.js:** `v20+` & `npm` / `pnpm`
- **Docker & Docker Compose** (for PostgreSQL & MinIO)

### 2. Environment Setup
```bash
cp .env.example .env
docker compose up -d
```

### 3. Database Migrations
Run the SQLx migrations:
```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

### 4. Build and Check Rust Crates
```bash
cargo check --workspace
```

### 5. Run the Web Application
```bash
cd web
npm install
npm run dev
```

Visit [http://localhost:3000/rw](http://localhost:3000/rw) for Kinyarwanda or [http://localhost:3000/en](http://localhost:3000/en) for English.

---

## 📜 Localization & Domain Concepts

| Concept | English (`en`) | Kinyarwanda (`rw`) |
| :--- | :--- | :--- |
| **Family Ledger** | Family Ledger | Igitabo cy'Umuryango |
| **Personal Scope** | Personal Finances | Imari Yanjye |
| **Custody Wallet** | Custody Wallet | Umutungo Ubitswe |
| **Handover** | Custodian Handover | Guhererekanya Umutungo |
| **Project** | Family Project | Umushinga w'Umuryango |
| **Emergency Fund** | Emergency Fund | Ikigega cy'Ubutabazi |
| **Future Plan** | Future Plan | Imigambi y'Ejo Hazaza |

---

## 🔒 Security & Dual Scope Isolation

- Multi-tenant tenant boundaries are reinforced through **PostgreSQL Row Level Security (RLS)**.
- Every transaction explicitly belongs to either a `PERSONAL` user scope or a `FAMILY` shared scope.
- Custody wallets record who holds the physical cash or electronic balance (`MoMo`/Bank), requiring dual acknowledgment when transferred.

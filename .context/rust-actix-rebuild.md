# Rust Actix-web Backend Standards — Umuryango Ledger

This document defines the architecture, routing, middleware, and database transaction standards for `crates/api-server`.

---

## 1. Application State & Lifecycle

The central application state is held in `AppState`:

```rust
pub struct AppState {
    pub db: sqlx::PgPool,
    pub jwt_secret: String,
    pub public_web_url: String,
    pub family_hub: Arc<FamilyHub>,
}
```

- Initialize `PgPoolOptions` with sensible connection limits (e.g., `max_connections(10)`).
- Mount route groups under `/api/v1/`:
  - `/api/v1/auth`: Registration, login, locale settings.
  - `/api/v1/families`: Family groups, invitations, and member roles.
  - `/api/v1/wallets`: Custody wallet management and balance queries.
  - `/api/v1/transactions`: Inflow/outflow logging with balance updates.
  - `/api/v1/projects`: Family development projects and milestones.
  - `/api/v1/loans`: Zero-embarrassment personal and family loans.
  - `/api/v1/plans`: Future financial goals and allocations.
  - `/api/v1/reports`: Mother's Report generation & zero-login public endpoints.
  - `/api/v1/releases`: Over-the-air updates and binary release checks.
  - `/api/v1/ws`: Real-time WebSocket connection.

---

## 2. Bilingual Error Handling & Locale Extraction

### Request Locale Extractor
Handlers receive the client's language preference using `RequestLocale`:

```rust
pub struct RequestLocale(pub umuryango_i18n::Locale);

impl FromRequest for RequestLocale {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Reads 'Accept-Language' header or '?lang=rw|en' query param
        let locale = Locale::from_request_headers(req);
        ready(Ok(RequestLocale(locale)))
    }
}
```

### ResponseError Implementation
Every API error implements `actix_web::ResponseError`:
- Extracts the error code from `LocalizableError`.
- Formats a localized message via `Translator::t(locale, code, args)`.
- Returns a structured JSON response:
  ```json
  {
    "code": "error-insufficient-custody",
    "message": "Konti 'BK Main' ifite 10,000 RWF gusa; ntihagije ngo ukoreshe 50,000 RWF.",
    "status": 422
  }
  ```

---

## 3. Atomic Custody Handovers

Handovers between custodians must be transactional and prevent race conditions:

```rust
// 1. Begin SQLx transaction
let mut tx = pool.begin().await?;

// 2. Lock both wallets with FOR UPDATE
let source_wallet = sqlx::query_as!(
    WalletRecord,
    "SELECT * FROM wallets WHERE id = $1 FOR UPDATE",
    req.from_wallet_id
)
.fetch_one(&mut *tx)
.await?;

let target_wallet = sqlx::query_as!(
    WalletRecord,
    "SELECT * FROM wallets WHERE id = $1 FOR UPDATE",
    req.to_wallet_id
)
.fetch_one(&mut *tx)
.await?;

// 3. Verify same family tenancy & sufficient custody
if source_wallet.family_id != target_wallet.family_id {
    return Err(AppError::InvalidHandover);
}
if source_wallet.balance < req.amount {
    return Err(AppError::InsufficientCustody { ... });
}

// 4. Update balances atomically
sqlx::query!(
    "UPDATE wallets SET balance = balance - $1 WHERE id = $2",
    req.amount, source_wallet.id
).execute(&mut *tx).await?;

sqlx::query!(
    "UPDATE wallets SET balance = balance + $1 WHERE id = $2",
    req.amount, target_wallet.id
).execute(&mut *tx).await?;

// 5. Commit
tx.commit().await?;
```

---

## 4. Real-Time WebSocket Hub (`FamilyHub`)

- In-memory broadcast routing using `DashMap<Uuid, broadcast::Sender<RealtimeEvent>>`.
- When an income, expense, or handover is successfully committed:
  ```rust
  app_state.family_hub.broadcast(
      family_id,
      RealtimeEvent::IncomeAdded { ... }
  );
  ```
- The WebSocket handler subscribes connected family members to their family channels and streams events over `actix-ws`.
- Includes periodic ping/pong frames to detect dead connections.

---

## 5. OpenAPI 3.1 & Interactive Swagger UI Documentation

The backend exposes full, type-safe OpenAPI 3.1 interactive documentation via `utoipa 6.0` and `utoipa-swagger-ui 10.0`:
- **Centralized Spec (`crates/api-server/src/openapi.rs`):** Defines `ApiDoc` with `SecurityAddon`.
- **Security Schemes:**
  - `bearer_auth`: HTTP Bearer JWT token header (`Authorization: Bearer <token>`).
  - `admin_key`: API Key header (`X-Admin-Key`) for privileged release management.
  - Public routes (`/api/v1/reports/shared/{token}`, `/api/v1/releases/check`, `/api/v1/ota/bundle`) configured with `security(())` so Swagger UI shows no authentication requirement.
- **Interactive UI:** Mounted at `/swagger-ui/` pointing to `/api-docs/openapi.json`.
- **Bilingual Header Parameter:** All endpoints document the `Accept-Language` header (`rw` or `en`) for localized responses and validation error messages.

use actix_web::{HttpRequest, HttpResponse, web};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use umuryango_core_domain::ledger::LedgerValidator;
use umuryango_core_domain::models::{
    FamilyRole, Transaction, TransactionType, Wallet, WalletChannel,
};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_core_domain::{DomainError, RealtimeEvent};
use umuryango_i18n::Locale;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::error::{ApiError, LocalizedErrorResponse};
use crate::middleware::{AuthenticatedUser, require_family_membership, set_current_user_context};
use crate::state::AppState;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTransactionPayload {
    #[schema(example = "family")]
    pub scope: String, // "personal" or "family"
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub family_id: Option<Uuid>,
    #[schema(value_type = String, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440001")]
    pub source_wallet_id: Uuid,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440002")]
    pub destination_wallet_id: Option<Uuid>,
    pub tx_type: TransactionType,
    #[schema(value_type = String, example = "45000.00")]
    pub amount: Decimal,
    #[schema(example = "Groceries")]
    pub category: String,
    #[schema(example = "Weekly market shopping")]
    pub description: Option<String>,
    #[schema(example = "Kimironko Market Vendor")]
    pub paid_to: Option<String>,
    #[schema(example = "https://storage.umuryango.rw/receipts/rec-123.jpg")]
    pub receipt_url: Option<String>,
    pub tx_date: Option<DateTime<Utc>>,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440003")]
    pub idempotency_key: Option<Uuid>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListTransactionsQuery {
    #[schema(example = "family")]
    pub scope: Option<String>,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub family_id: Option<Uuid>,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440001")]
    pub wallet_id: Option<Uuid>,
    #[schema(example = "EXPENSE")]
    pub tx_type: Option<String>,
    #[schema(example = "Groceries")]
    pub category: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionExecutionResult {
    pub transaction: Transaction,
    #[schema(value_type = String, example = "205000.00")]
    pub source_wallet_balance: Decimal,
    #[schema(value_type = Option<String>, example = "45000.00")]
    pub destination_wallet_balance: Option<Decimal>,
}

#[derive(sqlx::FromRow)]
struct LockedWalletRow {
    id: Uuid,
    scope: String,
    user_id: Option<Uuid>,
    family_id: Option<Uuid>,
    wallet_type: String,
    name: String,
    account_number: Option<String>,
    currency: String,
    balance: Decimal,
    custodian_id: Option<Uuid>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl LockedWalletRow {
    fn into_domain(self) -> Wallet {
        let channel = match self.wallet_type.to_lowercase().as_str() {
            "bank" => WalletChannel::Bank,
            "momo" | "mobile_money" => WalletChannel::MobileMoney,
            _ => WalletChannel::CashInHand,
        };
        let scope = if self.scope.eq_ignore_ascii_case("family") {
            LedgerScope::Family {
                family_id: self.family_id.unwrap_or_default(),
            }
        } else {
            LedgerScope::Personal {
                user_id: self.user_id.unwrap_or_default(),
            }
        };

        Wallet {
            id: self.id,
            scope,
            channel,
            name: self.name,
            account_number: self.account_number,
            currency: self.currency,
            balance: self.balance,
            custodian_id: self.custodian_id,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct ExistingTxRow {
    id: Uuid,
    scope: String,
    user_id: Uuid,
    family_id: Option<Uuid>,
    source_wallet_id: Uuid,
    destination_wallet_id: Option<Uuid>,
    tx_type: String,
    amount: Decimal,
    currency: String,
    category: String,
    description: Option<String>,
    paid_to: Option<String>,
    tx_date: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl ExistingTxRow {
    fn into_domain(self) -> Transaction {
        let t_type = match self.tx_type.to_lowercase().as_str() {
            "income" => TransactionType::Income,
            "expense" => TransactionType::Expense,
            _ => TransactionType::Handover,
        };
        let scope = if self.scope.eq_ignore_ascii_case("family") {
            LedgerScope::Family {
                family_id: self.family_id.unwrap_or_default(),
            }
        } else {
            LedgerScope::Personal {
                user_id: self.user_id,
            }
        };
        Transaction {
            id: self.id,
            scope,
            source_wallet_id: self.source_wallet_id,
            destination_wallet_id: self.destination_wallet_id,
            tx_type: t_type,
            amount: self.amount,
            currency: self.currency,
            category: self.category,
            description: self.description,
            authorized_by: self.user_id,
            paid_to: self.paid_to,
            tx_date: self.tx_date,
            created_at: self.created_at,
        }
    }
}

async fn find_existing_by_idempotency_key(
    db: &sqlx::PgPool,
    key: Uuid,
    loc: Locale,
) -> Result<Option<TransactionExecutionResult>, ApiError> {
    let existing = sqlx::query_as::<_, ExistingTxRow>(
        r#"
        SELECT id, scope::TEXT as scope, user_id, family_id, source_wallet_id, destination_wallet_id,
               tx_type::TEXT as tx_type, amount, currency, category, description, NULL::TEXT as paid_to, tx_date, created_at
        FROM transactions
        WHERE idempotency_key = $1
        "#,
    )
    .bind(key)
    .fetch_optional(db)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    if let Some(row) = existing {
        let src_balance =
            sqlx::query_scalar::<_, Decimal>("SELECT balance FROM wallets WHERE id = $1")
                .bind(row.source_wallet_id)
                .fetch_one(db)
                .await
                .unwrap_or(Decimal::ZERO);

        let dest_balance = match row.destination_wallet_id {
            Some(did) => {
                sqlx::query_scalar::<_, Decimal>("SELECT balance FROM wallets WHERE id = $1")
                    .bind(did)
                    .fetch_optional(db)
                    .await
                    .ok()
                    .flatten()
            }
            None => None,
        };

        Ok(Some(TransactionExecutionResult {
            transaction: row.into_domain(),
            source_wallet_balance: src_balance,
            destination_wallet_balance: dest_balance,
        }))
    } else {
        Ok(None)
    }
}

/// Create an idempotent double-entry or single-entry transaction
#[utoipa::path(
    post,
    path = "/api/v1/transactions",
    tag = "Transactions",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateTransactionPayload,
    params(
        ("X-Idempotency-Key" = Option<String>, Header, description = "Optional idempotency key (UUID) to prevent duplicate transactions"),
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 201, description = "Transaction recorded successfully", body = TransactionExecutionResult),
        (status = 200, description = "Transaction previously executed (idempotent result)", body = TransactionExecutionResult),
        (status = 400, description = "Bad request or validation error", body = LocalizedErrorResponse),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 403, description = "Forbidden (insufficient wallet custody permission)", body = LocalizedErrorResponse),
        (status = 409, description = "Insufficient funds or domain rule violation", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn create_transaction(
    req: HttpRequest,
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<CreateTransactionPayload>,
) -> Result<HttpResponse, ApiError> {
    let loc = user.locale;

    // 1. Check idempotency key from header or body
    let idempotency_key = req
        .headers()
        .get("X-Idempotency-Key")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .or(payload.idempotency_key);

    if let Some(key) = idempotency_key {
        if let Some(res) = find_existing_by_idempotency_key(&data.db, key, loc).await? {
            return Ok(HttpResponse::Ok().json(res));
        }
    }

    let is_family = payload.scope.trim().eq_ignore_ascii_case("family");

    let scope = if is_family {
        let Some(fid) = payload.family_id else {
            return Err(ApiError::conflict(
                "family_id is required for family transactions",
                loc,
            ));
        };
        LedgerScope::Family { family_id: fid }
    } else {
        LedgerScope::Personal {
            user_id: user.user_id,
        }
    };

    // Determine user role in family if scope is Family
    let actor_role = if let LedgerScope::Family { family_id } = scope {
        let member = require_family_membership(&data.db, user.user_id, family_id, loc).await?;
        member.role
    } else {
        FamilyRole::Head // Personal owner acts with full privileges
    };

    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::database(e, loc))?;

    // Set PostgreSQL session context for RLS policies
    set_current_user_context(&mut tx, user.user_id)
        .await
        .map_err(|e| ApiError::database(e, loc))?;

    // Select involved wallet(s) FOR UPDATE
    let mut wallet_ids = vec![payload.source_wallet_id];
    if let Some(dest_id) = payload.destination_wallet_id {
        if dest_id != payload.source_wallet_id {
            wallet_ids.push(dest_id);
        }
    }

    let rows = sqlx::query_as::<_, LockedWalletRow>(
        r#"
        SELECT id, scope::TEXT as scope, user_id, family_id, wallet_type::TEXT as wallet_type,
               name, account_number, currency, balance, custodian_id, is_active, created_at, updated_at
        FROM wallets
        WHERE id = ANY($1)
        FOR UPDATE
        "#,
    )
    .bind(&wallet_ids)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    if rows.len() != wallet_ids.len() {
        return Err(ApiError::domain(DomainError::NotFound, loc));
    }

    let mut domain_wallets = rows
        .into_iter()
        .map(LockedWalletRow::into_domain)
        .collect::<Vec<_>>();

    let tx_id = Uuid::new_v4();
    let tx_date = payload.tx_date.unwrap_or_else(Utc::now);
    let now = Utc::now();

    let domain_tx = Transaction {
        id: tx_id,
        scope: scope.clone(),
        source_wallet_id: payload.source_wallet_id,
        destination_wallet_id: payload.destination_wallet_id,
        tx_type: payload.tx_type,
        amount: payload.amount,
        currency: "RWF".to_string(),
        category: payload.category.clone(),
        description: payload.description.clone(),
        authorized_by: user.user_id,
        paid_to: payload.paid_to.clone(),
        tx_date,
        created_at: now,
    };

    // Validate and apply invariants via core-domain
    LedgerValidator::validate_and_apply_transaction(&domain_tx, actor_role, &mut domain_wallets)
        .map_err(|e| ApiError::domain(e, loc))?;

    // Persist updated wallet balances
    for w in &domain_wallets {
        sqlx::query(
            r#"
            UPDATE wallets
            SET balance = $1, updated_at = $2
            WHERE id = $3
            "#,
        )
        .bind(w.balance)
        .bind(now)
        .bind(w.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::database(e, loc))?;
    }

    // Persist transaction record
    let type_str = match payload.tx_type {
        TransactionType::Income => "income",
        TransactionType::Expense => "expense",
        TransactionType::Handover => "custody_handover",
    };

    let fid_opt = scope.family_id();
    let scope_db = scope.as_str();

    let insert_res = sqlx::query(
        r#"
        INSERT INTO transactions (
            id, scope, user_id, family_id, source_wallet_id, destination_wallet_id,
            tx_type, amount, currency, category, description, receipt_url, idempotency_key, tx_date, created_at
        )
        VALUES ($1, $2::financial_scope, $3, $4, $5, $6, $7::transaction_type, $8, 'RWF', $9, $10, $11, $12, $13, $14)
        "#,
    )
    .bind(tx_id)
    .bind(scope_db)
    .bind(user.user_id)
    .bind(fid_opt)
    .bind(payload.source_wallet_id)
    .bind(payload.destination_wallet_id)
    .bind(type_str)
    .bind(payload.amount)
    .bind(&payload.category)
    .bind(&payload.description)
    .bind(&payload.receipt_url)
    .bind(idempotency_key)
    .bind(tx_date)
    .bind(now)
    .execute(&mut *tx)
    .await;

    if let Err(e) = insert_res {
        let _ = tx.rollback().await;
        if let sqlx::Error::Database(ref db_err) = e {
            if db_err.code().as_deref() == Some("23505") {
                if let Some(key) = idempotency_key {
                    if let Some(res) = find_existing_by_idempotency_key(&data.db, key, loc).await? {
                        return Ok(HttpResponse::Ok().json(res));
                    }
                }
            }
        }
        return Err(ApiError::database(e, loc));
    }

    // If Handover, also record in custody_handovers table for formal tracking
    if payload.tx_type == TransactionType::Handover {
        if let Some(fid) = fid_opt {
            let dest_custodian = domain_wallets
                .iter()
                .find(|w| Some(w.id) == payload.destination_wallet_id)
                .and_then(|w| w.custodian_id)
                .unwrap_or(user.user_id);

            sqlx::query(
                r#"
                INSERT INTO custody_handovers (
                    id, family_id, wallet_id, from_custodian_id, to_custodian_id,
                    amount, currency, status, notes, initiated_at, acknowledged_at, acknowledged_by, created_at, updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, 'RWF', 'confirmed', $7, $8, $8, $4, $8, $8)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(fid)
            .bind(payload.source_wallet_id)
            .bind(user.user_id)
            .bind(dest_custodian)
            .bind(payload.amount)
            .bind(&payload.description)
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(|e| ApiError::database(e, loc))?;
        }
    }

    tx.commit().await.map_err(|e| ApiError::database(e, loc))?;

    let src_balance = domain_wallets
        .iter()
        .find(|w| w.id == payload.source_wallet_id)
        .map(|w| w.balance)
        .unwrap_or(Decimal::ZERO);

    let dest_balance = payload.destination_wallet_id.and_then(|did| {
        domain_wallets
            .iter()
            .find(|w| w.id == did)
            .map(|w| w.balance)
    });

    // 2. Publish RealtimeEvent to FamilyHub if this was a family transaction
    if let LedgerScope::Family { family_id } = scope {
        let user_name =
            sqlx::query_scalar::<_, String>("SELECT full_name FROM users WHERE id = $1")
                .bind(user.user_id)
                .fetch_one(&data.db)
                .await
                .unwrap_or_else(|_| "Family Member".to_string());

        let source_wallet = domain_wallets
            .iter()
            .find(|w| w.id == payload.source_wallet_id);
        let source_name = source_wallet
            .map(|w| w.name.clone())
            .unwrap_or_else(|| "Wallet".to_string());

        let dest_wallet = payload
            .destination_wallet_id
            .and_then(|did| domain_wallets.iter().find(|w| w.id == did));
        let dest_name = dest_wallet.map(|w| w.name.clone()).unwrap_or_default();

        let event = match payload.tx_type {
            TransactionType::Income => RealtimeEvent::IncomeAdded {
                family_id,
                wallet_id: payload.source_wallet_id,
                wallet_name: source_name,
                amount: payload.amount,
                currency: "RWF".to_string(),
                added_by_name: user_name,
                source_description: payload
                    .description
                    .clone()
                    .unwrap_or_else(|| payload.category.clone()),
                new_wallet_balance: src_balance,
                occurred_at: tx_date,
            },
            TransactionType::Expense => RealtimeEvent::ExpenseLogged {
                family_id,
                wallet_id: payload.source_wallet_id,
                wallet_name: source_name,
                amount: payload.amount,
                currency: "RWF".to_string(),
                logged_by_name: user_name,
                category: payload.category.clone(),
                new_wallet_balance: src_balance,
                occurred_at: tx_date,
            },
            TransactionType::Handover => RealtimeEvent::HandoverCompleted {
                family_id,
                from_wallet_name: source_name,
                to_wallet_name: dest_name,
                amount: payload.amount,
                currency: "RWF".to_string(),
                initiated_by_name: user_name,
                occurred_at: tx_date,
            },
        };

        data.family_hub.broadcast(family_id, event);
    }

    Ok(HttpResponse::Created().json(TransactionExecutionResult {
        transaction: domain_tx,
        source_wallet_balance: src_balance,
        destination_wallet_balance: dest_balance,
    }))
}

/// List transactions matching filters for personal or family scope
#[utoipa::path(
    get,
    path = "/api/v1/transactions",
    tag = "Transactions",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ListTransactionsQuery,
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "List of transactions retrieved", body = Vec<Transaction>),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 409, description = "Conflict or missing parameters", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn list_transactions(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<ListTransactionsQuery>,
) -> Result<HttpResponse, ApiError> {
    let loc = user.locale;
    let is_family = query
        .scope
        .as_deref()
        .map(|s| s.eq_ignore_ascii_case("family"))
        .unwrap_or(false);

    if is_family {
        let Some(fid) = query.family_id else {
            return Err(ApiError::conflict(
                "family_id is required for family transactions query",
                loc,
            ));
        };
        require_family_membership(&data.db, user.user_id, fid, loc).await?;
    }

    #[derive(sqlx::FromRow)]
    struct TxRow {
        id: Uuid,
        scope: String,
        user_id: Uuid,
        family_id: Option<Uuid>,
        source_wallet_id: Uuid,
        destination_wallet_id: Option<Uuid>,
        tx_type: String,
        amount: Decimal,
        currency: String,
        category: String,
        description: Option<String>,
        _receipt_url: Option<String>,
        tx_date: DateTime<Utc>,
        created_at: DateTime<Utc>,
    }

    let rows = sqlx::query_as::<_, TxRow>(
        r#"
        SELECT id, scope::TEXT as scope, user_id, family_id, source_wallet_id, destination_wallet_id,
               tx_type::TEXT as tx_type, amount, currency, category, description, receipt_url as _receipt_url, tx_date, created_at
        FROM transactions
        WHERE (
            ($1 = TRUE AND scope = 'family' AND family_id = $2)
            OR ($1 = FALSE AND scope = 'personal' AND user_id = $3)
        )
        AND ($4::UUID IS NULL OR source_wallet_id = $4 OR destination_wallet_id = $4)
        AND ($5::TIMESTAMPTZ IS NULL OR tx_date >= $5)
        AND ($6::TIMESTAMPTZ IS NULL OR tx_date <= $6)
        AND ($7::TEXT IS NULL OR tx_type::TEXT = $7)
        AND ($8::TEXT IS NULL OR category = $8)
        ORDER BY tx_date DESC
        LIMIT 100
        "#,
    )
    .bind(is_family)
    .bind(query.family_id)
    .bind(user.user_id)
    .bind(query.wallet_id)
    .bind(query.from)
    .bind(query.to)
    .bind(query.tx_type.as_deref())
    .bind(query.category.as_deref())
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    let txs = rows
        .into_iter()
        .map(|r| {
            let t_type = match r.tx_type.to_lowercase().as_str() {
                "income" => TransactionType::Income,
                "expense" => TransactionType::Expense,
                _ => TransactionType::Handover,
            };
            let scope = if r.scope.eq_ignore_ascii_case("family") {
                LedgerScope::Family {
                    family_id: r.family_id.unwrap_or_default(),
                }
            } else {
                LedgerScope::Personal { user_id: r.user_id }
            };
            Transaction {
                id: r.id,
                scope,
                source_wallet_id: r.source_wallet_id,
                destination_wallet_id: r.destination_wallet_id,
                tx_type: t_type,
                amount: r.amount,
                currency: r.currency,
                category: r.category,
                description: r.description,
                authorized_by: r.user_id,
                paid_to: None,
                tx_date: r.tx_date,
                created_at: r.created_at,
            }
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(txs))
}

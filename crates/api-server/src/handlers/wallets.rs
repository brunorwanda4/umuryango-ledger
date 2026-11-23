use actix_web::{HttpResponse, web};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use umuryango_core_domain::models::{FamilyRole, Wallet, WalletChannel};
use umuryango_core_domain::scope::LedgerScope;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::error::{ApiError, LocalizedErrorResponse};
use crate::middleware::{AuthenticatedUser, require_family_membership, require_role};
use crate::state::AppState;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListWalletsQuery {
    #[schema(example = "family")]
    pub scope: Option<String>,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub family_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWalletRequest {
    #[schema(example = "family")]
    pub scope: String, // "personal" or "family"
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub family_id: Option<Uuid>,
    pub channel: WalletChannel,
    #[schema(example = "BK Main Account")]
    pub name: String,
    #[schema(example = "00040-023948-23")]
    pub account_number: Option<String>,
    #[schema(example = "RWF")]
    pub currency: Option<String>,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub custodian_id: Option<Uuid>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CustodyWalletView {
    #[schema(value_type = String, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    pub scope: LedgerScope,
    pub channel: WalletChannel,
    #[schema(example = "BK Main Account")]
    pub name: String,
    #[schema(example = "00040-023948-23")]
    pub account_number: Option<String>,
    #[schema(example = "RWF")]
    pub currency: String,
    #[schema(value_type = String, example = "250000.00")]
    pub balance: Decimal,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub custodian_id: Option<Uuid>,
    #[schema(example = "Bruno Hapiman")]
    pub custodian_name: Option<String>,
    #[schema(example = true)]
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// List personal or family wallets with custody status
#[utoipa::path(
    get,
    path = "/api/v1/wallets",
    tag = "Wallets",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ListWalletsQuery,
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "List of wallets retrieved", body = Vec<CustodyWalletView>),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 409, description = "Conflict or missing parameters", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn list_wallets(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<ListWalletsQuery>,
) -> Result<HttpResponse, ApiError> {
    let scope_str = query
        .scope
        .as_deref()
        .unwrap_or("personal")
        .trim()
        .to_lowercase();

    if scope_str == "family" {
        let Some(family_id) = query.family_id else {
            let msg = umuryango_i18n::t(user.locale, "errors.family_id_required");
            return Err(ApiError::conflict(msg, user.locale));
        };

        // Verify membership in family
        require_family_membership(&data.db, user.user_id, family_id, user.locale).await?;

        #[derive(sqlx::FromRow)]
        struct FamilyWalletRow {
            id: Uuid,
            name: String,
            wallet_type: String,
            account_number: Option<String>,
            currency: String,
            balance: Decimal,
            custodian_id: Option<Uuid>,
            custodian_name: Option<String>,
            is_active: bool,
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
        }

        let rows = sqlx::query_as::<_, FamilyWalletRow>(
            r#"
            SELECT w.id, w.name, w.wallet_type::TEXT as wallet_type, w.account_number,
                   w.currency, w.balance, w.custodian_id, u.full_name as custodian_name,
                   w.is_active, w.created_at, w.updated_at
            FROM wallets w
            LEFT JOIN users u ON w.custodian_id = u.id
            WHERE w.scope = 'family' AND w.family_id = $1 AND w.is_active = TRUE
            ORDER BY w.name ASC
            "#,
        )
        .bind(family_id)
        .fetch_all(&data.db)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

        let views = rows
            .into_iter()
            .map(|r| {
                let channel = match r.wallet_type.to_lowercase().as_str() {
                    "bank" => WalletChannel::Bank,
                    "momo" | "mobile_money" => WalletChannel::MobileMoney,
                    _ => WalletChannel::CashInHand,
                };
                CustodyWalletView {
                    id: r.id,
                    scope: LedgerScope::Family { family_id },
                    channel,
                    name: r.name,
                    account_number: r.account_number,
                    currency: r.currency,
                    balance: r.balance,
                    custodian_id: r.custodian_id,
                    custodian_name: r.custodian_name,
                    is_active: r.is_active,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                }
            })
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().json(views))
    } else {
        // Personal Wallets
        #[derive(sqlx::FromRow)]
        struct PersonalWalletRow {
            id: Uuid,
            name: String,
            wallet_type: String,
            account_number: Option<String>,
            currency: String,
            balance: Decimal,
            is_active: bool,
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
        }

        let rows = sqlx::query_as::<_, PersonalWalletRow>(
            r#"
            SELECT id, name, wallet_type::TEXT as wallet_type, account_number,
                   currency, balance, is_active, created_at, updated_at
            FROM wallets
            WHERE scope = 'personal' AND user_id = $1 AND is_active = TRUE
            ORDER BY name ASC
            "#,
        )
        .bind(user.user_id)
        .fetch_all(&data.db)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

        let views = rows
            .into_iter()
            .map(|r| {
                let channel = match r.wallet_type.to_lowercase().as_str() {
                    "bank" => WalletChannel::Bank,
                    "momo" | "mobile_money" => WalletChannel::MobileMoney,
                    _ => WalletChannel::CashInHand,
                };
                CustodyWalletView {
                    id: r.id,
                    scope: LedgerScope::Personal {
                        user_id: user.user_id,
                    },
                    channel,
                    name: r.name,
                    account_number: r.account_number,
                    currency: r.currency,
                    balance: r.balance,
                    custodian_id: Some(user.user_id),
                    custodian_name: None,
                    is_active: r.is_active,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                }
            })
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().json(views))
    }
}

/// Create a new personal or family custody wallet
#[utoipa::path(
    post,
    path = "/api/v1/wallets",
    tag = "Wallets",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateWalletRequest,
    params(
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 201, description = "Wallet created successfully", body = Wallet),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 403, description = "Forbidden (insufficient role)", body = LocalizedErrorResponse),
        (status = 409, description = "Conflict or validation error", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn create_wallet(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<CreateWalletRequest>,
) -> Result<HttpResponse, ApiError> {
    let now = Utc::now();
    let wallet_id = Uuid::new_v4();
    let currency = payload
        .currency
        .clone()
        .unwrap_or_else(|| "RWF".to_string());
    let type_str = match payload.channel {
        WalletChannel::Bank => "bank",
        WalletChannel::MobileMoney => "momo",
        WalletChannel::CashInHand => "cash",
    };

    let is_family = payload.scope.trim().eq_ignore_ascii_case("family");

    if is_family {
        let Some(family_id) = payload.family_id else {
            let msg = umuryango_i18n::t(user.locale, "errors.family_id_required");
            return Err(ApiError::conflict(msg, user.locale));
        };

        // Caller must be HEAD or CUSTODIAN to provision wallets
        let member =
            require_family_membership(&data.db, user.user_id, family_id, user.locale).await?;
        require_role(
            &member,
            &[FamilyRole::Head, FamilyRole::Custodian],
            user.locale,
        )?;

        let custodian = payload.custodian_id.unwrap_or(user.user_id);

        sqlx::query(
            r#"
            INSERT INTO wallets (id, scope, user_id, family_id, wallet_type, name, account_number, currency, balance, custodian_id, is_active, created_at, updated_at)
            VALUES ($1, 'family', NULL, $2, $3::wallet_type, $4, $5, $6, 0.0000, $7, TRUE, $8, $8)
            "#,
        )
        .bind(wallet_id)
        .bind(family_id)
        .bind(type_str)
        .bind(&payload.name)
        .bind(&payload.account_number)
        .bind(&currency)
        .bind(custodian)
        .bind(now)
        .execute(&data.db)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

        let wallet = Wallet {
            id: wallet_id,
            scope: LedgerScope::Family { family_id },
            channel: payload.channel,
            name: payload.name.clone(),
            account_number: payload.account_number.clone(),
            currency,
            balance: Decimal::ZERO,
            custodian_id: Some(custodian),
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        Ok(HttpResponse::Created().json(wallet))
    } else {
        // Personal Wallet
        sqlx::query(
            r#"
            INSERT INTO wallets (id, scope, user_id, family_id, wallet_type, name, account_number, currency, balance, custodian_id, is_active, created_at, updated_at)
            VALUES ($1, 'personal', $2, NULL, $3::wallet_type, $4, $5, $6, 0.0000, $2, TRUE, $7, $7)
            "#,
        )
        .bind(wallet_id)
        .bind(user.user_id)
        .bind(type_str)
        .bind(&payload.name)
        .bind(&payload.account_number)
        .bind(&currency)
        .bind(now)
        .execute(&data.db)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

        let wallet = Wallet {
            id: wallet_id,
            scope: LedgerScope::Personal {
                user_id: user.user_id,
            },
            channel: payload.channel,
            name: payload.name.clone(),
            account_number: payload.account_number.clone(),
            currency,
            balance: Decimal::ZERO,
            custodian_id: Some(user.user_id),
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        Ok(HttpResponse::Created().json(wallet))
    }
}

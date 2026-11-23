use actix_web::{HttpResponse, web};
use chrono::{DateTime, Duration, NaiveDate, Utc};
use rand::{Rng, distributions::Alphanumeric};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use umuryango_core_domain::models::{
    FamilyRole, FuturePlan, Loan, LoanDirection, LoanStatus, Project, ProjectStatus, Transaction,
    TransactionType, Wallet, WalletChannel,
};
use umuryango_core_domain::reporting::{LocalizedExporter, MothersReport, generate_mothers_report};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::Locale;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::error::{ApiError, LocalizedErrorResponse};
use crate::middleware::{
    AuthenticatedUser, RequestLocale, require_family_membership, require_role,
};
use crate::state::AppState;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ReportSummaryQuery {
    #[schema(example = "family")]
    pub scope: Option<String>,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub family_id: Option<Uuid>,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateShareLinkPayload {
    #[schema(value_type = String, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub family_id: Uuid,
    #[schema(example = "rw")]
    pub locale: Option<Locale>,
    #[schema(example = 7)]
    pub valid_days: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SummaryReportResponse {
    pub report: MothersReport,
    #[schema(example = "RAPORO Y'UMURYANGO...")]
    pub whatsapp_summary: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ShareLinkResponse {
    #[schema(example = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6")]
    pub share_token: String,
    #[schema(example = "https://umuryango.rw/rw/report/a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6")]
    pub share_url: String,
    #[schema(example = "Raporo y'umuryango wacu...")]
    pub whatsapp_preview_text: String,
    pub expires_at: DateTime<Utc>,
}

fn generate_share_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

/// Generate financial summary report for a period
#[utoipa::path(
    get,
    path = "/api/v1/reports/summary",
    tag = "Reports",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ReportSummaryQuery,
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "Summary report generated successfully", body = SummaryReportResponse),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 409, description = "Conflict or missing parameters", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn get_summary_report(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<ReportSummaryQuery>,
) -> Result<HttpResponse, ApiError> {
    let loc = user.locale;
    let is_family = query
        .scope
        .as_deref()
        .map(|s| s.eq_ignore_ascii_case("family"))
        .unwrap_or(false);

    let scope = if is_family {
        let Some(fid) = query.family_id else {
            return Err(ApiError::conflict(
                "family_id is required for family report summary",
                loc,
            ));
        };
        require_family_membership(&data.db, user.user_id, fid, loc).await?;
        LedgerScope::Family { family_id: fid }
    } else {
        LedgerScope::Personal {
            user_id: user.user_id,
        }
    };

    let now = Utc::now();
    let period_start = query
        .period_start
        .unwrap_or_else(|| now - Duration::days(30));
    let period_end = query.period_end.unwrap_or(now);

    let report = fetch_and_generate_report(&data.db, scope, period_start, period_end, loc).await?;
    let whatsapp_summary = LocalizedExporter::format_mothers_report(&report, loc);

    Ok(HttpResponse::Ok().json(SummaryReportResponse {
        report,
        whatsapp_summary,
    }))
}

/// Create a shareable magic link for family elders with no login needed
#[utoipa::path(
    post,
    path = "/api/v1/reports/share",
    tag = "Reports",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateShareLinkPayload,
    params(
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 201, description = "Share link created successfully", body = ShareLinkResponse),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 403, description = "Forbidden (insufficient role)", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn create_share_link(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<CreateShareLinkPayload>,
) -> Result<HttpResponse, ApiError> {
    let loc = payload.locale.unwrap_or(user.locale);

    // Verify caller is HEAD or CUSTODIAN
    let member = require_family_membership(&data.db, user.user_id, payload.family_id, loc).await?;
    require_role(&member, &[FamilyRole::Head, FamilyRole::Custodian], loc)?;

    let token = generate_share_token();
    let days = payload.valid_days.unwrap_or(7);
    let expires_at = Utc::now() + Duration::days(days);
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO report_share_links (share_token, family_id, created_by, locale, expires_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(&token)
    .bind(payload.family_id)
    .bind(user.user_id)
    .bind(loc.as_str())
    .bind(expires_at)
    .bind(now)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    let share_url = format!("{}/{}/report/{}", data.public_web_url, loc.as_str(), token);

    // Generate preview summary text
    let scope = LedgerScope::Family {
        family_id: payload.family_id,
    };
    let report =
        fetch_and_generate_report(&data.db, scope, now - Duration::days(30), now, loc).await?;
    let whatsapp_preview_text = format!(
        "{}\n\nFungura raporo hano / Open report here:\n{}",
        LocalizedExporter::format_mothers_report(&report, loc),
        share_url
    );

    Ok(HttpResponse::Created().json(ShareLinkResponse {
        share_token: token,
        share_url,
        whatsapp_preview_text,
        expires_at,
    }))
}

/// Access a public family report via magic link token without authentication
#[utoipa::path(
    get,
    path = "/api/v1/reports/shared/{token}",
    tag = "Reports",
    security(),
    params(
        ("token" = String, Path, description = "32-character alphanumeric share token"),
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "Public report retrieved successfully", body = SummaryReportResponse),
        (status = 404, description = "Report link expired or invalid", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn get_public_report(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req_locale: RequestLocale,
) -> Result<HttpResponse, ApiError> {
    let token = path.into_inner();
    let default_locale = req_locale.0;

    #[derive(sqlx::FromRow)]
    struct ShareRow {
        family_id: Uuid,
        locale: String,
    }

    let share_link = sqlx::query_as::<_, ShareRow>(
        r#"
        SELECT family_id, locale
        FROM report_share_links
        WHERE share_token = $1 AND expires_at > NOW()
        "#,
    )
    .bind(&token)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| ApiError::database(e, default_locale))?;

    let Some(link) = share_link else {
        return Err(ApiError::not_found(default_locale));
    };

    let link_locale = Locale::from_str(&link.locale).unwrap_or(default_locale);
    let now = Utc::now();
    let period_start = now - Duration::days(30);
    let scope = LedgerScope::Family {
        family_id: link.family_id,
    };

    let report = fetch_and_generate_report(&data.db, scope, period_start, now, link_locale).await?;
    let whatsapp_summary = LocalizedExporter::format_mothers_report(&report, link_locale);

    Ok(HttpResponse::Ok().json(SummaryReportResponse {
        report,
        whatsapp_summary,
    }))
}

// Helper querying live DB data and computing Mother's Report
async fn fetch_and_generate_report(
    pool: &sqlx::PgPool,
    scope: LedgerScope,
    period_start: DateTime<Utc>,
    period_end: DateTime<Utc>,
    locale: Locale,
) -> Result<MothersReport, ApiError> {
    let (is_family, fid, uid) = match scope {
        LedgerScope::Family { family_id } => (true, Some(family_id), Uuid::nil()),
        LedgerScope::Personal { user_id } => (false, None, user_id),
    };

    // 1. Wallets
    #[derive(sqlx::FromRow)]
    struct WRow {
        id: Uuid,
        name: String,
        wallet_type: String,
        currency: String,
        balance: Decimal,
        custodian_id: Option<Uuid>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    let w_rows = sqlx::query_as::<_, WRow>(
        r#"
        SELECT id, name, wallet_type::TEXT as wallet_type, currency, balance, custodian_id, is_active, created_at, updated_at
        FROM wallets
        WHERE ($1 = TRUE AND scope = 'family' AND family_id = $2 AND is_active = TRUE)
           OR ($1 = FALSE AND scope = 'personal' AND user_id = $3 AND is_active = TRUE)
        "#,
    )
    .bind(is_family)
    .bind(fid)
    .bind(uid)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::database(e, locale))?;

    let wallets = w_rows
        .into_iter()
        .map(|r| {
            let channel = match r.wallet_type.to_lowercase().as_str() {
                "bank" => WalletChannel::Bank,
                "momo" | "mobile_money" => WalletChannel::MobileMoney,
                _ => WalletChannel::CashInHand,
            };
            Wallet {
                id: r.id,
                scope: scope.clone(),
                channel,
                name: r.name,
                account_number: None,
                currency: r.currency,
                balance: r.balance,
                custodian_id: r.custodian_id,
                is_active: r.is_active,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }
        })
        .collect::<Vec<_>>();

    // 2. Transactions
    #[derive(sqlx::FromRow)]
    struct TxRow {
        id: Uuid,
        source_wallet_id: Uuid,
        destination_wallet_id: Option<Uuid>,
        tx_type: String,
        amount: Decimal,
        currency: String,
        category: String,
        description: Option<String>,
        authorized_by: Uuid,
        tx_date: DateTime<Utc>,
        created_at: DateTime<Utc>,
    }

    let tx_rows = sqlx::query_as::<_, TxRow>(
        r#"
        SELECT id, source_wallet_id, destination_wallet_id, tx_type::TEXT as tx_type,
               amount, currency, category, description, user_id as authorized_by, tx_date, created_at
        FROM transactions
        WHERE (($1 = TRUE AND scope = 'family' AND family_id = $2)
           OR ($1 = FALSE AND scope = 'personal' AND user_id = $3))
           AND tx_date >= $4 AND tx_date <= $5
        ORDER BY tx_date ASC
        "#,
    )
    .bind(is_family)
    .bind(fid)
    .bind(uid)
    .bind(period_start)
    .bind(period_end)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::database(e, locale))?;

    let transactions = tx_rows
        .into_iter()
        .map(|r| {
            let t_type = match r.tx_type.to_lowercase().as_str() {
                "income" => TransactionType::Income,
                "expense" => TransactionType::Expense,
                _ => TransactionType::Handover,
            };
            Transaction {
                id: r.id,
                scope: scope.clone(),
                source_wallet_id: r.source_wallet_id,
                destination_wallet_id: r.destination_wallet_id,
                tx_type: t_type,
                amount: r.amount,
                currency: r.currency,
                category: r.category,
                description: r.description,
                authorized_by: r.authorized_by,
                paid_to: None,
                tx_date: r.tx_date,
                created_at: r.created_at,
            }
        })
        .collect::<Vec<_>>();

    // 3. Projects
    #[derive(sqlx::FromRow)]
    struct PRRow {
        id: Uuid,
        family_id: Uuid,
        name: String,
        description: Option<String>,
        target_amount: Decimal,
        allocated_amount: Decimal,
        status: String,
        start_date: Option<NaiveDate>,
        target_end_date: Option<NaiveDate>,
        created_by: Uuid,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    let p_rows = if is_family {
        sqlx::query_as::<_, PRRow>(
            r#"
            SELECT id, family_id, name, description, target_amount, allocated_amount,
                   status::TEXT as status, start_date, target_end_date, created_by, created_at, updated_at
            FROM projects
            WHERE family_id = $1
            "#,
        )
        .bind(fid)
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::database(e, locale))?
    } else {
        vec![]
    };

    let projects = p_rows
        .into_iter()
        .map(|r| {
            let status = match r.status.to_lowercase().as_str() {
                "in_progress" => ProjectStatus::InProgress,
                "completed" => ProjectStatus::Completed,
                "paused" => ProjectStatus::Paused,
                _ => ProjectStatus::Planning,
            };
            Project {
                id: r.id,
                family_id: r.family_id,
                name: r.name,
                description: r.description,
                target_amount: r.target_amount,
                allocated_amount: r.allocated_amount,
                status,
                start_date: r.start_date,
                target_end_date: r.target_end_date,
                created_by: r.created_by,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }
        })
        .collect::<Vec<_>>();

    // 4. Loans
    #[derive(sqlx::FromRow)]
    struct LRow {
        id: Uuid,
        counterparty_name: String,
        principal_amount: Decimal,
        balance_remaining: Decimal,
        interest_rate: Decimal,
        due_date: Option<NaiveDate>,
        loan_type: String,
        status: String,
        notes: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    let l_rows = sqlx::query_as::<_, LRow>(
        r#"
        SELECT id, counterparty_name, principal_amount, balance_remaining, interest_rate,
               due_date, loan_type::TEXT as loan_type, status::TEXT as status, notes, created_at, updated_at
        FROM loans
        WHERE ($1 = TRUE AND scope = 'family' AND family_id = $2)
           OR ($1 = FALSE AND scope = 'personal' AND user_id = $3)
        "#,
    )
    .bind(is_family)
    .bind(fid)
    .bind(uid)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::database(e, locale))?;

    let loans = l_rows
        .into_iter()
        .map(|r| {
            let direction = if r.loan_type.eq_ignore_ascii_case("given") {
                LoanDirection::Lent
            } else {
                LoanDirection::Borrowed
            };
            let status = match r.status.to_lowercase().as_str() {
                "repaid" => LoanStatus::Repaid,
                "defaulted" => LoanStatus::Defaulted,
                _ => LoanStatus::Active,
            };
            Loan {
                id: r.id,
                scope: scope.clone(),
                counterparty_name: r.counterparty_name,
                principal_amount: r.principal_amount,
                balance_remaining: r.balance_remaining,
                interest_rate: r.interest_rate,
                due_date: r.due_date,
                direction,
                status,
                notes: r.notes,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }
        })
        .collect::<Vec<_>>();

    // 5. Future Plans
    #[derive(sqlx::FromRow)]
    struct PlRow {
        id: Uuid,
        title: String,
        description: Option<String>,
        estimated_cost: Decimal,
        target_date: Option<NaiveDate>,
        is_achieved: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    let pl_rows = sqlx::query_as::<_, PlRow>(
        r#"
        SELECT id, title, description, estimated_cost, target_date, is_achieved, created_at, updated_at
        FROM future_plans
        WHERE ($1 = TRUE AND scope = 'family' AND family_id = $2)
           OR ($1 = FALSE AND scope = 'personal' AND user_id = $3)
        "#,
    )
    .bind(is_family)
    .bind(fid)
    .bind(uid)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::database(e, locale))?;

    let plans = pl_rows
        .into_iter()
        .map(|r| FuturePlan {
            id: r.id,
            scope: scope.clone(),
            title: r.title,
            description: r.description,
            estimated_cost: r.estimated_cost,
            target_date: r.target_date,
            is_achieved: r.is_achieved,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect::<Vec<_>>();

    let report = generate_mothers_report(
        scope,
        &wallets,
        &transactions,
        &projects,
        &loans,
        &plans,
        period_start,
        period_end,
    );

    Ok(report)
}

use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use umuryango_core_domain::models::{
    FuturePlan, Loan, LoanDirection, LoanStatus, Project, ProjectStatus,
};
use umuryango_core_domain::scope::LedgerScope;
use uuid::Uuid;

use crate::error::ApiError;
use crate::middleware::{require_family_membership, AuthenticatedUser};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct ListProjectsQuery {
    pub family_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub family_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub target_amount: Decimal,
    pub start_date: Option<NaiveDate>,
    pub target_end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct ListLoansQuery {
    pub scope: Option<String>,
    pub family_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLoanRequest {
    pub scope: String, // "personal" or "family"
    pub family_id: Option<Uuid>,
    pub counterparty_name: String,
    pub principal_amount: Decimal,
    pub interest_rate: Option<Decimal>,
    pub due_date: Option<NaiveDate>,
    pub direction: LoanDirection,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RepayLoanRequest {
    pub amount: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct ListPlansQuery {
    pub scope: Option<String>,
    pub family_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub scope: String,
    pub family_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub estimated_cost: Decimal,
    pub target_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct AllocatePlanRequest {
    pub amount: Decimal,
}

// ================= PROJECTS =================

pub async fn list_projects(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<ListProjectsQuery>,
) -> Result<HttpResponse, ApiError> {
    require_family_membership(&data.db, user.user_id, query.family_id, user.locale).await?;

    #[derive(sqlx::FromRow)]
    struct ProjectRow {
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

    let rows = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT id, family_id, name, description, target_amount, allocated_amount,
               status::TEXT as status, start_date, target_end_date, created_by, created_at, updated_at
        FROM projects
        WHERE family_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(query.family_id)
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let projects = rows
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

    Ok(HttpResponse::Ok().json(projects))
}

pub async fn create_project(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<CreateProjectRequest>,
) -> Result<HttpResponse, ApiError> {
    require_family_membership(&data.db, user.user_id, payload.family_id, user.locale).await?;

    let project_id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO projects (
            id, family_id, name, description, target_amount, allocated_amount,
            status, start_date, target_end_date, created_by, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, 0.0000, 'planning', $6, $7, $8, $9, $9)
        "#,
    )
    .bind(project_id)
    .bind(payload.family_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.target_amount)
    .bind(payload.start_date)
    .bind(payload.target_end_date)
    .bind(user.user_id)
    .bind(now)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let project = Project {
        id: project_id,
        family_id: payload.family_id,
        name: payload.name.clone(),
        description: payload.description.clone(),
        target_amount: payload.target_amount,
        allocated_amount: Decimal::ZERO,
        status: ProjectStatus::Planning,
        start_date: payload.start_date,
        target_end_date: payload.target_end_date,
        created_by: user.user_id,
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(project))
}

// ================= LOANS =================

pub async fn list_loans(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<ListLoansQuery>,
) -> Result<HttpResponse, ApiError> {
    let is_family = query
        .scope
        .as_deref()
        .map(|s| s.eq_ignore_ascii_case("family"))
        .unwrap_or(false);

    if is_family {
        let Some(fid) = query.family_id else {
            return Err(ApiError::conflict(
                "family_id is required for family loans",
                user.locale,
            ));
        };
        require_family_membership(&data.db, user.user_id, fid, user.locale).await?;
    }

    #[derive(sqlx::FromRow)]
    struct LoanRow {
        id: Uuid,
        scope: String,
        family_id: Option<Uuid>,
        user_id: Uuid,
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

    let rows = sqlx::query_as::<_, LoanRow>(
        r#"
        SELECT id, scope::TEXT as scope, family_id, user_id, counterparty_name,
               principal_amount, balance_remaining, interest_rate, due_date,
               loan_type::TEXT as loan_type, status::TEXT as status, notes, created_at, updated_at
        FROM loans
        WHERE ($1 = TRUE AND scope = 'family' AND family_id = $2)
           OR ($1 = FALSE AND scope = 'personal' AND user_id = $3)
        ORDER BY created_at DESC
        "#,
    )
    .bind(is_family)
    .bind(query.family_id)
    .bind(user.user_id)
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let loans = rows
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
            let scope = if r.scope.eq_ignore_ascii_case("family") {
                LedgerScope::Family {
                    family_id: r.family_id.unwrap_or_default(),
                }
            } else {
                LedgerScope::Personal { user_id: r.user_id }
            };
            Loan {
                id: r.id,
                scope,
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

    Ok(HttpResponse::Ok().json(loans))
}

pub async fn create_loan(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<CreateLoanRequest>,
) -> Result<HttpResponse, ApiError> {
    let is_family = payload.scope.trim().eq_ignore_ascii_case("family");
    let loan_id = Uuid::new_v4();
    let now = Utc::now();
    let interest = payload.interest_rate.unwrap_or(Decimal::ZERO);

    let (scope_db, fid_opt) = if is_family {
        let Some(fid) = payload.family_id else {
            return Err(ApiError::conflict(
                "family_id is required for family loan",
                user.locale,
            ));
        };
        require_family_membership(&data.db, user.user_id, fid, user.locale).await?;
        ("family", Some(fid))
    } else {
        ("personal", None)
    };

    let type_str = match payload.direction {
        LoanDirection::Lent => "given",
        LoanDirection::Borrowed => "taken",
    };

    sqlx::query(
        r#"
        INSERT INTO loans (\
            id, scope, family_id, user_id, counterparty_name,
            principal_amount, balance_remaining, interest_rate, due_date,
            loan_type, status, notes, created_at, updated_at
        )
        VALUES ($1, $2::financial_scope, $3, $4, $5, $6, $6, $7, $8, $9::loan_type, 'active', $10, $11, $11)
        "#,
    )
    .bind(loan_id)
    .bind(scope_db)
    .bind(fid_opt)
    .bind(user.user_id)
    .bind(&payload.counterparty_name)
    .bind(payload.principal_amount)
    .bind(interest)
    .bind(payload.due_date)
    .bind(type_str)
    .bind(&payload.notes)
    .bind(now)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let scope = if let Some(fid) = fid_opt {
        LedgerScope::Family { family_id: fid }
    } else {
        LedgerScope::Personal {
            user_id: user.user_id,
        }
    };

    let loan = Loan {
        id: loan_id,
        scope,
        counterparty_name: payload.counterparty_name.clone(),
        principal_amount: payload.principal_amount,
        balance_remaining: payload.principal_amount,
        interest_rate: interest,
        due_date: payload.due_date,
        direction: payload.direction,
        status: LoanStatus::Active,
        notes: payload.notes.clone(),
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(loan))
}

pub async fn repay_loan(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    payload: web::Json<RepayLoanRequest>,
) -> Result<HttpResponse, ApiError> {
    let loan_id = path.into_inner();

    #[derive(sqlx::FromRow)]
    struct LoanCheck {
        balance_remaining: Decimal,
        scope: String,
        family_id: Option<Uuid>,
        user_id: Uuid,
    }

    let loan = sqlx::query_as::<_, LoanCheck>(
        "SELECT balance_remaining, scope::TEXT as scope, family_id, user_id FROM loans WHERE id = $1",
    )
    .bind(loan_id)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let Some(l) = loan else {
        return Err(ApiError::not_found(user.locale));
    };

    if l.scope == "family" {
        if let Some(fid) = l.family_id {
            require_family_membership(&data.db, user.user_id, fid, user.locale).await?;
        }
    } else if l.user_id != user.user_id {
        return Err(ApiError::forbidden(
            "Access denied to personal loan",
            user.locale,
        ));
    }

    let new_balance = (l.balance_remaining - payload.amount).max(Decimal::ZERO);
    let new_status = if new_balance == Decimal::ZERO {
        "repaid"
    } else {
        "active"
    };

    sqlx::query(
        r#"
        UPDATE loans
        SET balance_remaining = $1, status = $2::loan_status, updated_at = NOW()
        WHERE id = $3
        "#,
    )
    .bind(new_balance)
    .bind(new_status)
    .bind(loan_id)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "repayment_recorded",
        "balance_remaining": new_balance,
        "loan_status": new_status
    })))
}

// ================= FUTURE PLANS =================

pub async fn list_plans(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<ListPlansQuery>,
) -> Result<HttpResponse, ApiError> {
    let is_family = query
        .scope
        .as_deref()
        .map(|s| s.eq_ignore_ascii_case("family"))
        .unwrap_or(false);

    if is_family {
        let Some(fid) = query.family_id else {
            return Err(ApiError::conflict(
                "family_id is required for family future plans",
                user.locale,
            ));
        };
        require_family_membership(&data.db, user.user_id, fid, user.locale).await?;
    }

    #[derive(sqlx::FromRow)]
    struct PlanRow {
        id: Uuid,
        scope: String,
        family_id: Option<Uuid>,
        user_id: Uuid,
        title: String,
        description: Option<String>,
        estimated_cost: Decimal,
        target_date: Option<NaiveDate>,
        is_achieved: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    let rows = sqlx::query_as::<_, PlanRow>(
        r#"
        SELECT id, scope::TEXT as scope, family_id, user_id, title, description,
               estimated_cost, target_date, is_achieved, created_at, updated_at
        FROM future_plans
        WHERE ($1 = TRUE AND scope = 'family' AND family_id = $2)
           OR ($1 = FALSE AND scope = 'personal' AND user_id = $3)
        ORDER BY created_at DESC
        "#,
    )
    .bind(is_family)
    .bind(query.family_id)
    .bind(user.user_id)
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let plans = rows
        .into_iter()
        .map(|r| {
            let scope = if r.scope.eq_ignore_ascii_case("family") {
                LedgerScope::Family {
                    family_id: r.family_id.unwrap_or_default(),
                }
            } else {
                LedgerScope::Personal { user_id: r.user_id }
            };
            FuturePlan {
                id: r.id,
                scope,
                title: r.title,
                description: r.description,
                estimated_cost: r.estimated_cost,
                target_date: r.target_date,
                is_achieved: r.is_achieved,
                created_at: r.created_at,
                updated_at: r.updated_at,
            }
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(plans))
}

pub async fn create_plan(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<CreatePlanRequest>,
) -> Result<HttpResponse, ApiError> {
    let is_family = payload.scope.trim().eq_ignore_ascii_case("family");
    let plan_id = Uuid::new_v4();
    let now = Utc::now();

    let (scope_db, fid_opt) = if is_family {
        let Some(fid) = payload.family_id else {
            return Err(ApiError::conflict(
                "family_id is required for family plan",
                user.locale,
            ));
        };
        require_family_membership(&data.db, user.user_id, fid, user.locale).await?;
        ("family", Some(fid))
    } else {
        ("personal", None)
    };

    sqlx::query(
        r#"
        INSERT INTO future_plans (
            id, scope, family_id, user_id, title, description,
            estimated_cost, target_date, is_achieved, created_at, updated_at
        )
        VALUES ($1, $2::financial_scope, $3, $4, $5, $6, $7, $8, FALSE, $9, $9)
        "#,
    )
    .bind(plan_id)
    .bind(scope_db)
    .bind(fid_opt)
    .bind(user.user_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.estimated_cost)
    .bind(payload.target_date)
    .bind(now)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let scope = if let Some(fid) = fid_opt {
        LedgerScope::Family { family_id: fid }
    } else {
        LedgerScope::Personal {
            user_id: user.user_id,
        }
    };

    let plan = FuturePlan {
        id: plan_id,
        scope,
        title: payload.title.clone(),
        description: payload.description.clone(),
        estimated_cost: payload.estimated_cost,
        target_date: payload.target_date,
        is_achieved: false,
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(plan))
}

pub async fn allocate_plan(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    payload: web::Json<AllocatePlanRequest>,
) -> Result<HttpResponse, ApiError> {
    let plan_id = path.into_inner();

    #[derive(sqlx::FromRow)]
    struct PlanCheck {
        estimated_cost: Decimal,
        scope: String,
        family_id: Option<Uuid>,
        user_id: Uuid,
    }

    let plan = sqlx::query_as::<_, PlanCheck>(
        "SELECT estimated_cost, scope::TEXT as scope, family_id, user_id FROM future_plans WHERE id = $1",
    )
    .bind(plan_id)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let Some(p) = plan else {
        return Err(ApiError::not_found(user.locale));
    };

    if p.scope == "family" {
        if let Some(fid) = p.family_id {
            require_family_membership(&data.db, user.user_id, fid, user.locale).await?;
        }
    } else if p.user_id != user.user_id {
        return Err(ApiError::forbidden(
            "Access denied to personal plan",
            user.locale,
        ));
    }

    let is_met = payload.amount >= p.estimated_cost;

    sqlx::query(
        r#"
        UPDATE future_plans
        SET is_achieved = $1, updated_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(is_met)
    .bind(plan_id)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "allocated",
        "is_achieved": is_met,
        "allocated_amount": payload.amount
    })))
}

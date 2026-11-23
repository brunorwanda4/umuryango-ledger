use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use umuryango_core_domain::models::{Family, FamilyRole};
use uuid::Uuid;

use crate::error::ApiError;
use crate::middleware::{require_family_membership, require_role, AuthenticatedUser};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateFamilyRequest {
    pub name: String,
    pub description: Option<String>,
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JoinFamilyRequest {
    pub invite_code: String,
    pub display_name: Option<String>,
    pub role: Option<FamilyRole>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRoleRequest {
    pub role: FamilyRole,
}

#[derive(Debug, Serialize)]
pub struct FamilyWithRole {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub currency: String,
    pub role: FamilyRole,
    pub invite_code: Option<String>,
    pub joined_at: DateTime<Utc>,
}

fn generate_invite_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>()
        .to_uppercase()
}

pub async fn list_families(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    #[derive(sqlx::FromRow)]
    struct FamilyRow {
        id: Uuid,
        name: String,
        description: Option<String>,
        currency: String,
        role: String,
        invite_code: Option<String>,
        joined_at: DateTime<Utc>,
    }

    let rows = sqlx::query_as::<_, FamilyRow>(
        r#"
        SELECT f.id, f.name, f.description, f.currency, fm.role::TEXT as role, f.invite_code, fm.joined_at
        FROM families f
        JOIN family_members fm ON f.id = fm.family_id
        WHERE fm.user_id = $1 AND fm.is_active = TRUE
        ORDER BY fm.joined_at ASC
        "#,
    )
    .bind(user.user_id)
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let families = rows
        .into_iter()
        .map(|r| {
            let role = match r.role.to_lowercase().as_str() {
                "head" => FamilyRole::Head,
                "custodian" | "treasurer" => FamilyRole::Custodian,
                _ => FamilyRole::Member,
            };
            FamilyWithRole {
                id: r.id,
                name: r.name,
                description: r.description,
                currency: r.currency,
                role,
                invite_code: r.invite_code,
                joined_at: r.joined_at,
            }
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(families))
}

pub async fn create_family(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<CreateFamilyRequest>,
) -> Result<HttpResponse, ApiError> {
    let family_id = Uuid::new_v4();
    let invite_code = generate_invite_code();
    let currency = payload
        .currency
        .clone()
        .unwrap_or_else(|| "RWF".to_string());
    let now = Utc::now();

    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

    // 1. Create family
    sqlx::query(
        r#"
        INSERT INTO families (id, name, description, currency, created_by, invite_code, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
        "#,
    )
    .bind(family_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&currency)
    .bind(user.user_id)
    .bind(&invite_code)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    // 2. Insert creator as HEAD
    sqlx::query(
        r#"
        INSERT INTO family_members (id, family_id, user_id, role, is_active, joined_at, updated_at)
        VALUES ($1, $2, $3, 'head', TRUE, $4, $4)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(family_id)
    .bind(user.user_id)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    // 3. Create initial family custody wallet
    let wallet_name = format!("{} Vault", payload.name);
    sqlx::query(
        r#"
        INSERT INTO wallets (id, scope, user_id, family_id, wallet_type, name, currency, balance, custodian_id, is_active, created_at, updated_at)
        VALUES ($1, 'family', NULL, $2, 'cash', $3, $4, 0.0000, $5, TRUE, $6, $6)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(family_id)
    .bind(wallet_name)
    .bind(&currency)
    .bind(user.user_id)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    tx.commit()
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

    let family = Family {
        id: family_id,
        name: payload.name.clone(),
        description: payload.description.clone(),
        currency,
        created_by: user.user_id,
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(serde_json::json!({
        "family": family,
        "invite_code": invite_code,
    })))
}

pub async fn join_family(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<JoinFamilyRequest>,
) -> Result<HttpResponse, ApiError> {
    let clean_code = payload.invite_code.trim().to_uppercase();

    #[derive(sqlx::FromRow)]
    struct TargetFamily {
        id: Uuid,
        name: String,
        currency: String,
    }

    let family = sqlx::query_as::<_, TargetFamily>(
        r#"
        SELECT id, name, currency
        FROM families
        WHERE invite_code = $1
        "#,
    )
    .bind(&clean_code)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let Some(fam) = family else {
        return Err(ApiError::not_found(user.locale));
    };

    let assigned_role = payload.role.unwrap_or(FamilyRole::Custodian);
    let role_str = match assigned_role {
        FamilyRole::Head => "head",
        FamilyRole::Custodian => "treasurer", // matches DB enum
        FamilyRole::Member => "contributor",
    };

    let now = Utc::now();
    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

    // Upsert membership
    sqlx::query(
        r#"
        INSERT INTO family_members (id, family_id, user_id, role, display_name, is_active, joined_at, updated_at)
        VALUES ($1, $2, $3, $4::family_role, $5, TRUE, $6, $6)
        ON CONFLICT (family_id, user_id) DO UPDATE SET
            is_active = TRUE,
            display_name = COALESCE(excluded.display_name, family_members.display_name),
            updated_at = excluded.updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(fam.id)
    .bind(user.user_id)
    .bind(role_str)
    .bind(&payload.display_name)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    // If joining as custodian, provision custody wallet
    if assigned_role == FamilyRole::Custodian {
        let name_label = payload
            .display_name
            .clone()
            .unwrap_or_else(|| "Member".to_string());
        let wallet_name = format!("{} Custody MoMo", name_label);

        sqlx::query(
            r#"
            INSERT INTO wallets (id, scope, user_id, family_id, wallet_type, name, currency, balance, custodian_id, is_active, created_at, updated_at)
            VALUES ($1, 'family', NULL, $2, 'momo', $3, $4, 0.0000, $5, TRUE, $6, $6)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(fam.id)
        .bind(wallet_name)
        .bind(&fam.currency)
        .bind(user.user_id)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;
    }

    tx.commit()
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "joined",
        "family_id": fam.id,
        "family_name": fam.name,
        "role": assigned_role
    })))
}

pub async fn update_member_role(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<(Uuid, Uuid)>,
    payload: web::Json<UpdateMemberRoleRequest>,
) -> Result<HttpResponse, ApiError> {
    let (family_id, member_id) = path.into_inner();

    // Verify caller is HEAD of this family
    let caller_member =
        require_family_membership(&data.db, user.user_id, family_id, user.locale).await?;
    require_role(&caller_member, &[FamilyRole::Head], user.locale)?;

    let role_str = match payload.role {
        FamilyRole::Head => "head",
        FamilyRole::Custodian => "treasurer",
        FamilyRole::Member => "contributor",
    };

    let result = sqlx::query(
        r#"
        UPDATE family_members
        SET role = $1::family_role, updated_at = NOW()
        WHERE id = $2 AND family_id = $3
        "#,
    )
    .bind(role_str)
    .bind(member_id)
    .bind(family_id)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::not_found(user.locale));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "updated",
        "role": payload.role
    })))
}

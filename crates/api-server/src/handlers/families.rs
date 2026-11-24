use actix_web::{HttpResponse, web};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use umuryango_core_domain::models::{Family, FamilyRole, PendingFamilyInvite};
pub use umuryango_core_domain::models::{FamilyMemberView, FamilyWithRole};
use umuryango_i18n::{Locale, Translator, terms};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::{ApiError, LocalizedErrorResponse};
use crate::handlers::auth::normalize_phone;
use crate::middleware::{AuthenticatedUser, require_family_membership, require_role};
use crate::state::AppState;

const INVITE_ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

fn generate_4char_invite_code() -> String {
    let mut rng = rand::thread_rng();
    (0..4)
        .map(|_| {
            let idx = rng.gen_range(0..INVITE_ALPHABET.len());
            INVITE_ALPHABET[idx] as char
        })
        .collect()
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateFamilyRequest {
    #[schema(example = "Umuryango w'Abahire")]
    pub name: String,
    #[schema(example = "Rwanda & Diaspora joint custody ledger")]
    pub description: Option<String>,
    #[schema(example = "RWF")]
    pub currency: Option<String>,
    #[serde(default)]
    #[schema(example = "[\"+250788111222\", \"+250788333444\"]")]
    pub member_phones: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct JoinFamilyRequest {
    #[schema(example = "HK0N")]
    pub invite_code: Option<String>,
    pub invite_id: Option<Uuid>,
    #[schema(example = "Eric Custodian")]
    pub display_name: Option<String>,
    pub role: Option<FamilyRole>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMemberRoleRequest {
    pub role: FamilyRole,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct InviteMemberRequest {
    #[schema(example = "+250788111222")]
    pub phone: String,
    pub role: Option<FamilyRole>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateFamilyResponse {
    pub family: Family,
    #[schema(example = "HK0N")]
    pub invite_code: String,
    #[schema(example = 2)]
    pub added_invites_count: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JoinFamilyResponse {
    #[schema(example = "joined")]
    pub status: String,
    #[schema(example = "9a756613-263a-4467-9d7a-11239c04901f")]
    pub family_id: Uuid,
    #[schema(example = "Umuryango w'Abahire")]
    pub family_name: String,
    pub role: FamilyRole,
    pub family: Family,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UpdateMemberRoleResponse {
    #[schema(example = "updated")]
    pub status: String,
    pub role: FamilyRole,
}

/// List all families where the authenticated user is an active member
#[utoipa::path(
    get,
    path = "/api/v1/families",
    tag = "Families",
    security(("bearer_auth" = [])),
    params(
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "List of user families with roles", body = Vec<FamilyWithRole>),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
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

/// List all active members of a family group
#[utoipa::path(
    get,
    path = "/api/v1/families/{family_id}/members",
    tag = "Families",
    security(("bearer_auth" = [])),
    params(
        ("family_id" = Uuid, Path, description = "Family UUID"),
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "List of active family members", body = Vec<FamilyMemberView>),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 403, description = "Forbidden (not a family member)", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn list_family_members(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let family_id = path.into_inner();
    require_family_membership(&data.db, user.user_id, family_id, user.locale).await?;

    #[derive(sqlx::FromRow)]
    struct MemberRow {
        id: Uuid,
        family_id: Uuid,
        user_id: Uuid,
        full_name: String,
        phone: String,
        display_name: Option<String>,
        role: String,
        is_active: bool,
        joined_at: DateTime<Utc>,
    }

    let rows = sqlx::query_as::<_, MemberRow>(
        r#"
        SELECT
            fm.id,
            fm.family_id,
            fm.user_id,
            u.full_name,
            u.phone,
            fm.display_name,
            fm.role::TEXT as role,
            fm.is_active,
            fm.joined_at
        FROM family_members fm
        JOIN users u ON fm.user_id = u.id
        WHERE fm.family_id = $1 AND fm.is_active = TRUE
        ORDER BY fm.joined_at ASC
        "#,
    )
    .bind(family_id)
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let members = rows
        .into_iter()
        .map(|r| {
            let role = match r.role.to_lowercase().as_str() {
                "head" => FamilyRole::Head,
                "custodian" | "treasurer" => FamilyRole::Custodian,
                _ => FamilyRole::Member,
            };
            FamilyMemberView {
                id: r.id,
                family_id: r.family_id,
                user_id: r.user_id,
                full_name: r.full_name,
                phone: r.phone,
                display_name: r.display_name,
                role,
                is_active: r.is_active,
                joined_at: r.joined_at,
            }
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(members))
}

/// Invite a family member by phone number (Head only)
#[utoipa::path(
    post,
    path = "/api/v1/families/{family_id}/invites",
    tag = "Families",
    security(("bearer_auth" = [])),
    params(
        ("family_id" = Uuid, Path, description = "Family UUID"),
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    request_body = InviteMemberRequest,
    responses(
        (status = 201, description = "Invitation recorded successfully"),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 403, description = "Forbidden (requires Head role)", body = LocalizedErrorResponse),
        (status = 409, description = "Invalid phone number", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn invite_member_by_phone(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    payload: web::Json<InviteMemberRequest>,
) -> Result<HttpResponse, ApiError> {
    let family_id = path.into_inner();
    let caller = require_family_membership(&data.db, user.user_id, family_id, user.locale).await?;
    require_role(&caller, &[FamilyRole::Head], user.locale)?;

    let clean_phone = normalize_phone(&payload.phone);
    if clean_phone.is_empty() {
        let msg = if user.locale == Locale::Rw {
            "Ugomba gutanga nimero ya telefoni yemewe."
        } else {
            "A valid phone number is required."
        };
        return Err(ApiError::conflict(msg, user.locale));
    }

    let role_str = match payload.role.unwrap_or(FamilyRole::Custodian) {
        FamilyRole::Head => "HEAD",
        FamilyRole::Custodian => "CUSTODIAN",
        FamilyRole::Member => "MEMBER",
    };

    sqlx::query(
        r#"
        INSERT INTO family_phone_invites (id, family_id, invited_phone, invited_by_user_id, role, status, created_at)
        VALUES ($1, $2, $3, $4, $5, 'PENDING', $6)
        ON CONFLICT (family_id, invited_phone) DO UPDATE SET
            role = excluded.role,
            status = 'PENDING'
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(family_id)
    .bind(&clean_phone)
    .bind(user.user_id)
    .bind(role_str)
    .bind(Utc::now())
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "invited",
        "invited_phone": clean_phone
    })))
}

/// List all pending family phone invites for the authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/families/pending-invites",
    tag = "Families",
    security(("bearer_auth" = [])),
    params(
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "Pending family invitations matching user phone", body = Vec<PendingFamilyInvite>),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn list_pending_invites(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    let user_phone = sqlx::query_scalar::<_, String>("SELECT phone FROM users WHERE id = $1")
        .bind(user.user_id)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?
        .unwrap_or_default();

    #[derive(sqlx::FromRow)]
    struct PendingInviteRow {
        id: Uuid,
        family_id: Uuid,
        family_name: String,
        invited_by_name: String,
        invited_phone: String,
        role: String,
        status: String,
        created_at: DateTime<Utc>,
    }

    let rows = sqlx::query_as::<_, PendingInviteRow>(
        r#"
        SELECT
            fpi.id,
            fpi.family_id,
            f.name as family_name,
            COALESCE(u.full_name, 'Family Head') as invited_by_name,
            fpi.invited_phone,
            fpi.role,
            fpi.status,
            fpi.created_at
        FROM family_phone_invites fpi
        JOIN families f ON fpi.family_id = f.id
        LEFT JOIN users u ON fpi.invited_by_user_id = u.id
        WHERE fpi.invited_phone = $1 AND fpi.status = 'PENDING'
        ORDER BY fpi.created_at DESC
        "#,
    )
    .bind(&user_phone)
    .fetch_all(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    let invites = rows
        .into_iter()
        .map(|r| {
            let role = match r.role.to_lowercase().as_str() {
                "head" => FamilyRole::Head,
                "custodian" | "treasurer" => FamilyRole::Custodian,
                _ => FamilyRole::Member,
            };
            PendingFamilyInvite {
                id: r.id,
                family_id: r.family_id,
                family_name: r.family_name,
                invited_by_name: r.invited_by_name,
                invited_phone: r.invited_phone,
                role,
                status: r.status,
                created_at: r.created_at,
            }
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(invites))
}

/// Create a new family group with the caller as HEAD, provision primary vault wallet, and register phone invites
#[utoipa::path(
    post,
    path = "/api/v1/families",
    tag = "Families",
    security(("bearer_auth" = [])),
    request_body = CreateFamilyRequest,
    params(
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 201, description = "Family created successfully", body = CreateFamilyResponse),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 409, description = "Validation error", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn create_family(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<CreateFamilyRequest>,
) -> Result<HttpResponse, ApiError> {
    let family_name = payload.name.trim();
    if family_name.is_empty() {
        let msg = if user.locale == Locale::Rw {
            "Izina ry'umuryango rirakenewe."
        } else {
            "Family name is required."
        };
        return Err(ApiError::conflict(msg, user.locale));
    }

    let family_id = Uuid::new_v4();
    let invite_code = generate_4char_invite_code();
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

    // 1. Create family with 4-character invite code
    sqlx::query(
        r#"
        INSERT INTO families (id, name, description, currency, created_by, invite_code, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
        "#,
    )
    .bind(family_id)
    .bind(family_name)
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
    let wallet_name = format!("{} Vault", family_name);
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

    // 4. Pre-invite members by phone number
    let mut added_invites_count = 0;
    for phone in &payload.member_phones {
        let clean = normalize_phone(phone);
        if clean.is_empty() {
            continue;
        }
        let res = sqlx::query(
            r#"
            INSERT INTO family_phone_invites (id, family_id, invited_phone, invited_by_user_id, role, status, created_at)
            VALUES ($1, $2, $3, $4, 'CUSTODIAN', 'PENDING', $5)
            ON CONFLICT (family_id, invited_phone) DO NOTHING
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(family_id)
        .bind(&clean)
        .bind(user.user_id)
        .bind(now)
        .execute(&mut *tx)
        .await;

        if let Ok(r) = res {
            if r.rows_affected() > 0 {
                added_invites_count += 1;
            }
        }
    }

    tx.commit()
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

    let family = Family {
        id: family_id,
        name: family_name.to_string(),
        description: payload.description.clone(),
        currency,
        invite_code: Some(invite_code.clone()),
        created_by: user.user_id,
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(CreateFamilyResponse {
        family,
        invite_code,
        added_invites_count,
    }))
}

/// Join an existing family via 4-character invite code OR pending invite id
#[utoipa::path(
    post,
    path = "/api/v1/families/join",
    tag = "Families",
    security(("bearer_auth" = [])),
    request_body = JoinFamilyRequest,
    params(
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "Joined family successfully", body = JoinFamilyResponse),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 404, description = "Family or invite not found", body = LocalizedErrorResponse),
        (status = 409, description = "Invalid 4-character code", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn join_family(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<JoinFamilyRequest>,
) -> Result<HttpResponse, ApiError> {
    #[derive(sqlx::FromRow)]
    struct TargetFamily {
        id: Uuid,
        name: String,
        description: Option<String>,
        currency: String,
        created_by: Uuid,
        invite_code: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    let (family, assigned_role) = if let Some(code) = &payload.invite_code {
        let clean_code = code.trim().to_uppercase();
        if clean_code.len() != 4 {
            let msg = Translator::t(user.locale, terms::ERR_INVALID_INVITE_CODE, None);
            return Err(ApiError::conflict(msg, user.locale));
        }

        let fam = sqlx::query_as::<_, TargetFamily>(
            r#"
            SELECT id, name, description, currency, created_by, invite_code, created_at, updated_at
            FROM families
            WHERE UPPER(TRIM(invite_code)) = $1
            "#,
        )
        .bind(&clean_code)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

        let Some(f) = fam else {
            let msg = Translator::t(user.locale, terms::ERR_INVALID_INVITE_CODE, None);
            return Err(ApiError::conflict(msg, user.locale));
        };

        (f, payload.role.unwrap_or(FamilyRole::Custodian))
    } else if let Some(invite_id) = payload.invite_id {
        #[derive(sqlx::FromRow)]
        struct InviteWithFamily {
            _id: Uuid,
            family_id: Uuid,
            role: String,
            fam_name: String,
            fam_description: Option<String>,
            fam_currency: String,
            fam_created_by: Uuid,
            fam_invite_code: Option<String>,
            fam_created_at: DateTime<Utc>,
            fam_updated_at: DateTime<Utc>,
        }

        let inv = sqlx::query_as::<_, InviteWithFamily>(
            r#"
            SELECT
                fpi.id as _id, fpi.family_id, fpi.role,
                f.name as fam_name, f.description as fam_description, f.currency as fam_currency,
                f.created_by as fam_created_by, f.invite_code as fam_invite_code,
                f.created_at as fam_created_at, f.updated_at as fam_updated_at
            FROM family_phone_invites fpi
            JOIN families f ON fpi.family_id = f.id
            WHERE fpi.id = $1 AND fpi.status = 'PENDING'
            "#,
        )
        .bind(invite_id)
        .fetch_optional(&data.db)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

        let Some(i) = inv else {
            return Err(ApiError::not_found(user.locale));
        };

        let role = match i.role.to_lowercase().as_str() {
            "head" => FamilyRole::Head,
            "custodian" | "treasurer" => FamilyRole::Custodian,
            _ => FamilyRole::Member,
        };

        let fam = TargetFamily {
            id: i.family_id,
            name: i.fam_name,
            description: i.fam_description,
            currency: i.fam_currency,
            created_by: i.fam_created_by,
            invite_code: i.fam_invite_code,
            created_at: i.fam_created_at,
            updated_at: i.fam_updated_at,
        };

        (fam, role)
    } else {
        let msg = Translator::t(user.locale, terms::ERR_INVALID_INVITE_CODE, None);
        return Err(ApiError::conflict(msg, user.locale));
    };

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

    // 1. Upsert membership
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
    .bind(family.id)
    .bind(user.user_id)
    .bind(role_str)
    .bind(&payload.display_name)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    // 2. If joining as custodian or head, provision custody wallet
    if assigned_role.can_transact() {
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
        .bind(family.id)
        .bind(wallet_name)
        .bind(&family.currency)
        .bind(user.user_id)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;
    }

    // 3. Mark only this user's matching pending phone invite(s) as ACCEPTED
    if let Some(invite_id) = payload.invite_id {
        let _ = sqlx::query(
            r#"
            UPDATE family_phone_invites
            SET status = 'ACCEPTED'
            WHERE id = $1
            "#,
        )
        .bind(invite_id)
        .execute(&mut *tx)
        .await;
    }

    let user_phone = sqlx::query_scalar::<_, String>("SELECT phone FROM users WHERE id = $1")
        .bind(user.user_id)
        .fetch_optional(&mut *tx)
        .await
        .ok()
        .flatten()
        .unwrap_or_default();

    if !user_phone.is_empty() {
        let _ = sqlx::query(
            r#"
            UPDATE family_phone_invites
            SET status = 'ACCEPTED'
            WHERE family_id = $1 AND invited_phone = $2 AND status = 'PENDING'
            "#,
        )
        .bind(family.id)
        .bind(&user_phone)
        .execute(&mut *tx)
        .await;
    }

    tx.commit()
        .await
        .map_err(|e| ApiError::database(e, user.locale))?;

    let full_family = Family {
        id: family.id,
        name: family.name.clone(),
        description: family.description,
        currency: family.currency,
        invite_code: family.invite_code,
        created_by: family.created_by,
        created_at: family.created_at,
        updated_at: family.updated_at,
    };

    Ok(HttpResponse::Ok().json(JoinFamilyResponse {
        status: "joined".to_string(),
        family_id: family.id,
        family_name: family.name,
        role: assigned_role,
        family: full_family,
    }))
}

/// Update member authorization role within a family (Head only)
#[utoipa::path(
    put,
    path = "/api/v1/families/{family_id}/members/{member_id}/role",
    tag = "Families",
    security(("bearer_auth" = [])),
    params(
        ("family_id" = Uuid, Path, description = "Family UUID"),
        ("member_id" = Uuid, Path, description = "Family Member UUID"),
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    request_body = UpdateMemberRoleRequest,
    responses(
        (status = 200, description = "Member role updated successfully", body = UpdateMemberRoleResponse),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 403, description = "Forbidden (requires Head role)", body = LocalizedErrorResponse),
        (status = 404, description = "Member or family not found", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
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

    Ok(HttpResponse::Ok().json(UpdateMemberRoleResponse {
        status: "updated".to_string(),
        role: payload.role,
    }))
}

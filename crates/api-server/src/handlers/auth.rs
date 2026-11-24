use actix_web::{HttpResponse, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use umuryango_core_domain::models::{FamilyRole, FamilyWithRole, PendingFamilyInvite, User};
use umuryango_i18n::{Locale, Translator, terms};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::{ApiError, LocalizedErrorResponse};
use crate::middleware::{AuthenticatedUser, RequestLocale, create_jwt};
use crate::state::AppState;

pub fn normalize_phone(raw: &str) -> String {
    let trimmed = raw.trim();
    let digits_only: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits_only.starts_with("250") && digits_only.len() == 12 {
        format!("+{}", digits_only)
    } else if digits_only.starts_with("07") && digits_only.len() == 10 {
        format!("+250{}", &digits_only[1..])
    } else if digits_only.starts_with('7') && digits_only.len() == 9 {
        format!("+250{}", digits_only)
    } else if trimmed.starts_with('+') {
        trimmed.to_string()
    } else if !digits_only.is_empty() {
        format!("+{}", digits_only)
    } else {
        trimmed.to_string()
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterPayload {
    #[schema(example = "Bruno Hapiman")]
    pub full_name: String,
    #[schema(example = "+250788123456")]
    pub phone: Option<String>,
    #[schema(example = "+250788123456")]
    pub phone_number: Option<String>,
    #[schema(example = "MALE")]
    pub gender: Option<String>,
    #[schema(example = "Secret123!")]
    pub password: String,
    #[schema(example = "Secret123!")]
    pub confirm_password: Option<String>,
    #[schema(example = "bruno@example.rw")]
    pub email: Option<String>,
    #[schema(example = "rw")]
    pub preferred_locale: Option<Locale>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
    #[schema(example = "+250788123456")]
    pub phone: Option<String>,
    #[schema(example = "+250788123456")]
    pub login: Option<String>,
    #[schema(example = "Secret123!")]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateLocalePayload {
    #[schema(example = "en")]
    pub locale: Locale,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
    pub user: User,
    #[serde(default)]
    pub families: Vec<FamilyWithRole>,
    #[serde(default)]
    pub pending_invites: Vec<PendingFamilyInvite>,
}

/// Register a new user and provision their default personal cash wallet
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Auth",
    request_body = RegisterPayload,
    params(
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 409, description = "Validation or duplicate conflict", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn register(
    data: web::Data<AppState>,
    locale: RequestLocale,
    payload: web::Json<RegisterPayload>,
) -> Result<HttpResponse, ApiError> {
    let loc = payload.preferred_locale.unwrap_or(locale.0);

    let full_name = payload.full_name.trim();
    if full_name.is_empty() {
        let msg = if loc == Locale::Rw {
            "Ugomba gutanga amazina yawe yose."
        } else {
            "Full name is required."
        };
        return Err(ApiError::conflict(msg, loc));
    }

    let raw_phone = payload
        .phone
        .as_deref()
        .or(payload.phone_number.as_deref())
        .unwrap_or("")
        .trim();

    if raw_phone.is_empty() && payload.email.is_none() {
        let msg = if loc == Locale::Rw {
            "Ugomba gutanga nimero ya telefoni cyangwa imeri."
        } else {
            "You must provide a phone number or email."
        };
        return Err(ApiError::conflict(msg, loc));
    }

    let clean_phone = normalize_phone(raw_phone);

    // Validate password confirmation if provided
    if let Some(confirm) = &payload.confirm_password {
        if confirm != &payload.password {
            let msg = Translator::t(loc, terms::ERR_PASSWORD_MISMATCH, None);
            return Err(ApiError::conflict(msg, loc));
        }
    }

    if payload.password.len() < 6 {
        let msg = if loc == Locale::Rw {
            "Ijambobanga rigomba kugira byibuze inyuguti 6."
        } else {
            "Password must be at least 6 characters long."
        };
        return Err(ApiError::conflict(msg, loc));
    }

    // Check if account with phone number already exists
    if !clean_phone.is_empty() {
        let existing = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE phone = $1")
            .bind(&clean_phone)
            .fetch_one(&data.db)
            .await
            .map_err(|e| ApiError::database(e, loc))?;

        if existing > 0 {
            let msg = Translator::t(loc, terms::ERR_PHONE_ALREADY_EXISTS, None);
            return Err(ApiError::conflict(msg, loc));
        }
    }

    // Hash password using Argon2id
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| ApiError::internal(format!("Password hashing error: {}", e), loc))?
        .to_string();

    let user_id = Uuid::new_v4();
    let now = Utc::now();
    let gender_val = payload.gender.as_deref().unwrap_or("OTHER").to_uppercase();

    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::database(e, loc))?;

    // Insert user
    sqlx::query(
        r#"
        INSERT INTO users (id, email, phone, full_name, gender, password_hash, preferred_locale, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7::app_locale, TRUE, $8, $8)
        "#,
    )
    .bind(user_id)
    .bind(&payload.email)
    .bind(&clean_phone)
    .bind(full_name)
    .bind(&gender_val)
    .bind(&password_hash)
    .bind(loc.as_str())
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    // Create default personal wallet
    let wallet_name = if loc == Locale::Rw {
        "Konti Yanjye Bwite"
    } else {
        "My Personal Wallet"
    };

    sqlx::query(
        r#"
        INSERT INTO wallets (id, scope, user_id, family_id, wallet_type, name, currency, balance, custodian_id, is_active, created_at, updated_at)
        VALUES ($1, 'personal', $2, NULL, 'cash', $3, 'RWF', 0.0000, $2, TRUE, $4, $4)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(wallet_name)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    tx.commit().await.map_err(|e| ApiError::database(e, loc))?;

    // Check for any PENDING family invites matching user's phone
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

    let pending_rows = sqlx::query_as::<_, PendingInviteRow>(
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
        WHERE (fpi.invited_phone = $1 OR fpi.invited_phone = $2)
          AND fpi.status = 'PENDING'
        ORDER BY fpi.created_at DESC
        "#,
    )
    .bind(&clean_phone)
    .bind(raw_phone)
    .fetch_all(&data.db)
    .await
    .unwrap_or_default();

    let pending_invites = pending_rows
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

    let token = create_jwt(user_id, loc, &data.jwt_secret, 72)
        .map_err(|e| ApiError::internal(format!("JWT creation error: {}", e), loc))?;

    let user = User {
        id: user_id,
        email: payload.email.clone(),
        phone: clean_phone.clone(),
        phone_number: Some(clean_phone),
        gender: gender_val,
        full_name: full_name.to_string(),
        preferred_locale: loc,
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(AuthResponse {
        token,
        user,
        families: Vec::new(),
        pending_invites,
    }))
}

/// Authenticate via phone and password, returning a JWT token with families and pending invites
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Auth",
    request_body = LoginPayload,
    params(
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "User authenticated successfully", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = LocalizedErrorResponse),
        (status = 403, description = "Account disabled", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn login(
    data: web::Data<AppState>,
    locale: RequestLocale,
    payload: web::Json<LoginPayload>,
) -> Result<HttpResponse, ApiError> {
    let loc = locale.0;

    let raw_login = payload
        .phone
        .as_deref()
        .or(payload.login.as_deref())
        .unwrap_or("")
        .trim();

    if raw_login.is_empty() || payload.password.is_empty() {
        let msg = umuryango_i18n::t(loc, "errors.phone_password_required");
        return Err(ApiError::unauthorized(msg, loc));
    }

    let clean_phone = normalize_phone(raw_login);

    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: Uuid,
        email: Option<String>,
        phone: String,
        gender: String,
        full_name: String,
        password_hash: String,
        preferred_locale: String,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, phone, gender, full_name, password_hash, preferred_locale::TEXT as preferred_locale, is_active, created_at, updated_at
        FROM users
        WHERE phone = $1 OR phone = $2 OR email = $2
        "#,
    )
    .bind(&clean_phone)
    .bind(raw_login)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    let Some(user_row) = row else {
        let msg = Translator::t(loc, terms::ERR_INVALID_CREDENTIALS, None);
        return Err(ApiError::unauthorized(msg, loc));
    };

    if !user_row.is_active {
        let msg = if loc == Locale::Rw {
            "Konti yawe yahagaritswe."
        } else {
            "Your account is disabled."
        };
        return Err(ApiError::forbidden(msg, loc));
    }

    // Verify Argon2 password hash
    let parsed_hash = PasswordHash::new(&user_row.password_hash)
        .map_err(|e| ApiError::internal(format!("Invalid stored password hash: {}", e), loc))?;

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        let msg = Translator::t(loc, terms::ERR_INVALID_CREDENTIALS, None);
        return Err(ApiError::unauthorized(msg, loc));
    }

    let user_locale = Locale::from_str(&user_row.preferred_locale).unwrap_or(loc);
    let token = create_jwt(user_row.id, user_locale, &data.jwt_secret, 72)
        .map_err(|e| ApiError::internal(format!("JWT creation error: {}", e), user_locale))?;

    // Load user's joined families
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

    let family_rows = sqlx::query_as::<_, FamilyRow>(
        r#"
        SELECT f.id, f.name, f.description, f.currency, fm.role::TEXT as role, f.invite_code, fm.joined_at
        FROM families f
        JOIN family_members fm ON f.id = fm.family_id
        WHERE fm.user_id = $1 AND fm.is_active = TRUE
        ORDER BY fm.joined_at ASC
        "#,
    )
    .bind(user_row.id)
    .fetch_all(&data.db)
    .await
    .unwrap_or_default();

    let families = family_rows
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

    // Load any pending family phone invites
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

    let pending_rows = sqlx::query_as::<_, PendingInviteRow>(
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
        WHERE (fpi.invited_phone = $1 OR fpi.invited_phone = $2)
          AND fpi.status = 'PENDING'
        ORDER BY fpi.created_at DESC
        "#,
    )
    .bind(&user_row.phone)
    .bind(&clean_phone)
    .fetch_all(&data.db)
    .await
    .unwrap_or_default();

    let pending_invites = pending_rows
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

    let user = User {
        id: user_row.id,
        email: user_row.email,
        phone: user_row.phone.clone(),
        phone_number: Some(user_row.phone),
        gender: user_row.gender,
        full_name: user_row.full_name,
        preferred_locale: user_locale,
        is_active: user_row.is_active,
        created_at: user_row.created_at,
        updated_at: user_row.updated_at,
    };

    Ok(HttpResponse::Ok().json(AuthResponse {
        token,
        user,
        families,
        pending_invites,
    }))
}

/// Update preferred language (rw / en) for the authenticated user
#[utoipa::path(
    put,
    path = "/api/v1/auth/locale",
    tag = "Auth",
    security(("bearer_auth" = [])),
    request_body = UpdateLocalePayload,
    params(
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "Locale updated successfully"),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn update_locale(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<UpdateLocalePayload>,
) -> Result<HttpResponse, ApiError> {
    sqlx::query(
        r#"
        UPDATE users
        SET preferred_locale = $1::app_locale, updated_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(payload.locale.as_str())
    .bind(user.user_id)
    .execute(&data.db)
    .await
    .map_err(|e| ApiError::database(e, user.locale))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "updated",
        "preferred_locale": payload.locale.as_str()
    })))
}

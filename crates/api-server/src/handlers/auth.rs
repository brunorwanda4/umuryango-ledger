use actix_web::{HttpResponse, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use umuryango_core_domain::models::User;
use umuryango_i18n::Locale;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::error::{ApiError, LocalizedErrorResponse};
use crate::middleware::{AuthenticatedUser, RequestLocale, create_jwt};
use crate::state::AppState;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterPayload {
    #[schema(example = "bruno@example.rw")]
    pub email: Option<String>,
    #[schema(example = "+250788123456")]
    pub phone_number: Option<String>,
    #[schema(example = "Bruno Hapiman")]
    pub full_name: String,
    #[schema(example = "Secret123!")]
    pub password: String,
    #[schema(example = "rw")]
    pub preferred_locale: Option<Locale>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
    #[schema(example = "bruno@example.rw")]
    pub login: String, // email or phone_number
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

    if payload.email.is_none() && payload.phone_number.is_none() {
        let msg = if loc == Locale::Rw {
            "Ugomba gutanga imeri cyangwa nimero ya terefone."
        } else {
            "You must provide an email or phone number."
        };
        return Err(ApiError::conflict(msg, loc));
    }

    if payload.password.len() < 6 {
        let msg = if loc == Locale::Rw {
            "Ijambobanga rigomba kugira byibuze inyuguti 6."
        } else {
            "Password must be at least 6 characters long."
        };
        return Err(ApiError::conflict(msg, loc));
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

    let mut tx = data
        .db
        .begin()
        .await
        .map_err(|e| ApiError::database(e, loc))?;

    // Insert user
    sqlx::query(
        r#"
        INSERT INTO users (id, email, phone_number, full_name, password_hash, preferred_locale, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6::app_locale, TRUE, $7, $7)
        "#,
    )
    .bind(user_id)
    .bind(&payload.email)
    .bind(&payload.phone_number)
    .bind(&payload.full_name)
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

    let token = create_jwt(user_id, loc, &data.jwt_secret, 72)
        .map_err(|e| ApiError::internal(format!("JWT creation error: {}", e), loc))?;

    let user = User {
        id: user_id,
        email: payload.email.clone(),
        phone_number: payload.phone_number.clone(),
        full_name: payload.full_name.clone(),
        preferred_locale: loc,
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(AuthResponse { token, user }))
}

/// Authenticate via email/phone and password, returning a JWT token
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

    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: Uuid,
        email: Option<String>,
        phone_number: Option<String>,
        full_name: String,
        password_hash: String,
        preferred_locale: String,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, email, phone_number, full_name, password_hash, preferred_locale::TEXT as preferred_locale, is_active, created_at, updated_at
        FROM users
        WHERE (email = $1 OR phone_number = $1)
        "#,
    )
    .bind(payload.login.trim())
    .fetch_optional(&data.db)
    .await
    .map_err(|e| ApiError::database(e, loc))?;

    let Some(user_row) = row else {
        let msg = if loc == Locale::Rw {
            "Imeri/telefone cyangwa ijambobanga ntabwo bihuye."
        } else {
            "Invalid email/phone or password."
        };
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
        let msg = if loc == Locale::Rw {
            "Imeri/telefone cyangwa ijambobanga ntabwo bihuye."
        } else {
            "Invalid email/phone or password."
        };
        return Err(ApiError::unauthorized(msg, loc));
    }

    let user_locale = Locale::from_str(&user_row.preferred_locale).unwrap_or(loc);
    let token = create_jwt(user_row.id, user_locale, &data.jwt_secret, 72)
        .map_err(|e| ApiError::internal(format!("JWT creation error: {}", e), user_locale))?;

    let user = User {
        id: user_row.id,
        email: user_row.email,
        phone_number: user_row.phone_number,
        full_name: user_row.full_name,
        preferred_locale: user_locale,
        is_active: user_row.is_active,
        created_at: user_row.created_at,
        updated_at: user_row.updated_at,
    };

    Ok(HttpResponse::Ok().json(AuthResponse { token, user }))
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

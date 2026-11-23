use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use chrono::{Duration, Utc};
use futures::future::{Ready, err, ok};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use umuryango_core_domain::DomainError;
use umuryango_core_domain::models::{FamilyMember, FamilyRole};
use umuryango_i18n::Locale;
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub locale: Locale,
    pub exp: usize,
}

pub fn create_jwt(
    user_id: Uuid,
    locale: Locale,
    secret: &str,
    hours_valid: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now() + Duration::hours(hours_valid);
    let claims = JwtClaims {
        sub: user_id,
        locale,
        exp: expiration.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub locale: Locale,
}

impl FromRequest for AuthenticatedUser {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Extract locale for any potential error
        let locale = req
            .headers()
            .get("Accept-Language")
            .and_then(|h| h.to_str().ok())
            .map(|h| Locale::from_header(Some(h)))
            .unwrap_or(Locale::Rw);

        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let Some(header_str) = auth_header else {
            let msg = umuryango_i18n::t(locale, "errors.missing_auth_header");
            return err(ApiError::unauthorized(msg, locale));
        };

        let token = if let Some(stripped) = header_str.strip_prefix("Bearer ") {
            stripped.trim()
        } else {
            header_str.trim()
        };

        let app_state = match req.app_data::<web::Data<AppState>>() {
            Some(state) => state,
            None => {
                return err(ApiError::internal(
                    "Application state not available",
                    locale,
                ));
            }
        };

        match decode_jwt(token, &app_state.jwt_secret) {
            Ok(claims) => {
                // If caller didn't specify Accept-Language, use token's preferred locale
                let final_locale = if req.headers().contains_key("Accept-Language") {
                    locale
                } else {
                    claims.locale
                };
                ok(AuthenticatedUser {
                    user_id: claims.sub,
                    locale: final_locale,
                })
            }
            Err(_) => {
                let msg = umuryango_i18n::t(locale, "errors.invalid_token");
                err(ApiError::unauthorized(msg, locale))
            }
        }
    }
}

/// Require that the authenticated user is an active member of `family_id`
pub async fn require_family_membership(
    pool: &PgPool,
    user_id: Uuid,
    family_id: Uuid,
    locale: Locale,
) -> Result<FamilyMember, ApiError> {
    #[derive(sqlx::FromRow)]
    struct MemberRow {
        id: Uuid,
        family_id: Uuid,
        user_id: Uuid,
        role: String,
        is_active: bool,
        joined_at: chrono::DateTime<Utc>,
        updated_at: chrono::DateTime<Utc>,
    }

    let row = sqlx::query_as::<_, MemberRow>(
        r#"
        SELECT id, family_id, user_id, role::TEXT as role, is_active, joined_at, updated_at
        FROM family_members
        WHERE family_id = $1 AND user_id = $2 AND is_active = TRUE
        "#,
    )
    .bind(family_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::database(e, locale))?;

    let Some(m) = row else {
        let msg = umuryango_i18n::t(locale, "errors.not_family_member");
        return Err(ApiError::forbidden(msg, locale));
    };

    let role = match m.role.to_lowercase().as_str() {
        "head" => FamilyRole::Head,
        "custodian" | "treasurer" => FamilyRole::Custodian,
        _ => FamilyRole::Member,
    };

    Ok(FamilyMember {
        id: m.id,
        family_id: m.family_id,
        user_id: m.user_id,
        role,
        is_active: m.is_active,
        joined_at: m.joined_at,
        updated_at: m.updated_at,
    })
}

/// Verify that member has one of the allowed roles
pub fn require_role(
    member: &FamilyMember,
    allowed_roles: &[FamilyRole],
    locale: Locale,
) -> Result<(), ApiError> {
    if allowed_roles.contains(&member.role) {
        Ok(())
    } else {
        Err(ApiError::domain(DomainError::UnauthorizedRole, locale))
    }
}

/// Sets PostgreSQL session context `app.current_user_id` inside an ongoing transaction for RLS
pub async fn set_current_user_context<'a>(
    tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT set_config('app.current_user_id', $1, true)")
        .bind(user_id.to_string())
        .execute(&mut **tx)
        .await?;
    Ok(())
}

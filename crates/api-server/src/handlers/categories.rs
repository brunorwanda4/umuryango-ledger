use actix_web::{HttpResponse, web};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use umuryango_core_domain::models::{CategoryKind, MoneyCategory};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::{t, t_args};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::error::{ApiError, LocalizedErrorResponse};
use crate::middleware::{AuthenticatedUser, require_family_membership};
use crate::state::AppState;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListCategoriesQuery {
    #[schema(example = "family")]
    pub scope: Option<String>,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub family_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCategoryPayload {
    #[schema(example = "family")]
    pub scope: String,
    #[schema(value_type = Option<String>, format = "uuid", example = "550e8400-e29b-41d4-a716-446655440000")]
    pub family_id: Option<Uuid>,
    pub kind: CategoryKind,
    #[schema(example = "School Fees")]
    pub name: String,
    #[schema(value_type = Option<String>, format = "uuid")]
    pub idempotency_key: Option<Uuid>,
}

#[derive(sqlx::FromRow)]
struct CategoryRow {
    id: Uuid,
    scope_type: String,
    family_id: Option<Uuid>,
    user_id: Option<Uuid>,
    kind: String,
    name: String,
    created_at: DateTime<Utc>,
}

impl CategoryRow {
    fn into_domain(self) -> MoneyCategory {
        let scope = if self.scope_type == "family" {
            LedgerScope::Family {
                family_id: self.family_id.unwrap_or_default(),
            }
        } else {
            LedgerScope::Personal {
                user_id: self.user_id.unwrap_or_default(),
            }
        };
        MoneyCategory {
            id: self.id,
            scope,
            kind: CategoryKind::from_db_str(&self.kind),
            name: self.name,
            created_at: self.created_at,
        }
    }
}

/// List personal or family money categories
#[utoipa::path(
    get,
    path = "/api/v1/categories",
    tag = "Categories",
    security(("bearer_auth" = [])),
    params(
        ListCategoriesQuery,
        ("Accept-Language" = Option<String>, Header, description = "Language preference (rw or en)")
    ),
    responses(
        (status = 200, description = "List of money categories", body = Vec<MoneyCategory>),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 409, description = "Conflict or missing family_id", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn list_categories(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    query: web::Query<ListCategoriesQuery>,
) -> Result<HttpResponse, ApiError> {
    let scope_str = query
        .scope
        .as_deref()
        .unwrap_or("personal")
        .trim()
        .to_lowercase();

    let rows = if scope_str == "family" {
        let Some(family_id) = query.family_id else {
            let msg = t(user.locale, "errors.family_id_required");
            return Err(ApiError::conflict(msg, user.locale));
        };
        require_family_membership(&data.db, user.user_id, family_id, user.locale).await?;

        sqlx::query_as::<_, CategoryRow>(
            r#"
            SELECT id, scope_type, family_id, user_id, kind, name, created_at
            FROM money_categories
            WHERE scope_type = 'family' AND family_id = $1
            ORDER BY name ASC
            "#,
        )
        .bind(family_id)
        .fetch_all(&data.db)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?
    } else {
        sqlx::query_as::<_, CategoryRow>(
            r#"
            SELECT id, scope_type, family_id, user_id, kind, name, created_at
            FROM money_categories
            WHERE scope_type = 'personal' AND user_id = $1
            ORDER BY name ASC
            "#,
        )
        .bind(user.user_id)
        .fetch_all(&data.db)
        .await
        .map_err(|e| ApiError::database(e, user.locale))?
    };

    let categories: Vec<MoneyCategory> = rows.into_iter().map(CategoryRow::into_domain).collect();
    Ok(HttpResponse::Ok().json(categories))
}

/// Create a new income or expense money category
#[utoipa::path(
    post,
    path = "/api/v1/categories",
    tag = "Categories",
    security(("bearer_auth" = [])),
    request_body = CreateCategoryPayload,
    responses(
        (status = 201, description = "Category created", body = MoneyCategory),
        (status = 401, description = "Unauthorized", body = LocalizedErrorResponse),
        (status = 409, description = "Validation conflict, duplicate category, or missing family_id", body = LocalizedErrorResponse),
        (status = 500, description = "Internal server error", body = LocalizedErrorResponse)
    )
)]
pub async fn create_category(
    data: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<CreateCategoryPayload>,
) -> Result<HttpResponse, ApiError> {
    let trimmed_name = payload.name.trim();
    if trimmed_name.is_empty() {
        let msg = t(user.locale, "categories.name_required");
        return Err(ApiError::conflict(msg, user.locale));
    }

    let scope_str = payload.scope.trim().to_lowercase();
    let (scope_type, family_id, user_id) = if scope_str == "family" {
        let Some(fid) = payload.family_id else {
            let msg = t(user.locale, "errors.family_id_required");
            return Err(ApiError::conflict(msg, user.locale));
        };
        require_family_membership(&data.db, user.user_id, fid, user.locale).await?;
        ("family", Some(fid), Some(user.user_id))
    } else {
        ("personal", None, Some(user.user_id))
    };

    let id = payload.idempotency_key.unwrap_or_else(Uuid::new_v4);
    let kind_str = payload.kind.as_str();

    let row = sqlx::query_as::<_, CategoryRow>(
        r#"
        INSERT INTO money_categories (id, scope_type, family_id, user_id, kind, name)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name
        RETURNING id, scope_type, family_id, user_id, kind, name, created_at
        "#,
    )
    .bind(id)
    .bind(scope_type)
    .bind(family_id)
    .bind(user_id)
    .bind(kind_str)
    .bind(trimmed_name)
    .fetch_one(&data.db)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            let msg = t_args(
                user.locale,
                "categories.duplicate_error",
                &[("name", trimmed_name)],
            );
            ApiError::conflict(msg, user.locale)
        }
        _ => ApiError::database(e, user.locale),
    })?;

    Ok(HttpResponse::Created().json(row.into_domain()))
}

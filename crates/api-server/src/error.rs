use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;
use umuryango_core_domain::DomainError;
use umuryango_i18n::{terms, Locale, Translator};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedErrorResponse {
    pub code: String,
    pub locale: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum ApiErrorKind {
    #[error("domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("not found")]
    NotFound,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("database error")]
    Database(#[source] sqlx::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Debug)]
pub struct ApiError {
    pub kind: ApiErrorKind,
    pub locale: Locale,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (locale: {})", self.kind, self.locale)
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ApiErrorKind::Domain(d) => Some(d),
            ApiErrorKind::Database(e) => Some(e),
            _ => None,
        }
    }
}

impl ApiError {
    #[allow(dead_code)]
    pub fn new(kind: ApiErrorKind, locale: Locale) -> Self {
        Self { kind, locale }
    }

    pub fn domain(err: DomainError, locale: Locale) -> Self {
        Self {
            kind: ApiErrorKind::Domain(err),
            locale,
        }
    }

    pub fn unauthorized(msg: impl Into<String>, locale: Locale) -> Self {
        Self {
            kind: ApiErrorKind::Unauthorized(msg.into()),
            locale,
        }
    }

    pub fn forbidden(msg: impl Into<String>, locale: Locale) -> Self {
        Self {
            kind: ApiErrorKind::Forbidden(msg.into()),
            locale,
        }
    }

    pub fn not_found(locale: Locale) -> Self {
        Self {
            kind: ApiErrorKind::NotFound,
            locale,
        }
    }

    pub fn conflict(msg: impl Into<String>, locale: Locale) -> Self {
        Self {
            kind: ApiErrorKind::Conflict(msg.into()),
            locale,
        }
    }

    pub fn internal(msg: impl Into<String>, locale: Locale) -> Self {
        Self {
            kind: ApiErrorKind::Internal(msg.into()),
            locale,
        }
    }

    pub fn database(err: sqlx::Error, locale: Locale) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::not_found(locale),
            sqlx::Error::Database(ref db_err) if db_err.code().as_deref() == Some("23505") => {
                let msg = if locale == Locale::Rw {
                    "Ibyo wanditse birasanzwe muri sisitemu."
                } else {
                    "Record already exists."
                };
                Self::conflict(msg, locale)
            }
            other => {
                error!("Database execution error: {:?}", other);
                Self {
                    kind: ApiErrorKind::Database(other),
                    locale,
                }
            }
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match &self.kind {
            ApiErrorKind::Domain(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiErrorKind::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiErrorKind::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiErrorKind::NotFound => StatusCode::NOT_FOUND,
            ApiErrorKind::Conflict(_) => StatusCode::CONFLICT,
            ApiErrorKind::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorKind::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, message) = match &self.kind {
            ApiErrorKind::Domain(d) => {
                let localized = d.localize(self.locale);
                let code_upper = localized
                    .code
                    .trim_start_matches("error-")
                    .to_uppercase()
                    .replace('-', "_");
                (code_upper, localized.message)
            }
            ApiErrorKind::Unauthorized(msg) => ("UNAUTHORIZED".to_string(), msg.clone()),
            ApiErrorKind::Forbidden(msg) => ("FORBIDDEN".to_string(), msg.clone()),
            ApiErrorKind::NotFound => {
                let msg = Translator::t(self.locale, terms::ERR_NOT_FOUND, None);
                ("NOT_FOUND".to_string(), msg)
            }
            ApiErrorKind::Conflict(msg) => ("CONFLICT".to_string(), msg.clone()),
            ApiErrorKind::Database(_) => {
                let msg = if self.locale == Locale::Rw {
                    "Habaye ikibazo mu bubiko bw'amakuru. Ongera ugerageze."
                } else {
                    "A database error occurred. Please try again."
                };
                ("DATABASE_ERROR".to_string(), msg.to_string())
            }
            ApiErrorKind::Internal(msg) => ("INTERNAL_ERROR".to_string(), msg.clone()),
        };

        HttpResponse::build(self.status_code()).json(LocalizedErrorResponse {
            code,
            locale: self.locale.as_str().to_string(),
            message,
        })
    }
}

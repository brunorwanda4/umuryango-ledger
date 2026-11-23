use rust_decimal::Decimal;
use thiserror::Error;
use umuryango_i18n::{Locale, LocalizableError, LocalizedError, localize_domain_error, terms};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error(
        "insufficient custody balance in wallet '{wallet}': available {available}, requested {requested}"
    )]
    InsufficientCustodyBalance {
        wallet: String,
        available: Decimal,
        requested: Decimal,
    },

    #[error("unauthorized role: your role in this family does not allow performing this action")]
    UnauthorizedRole,

    #[error(
        "invalid handover: handover must be between two different custody wallets within the same family"
    )]
    InvalidHandover,

    #[error("scope mismatch: cannot mix personal wallet funds with family ledger transactions")]
    ScopeMismatch,

    #[error("invalid amount: transaction amount must be greater than zero")]
    InvalidAmount,

    #[error("requested record was not found")]
    NotFound,

    #[error("validation error: {0}")]
    ValidationError(String),

    #[error("passwords do not match")]
    PasswordMismatch,

    #[error("phone already exists")]
    PhoneAlreadyExists,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("invalid invite code")]
    InvalidInviteCode,
}

impl LocalizableError for DomainError {
    fn error_code(&self) -> &'static str {
        match self {
            Self::InsufficientCustodyBalance { .. } => terms::ERR_INSUFFICIENT_CUSTODY,
            Self::UnauthorizedRole => terms::ERR_UNAUTHORIZED_ROLE,
            Self::InvalidHandover => terms::ERR_INVALID_HANDOVER,
            Self::ScopeMismatch => terms::ERR_SCOPE_MISMATCH,
            Self::InvalidAmount => terms::ERR_INVALID_AMOUNT,
            Self::NotFound => terms::ERR_NOT_FOUND,
            Self::ValidationError(_) => terms::ERR_VALIDATION,
            Self::PasswordMismatch => terms::ERR_PASSWORD_MISMATCH,
            Self::PhoneAlreadyExists => terms::ERR_PHONE_ALREADY_EXISTS,
            Self::InvalidCredentials => terms::ERR_INVALID_CREDENTIALS,
            Self::InvalidInviteCode => terms::ERR_INVALID_INVITE_CODE,
        }
    }

    fn error_args(&self) -> Vec<(&'static str, String)> {
        match self {
            Self::InsufficientCustodyBalance {
                wallet,
                available,
                requested,
            } => vec![
                ("wallet", wallet.clone()),
                ("available", available.to_string()),
                ("requested", requested.to_string()),
            ],
            Self::ValidationError(msg) => vec![("message", msg.clone())],
            _ => Vec::new(),
        }
    }
}

impl DomainError {
    pub fn localize(&self, locale: Locale) -> LocalizedError {
        localize_domain_error(locale, self)
    }
}

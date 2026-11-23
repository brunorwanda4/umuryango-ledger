use fluent_bundle::{FluentArgs, FluentValue};
use rust_decimal::Decimal;
use thiserror::Error;
use umuryango_i18n::{localize_domain_error, terms, Locale, LocalizableError, LocalizedError};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error("insufficient custody balance in wallet '{wallet}': available {available}, requested {requested}")]
    InsufficientCustodyBalance {
        wallet: String,
        available: Decimal,
        requested: Decimal,
    },

    #[error("unauthorized role: your role in this family does not allow performing this action")]
    UnauthorizedRole,

    #[error("invalid handover: handover must be between two different custody wallets within the same family")]
    InvalidHandover,

    #[error("scope mismatch: cannot mix personal wallet funds with family ledger transactions")]
    ScopeMismatch,

    #[error("invalid amount: transaction amount must be greater than zero")]
    InvalidAmount,

    #[error("requested record was not found")]
    NotFound,

    #[error("validation error: {0}")]
    ValidationError(String),
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
        }
    }

    fn to_fluent_args(&self) -> Option<FluentArgs<'static>> {
        match self {
            Self::InsufficientCustodyBalance {
                wallet,
                available,
                requested,
            } => {
                let mut args = FluentArgs::new();
                args.set("wallet", FluentValue::from(wallet.clone()));
                args.set("available", FluentValue::from(available.to_string()));
                args.set("requested", FluentValue::from(requested.to_string()));
                Some(args)
            }
            Self::ValidationError(msg) => {
                let mut args = FluentArgs::new();
                args.set("message", FluentValue::from(msg.clone()));
                Some(args)
            }
            _ => None,
        }
    }
}

impl DomainError {
    pub fn localize(&self, locale: Locale) -> LocalizedError {
        localize_domain_error(locale, self)
    }
}

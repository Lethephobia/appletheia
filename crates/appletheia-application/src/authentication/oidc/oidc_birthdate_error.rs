use thiserror::Error;

use super::{OidcBirthMonthError, OidcBirthYearError, OidcBirthdateFullError};

/// Errors that can occur when parsing an OIDC birthdate claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum OidcBirthdateError {
    #[error("invalid birthdate format")]
    InvalidFormat,

    #[error(transparent)]
    Year(#[from] OidcBirthYearError),

    #[error(transparent)]
    Month(#[from] OidcBirthMonthError),

    #[error(transparent)]
    Full(#[from] OidcBirthdateFullError),
}

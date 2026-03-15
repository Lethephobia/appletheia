use thiserror::Error;

/// Errors that can occur when constructing `AuthTokenExchangeCodeExpiresIn`.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum AuthTokenExchangeCodeExpiresInError {
    #[error("exchange code expires_in must be positive")]
    NonPositive,
}

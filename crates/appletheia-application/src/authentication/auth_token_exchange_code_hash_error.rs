use thiserror::Error;

/// Errors that can occur when constructing `AuthTokenExchangeCodeHash`.
#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum AuthTokenExchangeCodeHashError {
    #[error("auth token exchange code hash has invalid format")]
    InvalidFormat,
}

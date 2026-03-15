use thiserror::Error;

use super::AuthTokenExchangeCodeHashError;

/// Errors returned while hashing auth token exchange codes.
#[derive(Debug, Error)]
pub enum AuthTokenExchangeCodeHasherError {
    #[error(transparent)]
    Hash(#[from] AuthTokenExchangeCodeHashError),
}

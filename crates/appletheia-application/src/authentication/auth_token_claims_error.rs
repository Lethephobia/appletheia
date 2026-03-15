use thiserror::Error;

use super::AuthTokenExpiresInError;

/// Errors that can occur when deriving values from `AuthTokenClaims`.
#[derive(Debug, Error)]
pub enum AuthTokenClaimsError {
    #[error(transparent)]
    ExpiresIn(#[from] AuthTokenExpiresInError),
}

use appletheia::application::Retryability;

use appletheia::application::authentication::AuthTokenRevocationError;
use thiserror::Error;

/// Represents errors returned while revoking an access token.
#[derive(Debug, Error)]
pub enum LogoutCommandHandlerError {
    #[error("auth token revocation failed")]
    AuthTokenRevoker(#[from] AuthTokenRevocationError),
}

impl Retryability for LogoutCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::AuthTokenRevoker(error) => error.is_retryable(),
        }
    }
}

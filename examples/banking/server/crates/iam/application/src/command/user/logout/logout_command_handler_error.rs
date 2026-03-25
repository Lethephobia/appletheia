use appletheia::application::authentication::AuthTokenRevocationError;
use thiserror::Error;

/// Represents errors returned while revoking an access token.
#[derive(Debug, Error)]
pub enum LogoutCommandHandlerError {
    #[error("auth token revocation failed")]
    AuthTokenRevoker(#[from] AuthTokenRevocationError),
}

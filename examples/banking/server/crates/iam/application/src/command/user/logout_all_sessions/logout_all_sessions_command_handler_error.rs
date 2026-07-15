use appletheia::application::authentication::AuthTokenRevocationError;
use thiserror::Error;

/// Represents errors returned while revoking all sessions for a subject.
#[derive(Debug, Error)]
pub enum LogoutAllSessionsCommandHandlerError {
    #[error("auth token revocation failed")]
    AuthTokenRevoker(#[from] AuthTokenRevocationError),
}

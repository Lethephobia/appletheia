use appletheia::application::authentication::AuthTokenRevocationError;
use appletheia::application::request_context::Principal;
use thiserror::Error;

/// Represents errors returned while revoking all sessions for a subject.
#[derive(Debug, Error)]
pub enum LogoutAllSessionsCommandHandlerError {
    #[error("auth token revocation failed")]
    AuthTokenRevoker(#[from] AuthTokenRevocationError),

    #[error("logout all sessions requires an authenticated principal")]
    AuthenticatedPrincipalRequired,

    #[error("logout all sessions requires a subject principal")]
    SubjectPrincipalRequired(Principal),
}

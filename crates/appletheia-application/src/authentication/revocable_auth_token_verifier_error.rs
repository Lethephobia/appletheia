use thiserror::Error;

use super::AuthTokenVerifierError;

/// Errors returned by a revocable auth token verifier.
#[derive(Debug, Error)]
pub enum RevocableAuthTokenVerifierError {
    #[error(transparent)]
    Verifier(#[from] AuthTokenVerifierError),

    #[error("token revoked")]
    Revoked,

    #[error("revocable token verify failed")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

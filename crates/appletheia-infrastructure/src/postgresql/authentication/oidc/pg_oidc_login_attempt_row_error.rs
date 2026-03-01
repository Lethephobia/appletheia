use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgOidcLoginAttemptRowError {
    #[error("invalid persisted pkce code verifier")]
    InvalidPkceCodeVerifier,
}

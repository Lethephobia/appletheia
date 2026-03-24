use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgOidcLoginAttemptRowError {
    #[error("invalid persisted oidc state")]
    InvalidOidcState,

    #[error("invalid persisted oidc nonce")]
    InvalidOidcNonce,

    #[error("invalid persisted pkce code verifier")]
    InvalidPkceCodeVerifier,
}

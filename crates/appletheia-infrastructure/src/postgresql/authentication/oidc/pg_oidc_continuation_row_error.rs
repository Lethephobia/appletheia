use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgOidcContinuationRowError {
    #[error("invalid persisted oidc continuation payload")]
    InvalidPayload,

    #[error("invalid persisted oidc state")]
    InvalidOidcState,
}

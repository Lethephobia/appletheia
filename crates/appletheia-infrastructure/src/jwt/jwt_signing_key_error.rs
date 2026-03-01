use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtSigningKeyError {
    #[error("invalid signing key")]
    Backend(#[source] jsonwebtoken::errors::Error),
}

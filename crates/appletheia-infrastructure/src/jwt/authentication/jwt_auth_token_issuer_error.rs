use thiserror::Error;

use crate::jwt::JwtSigningKeyError;

#[derive(Debug, Error)]
pub enum JwtAuthTokenIssuerError {
    #[error("failed to prepare signing key")]
    SigningKey(#[source] JwtSigningKeyError),

    #[error("failed to encode jwt")]
    Encode(#[source] jsonwebtoken::errors::Error),
}

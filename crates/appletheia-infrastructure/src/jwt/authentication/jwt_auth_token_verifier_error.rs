use thiserror::Error;

use super::jwt_auth_token_claims_error::JwtAuthTokenClaimsError;

#[derive(Debug, Error)]
pub enum JwtAuthTokenVerifierError {
    #[error("missing key id (kid) in jwt header")]
    MissingKeyId,

    #[error("unknown key id (kid)")]
    UnknownKeyId,

    #[error("failed to decode jwt header")]
    DecodeHeader(#[source] jsonwebtoken::errors::Error),

    #[error("failed to create decoding key")]
    InvalidKey(#[source] jsonwebtoken::errors::Error),

    #[error("failed to decode jwt")]
    Decode(#[source] jsonwebtoken::errors::Error),

    #[error(transparent)]
    Claims(#[from] JwtAuthTokenClaimsError),
}

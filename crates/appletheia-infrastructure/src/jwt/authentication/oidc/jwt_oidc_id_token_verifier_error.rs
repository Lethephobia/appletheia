use thiserror::Error;

use super::jwt_oidc_id_token_claims_error::JwtOidcIdTokenClaimsError;

#[derive(Debug, Error)]
pub enum JwtOidcIdTokenVerifierError {
    #[error("failed to decode jwt header")]
    DecodeHeader(#[source] jsonwebtoken::errors::Error),

    #[error("missing key id (kid) in jwt header")]
    MissingKeyId,

    #[error("invalid key id (kid)")]
    InvalidKeyId(#[source] crate::jwt::JwkKeyIdError),

    #[error("unknown key id (kid)")]
    UnknownKeyId,

    #[error("failed to read jwks")]
    JwksSource(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("failed to create decoding key")]
    InvalidKey(#[source] jsonwebtoken::errors::Error),

    #[error("failed to decode jwt")]
    Decode(#[source] jsonwebtoken::errors::Error),

    #[error("nonce claim is missing")]
    MissingNonce,

    #[error("nonce claim does not match")]
    NonceMismatch,

    #[error("current time is invalid")]
    InvalidCurrentTime,

    #[error("issued-at claim is in the future")]
    InvalidIssuedAt,

    #[error("unsupported id token signing algorithm")]
    UnsupportedAlgorithm,

    #[error("access token hash does not match")]
    AccessTokenHashMismatch,

    #[error(transparent)]
    Claims(#[from] JwtOidcIdTokenClaimsError),
}

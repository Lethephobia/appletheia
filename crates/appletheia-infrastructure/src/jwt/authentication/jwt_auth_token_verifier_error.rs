use thiserror::Error;

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

    #[error("missing required claim: {name}")]
    MissingRequiredClaim { name: &'static str },

    #[error("invalid token id")]
    InvalidTokenId(#[source] uuid::Error),

    #[error("invalid claim value")]
    InvalidClaimValue(#[source] Box<dyn std::error::Error + Send + Sync>),
}

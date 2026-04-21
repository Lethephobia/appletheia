use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtAuthTokenClaimsError {
    #[error("missing required claim: {name}")]
    MissingRequiredClaim { name: &'static str },

    #[error("invalid token id")]
    InvalidTokenId(#[source] uuid::Error),

    #[error("invalid claim value: {name}")]
    InvalidClaimValue {
        name: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

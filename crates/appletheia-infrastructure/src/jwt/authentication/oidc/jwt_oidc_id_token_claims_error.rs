use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtOidcIdTokenClaimsError {
    #[error("invalid claim value: {name}")]
    InvalidClaimValue {
        name: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("invalid timestamp claim: {name}")]
    InvalidTimestampClaim { name: &'static str },
}

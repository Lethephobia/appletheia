use super::{AuthToken, AuthTokenClaims, AuthTokenVerifierError};

#[allow(async_fn_in_trait)]
pub trait AuthTokenVerifier: Send + Sync {
    async fn verify(&self, token: &AuthToken) -> Result<AuthTokenClaims, AuthTokenVerifierError>;
}

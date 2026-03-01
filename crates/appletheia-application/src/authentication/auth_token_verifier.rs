use super::{AuthToken, AuthTokenClaims, AuthTokenVerifyError};

#[allow(async_fn_in_trait)]
pub trait AuthTokenVerifier: Send + Sync {
    async fn verify(&self, token: &AuthToken) -> Result<AuthTokenClaims, AuthTokenVerifyError>;
}

use super::{OidcIdToken, OidcIdTokenClaims, OidcIdTokenVerifyContext, OidcIdTokenVerifyError};

#[allow(async_fn_in_trait)]
pub trait OidcIdTokenVerifier: Send + Sync {
    async fn verify(
        &self,
        id_token: &OidcIdToken,
        context: OidcIdTokenVerifyContext,
    ) -> Result<OidcIdTokenClaims, OidcIdTokenVerifyError>;
}

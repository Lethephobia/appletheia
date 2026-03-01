use super::{OidcTokenClientError, OidcTokenRequest, OidcTokenResponse};

#[allow(async_fn_in_trait)]
pub trait OidcTokenClient: Send + Sync {
    async fn request_token(
        &self,
        request: OidcTokenRequest,
    ) -> Result<OidcTokenResponse, OidcTokenClientError>;
}

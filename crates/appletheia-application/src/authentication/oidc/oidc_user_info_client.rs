use super::{OidcAccessToken, OidcUserInfo, OidcUserInfoClientError, OidcUserInfoEndpointUrl};

/// Fetches OIDC user info from a provider endpoint.
#[allow(async_fn_in_trait)]
pub trait OidcUserInfoClient: Send + Sync {
    /// Reads user info using the provided access token.
    async fn read_user_info(
        &self,
        endpoint_url: &OidcUserInfoEndpointUrl,
        access_token: &OidcAccessToken,
    ) -> Result<OidcUserInfo, OidcUserInfoClientError>;
}

use appletheia_application::authentication::oidc::{
    OidcAccessToken, OidcUserInfo, OidcUserInfoClient, OidcUserInfoClientError,
    OidcUserInfoEndpointUrl,
};
use reqwest::header::{ACCEPT, AUTHORIZATION};

use super::http_oidc_user_info_client_error::HttpOidcUserInfoClientError;
use super::oidc_user_info_body::OidcUserInfoBody;

/// Fetches OIDC user info over HTTP.
#[derive(Clone, Debug)]
pub struct HttpOidcUserInfoClient {
    client: reqwest::Client,
}

impl HttpOidcUserInfoClient {
    /// Creates a new HTTP user-info client.
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Default for HttpOidcUserInfoClient {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}

impl OidcUserInfoClient for HttpOidcUserInfoClient {
    async fn read_user_info(
        &self,
        endpoint_url: &OidcUserInfoEndpointUrl,
        access_token: &OidcAccessToken,
    ) -> Result<OidcUserInfo, OidcUserInfoClientError> {
        let response = self
            .client
            .get(endpoint_url.value().clone())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", access_token.value()))
            .send()
            .await
            .map_err(|source| OidcUserInfoClientError::Backend(Box::new(source)))?;

        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|source| OidcUserInfoClientError::Backend(Box::new(source)))?;

        if !status.is_success() {
            let body = String::from_utf8_lossy(&bytes).to_string();
            let error = HttpOidcUserInfoClientError::UnexpectedStatus {
                status: status.as_u16(),
                body,
            };
            return Err(OidcUserInfoClientError::Backend(Box::new(error)));
        }

        let body = OidcUserInfoBody::try_from_json_bytes(&bytes)
            .map_err(|source| OidcUserInfoClientError::Backend(Box::new(source)))?;

        body.try_into_user_info()
            .map_err(|source| OidcUserInfoClientError::Backend(Box::new(source)))
    }
}

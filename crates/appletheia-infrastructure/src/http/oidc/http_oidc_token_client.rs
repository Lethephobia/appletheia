use appletheia_application::authentication::oidc::{
    OidcAccessToken, OidcClientAuth, OidcIdToken, OidcRefreshToken, OidcTokenClient,
    OidcTokenClientError, OidcTokenExpiresIn, OidcTokenGrant, OidcTokenRequest, OidcTokenResponse,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use super::http_oidc_token_client_error::HttpOidcTokenClientError;

#[derive(Debug, Clone)]
pub struct HttpOidcTokenClient {
    client: reqwest::Client,
}

impl HttpOidcTokenClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    fn build_form_body(request: &OidcTokenRequest) -> String {
        let mut serializer = url::form_urlencoded::Serializer::new(String::new());

        serializer.append_pair("client_id", request.client_id.value());

        match &request.client_auth {
            OidcClientAuth::None | OidcClientAuth::ClientSecretBasic { .. } => {}
            OidcClientAuth::ClientSecretPost { client_secret } => {
                serializer.append_pair("client_secret", client_secret.value());
            }
        }

        match &request.grant {
            OidcTokenGrant::AuthorizationCode {
                authorization_code,
                redirect_uri,
                pkce_code_verifier,
            } => {
                serializer.append_pair("grant_type", "authorization_code");
                serializer.append_pair("code", authorization_code.value());
                serializer.append_pair("redirect_uri", redirect_uri.value().as_str());
                if let Some(code_verifier) = pkce_code_verifier {
                    serializer.append_pair("code_verifier", code_verifier.value());
                }
            }
            OidcTokenGrant::RefreshToken {
                refresh_token,
                scopes,
            } => {
                serializer.append_pair("grant_type", "refresh_token");
                serializer.append_pair("refresh_token", refresh_token.value());
                if let Some(scopes) = scopes {
                    serializer.append_pair("scope", &scopes.to_scope_string());
                }
            }
        }

        serializer.finish()
    }

    fn build_basic_authorization_value(
        client_id: &str,
        client_secret: &str,
    ) -> Result<String, OidcTokenClientError> {
        let raw = format!("{client_id}:{client_secret}");
        let encoded = STANDARD.encode(raw.as_bytes());
        Ok(format!("Basic {encoded}"))
    }
}

impl Default for HttpOidcTokenClient {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}

impl OidcTokenClient for HttpOidcTokenClient {
    async fn request_token(
        &self,
        request: OidcTokenRequest,
    ) -> Result<OidcTokenResponse, OidcTokenClientError> {
        let form_body = Self::build_form_body(&request);

        let mut builder = self
            .client
            .post(request.token_endpoint_url)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(form_body);

        if let OidcClientAuth::ClientSecretBasic { client_secret } = request.client_auth {
            let header_value = Self::build_basic_authorization_value(
                request.client_id.value(),
                client_secret.value(),
            )?;
            builder = builder.header(AUTHORIZATION, header_value);
        }

        let response = builder
            .send()
            .await
            .map_err(|source| OidcTokenClientError::Backend(Box::new(source)))?;

        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(|source| OidcTokenClientError::Backend(Box::new(source)))?;

        if !status.is_success() {
            let body = String::from_utf8_lossy(&bytes).to_string();
            let error = HttpOidcTokenClientError::UnexpectedStatus {
                status: status.as_u16(),
                body,
            };
            return Err(OidcTokenClientError::Backend(Box::new(error)));
        }

        #[derive(Debug, serde::Deserialize)]
        struct TokenResponseBody {
            id_token: Option<String>,
            access_token: Option<String>,
            refresh_token: Option<String>,
            expires_in: Option<u64>,
        }

        let decoded: TokenResponseBody = serde_json::from_slice(&bytes)
            .map_err(|source| OidcTokenClientError::Backend(Box::new(source)))?;

        let expires_in = match decoded.expires_in {
            None => None,
            Some(seconds) => Some(
                OidcTokenExpiresIn::from_seconds(seconds)
                    .map_err(|source| OidcTokenClientError::Backend(Box::new(source)))?,
            ),
        };

        Ok(OidcTokenResponse {
            id_token: decoded.id_token.map(OidcIdToken::new),
            access_token: decoded.access_token.map(OidcAccessToken::new),
            refresh_token: decoded.refresh_token.map(OidcRefreshToken::new),
            expires_in,
        })
    }
}

use appletheia_application::authentication::oidc::{
    OidcAuthorizationEndpointUrl, OidcIssuerUrl, OidcJwksUri, OidcProviderMetadata,
    OidcTokenEndpointUrl,
};
use serde::Deserialize;
use url::Url;

use super::OidcProviderMetadataBodyError;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct OidcProviderMetadataBody {
    #[serde(rename = "issuer")]
    issuer_url: Url,

    #[serde(rename = "authorization_endpoint")]
    authorization_endpoint_url: Url,

    #[serde(rename = "token_endpoint")]
    token_endpoint_url: Url,

    #[serde(rename = "jwks_uri")]
    jwks_uri: Url,
}

impl OidcProviderMetadataBody {
    pub fn try_from_json_bytes(bytes: &[u8]) -> Result<Self, OidcProviderMetadataBodyError> {
        serde_json::from_slice(bytes).map_err(OidcProviderMetadataBodyError::InvalidJson)
    }

    pub fn into_provider_metadata(&self) -> OidcProviderMetadata {
        OidcProviderMetadata {
            issuer_url: OidcIssuerUrl::new(self.issuer_url.clone()),
            authorization_endpoint_url: OidcAuthorizationEndpointUrl::new(
                self.authorization_endpoint_url.clone(),
            ),
            token_endpoint_url: OidcTokenEndpointUrl::new(self.token_endpoint_url.clone()),
            jwks_uri: OidcJwksUri::new(self.jwks_uri.clone()),
        }
    }
}

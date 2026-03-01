use super::{OidcAuthorizationEndpointUrl, OidcIssuerUrl, OidcJwksUri, OidcTokenEndpointUrl};

#[derive(Clone, Debug)]
pub struct OidcProviderMetadata {
    pub issuer_url: OidcIssuerUrl,
    pub authorization_endpoint_url: OidcAuthorizationEndpointUrl,
    pub token_endpoint_url: OidcTokenEndpointUrl,
    pub jwks_uri: OidcJwksUri,
}

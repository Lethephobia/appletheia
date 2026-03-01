use super::{OidcAccessToken, OidcClientId, OidcIssuerUrl, OidcJwksUri, OidcNonce};

#[derive(Clone, Debug)]
pub struct OidcIdTokenVerifyContext {
    pub issuer_url: OidcIssuerUrl,
    pub client_id: OidcClientId,
    pub jwks_uri: OidcJwksUri,
    pub access_token: Option<OidcAccessToken>,
    pub expected_nonce: Option<OidcNonce>,
}

use super::{OidcClientAuth, OidcClientId, OidcIssuerUrl, OidcPkceMode, OidcRedirectUri};

#[derive(Clone, Debug)]
pub struct OidcProviderConfig {
    pub issuer_url: OidcIssuerUrl,
    pub client_id: OidcClientId,
    pub redirect_uri: OidcRedirectUri,
    pub client_auth: OidcClientAuth,
    pub pkce_mode: OidcPkceMode,
}

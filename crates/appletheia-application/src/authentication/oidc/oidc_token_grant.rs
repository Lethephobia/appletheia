use super::{
    OidcAuthorizationCode, OidcPkceCodeVerifier, OidcRedirectUri, OidcRefreshToken, OidcScopes,
};

#[derive(Clone, Debug)]
pub enum OidcTokenGrant {
    AuthorizationCode {
        authorization_code: OidcAuthorizationCode,
        redirect_uri: OidcRedirectUri,
        pkce_code_verifier: Option<OidcPkceCodeVerifier>,
    },
    RefreshToken {
        refresh_token: OidcRefreshToken,
        scopes: Option<OidcScopes>,
    },
}

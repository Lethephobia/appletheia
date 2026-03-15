use super::{
    OidcAuthorizationCode, OidcRedirectUri, OidcRefreshToken, OidcScopes, PkceCodeVerifier,
};

#[derive(Clone, Debug)]
pub enum OidcTokenGrant {
    AuthorizationCode {
        authorization_code: OidcAuthorizationCode,
        redirect_uri: OidcRedirectUri,
        pkce_code_verifier: Option<PkceCodeVerifier>,
    },
    RefreshToken {
        refresh_token: OidcRefreshToken,
        scopes: Option<OidcScopes>,
    },
}

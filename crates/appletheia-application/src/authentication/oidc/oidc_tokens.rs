use super::{OidcAccessToken, OidcIdToken, OidcRefreshToken, OidcTokenExpiresIn};

/// Stores OIDC tokens returned by a completed authentication flow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OidcTokens {
    id_token: OidcIdToken,
    access_token: Option<OidcAccessToken>,
    refresh_token: Option<OidcRefreshToken>,
    expires_in: Option<OidcTokenExpiresIn>,
}

impl OidcTokens {
    /// Creates a new set of OIDC tokens.
    pub fn new(
        id_token: OidcIdToken,
        access_token: Option<OidcAccessToken>,
        refresh_token: Option<OidcRefreshToken>,
        expires_in: Option<OidcTokenExpiresIn>,
    ) -> Self {
        Self {
            id_token,
            access_token,
            refresh_token,
            expires_in,
        }
    }

    /// Returns the ID token.
    pub fn id_token(&self) -> &OidcIdToken {
        &self.id_token
    }

    /// Returns the access token, if present.
    pub fn access_token(&self) -> Option<&OidcAccessToken> {
        self.access_token.as_ref()
    }

    /// Returns the refresh token, if present.
    pub fn refresh_token(&self) -> Option<&OidcRefreshToken> {
        self.refresh_token.as_ref()
    }

    /// Returns the OIDC token lifetime, if present.
    pub fn expires_in(&self) -> Option<OidcTokenExpiresIn> {
        self.expires_in
    }
}

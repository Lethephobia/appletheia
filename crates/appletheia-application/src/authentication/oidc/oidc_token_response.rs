use super::{OidcAccessToken, OidcIdToken, OidcRefreshToken, OidcTokenExpiresIn};

#[derive(Clone, Debug)]
pub struct OidcTokenResponse {
    pub id_token: Option<OidcIdToken>,
    pub access_token: Option<OidcAccessToken>,
    pub refresh_token: Option<OidcRefreshToken>,
    pub expires_in: Option<OidcTokenExpiresIn>,
}

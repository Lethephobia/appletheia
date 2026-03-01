use super::{
    OidcAccessToken, OidcIdToken, OidcIdTokenClaims, OidcRefreshToken, OidcTokenExpiresIn,
};

#[derive(Clone, Debug)]
pub struct OidcCompleteResult {
    pub id_token: OidcIdToken,
    pub id_token_claims: OidcIdTokenClaims,
    pub access_token: Option<OidcAccessToken>,
    pub refresh_token: Option<OidcRefreshToken>,
    pub expires_in: Option<OidcTokenExpiresIn>,
}

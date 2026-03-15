use super::{OidcIdTokenClaims, OidcTokens};

#[derive(Clone, Debug)]
pub struct OidcCompleteResult {
    pub tokens: OidcTokens,
    pub id_token_claims: OidcIdTokenClaims,
}

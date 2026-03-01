use serde::{Deserialize, Serialize};

use super::{
    OidcAccessTokenHash, OidcAudiences, OidcAuthTime, OidcIdTokenExpiresAt, OidcIdTokenIssuedAt,
    OidcIssuerUrl, OidcNonce, OidcSubject,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcIdTokenClaims {
    pub issuer_url: OidcIssuerUrl,
    pub subject: OidcSubject,
    pub audiences: OidcAudiences,
    pub expires_at: OidcIdTokenExpiresAt,
    pub issued_at: Option<OidcIdTokenIssuedAt>,
    pub auth_time: Option<OidcAuthTime>,
    pub nonce: Option<OidcNonce>,
    pub access_token_hash: Option<OidcAccessTokenHash>,
}

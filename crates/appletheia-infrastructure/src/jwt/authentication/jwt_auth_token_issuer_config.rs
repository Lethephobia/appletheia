use appletheia_application::{AuthTokenAudiences, AuthTokenExpiresIn, AuthTokenIssuerUrl};

use crate::jwt::JwtSigningKey;

#[derive(Clone, Debug)]
pub struct JwtAuthTokenIssuerConfig {
    issuer_url: AuthTokenIssuerUrl,
    audiences: AuthTokenAudiences,
    expires_in: AuthTokenExpiresIn,
    signing_key: JwtSigningKey,
}

impl JwtAuthTokenIssuerConfig {
    pub fn new(
        issuer_url: AuthTokenIssuerUrl,
        audiences: AuthTokenAudiences,
        expires_in: AuthTokenExpiresIn,
        signing_key: JwtSigningKey,
    ) -> Self {
        Self {
            issuer_url,
            audiences,
            expires_in,
            signing_key,
        }
    }

    pub fn issuer_url(&self) -> &AuthTokenIssuerUrl {
        &self.issuer_url
    }

    pub fn audiences(&self) -> &AuthTokenAudiences {
        &self.audiences
    }

    pub fn expires_in(&self) -> AuthTokenExpiresIn {
        self.expires_in
    }

    pub fn signing_key(&self) -> &JwtSigningKey {
        &self.signing_key
    }
}

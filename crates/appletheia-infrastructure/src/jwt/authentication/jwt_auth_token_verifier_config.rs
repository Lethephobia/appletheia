use appletheia_application::{AuthTokenAudiences, AuthTokenIssuerUrls};

use crate::jwt::Jwks;
use crate::jwt::JwtLeewaySeconds;

#[derive(Clone, Debug)]
pub struct JwtAuthTokenVerifierConfig {
    allowed_issuer_urls: AuthTokenIssuerUrls,
    allowed_audiences: AuthTokenAudiences,
    jwks: Jwks,
    leeway_seconds: JwtLeewaySeconds,
}

impl JwtAuthTokenVerifierConfig {
    pub fn new(
        allowed_issuer_urls: AuthTokenIssuerUrls,
        allowed_audiences: AuthTokenAudiences,
        jwks: Jwks,
    ) -> Self {
        Self {
            allowed_issuer_urls,
            allowed_audiences,
            jwks,
            leeway_seconds: JwtLeewaySeconds::default(),
        }
    }

    pub fn with_leeway_seconds(mut self, leeway_seconds: JwtLeewaySeconds) -> Self {
        self.leeway_seconds = leeway_seconds;
        self
    }

    pub fn allowed_issuer_urls(&self) -> &AuthTokenIssuerUrls {
        &self.allowed_issuer_urls
    }

    pub fn allowed_audiences(&self) -> &AuthTokenAudiences {
        &self.allowed_audiences
    }

    pub fn jwks(&self) -> &Jwks {
        &self.jwks
    }

    pub fn leeway_seconds(&self) -> JwtLeewaySeconds {
        self.leeway_seconds
    }
}

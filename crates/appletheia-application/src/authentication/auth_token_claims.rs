use serde::{Deserialize, Serialize};

use crate::authorization::AggregateRef;

use super::{
    AuthTokenAudiences, AuthTokenExpiresAt, AuthTokenId, AuthTokenIssuedAt, AuthTokenIssuerUrl,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuthTokenClaims {
    issuer_url: AuthTokenIssuerUrl,
    audiences: AuthTokenAudiences,
    subject: AggregateRef,
    issued_at: AuthTokenIssuedAt,
    expires_at: AuthTokenExpiresAt,
    token_id: AuthTokenId,
}

impl AuthTokenClaims {
    pub fn new(
        issuer_url: AuthTokenIssuerUrl,
        audiences: AuthTokenAudiences,
        subject: AggregateRef,
        issued_at: AuthTokenIssuedAt,
        expires_at: AuthTokenExpiresAt,
        token_id: AuthTokenId,
    ) -> Self {
        Self {
            issuer_url,
            audiences,
            subject,
            issued_at,
            expires_at,
            token_id,
        }
    }

    pub fn issuer_url(&self) -> &AuthTokenIssuerUrl {
        &self.issuer_url
    }

    pub fn audiences(&self) -> &AuthTokenAudiences {
        &self.audiences
    }

    pub fn subject(&self) -> &AggregateRef {
        &self.subject
    }

    pub fn issued_at(&self) -> AuthTokenIssuedAt {
        self.issued_at
    }

    pub fn expires_at(&self) -> AuthTokenExpiresAt {
        self.expires_at
    }

    pub fn token_id(&self) -> AuthTokenId {
        self.token_id
    }
}

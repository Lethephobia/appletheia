use serde::{Deserialize, Serialize};

use crate::authorization::AggregateRef;

use super::{
    AuthTokenAudiences, AuthTokenClaimsError, AuthTokenExpiresAt, AuthTokenExpiresIn, AuthTokenId,
    AuthTokenIssuedAt, AuthTokenIssuerUrl,
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

    pub fn expires_in(&self) -> Result<AuthTokenExpiresIn, AuthTokenClaimsError> {
        let duration = self.expires_at.value() - self.issued_at.value();
        Ok(AuthTokenExpiresIn::new(duration)?)
    }

    pub fn token_id(&self) -> AuthTokenId {
        self.token_id
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    use super::AuthTokenClaims;
    use crate::authentication::{
        AuthTokenAudience, AuthTokenAudiences, AuthTokenClaimsError, AuthTokenExpiresAt,
        AuthTokenExpiresIn, AuthTokenExpiresInError, AuthTokenId, AuthTokenIssuedAt,
        AuthTokenIssuerUrl,
    };
    use crate::authorization::AggregateRef;
    use crate::event::{AggregateIdValue, AggregateTypeOwned};

    fn claims(issued_at: AuthTokenIssuedAt, expires_at: AuthTokenExpiresAt) -> AuthTokenClaims {
        let issuer_url = "https://example.com"
            .parse::<AuthTokenIssuerUrl>()
            .expect("valid issuer");
        let audience = AuthTokenAudience::new("web".to_owned()).expect("valid audience");
        let audiences = AuthTokenAudiences::new(audience, Vec::new()).expect("valid audiences");
        let subject = AggregateRef {
            aggregate_type: AggregateTypeOwned::new("user".to_owned()).expect("valid type"),
            aggregate_id: AggregateIdValue::from(Uuid::from_u128(1)),
        };

        AuthTokenClaims::new(
            issuer_url,
            audiences,
            subject,
            issued_at,
            expires_at,
            AuthTokenId::from(Uuid::from_u128(2)),
        )
    }

    #[test]
    fn expires_in_returns_duration_between_issued_at_and_expires_at() {
        let issued_at = AuthTokenIssuedAt::from(Utc::now());
        let expires_at = AuthTokenExpiresAt::from(issued_at.value() + Duration::minutes(15));
        let claims = claims(issued_at, expires_at);

        let expires_in = claims.expires_in().expect("expires_in should be valid");

        assert_eq!(
            expires_in,
            AuthTokenExpiresIn::new(Duration::minutes(15)).expect("positive duration"),
        );
    }

    #[test]
    fn expires_in_returns_error_when_expires_at_is_not_after_issued_at() {
        let issued_at = AuthTokenIssuedAt::from(Utc::now());
        let expires_at = AuthTokenExpiresAt::from(issued_at.value());
        let claims = claims(issued_at, expires_at);

        let error = claims
            .expires_in()
            .expect_err("non-positive duration should fail");

        assert!(matches!(
            error,
            AuthTokenClaimsError::ExpiresIn(AuthTokenExpiresInError::NonPositive)
        ));
    }
}

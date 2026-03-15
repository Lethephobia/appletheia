use super::{AuthToken, AuthTokenClaims, AuthTokenClaimsError, AuthTokenExpiresIn};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AuthTokenIssueResult {
    token: AuthToken,
    claims: AuthTokenClaims,
}

impl AuthTokenIssueResult {
    pub fn new(token: AuthToken, claims: AuthTokenClaims) -> Self {
        Self { token, claims }
    }

    pub fn token(&self) -> &AuthToken {
        &self.token
    }

    pub fn claims(&self) -> &AuthTokenClaims {
        &self.claims
    }

    pub fn expires_in(&self) -> Result<AuthTokenExpiresIn, AuthTokenClaimsError> {
        self.claims.expires_in()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    use super::AuthTokenIssueResult;
    use crate::authentication::{
        AuthToken, AuthTokenAudience, AuthTokenAudiences, AuthTokenClaims, AuthTokenExpiresAt,
        AuthTokenExpiresIn, AuthTokenId, AuthTokenIssuedAt, AuthTokenIssuerUrl,
    };
    use crate::authorization::AggregateRef;
    use crate::event::{AggregateIdValue, AggregateTypeOwned};

    #[test]
    fn expires_in_delegates_to_claims() {
        let issuer_url = AuthTokenIssuerUrl::parse("https://example.com").expect("valid issuer");
        let audience = AuthTokenAudience::new("web".to_owned()).expect("valid audience");
        let audiences = AuthTokenAudiences::new(audience, Vec::new()).expect("valid audiences");
        let subject = AggregateRef {
            aggregate_type: AggregateTypeOwned::new("user".to_owned()).expect("valid type"),
            aggregate_id: AggregateIdValue::from(Uuid::from_u128(1)),
        };
        let issued_at = AuthTokenIssuedAt::from(Utc::now());
        let expires_at = AuthTokenExpiresAt::from(issued_at.value() + Duration::minutes(10));
        let claims = AuthTokenClaims::new(
            issuer_url,
            audiences,
            subject,
            issued_at,
            expires_at,
            AuthTokenId::from(Uuid::from_u128(2)),
        );
        let result = AuthTokenIssueResult::new(AuthToken::new("token".to_owned()), claims);

        let expires_in = result.expires_in().expect("expires_in should be valid");

        assert_eq!(
            expires_in,
            AuthTokenExpiresIn::new(Duration::minutes(10)).expect("positive duration"),
        );
    }
}

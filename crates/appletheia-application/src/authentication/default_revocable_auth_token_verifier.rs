use super::{
    AuthToken, AuthTokenClaims, AuthTokenRevocationChecker, AuthTokenVerifier,
    RevocableAuthTokenVerifier, RevocableAuthTokenVerifierError,
};

/// Wraps a verifier and rejects tokens that are revoked by policy.
#[derive(Clone, Debug)]
pub struct DefaultRevocableAuthTokenVerifier<V, C>
where
    V: AuthTokenVerifier,
    C: AuthTokenRevocationChecker,
{
    verifier: V,
    revocation_checker: C,
}

impl<V, C> DefaultRevocableAuthTokenVerifier<V, C>
where
    V: AuthTokenVerifier,
    C: AuthTokenRevocationChecker,
{
    /// Creates a verifier that combines signature validation with revocation checks.
    pub fn new(verifier: V, revocation_checker: C) -> Self {
        Self {
            verifier,
            revocation_checker,
        }
    }
}

impl<V, C> RevocableAuthTokenVerifier for DefaultRevocableAuthTokenVerifier<V, C>
where
    V: AuthTokenVerifier,
    C: AuthTokenRevocationChecker,
{
    type Uow = C::Uow;

    async fn verify(
        &self,
        uow: &mut Self::Uow,
        token: &AuthToken,
    ) -> Result<AuthTokenClaims, RevocableAuthTokenVerifierError> {
        let claims = self.verifier.verify(token).await?;

        let revoked = match self.revocation_checker.is_token_revoked(uow, &claims).await {
            Ok(revoked) => revoked,
            Err(error) => return Err(RevocableAuthTokenVerifierError::Backend(Box::new(error))),
        };

        if revoked {
            return Err(RevocableAuthTokenVerifierError::Revoked);
        }

        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use thiserror::Error;
    use uuid::Uuid;

    use super::DefaultRevocableAuthTokenVerifier;
    use crate::authentication::{
        AuthToken, AuthTokenAudience, AuthTokenAudiences, AuthTokenClaims, AuthTokenExpiresAt,
        AuthTokenId, AuthTokenIssuedAt, AuthTokenIssuerUrl, AuthTokenRevocationChecker,
        AuthTokenRevocationError, AuthTokenVerifier, AuthTokenVerifierError,
        RevocableAuthTokenVerifier, RevocableAuthTokenVerifierError,
    };
    use crate::authorization::AggregateRef;
    use crate::event::{AggregateIdValue, AggregateTypeOwned};
    use crate::unit_of_work::{UnitOfWork, UnitOfWorkError};

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone)]
    struct StubVerifier {
        claims: AuthTokenClaims,
    }

    impl AuthTokenVerifier for StubVerifier {
        async fn verify(
            &self,
            _token: &AuthToken,
        ) -> Result<AuthTokenClaims, AuthTokenVerifierError> {
            Ok(self.claims.clone())
        }
    }

    #[derive(Clone, Copy)]
    enum StubCheckResult {
        Active,
        Revoked,
        BackendError,
    }

    #[derive(Clone, Copy)]
    struct StubChecker {
        result: StubCheckResult,
    }

    impl AuthTokenRevocationChecker for StubChecker {
        type Uow = TestUow;

        async fn is_token_revoked(
            &self,
            _uow: &mut Self::Uow,
            _claims: &AuthTokenClaims,
        ) -> Result<bool, AuthTokenRevocationError> {
            match self.result {
                StubCheckResult::Active => Ok(false),
                StubCheckResult::Revoked => Ok(true),
                StubCheckResult::BackendError => Err(AuthTokenRevocationError::Backend(Box::new(
                    StubCheckerBackendError,
                ))),
            }
        }
    }

    #[derive(Debug, Error)]
    #[error("stub checker backend error")]
    struct StubCheckerBackendError;

    fn claims() -> AuthTokenClaims {
        let issuer_url = "https://example.com"
            .parse::<AuthTokenIssuerUrl>()
            .expect("valid issuer");
        let audience = AuthTokenAudience::new("web".to_owned()).expect("valid audience");
        let audiences = AuthTokenAudiences::new(vec![audience]).expect("valid audiences");
        let subject = AggregateRef {
            aggregate_type: AggregateTypeOwned::new("user".to_owned()).expect("valid type"),
            aggregate_id: AggregateIdValue::from(Uuid::from_u128(1)),
        };

        AuthTokenClaims::new(
            issuer_url,
            audiences,
            subject,
            AuthTokenIssuedAt::now(),
            AuthTokenExpiresAt::now(),
            AuthTokenId::from(Uuid::from_u128(2)),
        )
    }

    #[tokio::test]
    async fn returns_claims_when_token_is_not_revoked() {
        let claims = claims();
        let verifier = DefaultRevocableAuthTokenVerifier::new(
            StubVerifier {
                claims: claims.clone(),
            },
            StubChecker {
                result: StubCheckResult::Active,
            },
        );
        let mut uow = TestUow;

        let verified = verifier
            .verify(&mut uow, &AuthToken::new("token".to_owned()))
            .await
            .expect("token should be active");

        assert_eq!(verified, claims);
    }

    #[tokio::test]
    async fn returns_revoked_when_checker_flags_token() {
        let verifier = DefaultRevocableAuthTokenVerifier::new(
            StubVerifier { claims: claims() },
            StubChecker {
                result: StubCheckResult::Revoked,
            },
        );
        let mut uow = TestUow;

        let error = verifier
            .verify(&mut uow, &AuthToken::new("token".to_owned()))
            .await
            .expect_err("revoked token should fail");

        assert!(matches!(error, RevocableAuthTokenVerifierError::Revoked));
    }

    #[tokio::test]
    async fn returns_backend_error_when_checker_fails() {
        let verifier = DefaultRevocableAuthTokenVerifier::new(
            StubVerifier { claims: claims() },
            StubChecker {
                result: StubCheckResult::BackendError,
            },
        );
        let mut uow = TestUow;

        let error = verifier
            .verify(&mut uow, &AuthToken::new("token".to_owned()))
            .await
            .expect_err("checker failure should bubble up");

        assert!(matches!(error, RevocableAuthTokenVerifierError::Backend(_)));
    }
}

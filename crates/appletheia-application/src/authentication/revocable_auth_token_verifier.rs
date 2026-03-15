use crate::unit_of_work::UnitOfWork;

use super::{AuthToken, AuthTokenClaims, RevocableAuthTokenVerifierError};

/// Verifies auth tokens including revocation policy checks.
#[allow(async_fn_in_trait)]
pub trait RevocableAuthTokenVerifier: Send + Sync {
    type Uow: UnitOfWork;

    /// Verifies the token and rejects revoked tokens.
    async fn verify(
        &self,
        uow: &mut Self::Uow,
        token: &AuthToken,
    ) -> Result<AuthTokenClaims, RevocableAuthTokenVerifierError>;
}

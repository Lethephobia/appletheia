use crate::unit_of_work::UnitOfWork;

use super::{AuthTokenClaims, AuthTokenRevocationError};

/// Checks whether a token is revoked under the current revocation policy.
#[allow(async_fn_in_trait)]
pub trait AuthTokenRevocationChecker: Send + Sync {
    type Uow: UnitOfWork;

    /// Returns `true` when the token represented by `claims` is revoked.
    async fn is_token_revoked(
        &self,
        uow: &mut Self::Uow,
        claims: &AuthTokenClaims,
    ) -> Result<bool, AuthTokenRevocationError>;
}

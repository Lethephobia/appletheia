use crate::authorization::AggregateRef;
use crate::unit_of_work::UnitOfWork;

use super::{AuthTokenExpiresAt, AuthTokenId, AuthTokenIssuedAt, AuthTokenRevocationError};

/// Updates token revocation state within the current unit of work.
#[allow(async_fn_in_trait)]
pub trait AuthTokenRevoker: Send + Sync {
    type Uow: UnitOfWork;

    /// Revokes a single token until its natural expiration time.
    async fn revoke_token(
        &self,
        uow: &mut Self::Uow,
        token_id: AuthTokenId,
        expires_at: AuthTokenExpiresAt,
    ) -> Result<(), AuthTokenRevocationError>;

    /// Advances the subject-wide revocation cutoff if `issued_at` is newer.
    async fn advance_revocation_cutoff(
        &self,
        uow: &mut Self::Uow,
        subject: &AggregateRef,
        issued_at: AuthTokenIssuedAt,
    ) -> Result<(), AuthTokenRevocationError>;
}

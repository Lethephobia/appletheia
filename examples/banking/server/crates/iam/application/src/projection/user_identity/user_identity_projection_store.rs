use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{Email, UserId, UserIdentityProvider, UserIdentitySubject};

use super::{UserIdentityProjectionStoreError, UserIdentityProjectionUpsert};

/// Persists normalized user identity projections for projection-side reads.
#[allow(async_fn_in_trait)]
pub trait UserIdentityProjectionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: UserIdentityProjectionUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), UserIdentityProjectionStoreError>;

    async fn update_email(
        &self,
        uow: &mut Self::Uow,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        event_sequence: EventSequence,
    ) -> Result<(), UserIdentityProjectionStoreError>;

    async fn delete_by_user(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        event_sequence: EventSequence,
    ) -> Result<(), UserIdentityProjectionStoreError>;
}

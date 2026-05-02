use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{Email, UserId, UserIdentityProvider, UserIdentitySubject};

use super::{UserIdentityViewStoreError, UserIdentityViewUpsert};

/// Persists normalized user identity views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait UserIdentityViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: UserIdentityViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), UserIdentityViewStoreError>;

    async fn update_email(
        &self,
        uow: &mut Self::Uow,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
        event_sequence: EventSequence,
    ) -> Result<(), UserIdentityViewStoreError>;

    async fn delete_by_user(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
        event_sequence: EventSequence,
    ) -> Result<(), UserIdentityViewStoreError>;
}

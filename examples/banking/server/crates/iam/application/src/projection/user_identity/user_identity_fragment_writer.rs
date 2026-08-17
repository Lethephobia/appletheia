use appletheia::application::read_model::MaterializationEventContext;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::{UserId, UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;

use super::{
    UserIdentityFragment, UserIdentityFragmentKey, UserIdentityFragmentUpsert,
    UserIdentityFragmentWriterError,
};

/// Persists user identity fragments independently of composed read models.
#[allow(async_fn_in_trait)]
pub trait UserIdentityFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        upsert: UserIdentityFragmentUpsert,
    ) -> Result<Option<UserIdentityFragment>, UserIdentityFragmentWriterError>;

    async fn update_email(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
        provider: UserIdentityProvider,
        subject: UserIdentitySubject,
        email: Option<Email>,
    ) -> Result<Option<UserIdentityFragment>, UserIdentityFragmentWriterError>;

    async fn delete_for_user(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        user_id: UserId,
    ) -> Result<Vec<UserIdentityFragmentKey>, UserIdentityFragmentWriterError>;
}

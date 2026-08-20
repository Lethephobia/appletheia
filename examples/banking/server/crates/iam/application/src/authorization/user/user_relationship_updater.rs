use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::UserId;

use super::UserRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait UserRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
    ) -> Result<(), UserRelationshipUpdaterError>;
}

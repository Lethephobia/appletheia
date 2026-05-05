use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::account::{AccountId, AccountOwner};

use super::AccountRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait AccountRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        account_id: AccountId,
        owner: AccountOwner,
    ) -> Result<(), AccountRelationshipUpdaterError>;

    async fn replace_owner(
        &self,
        uow: &mut Self::Uow,
        account_id: AccountId,
        owner: AccountOwner,
    ) -> Result<(), AccountRelationshipUpdaterError>;
}

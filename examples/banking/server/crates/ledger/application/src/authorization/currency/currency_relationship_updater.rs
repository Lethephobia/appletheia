use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::currency::{CurrencyId, CurrencyOwner};

use super::CurrencyRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait CurrencyRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_owner(
        &self,
        uow: &mut Self::Uow,
        currency_id: CurrencyId,
        owner: CurrencyOwner,
    ) -> Result<(), CurrencyRelationshipUpdaterError>;

    async fn replace_owner(
        &self,
        uow: &mut Self::Uow,
        currency_id: CurrencyId,
        owner: CurrencyOwner,
    ) -> Result<(), CurrencyRelationshipUpdaterError>;
}

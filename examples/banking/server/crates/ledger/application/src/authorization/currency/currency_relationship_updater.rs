use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::currency::CurrencyId;
use banking_ledger_domain::currency_registrar::CurrencyRegistrarId;

use super::CurrencyRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait CurrencyRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_currency_registrar(
        &self,
        uow: &mut Self::Uow,
        currency_id: CurrencyId,
        currency_registrar_id: CurrencyRegistrarId,
    ) -> Result<(), CurrencyRelationshipUpdaterError>;
}

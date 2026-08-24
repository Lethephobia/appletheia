use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::currency::CurrencyId;
use banking_ledger_domain::token_binding::TokenBindingId;

use super::TokenBindingRelationshipUpdaterError;

#[allow(async_fn_in_trait)]
pub trait TokenBindingRelationshipUpdater: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        token_binding_id: TokenBindingId,
        currency_id: CurrencyId,
    ) -> Result<(), TokenBindingRelationshipUpdaterError>;
}

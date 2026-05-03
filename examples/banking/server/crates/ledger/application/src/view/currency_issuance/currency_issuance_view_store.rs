use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::currency_issuance::{CurrencyIssuanceId, CurrencyIssuanceStatus};

use super::{CurrencyIssuanceViewStoreError, CurrencyIssuanceViewUpsert};

/// Persists normalized currency issuance views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait CurrencyIssuanceViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: CurrencyIssuanceViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyIssuanceViewStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        status: CurrencyIssuanceStatus,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyIssuanceViewStoreError>;
}

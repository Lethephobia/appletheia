use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::currency_issuance::{CurrencyIssuanceId, CurrencyIssuanceStatus};

use super::{CurrencyIssuanceProjectionStoreError, CurrencyIssuanceProjectionUpsert};

/// Persists normalized currency issuance projections for projection-side reads.
#[allow(async_fn_in_trait)]
pub trait CurrencyIssuanceProjectionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: CurrencyIssuanceProjectionUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyIssuanceProjectionStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        status: CurrencyIssuanceStatus,
        event_sequence: EventSequence,
    ) -> Result<(), CurrencyIssuanceProjectionStoreError>;
}

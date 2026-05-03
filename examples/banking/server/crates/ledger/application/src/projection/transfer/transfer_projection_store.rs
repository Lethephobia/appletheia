use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::transfer::{TransferId, TransferStatus};

use super::{TransferProjectionStoreError, TransferProjectionUpsert};

/// Persists normalized transfer projections for projection-side reads.
#[allow(async_fn_in_trait)]
pub trait TransferProjectionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: TransferProjectionUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), TransferProjectionStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        status: TransferStatus,
        event_sequence: EventSequence,
    ) -> Result<(), TransferProjectionStoreError>;
}

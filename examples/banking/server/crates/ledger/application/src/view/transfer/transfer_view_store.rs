use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use banking_ledger_domain::transfer::{TransferId, TransferStatus};

use super::{TransferViewStoreError, TransferViewUpsert};

/// Persists normalized transfer views for query-side reads.
#[allow(async_fn_in_trait)]
pub trait TransferViewStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert(
        &self,
        uow: &mut Self::Uow,
        input: TransferViewUpsert,
        event_sequence: EventSequence,
    ) -> Result<(), TransferViewStoreError>;

    async fn update_status(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        status: TransferStatus,
        event_sequence: EventSequence,
    ) -> Result<(), TransferViewStoreError>;
}

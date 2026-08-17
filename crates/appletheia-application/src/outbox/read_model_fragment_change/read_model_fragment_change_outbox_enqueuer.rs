use crate::read_model::ReadModelFragmentChangeEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::ReadModelFragmentChangeOutboxEnqueueError;

/// Enqueues source-fragment changes in the projection transaction.
#[allow(async_fn_in_trait)]
pub trait ReadModelFragmentChangeOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;

    /// Enqueues one or more source-partition batches atomically.
    async fn enqueue_fragment_changes(
        &self,
        uow: &mut Self::Uow,
        fragment_changes: &[ReadModelFragmentChangeEnvelope],
    ) -> Result<(), ReadModelFragmentChangeOutboxEnqueueError>;
}

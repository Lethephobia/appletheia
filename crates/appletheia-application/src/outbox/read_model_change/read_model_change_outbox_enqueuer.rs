use crate::read_model::ReadModelChangeEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::ReadModelChangeOutboxEnqueueError;

/// Enqueues read model changes in the projection transaction.
#[allow(async_fn_in_trait)]
pub trait ReadModelChangeOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;

    async fn enqueue(
        &self,
        uow: &mut Self::Uow,
        change: &ReadModelChangeEnvelope,
    ) -> Result<(), ReadModelChangeOutboxEnqueueError>;
}

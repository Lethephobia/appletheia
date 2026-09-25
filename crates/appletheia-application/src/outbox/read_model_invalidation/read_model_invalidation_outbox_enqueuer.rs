use crate::read_model::ReadModelInvalidationEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::ReadModelInvalidationOutboxEnqueueError;

/// Enqueues one read-model invalidation in the projection transaction.
#[allow(async_fn_in_trait)]
pub trait ReadModelInvalidationOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;

    async fn enqueue_invalidation(
        &self,
        uow: &mut Self::Uow,
        invalidation: &ReadModelInvalidationEnvelope,
    ) -> Result<(), ReadModelInvalidationOutboxEnqueueError>;
}

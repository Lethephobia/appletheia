use crate::read_model::ReadModelInvalidationEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::ReadModelInvalidationOutboxEnqueueError;

/// Enqueues read-model invalidations in the projection transaction.
#[allow(async_fn_in_trait)]
pub trait ReadModelInvalidationOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;

    async fn enqueue_invalidations(
        &self,
        uow: &mut Self::Uow,
        invalidations: &[ReadModelInvalidationEnvelope],
    ) -> Result<(), ReadModelInvalidationOutboxEnqueueError>;
}

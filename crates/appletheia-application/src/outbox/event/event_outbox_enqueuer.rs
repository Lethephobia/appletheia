use crate::event::EventEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::EventOutboxEnqueueError;

/// Enqueues persisted events in the repository transaction.
#[allow(async_fn_in_trait)]
pub trait EventOutboxEnqueuer: Send + Sync {
    type Uow: UnitOfWork;

    async fn enqueue_events(
        &self,
        uow: &mut Self::Uow,
        events: &[EventEnvelope],
    ) -> Result<(), EventOutboxEnqueueError>;
}

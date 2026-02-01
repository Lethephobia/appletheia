use crate::event::{EventEnvelope, EventFeedBatchSize, EventSelector, EventSequence};
use crate::messaging::Subscription;
use crate::unit_of_work::UnitOfWork;

use super::EventFeedReaderError;

#[allow(async_fn_in_trait)]
pub trait EventFeedReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn read_after(
        &self,
        uow: &mut Self::Uow,
        after: Option<EventSequence>,
        limit: EventFeedBatchSize,
        subscription: Subscription<'_, EventSelector>,
    ) -> Result<Vec<EventEnvelope>, EventFeedReaderError>;
}

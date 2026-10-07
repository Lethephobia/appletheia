use appletheia_domain::{Aggregate, AggregateVersionRange, Event};

use super::event_reader_error::EventReaderError;
use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait EventReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn read_events<A: Aggregate>(
        &self,
        uow: &mut Self::Uow,
        aggregate_id: A::Id,
        range: AggregateVersionRange,
    ) -> Result<Vec<Event<A::Id, A::EventPayload>>, EventReaderError>;
}

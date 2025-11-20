use crate::{Aggregate, AggregateVersion, Event};

use super::event_reader_error::EventReaderError;

#[allow(async_fn_in_trait)]
pub trait EventReader<A: Aggregate> {
    async fn read_events(
        &mut self,
        aggregate_id: A::Id,
        after: Option<AggregateVersion>,
        as_of: Option<AggregateVersion>,
    ) -> Result<Vec<Event<A::Id, A::EventPayload>>, EventReaderError>;
}

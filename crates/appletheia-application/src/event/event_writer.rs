use appletheia_domain::{Aggregate, Event};

use crate::event::EventEnvelope;
use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

use super::event_writer_error::EventWriterError;

#[allow(async_fn_in_trait)]
pub trait EventWriter<A: Aggregate>: Send + Sync {
    type Uow: UnitOfWork;

    /// Persists domain events and returns their database-ordered envelopes.
    async fn write_events(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<Vec<EventEnvelope>, EventWriterError>;
}

use appletheia_domain::{Aggregate, Event};

use crate::request_context::RequestContext;
use crate::unit_of_work::UnitOfWork;

use super::event_writer_error::EventWriterError;

#[allow(async_fn_in_trait)]
pub trait EventWriter<A: Aggregate> {
    type Uow: UnitOfWork;

    async fn write_events_and_outbox(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        events: &[Event<A::Id, A::EventPayload>],
    ) -> Result<(), EventWriterError>;
}

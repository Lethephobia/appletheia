use std::error::Error;

use super::{
    projector_event_handler::ProjectorEventHandler,
    typed_projector_event_handler::TypedProjectorEventHandler,
};
use crate::{
    event::{EventEnvelopeError, EventSelector},
    unit_of_work::UnitOfWork,
};
use appletheia_domain::{Aggregate, Event, EventName};

/// Associates an event selector with a typed asynchronous handler.
pub struct ProjectorRoute<'a, U: UnitOfWork, E: Error + Send + Sync + 'static> {
    pub selector: EventSelector,
    pub(crate) handler: Box<dyn ProjectorEventHandler<U, E> + 'a>,
}

impl<'a, U: UnitOfWork + 'a, E: Error + Send + Sync + 'static> ProjectorRoute<'a, U, E> {
    pub fn on_event<A, H>(event_name: EventName, handler: H) -> Self
    where
        A: Aggregate,
        E: From<EventEnvelopeError>,
        H: for<'b> AsyncFn(&'b mut U, &'b Event<A::Id, A::EventPayload>) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        Self {
            selector: EventSelector::new::<A>(event_name),
            handler: Box::new(TypedProjectorEventHandler::<A, H>::new(handler)),
        }
    }
}

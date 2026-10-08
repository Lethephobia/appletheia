use std::{error::Error, marker::PhantomData};

use super::ProjectorRoute;
use crate::{event::EventEnvelopeError, unit_of_work::UnitOfWork};
use appletheia_domain::{Aggregate, Event, EventName};

pub struct ProjectorEventRouteHandlerBuilder<A: Aggregate> {
    event_name: EventName,
    aggregate: PhantomData<fn() -> A>,
}

impl<A: Aggregate> ProjectorEventRouteHandlerBuilder<A> {
    pub(crate) fn new(event_name: EventName) -> Self {
        Self {
            event_name,
            aggregate: PhantomData,
        }
    }

    pub fn handle<'a, U, E, H>(self, handler: H) -> ProjectorRoute<'a, U, E>
    where
        U: UnitOfWork + 'a,
        E: Error + From<EventEnvelopeError> + Send + Sync + 'static,
        H: for<'b> AsyncFn(&'b mut U, &'b Event<A::Id, A::EventPayload>) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        ProjectorRoute::on_event::<A, H>(self.event_name, handler)
    }
}

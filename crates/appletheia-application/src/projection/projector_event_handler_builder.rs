use std::{error::Error, marker::PhantomData};

use super::{ProjectorDefinitionBuilder, ProjectorRoute};
use crate::{event::EventEnvelopeError, unit_of_work::UnitOfWork};
use appletheia_domain::{Aggregate, Event, EventName};

pub struct ProjectorEventHandlerBuilder<
    'a,
    U: UnitOfWork,
    E: Error + Send + Sync + 'static,
    A: Aggregate,
> {
    definition_builder: ProjectorDefinitionBuilder<'a, U, E>,
    event_name: EventName,
    aggregate: PhantomData<fn() -> A>,
}

impl<'a, U: UnitOfWork + 'a, E: Error + Send + Sync + 'static, A: Aggregate>
    ProjectorEventHandlerBuilder<'a, U, E, A>
{
    pub(crate) fn new(
        definition_builder: ProjectorDefinitionBuilder<'a, U, E>,
        event_name: EventName,
    ) -> Self {
        Self {
            definition_builder,
            event_name,
            aggregate: PhantomData,
        }
    }

    pub fn handle<H>(self, handler: H) -> ProjectorDefinitionBuilder<'a, U, E>
    where
        E: From<EventEnvelopeError>,
        H: for<'b> AsyncFn(&'b mut U, &'b Event<A::Id, A::EventPayload>) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        self.definition_builder
            .add_route(ProjectorRoute::on_event::<A, H>(self.event_name, handler))
    }
}

use super::{SagaContext, SagaDefinitionBuilder, SagaName, SagaRoute, SagaState, SagaStep};
use crate::event::EventEnvelopeError;
use appletheia_domain::{Aggregate, Event, EventName};
use std::{error::Error, marker::PhantomData};

/// Registers a typed event callback for a selected start condition.
pub struct SagaStartEventHandlerBuilder<
    'a,
    S: SagaState,
    T: SagaStep,
    E: Error + Send + Sync + 'static,
    A: Aggregate,
> {
    name: SagaName,
    routes: Vec<SagaRoute<'a, S, T, E>>,
    step: T,
    event_name: EventName,
    aggregate: PhantomData<fn() -> A>,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static, A: Aggregate>
    SagaStartEventHandlerBuilder<'a, S, T, E, A>
{
    pub(crate) fn new(
        name: SagaName,
        routes: Vec<SagaRoute<'a, S, T, E>>,
        step: T,
        event_name: EventName,
    ) -> Self {
        Self {
            name,
            routes,
            step,
            event_name,
            aggregate: PhantomData,
        }
    }

    pub fn handle<H>(mut self, handler: H) -> SagaDefinitionBuilder<'a, S, T, E>
    where
        E: From<EventEnvelopeError>,
        H: Fn(&mut SagaContext<'_, S, T>, &Event<A::Id, A::EventPayload>) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        let route = SagaRoute::starts_on::<A, H>(self.event_name, self.step, handler);
        self.routes.push(route);
        SagaDefinitionBuilder {
            name: self.name,
            routes: self.routes,
        }
    }
}

use super::{SagaContext, SagaDefinitionBuilder, SagaRoute, SagaState, SagaStep};
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
    definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
    step: T,
    event_name: EventName,
    aggregate: PhantomData<fn() -> A>,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static, A: Aggregate>
    SagaStartEventHandlerBuilder<'a, S, T, E, A>
{
    pub(crate) fn new(
        definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
        step: T,
        event_name: EventName,
    ) -> Self {
        Self {
            definition_builder,
            step,
            event_name,
            aggregate: PhantomData,
        }
    }

    pub fn handle<H>(self, handler: H) -> SagaDefinitionBuilder<'a, S, T, E>
    where
        H: Fn(&mut SagaContext<'_, S, T>, &Event<A::Id, A::EventPayload>) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        let route = SagaRoute::starts_on::<A, H>(self.event_name, self.step, handler);
        self.definition_builder.add_route(route)
    }
}

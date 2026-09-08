use super::{SagaContext, SagaDefinitionBuilder, SagaRoute, SagaState, SagaStep};
use crate::event::EventSelector;
use appletheia_domain::{Aggregate, Event};
use std::{error::Error, marker::PhantomData};

/// Registers a typed event callback for a selected continuation condition.
pub struct SagaEventHandlerBuilder<
    'a,
    S: SagaState,
    T: SagaStep,
    E: Error + Send + Sync + 'static,
    A: Aggregate,
> {
    definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
    step: T,
    selector: EventSelector,
    caused_by: T,
    aggregate: PhantomData<fn() -> A>,
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static, A: Aggregate>
    SagaEventHandlerBuilder<'a, S, T, E, A>
{
    pub(crate) fn new(
        definition_builder: SagaDefinitionBuilder<'a, S, T, E>,
        step: T,
        selector: EventSelector,
        caused_by: T,
    ) -> Self {
        Self {
            definition_builder,
            step,
            selector,
            caused_by,
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
        let route = SagaRoute::OnEvent {
            step: self.step,
            selector: self.selector,
            caused_by: self.caused_by,
            handler: SagaRoute::event_handler::<A, H>(handler),
        };
        self.definition_builder.add_route(route)
    }
}

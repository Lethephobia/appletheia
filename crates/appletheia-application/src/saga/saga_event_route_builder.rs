use std::{error::Error, marker::PhantomData};

use super::{SagaContext, SagaEventRouteHandlerBuilder, SagaRoute, SagaState, SagaStep};
use crate::event::EventEnvelopeError;
use appletheia_domain::{Aggregate, Event, EventName};

pub struct SagaEventRouteBuilder<T: SagaStep, A: Aggregate> {
    step: T,
    event_name: EventName,
    aggregate: PhantomData<fn() -> A>,
}

impl<T: SagaStep, A: Aggregate> SagaEventRouteBuilder<T, A> {
    pub(crate) fn new(step: T, event_name: EventName) -> Self {
        Self {
            step,
            event_name,
            aggregate: PhantomData,
        }
    }

    pub fn caused_by(self, step: T) -> SagaEventRouteHandlerBuilder<T, A> {
        SagaEventRouteHandlerBuilder::new(self.step, self.event_name, step)
    }

    pub fn handle<'a, S, E, H>(self, handler: H) -> SagaRoute<'a, S, T, E>
    where
        S: SagaState,
        E: Error + From<EventEnvelopeError> + Send + Sync + 'static,
        H: Fn(&mut SagaContext<'_, S, T>, &Event<A::Id, A::EventPayload>) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        SagaRoute::starts_on::<A, H>(self.event_name, self.step, handler)
    }
}

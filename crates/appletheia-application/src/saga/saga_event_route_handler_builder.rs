use std::{error::Error, marker::PhantomData};

use super::{SagaContext, SagaRoute, SagaState, SagaStep};
use crate::event::EventEnvelopeError;
use appletheia_domain::{Aggregate, Event, EventName};

pub struct SagaEventRouteHandlerBuilder<T: SagaStep, A: Aggregate> {
    step: T,
    event_name: EventName,
    caused_by: T,
    aggregate: PhantomData<fn() -> A>,
}

impl<T: SagaStep, A: Aggregate> SagaEventRouteHandlerBuilder<T, A> {
    pub(crate) fn new(step: T, event_name: EventName, caused_by: T) -> Self {
        Self {
            step,
            event_name,
            caused_by,
            aggregate: PhantomData,
        }
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
        SagaRoute::on_event::<A, H>(self.caused_by, self.event_name, self.step, handler)
    }
}

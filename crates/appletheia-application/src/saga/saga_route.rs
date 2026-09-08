use super::{
    SagaContext, SagaEventHandler, SagaFailureHandler, SagaRouteError, SagaState, SagaStep,
};
use crate::{command::CommandFailureEnvelope, event::EventSelector};
use appletheia_domain::{Aggregate, Event, EventName, EventPayload};
use std::error::Error;

/// Declares an input condition, the step for outgoing commands, and a callback.
pub enum SagaRoute<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> {
    StartsOn {
        step: T,
        selector: EventSelector,
        handler: Box<SagaEventHandler<'a, S, T, E>>,
    },
    OnEvent {
        step: T,
        selector: EventSelector,
        caused_by: T,
        handler: Box<SagaEventHandler<'a, S, T, E>>,
    },
    OnCommandFailed {
        step: T,
        caused_by: T,
        handler: Box<SagaFailureHandler<'a, S, T, E>>,
    },
}

impl<'a, S: SagaState, T: SagaStep, E: Error + Send + Sync + 'static> SagaRoute<'a, S, T, E> {
    pub fn starts_on<A, H>(event_name: EventName, dispatch_step: T, handler: H) -> Self
    where
        A: Aggregate,
        H: Fn(&mut SagaContext<'_, S, T>, &Event<A::Id, A::EventPayload>) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        Self::StartsOn {
            step: dispatch_step,
            selector: EventSelector::new::<A>(event_name),
            handler: Self::event_handler::<A, H>(handler),
        }
    }

    pub fn on_event<A, H>(caused_by: T, event_name: EventName, dispatch_step: T, handler: H) -> Self
    where
        A: Aggregate,
        H: Fn(&mut SagaContext<'_, S, T>, &Event<A::Id, A::EventPayload>) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        Self::OnEvent {
            step: dispatch_step,
            selector: EventSelector::new::<A>(event_name),
            caused_by,
            handler: Self::event_handler::<A, H>(handler),
        }
    }

    pub fn on_command_failed<H>(caused_by: T, dispatch_step: T, handler: H) -> Self
    where
        H: Fn(&mut SagaContext<'_, S, T>, &CommandFailureEnvelope) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        Self::OnCommandFailed {
            step: dispatch_step,
            caused_by,
            handler: Box::new(handler),
        }
    }

    pub(crate) fn event_handler<A, H>(handler: H) -> Box<SagaEventHandler<'a, S, T, E>>
    where
        A: Aggregate,
        H: Fn(&mut SagaContext<'_, S, T>, &Event<A::Id, A::EventPayload>) -> Result<(), E>
            + Send
            + Sync
            + 'a,
    {
        Box::new(move |ctx, envelope| {
            let decoded = envelope.try_into_domain_event::<A>()?;
            if decoded.payload().name().value() != envelope.event_name.value() {
                return Err(SagaRouteError::EventNameMismatch);
            }
            handler(ctx, &decoded).map_err(SagaRouteError::Handler)
        })
    }
}

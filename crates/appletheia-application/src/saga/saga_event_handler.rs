use super::{SagaContext, SagaRouteError};
use crate::event::EventEnvelope;

/// Handles an envelope after the route has matched and ownership has been checked.
pub type SagaEventHandler<'a, S, T, E> = dyn Fn(&mut SagaContext<'_, S, T>, &EventEnvelope) -> Result<(), SagaRouteError<E>>
    + Send
    + Sync
    + 'a;

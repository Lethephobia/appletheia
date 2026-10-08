use std::{error::Error, future::Future, marker::PhantomData, pin::Pin};

use super::projector_event_handler::ProjectorEventHandler;
use crate::{
    event::{EventEnvelope, EventEnvelopeError},
    unit_of_work::UnitOfWork,
};
use appletheia_domain::{Aggregate, Event};

pub(super) struct TypedProjectorEventHandler<A: Aggregate, H: Send + Sync> {
    handler: H,
    aggregate: PhantomData<fn() -> A>,
}

impl<A: Aggregate, H: Send + Sync> TypedProjectorEventHandler<A, H> {
    pub(super) fn new(handler: H) -> Self {
        Self {
            handler,
            aggregate: PhantomData,
        }
    }
}

impl<A, H, U, E> ProjectorEventHandler<U, E> for TypedProjectorEventHandler<A, H>
where
    A: Aggregate,
    U: UnitOfWork,
    E: Error + From<EventEnvelopeError> + Send + Sync + 'static,
    H: for<'a> AsyncFn(&'a mut U, &'a Event<A::Id, A::EventPayload>) -> Result<(), E> + Send + Sync,
{
    fn handle<'a>(
        &'a self,
        uow: &'a mut U,
        event: &'a EventEnvelope,
    ) -> Pin<Box<dyn Future<Output = Result<(), E>> + 'a>> {
        Box::pin(async move {
            let decoded = event.try_to_domain_event::<A>()?;
            (self.handler)(uow, &decoded).await
        })
    }
}

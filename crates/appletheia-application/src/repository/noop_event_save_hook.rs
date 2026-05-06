use std::convert::Infallible;
use std::marker::PhantomData;

use appletheia_domain::{Aggregate, Event};

use crate::unit_of_work::UnitOfWork;

use super::EventSaveHook;

#[derive(Clone, Copy, Debug, Default)]
pub struct NoopEventSaveHook<Uow> {
    _marker: PhantomData<fn() -> Uow>,
}

impl<Uow> NoopEventSaveHook<Uow> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<A, Uow> EventSaveHook<A> for NoopEventSaveHook<Uow>
where
    A: Aggregate,
    Uow: UnitOfWork,
{
    type Uow = Uow;
    type Error = Infallible;

    async fn after_event_saved(
        &self,
        _uow: &mut Self::Uow,
        _event: &Event<A::Id, A::EventPayload>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

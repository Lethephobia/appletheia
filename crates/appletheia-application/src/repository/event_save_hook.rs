use std::error::Error;

use appletheia_domain::{Aggregate, Event};

use crate::unit_of_work::UnitOfWork;

#[allow(async_fn_in_trait)]
pub trait EventSaveHook<A: Aggregate>: Send + Sync {
    type Uow: UnitOfWork;
    type Error: Error + Send + Sync + 'static;

    async fn after_event_saved(
        &self,
        uow: &mut Self::Uow,
        event: &Event<A::Id, A::EventPayload>,
    ) -> Result<(), Self::Error>;
}

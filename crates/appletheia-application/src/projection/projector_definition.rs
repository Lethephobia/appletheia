use std::error::Error;

use crate::event::{EventEnvelope, EventSelector};
use crate::messaging::Subscription;
use crate::unit_of_work::UnitOfWork;

use super::ProjectorName;

#[allow(async_fn_in_trait)]
pub trait ProjectorDefinition: Send + Sync {
    type Uow: UnitOfWork;
    type Error: Error + Send + Sync + 'static;

    const NAME: ProjectorName;
    const SUBSCRIPTION: Subscription<'static, EventSelector>;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error>;
}

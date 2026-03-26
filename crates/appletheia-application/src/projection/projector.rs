use std::error::Error;

use crate::event::EventEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::ProjectorSpec;

/// Projects events into an external read model or side effect.
#[allow(async_fn_in_trait)]
pub trait Projector: Send + Sync {
    type Spec: ProjectorSpec;
    type Uow: UnitOfWork;
    type Error: Error + Send + Sync + 'static;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error>;
}

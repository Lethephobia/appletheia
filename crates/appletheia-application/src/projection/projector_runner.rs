use crate::event::EventEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::{ProjectorDefinition, ProjectorRunReport, ProjectorRunnerError};

#[allow(async_fn_in_trait)]
pub trait ProjectorRunner: Send + Sync {
    type Uow: UnitOfWork;

    async fn project<P: ProjectorDefinition<Uow = Self::Uow>>(
        &self,
        projector: &P,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError>;
}

use crate::event::EventEnvelope;
use crate::unit_of_work::UnitOfWork;
use std::error::Error;

use super::{ProjectorDefinition, ProjectorRunReport, ProjectorRunnerError};

#[allow(async_fn_in_trait)]
pub trait ProjectorRunner: Send + Sync {
    type Uow: UnitOfWork;

    async fn project<E>(
        &self,
        projector_definition: &ProjectorDefinition<'_, Self::Uow, E>,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError>
    where
        E: Error + Send + Sync + 'static;
}

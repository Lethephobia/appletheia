use crate::unit_of_work::UnitOfWork;

use super::{ProjectorDefinition, ProjectorRebuildReport, ProjectorRebuilderError};

#[allow(async_fn_in_trait)]
pub trait ProjectorRebuilder: Send {
    type Uow: UnitOfWork;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_until_idle<P: ProjectorDefinition<Uow = Self::Uow>>(
        &mut self,
        projector: &P,
    ) -> Result<ProjectorRebuildReport, ProjectorRebuilderError>;
}

use crate::event::EventEnvelope;
use crate::unit_of_work::UnitOfWork;

use super::{Projector, ProjectorRunReport, ProjectorRunnerError};

#[allow(async_fn_in_trait)]
pub trait ProjectorRunner: Send + Sync {
    type Uow: UnitOfWork;

    async fn project<PJ: Projector<Uow = Self::Uow>>(
        &self,
        projector: &PJ,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError>;
}

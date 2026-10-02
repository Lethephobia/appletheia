use crate::event::EventEnvelope;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    MaterializationEventContext, Projector, ProjectorNameOwned, ProjectorProcessedEventStore,
    ProjectorRunReport, ProjectorRunner, ProjectorRunnerError, ProjectorSpec,
};

/// Persists projection updates and records processed events in one transaction.
pub struct DefaultProjectorRunner<P, U>
where
    P: ProjectorProcessedEventStore,
    U: UnitOfWorkFactory<Uow = P::Uow>,
{
    processed_event_store: P,
    uow_factory: U,
}

impl<P, U> DefaultProjectorRunner<P, U>
where
    P: ProjectorProcessedEventStore,
    U: UnitOfWorkFactory<Uow = P::Uow>,
{
    pub fn new(processed_event_store: P, uow_factory: U) -> Self {
        Self {
            processed_event_store,
            uow_factory,
        }
    }

    async fn project_inner<PJ>(
        &self,
        uow: &mut P::Uow,
        projector: &PJ,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError>
    where
        PJ: Projector<Uow = P::Uow>,
        P: ProjectorProcessedEventStore,
    {
        let descriptor = <PJ::Spec as ProjectorSpec>::DESCRIPTOR;
        let inserted = self
            .processed_event_store
            .mark_processed(
                uow,
                ProjectorNameOwned::from(descriptor.name),
                event.event_id,
            )
            .await?;

        if !inserted {
            return Ok(ProjectorRunReport::SkippedAlreadyProcessed);
        }

        projector
            .project(uow, MaterializationEventContext::from(event), event)
            .await
            .map_err(|source| ProjectorRunnerError::Projection(Box::new(source)))?;

        Ok(ProjectorRunReport::Applied)
    }
}

impl<P, U> ProjectorRunner for DefaultProjectorRunner<P, U>
where
    P: ProjectorProcessedEventStore,
    U: UnitOfWorkFactory<Uow = P::Uow>,
{
    type Uow = P::Uow;

    async fn project<PJ: Projector<Uow = P::Uow>>(
        &self,
        projector: &PJ,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError> {
        let mut uow = self.uow_factory.begin().await?;
        match self.project_inner(&mut uow, projector, event).await {
            Ok(report) => {
                uow.commit().await?;
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}

use crate::event::EventEnvelope;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

use super::{
    ProjectorDefinition, ProjectorNameOwned, ProjectorProcessedEventStore, ProjectorRunReport,
    ProjectorRunner, ProjectorRunnerError,
};

pub struct DefaultProjectorRunner<P, U> {
    processed_event_store: P,
    uow_factory: U,
}

impl<P, U> DefaultProjectorRunner<P, U> {
    pub fn new(processed_event_store: P, uow_factory: U) -> Self {
        Self {
            processed_event_store,
            uow_factory,
        }
    }

    async fn project_inner<D: ProjectorDefinition<Uow = P::Uow>>(
        &self,
        uow: &mut P::Uow,
        projector: &D,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError>
    where
        P: ProjectorProcessedEventStore,
    {
        let projector_name = ProjectorNameOwned::from(D::NAME);
        let event_id = event.event_id;

        let inserted = self
            .processed_event_store
            .mark_processed(uow, projector_name, event_id)
            .await?;

        if !inserted {
            return Ok(ProjectorRunReport::SkippedAlreadyProcessed);
        }

        projector
            .project(uow, event)
            .await
            .map_err(|source| ProjectorRunnerError::Definition(Box::new(source)))?;

        Ok(ProjectorRunReport::Applied)
    }
}

impl<P, U> ProjectorRunner for DefaultProjectorRunner<P, U>
where
    P: ProjectorProcessedEventStore,
    U: UnitOfWorkFactory<Uow = P::Uow>,
{
    type Uow = P::Uow;

    async fn project<D: ProjectorDefinition<Uow = P::Uow>>(
        &self,
        projector: &D,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError> {
        let mut uow = self.uow_factory.begin().await?;

        let result = self.project_inner(&mut uow, projector, event).await;
        match result {
            Ok(report) => {
                uow.commit().await?;
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}

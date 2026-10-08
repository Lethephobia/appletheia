use std::error::Error;

use crate::event::EventEnvelope;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    ProjectorDefinition, ProjectorNameOwned, ProjectorProcessedEventStore, ProjectorRunReport,
    ProjectorRunner, ProjectorRunnerError,
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

    async fn project_inner<E>(
        &self,
        uow: &mut P::Uow,
        projector_definition: &ProjectorDefinition<'_, P::Uow, E>,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError>
    where
        E: Error + Send + Sync + 'static,
        P: ProjectorProcessedEventStore,
    {
        let Some(route) = projector_definition.find_event_route(event) else {
            return Ok(ProjectorRunReport::SkippedNotSubscribed);
        };
        let inserted = self
            .processed_event_store
            .mark_processed(
                uow,
                ProjectorNameOwned::from(projector_definition.name()),
                event.event_id,
            )
            .await?;

        if !inserted {
            return Ok(ProjectorRunReport::SkippedAlreadyProcessed);
        }

        route
            .handler
            .handle(uow, event)
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

    async fn project<E>(
        &self,
        projector_definition: &ProjectorDefinition<'_, P::Uow, E>,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError>
    where
        E: Error + Send + Sync + 'static,
    {
        let mut uow = self.uow_factory.begin().await?;
        match self
            .project_inner(&mut uow, projector_definition, event)
            .await
        {
            Ok(report) => {
                uow.commit().await?;
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}

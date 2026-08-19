use crate::event::EventEnvelope;
use crate::outbox::read_model_invalidation::ReadModelInvalidationOutboxEnqueuer;
use crate::read_model::{
    MaterializationEventContext, ReadModelDependency, ReadModelDependencyTopic,
    ReadModelInvalidationEnvelope,
};
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    Projector, ProjectorNameOwned, ProjectorProcessedEventStore, ProjectorRunReport,
    ProjectorRunner, ProjectorRunnerError, ProjectorSpec,
};

/// Persists fragment updates and emits payload-free read-model invalidations.
pub struct DefaultProjectorRunner<P, E, U> {
    processed_event_store: P,
    invalidation_outbox_enqueuer: E,
    uow_factory: U,
}

impl<P, E, U> DefaultProjectorRunner<P, E, U> {
    pub fn new(processed_event_store: P, invalidation_outbox_enqueuer: E, uow_factory: U) -> Self {
        Self {
            processed_event_store,
            invalidation_outbox_enqueuer,
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
        E: ReadModelInvalidationOutboxEnqueuer<Uow = P::Uow>,
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

        let invalidated_partitions = projector
            .project(uow, MaterializationEventContext::from(event), event)
            .await
            .map_err(|source| ProjectorRunnerError::Definition(Box::new(source)))?;

        let mut invalidated_dependencies = invalidated_partitions
            .into_iter()
            .map(|partition| {
                partition
                    .try_into_serialized::<PJ::Fragment>()
                    .map(ReadModelDependency::Partition)
                    .map_err(|source| ProjectorRunnerError::Definition(Box::new(source)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if !invalidated_dependencies.is_empty() {
            invalidated_dependencies.push(ReadModelDependency::Topic(
                ReadModelDependencyTopic::all::<PJ::Fragment>(),
            ));
            let invalidation = ReadModelInvalidationEnvelope::try_new(
                event,
                descriptor.name,
                invalidated_dependencies,
            )?;
            self.invalidation_outbox_enqueuer
                .enqueue_invalidations(uow, std::slice::from_ref(&invalidation))
                .await?;
        }

        Ok(ProjectorRunReport::Applied)
    }
}

impl<P, E, U> ProjectorRunner for DefaultProjectorRunner<P, E, U>
where
    P: ProjectorProcessedEventStore,
    E: ReadModelInvalidationOutboxEnqueuer<Uow = P::Uow>,
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

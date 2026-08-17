use crate::event::EventEnvelope;
use crate::outbox::read_model_fragment_change::ReadModelFragmentChangeOutboxEnqueuer;
use crate::read_model::MaterializationEventContext;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::read_model_fragment_change_batches::ReadModelFragmentChangeBatches;
use super::{
    Projector, ProjectorNameOwned, ProjectorProcessedEventStore, ProjectorRunReport,
    ProjectorRunner, ProjectorRunnerError, ProjectorSpec,
};

/// Persists fragment changes without coupling projection to any read model tree.
pub struct DefaultProjectorRunner<P, E, U> {
    processed_event_store: P,
    fragment_change_outbox_enqueuer: E,
    uow_factory: U,
}

impl<P, E, U> DefaultProjectorRunner<P, E, U> {
    pub fn new(
        processed_event_store: P,
        fragment_change_outbox_enqueuer: E,
        uow_factory: U,
    ) -> Self {
        Self {
            processed_event_store,
            fragment_change_outbox_enqueuer,
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
        E: ReadModelFragmentChangeOutboxEnqueuer<Uow = P::Uow>,
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

        let fragment_changes = projector
            .project(uow, MaterializationEventContext::from(event), event)
            .await
            .map_err(|source| ProjectorRunnerError::Definition(Box::new(source)))?;

        let batches = ReadModelFragmentChangeBatches::from_changes(fragment_changes)
            .map_err(|source| ProjectorRunnerError::Definition(Box::new(source)))?;
        let fragment_change_envelopes = batches.try_into_envelopes(event, descriptor.name)?;
        if !fragment_change_envelopes.is_empty() {
            self.fragment_change_outbox_enqueuer
                .enqueue_fragment_changes(uow, &fragment_change_envelopes)
                .await?;
        }

        Ok(ProjectorRunReport::Applied)
    }
}

impl<P, E, U> ProjectorRunner for DefaultProjectorRunner<P, E, U>
where
    P: ProjectorProcessedEventStore,
    E: ReadModelFragmentChangeOutboxEnqueuer<Uow = P::Uow>,
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

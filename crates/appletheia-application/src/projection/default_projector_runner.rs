use crate::event::EventEnvelope;
use crate::messaging::Subscription;
use crate::outbox::read_model_change::ReadModelChangeOutboxEnqueuer;
use crate::read_model::ReadModelChangeEnvelope;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    Projector, ProjectorNameOwned, ProjectorProcessedEventStore, ProjectorRunReport,
    ProjectorRunner, ProjectorRunnerError,
};

/// Persists projection updates and records processed events in one transaction.
pub struct DefaultProjectorRunner<P, E, U>
where
    P: ProjectorProcessedEventStore,
    E: ReadModelChangeOutboxEnqueuer<Uow = P::Uow>,
    U: UnitOfWorkFactory<Uow = P::Uow>,
{
    processed_event_store: P,
    read_model_change_outbox_enqueuer: E,
    uow_factory: U,
}

impl<P, E, U> DefaultProjectorRunner<P, E, U>
where
    P: ProjectorProcessedEventStore,
    E: ReadModelChangeOutboxEnqueuer<Uow = P::Uow>,
    U: UnitOfWorkFactory<Uow = P::Uow>,
{
    pub fn new(
        processed_event_store: P,
        read_model_change_outbox_enqueuer: E,
        uow_factory: U,
    ) -> Self {
        Self {
            processed_event_store,
            read_model_change_outbox_enqueuer,
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
        if !Subscription::AnyOf(PJ::EVENT_SELECTORS).matches(event) {
            return Ok(ProjectorRunReport::SkippedNotSubscribed);
        }
        let inserted = self
            .processed_event_store
            .mark_processed(uow, ProjectorNameOwned::from(PJ::NAME), event.event_id)
            .await?;

        if !inserted {
            return Ok(ProjectorRunReport::SkippedAlreadyProcessed);
        }

        let changes = projector
            .project(uow, event)
            .await
            .map_err(|source| ProjectorRunnerError::Projection(Box::new(source)))?;

        for change in changes {
            let envelope = ReadModelChangeEnvelope::new(event.event_id, change);
            self.read_model_change_outbox_enqueuer
                .enqueue(uow, &envelope)
                .await?;
        }

        Ok(ProjectorRunReport::Applied)
    }
}

impl<P, E, U> ProjectorRunner for DefaultProjectorRunner<P, E, U>
where
    P: ProjectorProcessedEventStore,
    E: ReadModelChangeOutboxEnqueuer<Uow = P::Uow>,
    U: UnitOfWorkFactory<Uow = P::Uow>,
{
    type Uow = P::Uow;

    async fn project<PJ>(
        &self,
        projector: &PJ,
        event: &EventEnvelope,
    ) -> Result<ProjectorRunReport, ProjectorRunnerError>
    where
        PJ: Projector<Uow = P::Uow>,
    {
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

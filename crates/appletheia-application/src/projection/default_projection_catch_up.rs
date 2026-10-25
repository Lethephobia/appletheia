use tokio_util::sync::CancellationToken;

use crate::event::EventFeedReader;
use crate::messaging::Subscription;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    ProcessedEventCount, ProjectionCatchUp, ProjectionCatchUpConfig, ProjectionCatchUpError,
    ProjectionCatchUpReport, ProjectionCheckpointStore, Projector, ProjectorNameOwned,
    ProjectorProcessedEventStore,
};

/// Catches a projection up to the event feed while persisting its checkpoint.
pub struct DefaultProjectionCatchUp<F, C, P, U>
where
    F: EventFeedReader,
    C: ProjectionCheckpointStore<Uow = F::Uow>,
    P: ProjectorProcessedEventStore<Uow = F::Uow>,
    U: UnitOfWorkFactory<Uow = F::Uow>,
{
    feed_reader: F,
    checkpoint_store: C,
    processed_event_store: P,
    uow_factory: U,
    config: ProjectionCatchUpConfig,
    stop_token: CancellationToken,
}

impl<F, C, P, U> DefaultProjectionCatchUp<F, C, P, U>
where
    F: EventFeedReader,
    C: ProjectionCheckpointStore<Uow = F::Uow>,
    P: ProjectorProcessedEventStore<Uow = F::Uow>,
    U: UnitOfWorkFactory<Uow = F::Uow>,
{
    pub fn new(
        feed_reader: F,
        checkpoint_store: C,
        processed_event_store: P,
        uow_factory: U,
        config: ProjectionCatchUpConfig,
    ) -> Self {
        Self {
            feed_reader,
            checkpoint_store,
            processed_event_store,
            uow_factory,
            config,
            stop_token: CancellationToken::new(),
        }
    }
}

impl<F, C, P, U> ProjectionCatchUp for DefaultProjectionCatchUp<F, C, P, U>
where
    F: EventFeedReader,
    C: ProjectionCheckpointStore<Uow = F::Uow>,
    P: ProjectorProcessedEventStore<Uow = F::Uow>,
    U: UnitOfWorkFactory<Uow = F::Uow>,
{
    type Uow = F::Uow;

    fn is_stop_requested(&self) -> bool {
        self.stop_token.is_cancelled()
    }

    fn request_graceful_stop(&self) {
        self.stop_token.cancel();
    }

    async fn run_until_idle<PJ: Projector<Uow = F::Uow>>(
        &mut self,
        projector: &PJ,
    ) -> Result<ProjectionCatchUpReport, ProjectionCatchUpError> {
        let selectors = PJ::EVENT_SELECTORS;
        if selectors.is_empty() {
            return Ok(ProjectionCatchUpReport {
                processed_event_count: ProcessedEventCount::zero(),
            });
        }
        let subscription = Subscription::AnyOf(selectors);
        let projector_name = ProjectorNameOwned::from(PJ::NAME);
        let mut processed_event_count = ProcessedEventCount::zero();

        while !self.is_stop_requested() {
            let events = {
                let mut uow = self.uow_factory.begin().await?;
                let after = match self
                    .checkpoint_store
                    .load(&mut uow, projector_name.clone())
                    .await
                {
                    Ok(after) => after,
                    Err(source) => {
                        return Err(uow
                            .rollback_with_operation_error(ProjectionCatchUpError::from(source))
                            .await?);
                    }
                };
                let events = match self
                    .feed_reader
                    .read_after(&mut uow, after, self.config.batch_size, subscription)
                    .await
                {
                    Ok(events) => events,
                    Err(source) => {
                        return Err(uow
                            .rollback_with_operation_error(ProjectionCatchUpError::from(source))
                            .await?);
                    }
                };
                uow.commit().await?;
                events
            };

            if events.is_empty() {
                break;
            }

            for event in events {
                if self.is_stop_requested() {
                    break;
                }

                if !subscription.matches(&event) {
                    continue;
                }
                let mut uow = self.uow_factory.begin().await?;
                let inserted = match self
                    .processed_event_store
                    .mark_processed(&mut uow, projector_name.clone(), event.event_id)
                    .await
                {
                    Ok(inserted) => inserted,
                    Err(source) => {
                        return Err(uow
                            .rollback_with_operation_error(ProjectionCatchUpError::from(source))
                            .await?);
                    }
                };

                if let Err(source) = self
                    .checkpoint_store
                    .save(&mut uow, projector_name.clone(), event.event_sequence)
                    .await
                {
                    return Err(uow
                        .rollback_with_operation_error(ProjectionCatchUpError::from(source))
                        .await?);
                }

                if !inserted {
                    uow.commit().await?;
                    processed_event_count = processed_event_count.saturating_add(1);
                    continue;
                }

                if let Err(source) = projector.project(&mut uow, &event).await {
                    return Err(uow
                        .rollback_with_operation_error(ProjectionCatchUpError::Projection(
                            Box::new(source),
                        ))
                        .await?);
                }

                uow.commit().await?;
                processed_event_count = processed_event_count.saturating_add(1);
            }
        }

        Ok(ProjectionCatchUpReport {
            processed_event_count,
        })
    }
}

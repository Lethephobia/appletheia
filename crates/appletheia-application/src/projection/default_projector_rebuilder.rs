use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::event::EventFeedReader;
use crate::messaging::Subscription;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    ProcessedEventCount, ProjectionCheckpointStore, Projector, ProjectorNameOwned,
    ProjectorProcessedEventStore, ProjectorRebuildReport, ProjectorRebuilder,
    ProjectorRebuilderConfig, ProjectorRebuilderError,
};

/// Replays events into a projector while persisting its checkpoint.
pub struct DefaultProjectorRebuilder<F, C, P, U>
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
    config: ProjectorRebuilderConfig,
    stop_requested: AtomicBool,
}

impl<F, C, P, U> DefaultProjectorRebuilder<F, C, P, U>
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
        config: ProjectorRebuilderConfig,
    ) -> Self {
        Self {
            feed_reader,
            checkpoint_store,
            processed_event_store,
            uow_factory,
            config,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<F, C, P, U> ProjectorRebuilder for DefaultProjectorRebuilder<F, C, P, U>
where
    F: EventFeedReader,
    C: ProjectionCheckpointStore<Uow = F::Uow>,
    P: ProjectorProcessedEventStore<Uow = F::Uow>,
    U: UnitOfWorkFactory<Uow = F::Uow>,
{
    type Uow = F::Uow;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_until_idle<PJ: Projector<Uow = F::Uow>>(
        &mut self,
        projector: &PJ,
    ) -> Result<ProjectorRebuildReport, ProjectorRebuilderError> {
        let projector_definition = projector.definition()?;
        let selectors = projector_definition.event_selectors();
        if selectors.is_empty() {
            return Ok(ProjectorRebuildReport {
                processed_event_count: ProcessedEventCount::zero(),
            });
        }
        let subscription = Subscription::AnyOf(&selectors);
        let projector_name = ProjectorNameOwned::from(projector_definition.name());
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
                        let error = ProjectorRebuilderError::from(source);
                        return Err(uow.rollback_with_operation_error(error).await?);
                    }
                };
                let events = match self
                    .feed_reader
                    .read_after(&mut uow, after, self.config.batch_size, subscription)
                    .await
                {
                    Ok(events) => events,
                    Err(source) => {
                        let error = ProjectorRebuilderError::from(source);
                        return Err(uow.rollback_with_operation_error(error).await?);
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

                let Some(route) = projector_definition.find_event_route(&event) else {
                    continue;
                };
                let mut uow = self.uow_factory.begin().await?;
                let inserted = match self
                    .processed_event_store
                    .mark_processed(&mut uow, projector_name.clone(), event.event_id)
                    .await
                {
                    Ok(inserted) => inserted,
                    Err(source) => {
                        let error = ProjectorRebuilderError::from(source);
                        return Err(uow.rollback_with_operation_error(error).await?);
                    }
                };

                if let Err(source) = self
                    .checkpoint_store
                    .save(&mut uow, projector_name.clone(), event.event_sequence)
                    .await
                {
                    let error = ProjectorRebuilderError::from(source);
                    return Err(uow.rollback_with_operation_error(error).await?);
                }

                if !inserted {
                    uow.commit().await?;
                    processed_event_count = processed_event_count.saturating_add(1);
                    continue;
                }

                if let Err(source) = route.handler.handle(&mut uow, &event).await {
                    let error = ProjectorRebuilderError::Projection(Box::new(source));
                    return Err(uow.rollback_with_operation_error(error).await?);
                }

                uow.commit().await?;
                processed_event_count = processed_event_count.saturating_add(1);
            }
        }

        Ok(ProjectorRebuildReport {
            processed_event_count,
        })
    }
}

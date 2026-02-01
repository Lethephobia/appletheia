use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::event::EventFeedReader;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

use super::ProcessedEventCount;
use super::{
    ProjectionCheckpointStore, ProjectorDefinition, ProjectorNameOwned,
    ProjectorProcessedEventStore, ProjectorRebuildReport, ProjectorRebuilder,
    ProjectorRebuilderConfig, ProjectorRebuilderError,
};

pub struct DefaultProjectorRebuilder<F, C, P, U> {
    feed_reader: F,
    checkpoint_store: C,
    processed_event_store: P,
    uow_factory: U,
    config: ProjectorRebuilderConfig,
    stop_requested: AtomicBool,
}

impl<F, C, P, U> DefaultProjectorRebuilder<F, C, P, U> {
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

    async fn run_until_idle<D: ProjectorDefinition<Uow = F::Uow>>(
        &mut self,
        projector: &D,
    ) -> Result<ProjectorRebuildReport, ProjectorRebuilderError> {
        let projector_name = ProjectorNameOwned::from(D::NAME);

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
                    .read_after(&mut uow, after, self.config.batch_size, D::SUBSCRIPTION)
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

                if inserted {
                    if let Err(source) = projector.project(&mut uow, &event).await {
                        let error = ProjectorRebuilderError::Definition(Box::new(source));
                        return Err(uow.rollback_with_operation_error(error).await?);
                    }
                }

                if let Err(source) = self
                    .checkpoint_store
                    .save(&mut uow, projector_name.clone(), event.event_sequence)
                    .await
                {
                    let error = ProjectorRebuilderError::from(source);
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

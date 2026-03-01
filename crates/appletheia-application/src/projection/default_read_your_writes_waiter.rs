use std::time::Duration as StdDuration;

use tokio::time::Instant;

use crate::event::EventSequenceLookup;
use crate::request_context::{CausationId, MessageId};
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    ProjectorNameOwned, ProjectorProcessedEventStore, ReadYourWritesPollInterval,
    ReadYourWritesTimeout, ReadYourWritesWaitError, ReadYourWritesWaiter,
};

#[derive(Debug)]
pub struct DefaultReadYourWritesWaiter<U, L, P>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    L: EventSequenceLookup<Uow = U::Uow>,
    P: ProjectorProcessedEventStore<Uow = U::Uow>,
{
    uow_factory: U,
    lookup: L,
    processed_event_store: P,
}

impl<U, L, P> DefaultReadYourWritesWaiter<U, L, P>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    L: EventSequenceLookup<Uow = U::Uow>,
    P: ProjectorProcessedEventStore<Uow = U::Uow>,
{
    pub fn new(uow_factory: U, lookup: L, processed_event_store: P) -> Self {
        Self {
            uow_factory,
            lookup,
            processed_event_store,
        }
    }
}

impl<U, L, P> ReadYourWritesWaiter for DefaultReadYourWritesWaiter<U, L, P>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    L: EventSequenceLookup<Uow = U::Uow>,
    P: ProjectorProcessedEventStore<Uow = U::Uow>,
{
    async fn wait(
        &self,
        after: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
        projector_names: &[ProjectorNameOwned],
    ) -> Result<(), ReadYourWritesWaitError> {
        if projector_names.is_empty() {
            return Ok(());
        }

        let target_event_id = {
            let mut uow = self.uow_factory.begin().await?;
            let causation_id = CausationId::from(after);
            let event_id = self
                .lookup
                .last_event_id_by_causation_id(&mut uow, causation_id)
                .await;
            let event_id = match event_id {
                Ok(value) => value,
                Err(operation_error) => {
                    let operation_error =
                        uow.rollback_with_operation_error(operation_error).await?;
                    return Err(operation_error.into());
                }
            };
            uow.commit().await?;
            event_id.ok_or(ReadYourWritesWaitError::UnknownMessageId { message_id: after })?
        };

        let deadline = Instant::now() + timeout.value();
        let poll_duration = poll_interval.value();

        loop {
            let mut pending: Vec<ProjectorNameOwned> = Vec::new();

            for projector_name in projector_names {
                let processed = {
                    let mut uow = self.uow_factory.begin().await?;
                    let processed = self
                        .processed_event_store
                        .is_processed(&mut uow, projector_name.clone(), target_event_id)
                        .await;
                    let processed = match processed {
                        Ok(value) => value,
                        Err(operation_error) => {
                            let operation_error =
                                uow.rollback_with_operation_error(operation_error).await?;
                            return Err(operation_error.into());
                        }
                    };
                    uow.commit().await?;
                    processed
                };

                if processed {
                    continue;
                }

                pending.push(projector_name.clone());
            }

            if pending.is_empty() {
                return Ok(());
            }

            let now = Instant::now();
            if now >= deadline {
                return Err(ReadYourWritesWaitError::Timeout {
                    target_event_id,
                    pending,
                    timeout,
                });
            }

            let remaining = deadline
                .checked_duration_since(now)
                .unwrap_or(StdDuration::ZERO);
            let sleep_duration = poll_duration.min(remaining);

            if sleep_duration > StdDuration::ZERO {
                tokio::time::sleep(sleep_duration).await;
            } else {
                tokio::task::yield_now().await;
            }
        }
    }
}

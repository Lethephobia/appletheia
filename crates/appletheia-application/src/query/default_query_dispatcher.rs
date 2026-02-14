use std::time::Duration as StdDuration;

use tokio::time::Instant;

use crate::event::{EventSequence, EventSequenceLookup};
use crate::projection::{ProjectionCheckpointStore, ProjectorNameOwned};
use crate::request_context::{CausationId, MessageId, RequestContext};
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    QueryConsistency, QueryDispatchError, QueryDispatcher, QueryHandler, QueryOptions, ReadModel,
    ReadYourWritesPollInterval, ReadYourWritesTimeout,
};

pub struct DefaultQueryDispatcher<L, C, U>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    L: EventSequenceLookup<Uow = U::Uow>,
    C: ProjectionCheckpointStore<Uow = U::Uow>,
{
    lookup: L,
    checkpoint_store: C,
    uow_factory: U,
}

impl<L, C, U> DefaultQueryDispatcher<L, C, U>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    L: EventSequenceLookup<Uow = U::Uow>,
    C: ProjectionCheckpointStore<Uow = U::Uow>,
{
    pub fn new(lookup: L, checkpoint_store: C, uow_factory: U) -> Self {
        Self {
            lookup,
            checkpoint_store,
            uow_factory,
        }
    }

    async fn wait_for_read_your_writes<R, HE>(
        &self,
        after: MessageId,
        timeout: ReadYourWritesTimeout,
        poll_interval: ReadYourWritesPollInterval,
    ) -> Result<(), QueryDispatchError<HE>>
    where
        R: ReadModel,
        HE: std::error::Error + Send + Sync + 'static,
    {
        let projector_name = ProjectorNameOwned::from(R::PROJECTOR);

        let target = {
            let mut uow = self.uow_factory.begin().await?;
            let causation_id = CausationId::from(after);
            let seq = self
                .lookup
                .max_event_sequence_by_causation_id(&mut uow, causation_id)
                .await;
            let seq = match seq {
                Ok(value) => value,
                Err(operation_error) => {
                    let operation_error =
                        uow.rollback_with_operation_error(operation_error).await?;
                    return Err(operation_error.into());
                }
            };
            uow.commit().await?;
            seq.ok_or(QueryDispatchError::UnknownMessageId { message_id: after })?
        };

        let deadline = Instant::now() + timeout.value();
        let poll_duration = poll_interval.value();

        let mut last_checkpoint: Option<EventSequence>;

        loop {
            let checkpoint = {
                let mut uow = self.uow_factory.begin().await?;
                let checkpoint = self
                    .checkpoint_store
                    .load(&mut uow, projector_name.clone())
                    .await;
                let checkpoint = match checkpoint {
                    Ok(value) => value,
                    Err(operation_error) => {
                        let operation_error =
                            uow.rollback_with_operation_error(operation_error).await?;
                        return Err(operation_error.into());
                    }
                };
                uow.commit().await?;
                checkpoint
            };

            last_checkpoint = checkpoint;

            if checkpoint.is_some_and(|seq| seq >= target) {
                return Ok(());
            }

            if Instant::now() >= deadline {
                return Err(QueryDispatchError::Timeout {
                    projector_name,
                    target,
                    last_checkpoint,
                    timeout,
                });
            }

            if poll_duration > StdDuration::ZERO {
                tokio::time::sleep(poll_duration).await;
            } else {
                tokio::task::yield_now().await;
            }
        }
    }
}

impl<L, C, U> QueryDispatcher for DefaultQueryDispatcher<L, C, U>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    L: EventSequenceLookup<Uow = U::Uow>,
    C: ProjectionCheckpointStore<Uow = U::Uow>,
{
    type Uow = U::Uow;

    async fn dispatch<H>(
        &self,
        handler: &H,
        request_context: &RequestContext,
        query: H::Query,
        options: QueryOptions,
    ) -> Result<H::Output, QueryDispatchError<H::Error>>
    where
        H: QueryHandler<Uow = Self::Uow>,
    {
        match options.consistency {
            QueryConsistency::Eventual => {}
            QueryConsistency::ReadYourWrites {
                after,
                timeout,
                poll_interval,
            } => {
                self.wait_for_read_your_writes::<H::ReadModel, H::Error>(
                    after,
                    timeout,
                    poll_interval,
                )
                .await?;
            }
        }

        let mut uow = self.uow_factory.begin().await?;

        let result = handler.handle(&mut uow, request_context, query).await;
        match result {
            Ok(output) => {
                uow.commit().await?;
                Ok(output)
            }
            Err(operation_error) => {
                let operation_error = uow
                    .rollback_with_operation_error(operation_error)
                    .await
                    .map_err(QueryDispatchError::UnitOfWork)?;
                Err(QueryDispatchError::Handler(operation_error))
            }
        }
    }
}

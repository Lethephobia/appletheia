use crate::event::EventEnvelope;
use crate::outbox::command::CommandOutboxEnqueuer;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

use super::SagaInstance;
use super::{
    EnqueuedCommandCount, SagaDefinition, SagaNameOwned, SagaProcessedEventStore, SagaRunReport,
    SagaRunner, SagaRunnerError, SagaStatus, SagaStore,
};

pub struct DefaultSagaRunner<S, P, Q, U> {
    saga_store: S,
    processed_event_store: P,
    command_outbox_enqueuer: Q,
    uow_factory: U,
}

impl<S, P, Q, U> DefaultSagaRunner<S, P, Q, U> {
    pub fn new(
        saga_store: S,
        processed_event_store: P,
        command_outbox_enqueuer: Q,
        uow_factory: U,
    ) -> Self {
        Self {
            saga_store,
            processed_event_store,
            command_outbox_enqueuer,
            uow_factory,
        }
    }
}

impl<S, P, Q, U> DefaultSagaRunner<S, P, Q, U>
where
    S: SagaStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event_inner<D: SagaDefinition>(
        &self,
        uow: &mut S::Uow,
        saga: &D,
        event: &EventEnvelope,
    ) -> Result<(SagaInstance<D::State>, SagaRunReport), SagaRunnerError> {
        let saga_name = SagaNameOwned::from(D::NAME);
        let correlation_id = event.correlation_id;

        let mut instance = self
            .saga_store
            .load::<D::State>(uow, saga_name.clone(), correlation_id)
            .await?;

        if instance.is_terminal() {
            let report = if instance.is_succeeded() {
                SagaRunReport::SkippedSucceeded
            } else {
                SagaRunReport::SkippedFailed
            };
            return Ok((instance, report));
        }

        let inserted = self
            .processed_event_store
            .mark_processed(uow, saga_name.clone(), correlation_id, event.event_id)
            .await?;
        if !inserted {
            return Ok((instance, SagaRunReport::AlreadyProcessed));
        }

        if instance.is_terminal() {
            let report = if instance.is_succeeded() {
                SagaRunReport::SkippedSucceeded
            } else {
                SagaRunReport::SkippedFailed
            };
            return Ok((instance, report));
        }

        saga.on_event(&mut instance, event)
            .map_err(|source| SagaRunnerError::Definition(Box::new(source)))?;

        if instance.is_terminal() && instance.state.is_none() {
            return Err(SagaRunnerError::TerminalOutcomeRequiresState);
        }

        self.saga_store.save(uow, &instance).await?;

        let commands = instance.uncommitted_commands().to_vec();
        if commands.is_empty() {
            let report = match &instance.status {
                SagaStatus::InProgress => SagaRunReport::InProgress {
                    enqueued_command_count: EnqueuedCommandCount::zero(),
                },
                SagaStatus::Succeeded => SagaRunReport::Succeeded,
                SagaStatus::Failed => SagaRunReport::Failed,
            };
            return Ok((instance, report));
        }

        self.command_outbox_enqueuer
            .enqueue_commands(uow, &commands)
            .await?;

        let enqueued_command_count = EnqueuedCommandCount::from_usize_saturating(commands.len());
        let report = match &instance.status {
            SagaStatus::InProgress => SagaRunReport::InProgress {
                enqueued_command_count,
            },
            SagaStatus::Succeeded => SagaRunReport::Succeeded,
            SagaStatus::Failed => SagaRunReport::Failed,
        };

        Ok((instance, report))
    }
}

impl<S, P, Q, U> SagaRunner for DefaultSagaRunner<S, P, Q, U>
where
    S: SagaStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event<D: SagaDefinition>(
        &self,
        saga: &D,
        event: &EventEnvelope,
    ) -> Result<SagaRunReport, SagaRunnerError> {
        let mut uow = self.uow_factory.begin().await?;

        let result = self.handle_event_inner(&mut uow, saga, event).await;
        match result {
            Ok((mut instance, report)) => {
                uow.commit().await?;
                instance.clear_uncommitted_commands();
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}

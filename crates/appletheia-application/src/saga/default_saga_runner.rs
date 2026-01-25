use crate::event::EventEnvelope;
use crate::outbox::OrderingKey;
use crate::outbox::command::CommandOutboxEnqueuer;
use crate::unit_of_work::UnitOfWork;

use super::{
    SagaDefinition, SagaNameOwned, SagaProcessedEventStore, SagaRunReport, SagaRunner,
    SagaRunnerError, SagaStatus, SagaStore,
};
use super::SagaInstance;

pub struct DefaultSagaRunner<S, P, Q> {
    saga_store: S,
    processed_event_store: P,
    command_outbox_enqueuer: Q,
}

impl<S, P, Q> DefaultSagaRunner<S, P, Q> {
    pub fn new(saga_store: S, processed_event_store: P, command_outbox_enqueuer: Q) -> Self {
        Self {
            saga_store,
            processed_event_store,
            command_outbox_enqueuer,
        }
    }
}

impl<S, P, Q> DefaultSagaRunner<S, P, Q>
where
    S: SagaStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
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
                    enqueued_command_count: 0,
                },
                SagaStatus::Succeeded => SagaRunReport::Succeeded,
                SagaStatus::Failed => SagaRunReport::Failed,
            };
            return Ok((instance, report));
        }

        let ordering_key = OrderingKey::new(correlation_id.to_string())?;

        self.command_outbox_enqueuer
            .enqueue_commands(uow, &ordering_key, &commands)
            .await?;

        let enqueued_command_count = commands.len().min(u32::MAX as usize) as u32;
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

impl<S, P, Q> SagaRunner for DefaultSagaRunner<S, P, Q>
where
    S: SagaStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
{
    type Uow = S::Uow;

    async fn handle_event<D: SagaDefinition>(
        &self,
        uow: &mut Self::Uow,
        saga: &D,
        event: &EventEnvelope,
    ) -> Result<SagaRunReport, SagaRunnerError> {
        uow.begin().await?;

        let result = self.handle_event_inner(uow, saga, event).await;
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

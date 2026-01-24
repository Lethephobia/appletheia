use crate::event::EventEnvelope;
use crate::outbox::OrderingKey;
use crate::outbox::command::CommandOutboxEnqueuer;
use crate::unit_of_work::UnitOfWork;

use super::{
    SagaDefinition, SagaNameOwned, SagaProcessedEventStore, SagaRunReport, SagaRunner,
    SagaRunnerError, SagaStatus, SagaStore,
};

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
    ) -> Result<SagaRunReport, SagaRunnerError> {
        let saga_name = SagaNameOwned::from(D::NAME);
        let correlation_id = event.correlation_id;

        let mut instance = self
            .saga_store
            .load::<D::State>(uow, saga_name.clone(), correlation_id)
            .await?;

        if instance.is_terminal() {
            return Ok(if instance.is_succeeded() {
                SagaRunReport::SkippedSucceeded
            } else {
                SagaRunReport::SkippedFailed
            });
        }

        let inserted = self
            .processed_event_store
            .mark_processed(uow, saga_name.clone(), correlation_id, event.event_id)
            .await?;
        if !inserted {
            return Ok(SagaRunReport::AlreadyProcessed);
        }

        if instance.is_terminal() {
            return Ok(if instance.is_succeeded() {
                SagaRunReport::SkippedSucceeded
            } else {
                SagaRunReport::SkippedFailed
            });
        }

        saga.on_event(&mut instance, event)
            .map_err(|source| SagaRunnerError::Definition(Box::new(source)))?;

        if instance.is_terminal() && instance.state.is_none() {
            return Err(SagaRunnerError::TerminalOutcomeRequiresState);
        }

        self.saga_store.save(uow, &instance).await?;

        let command_envelopes = instance.uncommitted_commands().to_vec();
        if command_envelopes.is_empty() {
            return Ok(match &instance.status {
                SagaStatus::InProgress => SagaRunReport::InProgress {
                    commands_enqueued: 0,
                },
                SagaStatus::Succeeded => SagaRunReport::Succeeded,
                SagaStatus::Failed => SagaRunReport::Failed,
            });
        }

        let ordering_key = OrderingKey::new(correlation_id.to_string())?;

        self.command_outbox_enqueuer
            .enqueue_commands(uow, &ordering_key, &command_envelopes)
            .await?;

        let commands_enqueued = command_envelopes.len();
        instance.clear_uncommitted_commands();
        Ok(match &instance.status {
            SagaStatus::InProgress => SagaRunReport::InProgress { commands_enqueued },
            SagaStatus::Succeeded => SagaRunReport::Succeeded,
            SagaStatus::Failed => SagaRunReport::Failed,
        })
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
            Ok(report) => {
                uow.commit().await?;
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}

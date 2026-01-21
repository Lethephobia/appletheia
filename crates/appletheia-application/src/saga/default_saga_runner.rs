use crate::event::AppEvent;
use crate::outbox::OrderingKey;
use crate::outbox::command::{CommandEnvelope, CommandOutboxEnqueuer};
use crate::request_context::{CausationId, MessageId};

use super::{
    SagaDefinition, SagaInstance, SagaOutcome, SagaProcessedEventStore, SagaRunReport, SagaRunner,
    SagaRunnerError, SagaStatus, SagaStore,
};

pub struct DefaultSagaRunner<S, P, Q> {
    store: S,
    processed_events: P,
    command_outbox: Q,
}

impl<S, P, Q> DefaultSagaRunner<S, P, Q> {
    pub fn new(store: S, processed_events: P, command_outbox: Q) -> Self {
        Self {
            store,
            processed_events,
            command_outbox,
        }
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
        event: &AppEvent,
    ) -> Result<SagaRunReport, SagaRunnerError> {
        let saga_name = D::NAME;
        let correlation_id = event.correlation_id;

        let mut instance: SagaInstance<D::State> = self
            .store
            .load::<D::State>(uow, saga_name, correlation_id)
            .await?;

        if instance.is_terminal() {
            return Ok(if instance.is_succeeded() {
                SagaRunReport::SkippedSucceeded
            } else {
                SagaRunReport::SkippedFailed
            });
        }

        let inserted = self
            .processed_events
            .mark_processed(uow, saga_name, correlation_id, event.event_id)
            .await?;
        if !inserted {
            return Ok(SagaRunReport::AlreadyProcessed);
        }

        let outcome = match &mut instance.status {
            SagaStatus::InProgress { state } => {
                let outcome = saga.on_event(state, event);
                match &outcome {
                    SagaOutcome::Succeeded { .. } => {
                        let state_value = state
                            .take()
                            .ok_or(SagaRunnerError::TerminalOutcomeRequiresState)?;
                        instance.status = SagaStatus::Succeeded { state: state_value };
                    }
                    SagaOutcome::Failed { error, .. } => {
                        let state_value = state
                            .take()
                            .ok_or(SagaRunnerError::TerminalOutcomeRequiresState)?;
                        instance.status = SagaStatus::Failed {
                            state: state_value,
                            error: error.clone(),
                        };
                    }
                    SagaOutcome::InProgress { .. } => {}
                }

                outcome
            }
            SagaStatus::Succeeded { .. } => return Ok(SagaRunReport::SkippedSucceeded),
            SagaStatus::Failed { .. } => return Ok(SagaRunReport::SkippedFailed),
        };

        self.store.save(uow, &instance).await?;

        let ordering_key = OrderingKey::new(correlation_id.to_string())?;
        let causation_id = CausationId::from(event.event_id);

        let commands = outcome.into_commands();
        let command_envelopes: Vec<CommandEnvelope> = commands
            .into_iter()
            .map(|command| CommandEnvelope {
                command_name: command.command_name,
                payload: command.payload,
                correlation_id,
                message_id: MessageId::new(),
                causation_id,
            })
            .collect();

        self.command_outbox
            .enqueue_commands(uow, &ordering_key, &command_envelopes)
            .await?;

        let commands_enqueued = command_envelopes.len();
        Ok(match instance.status {
            SagaStatus::InProgress { .. } => SagaRunReport::InProgress { commands_enqueued },
            SagaStatus::Succeeded { .. } => SagaRunReport::Succeeded { commands_enqueued },
            SagaStatus::Failed { .. } => SagaRunReport::Failed { commands_enqueued },
        })
    }
}

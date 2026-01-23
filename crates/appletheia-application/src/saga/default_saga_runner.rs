use std::marker::PhantomData;

use appletheia_domain::AggregateType;

use crate::command::CommandName;
use crate::event::AppEvent;
use crate::outbox::OrderingKey;
use crate::outbox::command::{CommandEnvelope, CommandOutboxEnqueuer};
use crate::request_context::{CausationId, MessageId};

use super::{
    SagaDefinition, SagaOutcome, SagaProcessedEventStore, SagaRunReport, SagaRunner,
    SagaRunnerError, SagaStatus, SagaStore,
};

pub struct DefaultSagaRunner<AT, CN, S, P, Q> {
    saga_store: S,
    processed_event_store: P,
    command_outbox_enqueuer: Q,
    _marker: PhantomData<(AT, CN)>,
}

impl<AT, CN, S, P, Q> DefaultSagaRunner<AT, CN, S, P, Q> {
    pub fn new(saga_store: S, processed_event_store: P, command_outbox_enqueuer: Q) -> Self {
        Self {
            saga_store,
            processed_event_store,
            command_outbox_enqueuer,
            _marker: PhantomData,
        }
    }
}

impl<AT, CN, S, P, Q> SagaRunner for DefaultSagaRunner<AT, CN, S, P, Q>
where
    S: SagaStore,
    P: SagaProcessedEventStore<Uow = S::Uow, SagaName = S::SagaName>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow, CommandName = CN>,
    AT: AggregateType,
    CN: CommandName,
{
    type Uow = S::Uow;
    type SagaName = S::SagaName;
    type AggregateType = AT;
    type CommandName = CN;

    async fn handle_event<
        D: SagaDefinition<SagaName = Self::SagaName, AggregateType = AT, CommandName = CN>,
    >(
        &self,
        uow: &mut Self::Uow,
        saga: &D,
        event: &AppEvent<AT>,
    ) -> Result<SagaRunReport, SagaRunnerError> {
        let saga_name = D::NAME;
        let correlation_id = event.correlation_id;

        let mut instance = self
            .saga_store
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
            .processed_event_store
            .mark_processed(uow, saga_name, correlation_id, event.event_id)
            .await?;
        if !inserted {
            return Ok(SagaRunReport::AlreadyProcessed);
        }

        let outcome = match &mut instance.status {
            SagaStatus::InProgress { state } => {
                let outcome = saga.on_event(state, event);
                match &outcome {
                    SagaOutcome::Succeeded => {
                        let state_value = state
                            .take()
                            .ok_or(SagaRunnerError::TerminalOutcomeRequiresState)?;
                        instance.status = SagaStatus::Succeeded { state: state_value };
                    }
                    SagaOutcome::Failed { error } => {
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

        self.saga_store.save(uow, &instance).await?;

        let commands = outcome.into_commands();
        if commands.is_empty() {
            return Ok(match &instance.status {
                SagaStatus::InProgress { .. } => SagaRunReport::InProgress {
                    commands_enqueued: 0,
                },
                SagaStatus::Succeeded { .. } => SagaRunReport::Succeeded {
                    commands_enqueued: 0,
                },
                SagaStatus::Failed { .. } => SagaRunReport::Failed {
                    commands_enqueued: 0,
                },
            });
        }

        let ordering_key = OrderingKey::new(correlation_id.to_string())?;
        let causation_id = CausationId::from(event.event_id);
        let command_envelopes = commands
            .into_iter()
            .map(|command| CommandEnvelope {
                command_name: command.command_name,
                payload: command.payload,
                correlation_id,
                message_id: MessageId::new(),
                causation_id,
            })
            .collect::<Vec<_>>();

        self.command_outbox_enqueuer
            .enqueue_commands(uow, &ordering_key, &command_envelopes)
            .await?;

        let commands_enqueued = command_envelopes.len();
        Ok(match &instance.status {
            SagaStatus::InProgress { .. } => SagaRunReport::InProgress { commands_enqueued },
            SagaStatus::Succeeded { .. } => SagaRunReport::Succeeded { commands_enqueued },
            SagaStatus::Failed { .. } => SagaRunReport::Failed { commands_enqueued },
        })
    }
}

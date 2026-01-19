use chrono::Utc;

use crate::event::AppEvent;
use crate::outbox::OrderingKey;
use crate::outbox::command::{CommandEnvelope, CommandOutboxEnqueuer};
use crate::request_context::{CausationId, MessageId};

use super::{SagaCompletion, SagaDefinition, SagaRunReport, SagaRunStatus, SagaRunnerError};
use super::{SagaOutcome, SagaStore};

pub struct SagaRunner<S, Q> {
    store: S,
    command_outbox: Q,
}

impl<S, Q> SagaRunner<S, Q> {
    pub fn new(store: S, command_outbox: Q) -> Self {
        Self {
            store,
            command_outbox,
        }
    }
}

impl<S, Q> SagaRunner<S, Q>
where
    S: SagaStore,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
{
    pub async fn handle_event<D: SagaDefinition>(
        &self,
        uow: &mut S::Uow,
        saga: &D,
        event: &AppEvent,
    ) -> Result<SagaRunReport, SagaRunnerError> {
        let saga_name = D::NAME;
        let correlation_id = event.correlation_id;

        let instance = match self
            .store
            .load_for_update(uow, saga_name, correlation_id)
            .await?
        {
            Some(instance) => instance,
            None => {
                let state = saga.initial_state(event);
                let state_json =
                    serde_json::to_value(&state).map_err(SagaRunnerError::StateSerialize)?;
                self.store
                    .insert_instance_if_absent(uow, saga_name, correlation_id, state_json)
                    .await?;
                self.store
                    .load_for_update(uow, saga_name, correlation_id)
                    .await?
                    .ok_or_else(|| {
                        SagaRunnerError::Store(super::SagaStoreError::Persistence(Box::new(
                            std::io::Error::other("failed to load saga instance after insert"),
                        )))
                    })?
            }
        };

        if instance.is_terminal() {
            return Ok(SagaRunReport {
                status: SagaRunStatus::SkippedTerminal,
                commands_enqueued: 0,
                completed: instance.is_completed(),
                failed: instance.is_failed(),
            });
        }

        let inserted = self
            .store
            .mark_event_processed(uow, saga_name, correlation_id, event.event_id)
            .await?;
        if !inserted {
            return Ok(SagaRunReport {
                status: SagaRunStatus::AlreadyProcessed,
                commands_enqueued: 0,
                completed: false,
                failed: false,
            });
        }

        let mut state: D::State = serde_json::from_value(instance.state.clone())
            .map_err(SagaRunnerError::StateDeserialize)?;

        let SagaOutcome {
            commands,
            completion,
        } = saga.on_event(&mut state, event);

        let state_json = serde_json::to_value(&state).map_err(SagaRunnerError::StateSerialize)?;

        let (completed_at, failed_at, last_error) = match completion {
            SagaCompletion::InProgress => (None, None, None),
            SagaCompletion::Completed => (Some(Utc::now()), None, None),
            SagaCompletion::Failed { error } => (None, Some(Utc::now()), Some(error)),
        };

        self.store
            .update_instance(
                uow,
                saga_name,
                correlation_id,
                super::SagaInstanceUpdate {
                    state: state_json,
                    completed_at,
                    failed_at,
                    last_error,
                },
            )
            .await?;

        let ordering_key = OrderingKey::new(correlation_id.to_string())?;
        let causation_id = CausationId::from(event.event_id);

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

        Ok(SagaRunReport {
            status: SagaRunStatus::Applied,
            commands_enqueued: command_envelopes.len(),
            completed: completed_at.is_some(),
            failed: failed_at.is_some(),
        })
    }
}

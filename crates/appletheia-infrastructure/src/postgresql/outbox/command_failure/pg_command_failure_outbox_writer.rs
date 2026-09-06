use appletheia_application::command::CommandTerminalReason;
use appletheia_application::messaging::PublishDispatchError;
use appletheia_application::outbox::command_failure::CommandFailureOutbox;
use appletheia_application::outbox::{OutboxLifecycle, OutboxWriter, OutboxWriterError};
use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder};

use crate::postgresql::unit_of_work::PgUnitOfWork;

/// Persists terminal command-failure relay lifecycle state.
#[derive(Clone, Copy, Debug, Default)]
pub struct PgCommandFailureOutboxWriter;

impl PgCommandFailureOutboxWriter {
    pub fn new() -> Self {
        Self
    }
}

impl OutboxWriter for PgCommandFailureOutboxWriter {
    type Uow = PgUnitOfWork;
    type Outbox = CommandFailureOutbox;

    async fn write_outbox(
        &self,
        uow: &mut Self::Uow,
        outboxes: &[Self::Outbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO command_failure_outbox (
              id, failure_sequence, failure_id, command_message_id, command_name,
              saga_name, saga_instance_id, saga_step, terminal_reason,
              command_attempt_count, correlation_id, causation_id, failed_at,
              published_at, attempt_count, next_attempt_after, lease_owner,
              lease_until, last_error, dead_lettered_at
            )
            "#,
        );

        let mut prepared = Vec::with_capacity(outboxes.len());
        for outbox in outboxes {
            let last_error = outbox
                .last_error
                .as_ref()
                .map(|error: &PublishDispatchError| serde_json::to_value(error))
                .transpose()
                .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
            let dead_lettered_at = match outbox.lifecycle {
                OutboxLifecycle::Active => None,
                OutboxLifecycle::DeadLettered { dead_lettered_at } => {
                    Some(DateTime::<Utc>::from(dead_lettered_at))
                }
            };
            prepared.push((outbox, last_error, dead_lettered_at));
        }

        query_builder.push_values(
            prepared,
            |mut separated, (outbox, last_error, dead_lettered_at)| {
                let failure = &outbox.failure;
                let terminal_reason = match failure.terminal_reason {
                    CommandTerminalReason::NonRetryable => "non_retryable",
                    CommandTerminalReason::RetryExhausted => "retry_exhausted",
                };
                separated
                    .push_bind(outbox.id.value())
                    .push_bind(outbox.sequence)
                    .push_bind(failure.failure_id.value())
                    .push_bind(failure.command_message_id.value())
                    .push_bind(failure.command_name.value())
                    .push_bind(failure.origin.saga_name.value())
                    .push_bind(failure.origin.saga_instance_id.value())
                    .push_bind(failure.origin.step.value().clone())
                    .push_bind(terminal_reason)
                    .push_bind(i64::from(failure.attempt_count.value()))
                    .push_bind(failure.correlation_id.value())
                    .push_bind(failure.causation_id.value())
                    .push_bind(DateTime::<Utc>::from(failure.failed_at))
                    .push_bind(outbox.state.published_at().map(DateTime::<Utc>::from))
                    .push_bind(outbox.state.attempt_count().value())
                    .push_bind(
                        outbox
                            .state
                            .next_attempt_after()
                            .unwrap_or_default()
                            .value(),
                    )
                    .push_bind(outbox.state.lease_owner().map(ToString::to_string))
                    .push_bind(outbox.state.lease_until().map(DateTime::<Utc>::from))
                    .push_bind(last_error)
                    .push_bind(dead_lettered_at);
            },
        );
        query_builder.push(
            r#"
            ON CONFLICT (id) DO UPDATE SET
              published_at = EXCLUDED.published_at,
              attempt_count = EXCLUDED.attempt_count,
              next_attempt_after = EXCLUDED.next_attempt_after,
              lease_owner = EXCLUDED.lease_owner,
              lease_until = EXCLUDED.lease_until,
              last_error = EXCLUDED.last_error,
              dead_lettered_at = EXCLUDED.dead_lettered_at
            "#,
        );
        query_builder
            .build()
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
        Ok(())
    }
}

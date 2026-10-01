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

    fn serialize_last_error(
        outbox: &CommandFailureOutbox,
    ) -> Result<Option<serde_json::Value>, OutboxWriterError> {
        outbox
            .last_error
            .as_ref()
            .map(|error: &PublishDispatchError| serde_json::to_value(error))
            .transpose()
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))
    }

    fn terminal_reason(outbox: &CommandFailureOutbox) -> &'static str {
        match outbox.failure.terminal_reason {
            CommandTerminalReason::NonRetryable => "non_retryable",
            CommandTerminalReason::RetryExhausted => "retry_exhausted",
        }
    }

    async fn upsert_outbox_rows(
        uow: &mut PgUnitOfWork,
        outboxes: &[&CommandFailureOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut prepared = Vec::with_capacity(outboxes.len());
        for outbox in outboxes {
            prepared.push((*outbox, Self::serialize_last_error(outbox)?));
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO command_failure_outbox (
              id, failure_sequence, failure_id, command_message_id, command_name, command,
              saga_name, saga_instance_id, saga_step, terminal_reason,
              command_attempt_count, correlation_id, causation_id, failed_at,
              published_at, attempt_count, next_attempt_after, lease_owner,
              lease_until, last_error
            )
            "#,
        );
        query_builder.push_values(prepared, |mut separated, (outbox, last_error)| {
            let failure = &outbox.failure;
            separated
                .push_bind(outbox.id.value())
                .push_bind(outbox.sequence)
                .push_bind(failure.failure_id.value())
                .push_bind(failure.command_message_id.value())
                .push_bind(failure.command_name.value())
                .push_bind(failure.command.value())
                .push_bind(failure.origin.saga_name.value())
                .push_bind(failure.origin.saga_instance_id.value())
                .push_bind(failure.origin.step.value().clone())
                .push_bind(Self::terminal_reason(outbox))
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
                .push_bind(last_error);
        });
        query_builder.push(
            r#"
            ON CONFLICT (id) DO UPDATE SET
              published_at = EXCLUDED.published_at,
              attempt_count = EXCLUDED.attempt_count,
              next_attempt_after = EXCLUDED.next_attempt_after,
              lease_owner = EXCLUDED.lease_owner,
              lease_until = EXCLUDED.lease_until,
              last_error = EXCLUDED.last_error
            "#,
        );
        query_builder
            .build()
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
        Ok(())
    }

    async fn insert_dead_letters(
        uow: &mut PgUnitOfWork,
        outboxes: &[&CommandFailureOutbox],
    ) -> Result<(), OutboxWriterError> {
        let mut prepared = Vec::with_capacity(outboxes.len());
        for outbox in outboxes {
            let dead_lettered_at = match outbox.lifecycle {
                OutboxLifecycle::DeadLettered { dead_lettered_at } => {
                    DateTime::<Utc>::from(dead_lettered_at)
                }
                OutboxLifecycle::Active => Utc::now(),
            };
            prepared.push((
                *outbox,
                Self::serialize_last_error(outbox)?,
                dead_lettered_at,
            ));
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO command_failure_dead_letters (
              command_failure_outbox_id, failure_sequence, failure_id,
              command_message_id, command_name, command, saga_name, saga_instance_id,
              saga_step, terminal_reason, command_attempt_count, correlation_id,
              causation_id, failed_at, published_at, attempt_count,
              next_attempt_after, lease_owner, lease_until, last_error,
              dead_lettered_at
            )
            "#,
        );
        query_builder.push_values(
            prepared,
            |mut separated, (outbox, last_error, dead_lettered_at)| {
                let failure = &outbox.failure;
                separated
                    .push_bind(outbox.id.value())
                    .push_bind(outbox.sequence)
                    .push_bind(failure.failure_id.value())
                    .push_bind(failure.command_message_id.value())
                    .push_bind(failure.command_name.value())
                    .push_bind(failure.command.value())
                    .push_bind(failure.origin.saga_name.value())
                    .push_bind(failure.origin.saga_instance_id.value())
                    .push_bind(failure.origin.step.value().clone())
                    .push_bind(Self::terminal_reason(outbox))
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
        query_builder
            .build()
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
        Ok(())
    }

    async fn delete_outboxes(
        uow: &mut PgUnitOfWork,
        outboxes: &[&CommandFailureOutbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder =
            QueryBuilder::<Postgres>::new("DELETE FROM command_failure_outbox WHERE id IN (");
        {
            let mut separated = query_builder.separated(", ");
            for outbox in outboxes {
                separated.push_bind(outbox.id.value());
            }
        }
        query_builder.push(")");
        query_builder
            .build()
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
        Ok(())
    }

    async fn delete_dead_letters(
        uow: &mut PgUnitOfWork,
        outboxes: &[&CommandFailureOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            "DELETE FROM command_failure_dead_letters WHERE command_failure_outbox_id IN (",
        );
        {
            let mut separated = query_builder.separated(", ");
            for outbox in outboxes {
                separated.push_bind(outbox.id.value());
            }
        }
        query_builder.push(")");
        query_builder
            .build()
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
        Ok(())
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

        let mut active_outboxes = Vec::new();
        let mut dead_lettered_outboxes = Vec::new();
        for outbox in outboxes {
            if matches!(outbox.lifecycle, OutboxLifecycle::DeadLettered { .. }) {
                dead_lettered_outboxes.push(outbox);
            } else {
                active_outboxes.push(outbox);
            }
        }

        Self::upsert_outbox_rows(uow, &active_outboxes).await?;
        Self::delete_dead_letters(uow, &active_outboxes).await?;

        if !dead_lettered_outboxes.is_empty() {
            Self::insert_dead_letters(uow, &dead_lettered_outboxes).await?;
            Self::delete_outboxes(uow, &dead_lettered_outboxes).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postgresql::outbox::command_failure::{
        PgCommandFailureOutboxEnqueuer, PgCommandFailureOutboxFetcher,
    };
    use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;
    use appletheia_application::command::{
        CommandAttemptCount, CommandFailureEnvelope, CommandFailureId, SerializedCommand,
    };
    use appletheia_application::messaging::PublishDispatchError;
    use appletheia_application::outbox::command_failure::CommandFailureOutboxEnqueuer;
    use appletheia_application::outbox::{
        Outbox, OutboxBatchSize, OutboxFetcher, OutboxMaxAttempts, OutboxRetryDelay,
        OutboxRetryOptions,
    };
    use appletheia_application::request_context::{CausationId, CorrelationId, MessageId};
    use appletheia_application::saga::{
        SagaCommandOrigin, SagaInstanceId, SagaName, SagaNameOwned, SerializedSagaStep,
    };
    use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkFactory};
    use sqlx::PgPool;
    use std::num::NonZeroU32;
    use uuid::Uuid;

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn failed_command_survives_enqueue_dead_letter_and_redrive(pool: PgPool) {
        let message_id = MessageId::new();
        let failure = CommandFailureEnvelope {
            failure_id: CommandFailureId::new(),
            command_message_id: message_id,
            command_name: "close".parse().unwrap(),
            command: SerializedCommand::new(serde_json::json!({"account_id": 42})).unwrap(),
            origin: SagaCommandOrigin {
                saga_name: SagaNameOwned::from(SagaName::new("closure")),
                saga_instance_id: SagaInstanceId::new(),
                step: SerializedSagaStep::try_from(serde_json::json!("close")).unwrap(),
            },
            terminal_reason: CommandTerminalReason::NonRetryable,
            attempt_count: CommandAttemptCount::first(),
            correlation_id: CorrelationId::from(Uuid::now_v7()),
            causation_id: CausationId::from(message_id),
            // PostgreSQL stores timestamps at microsecond precision.
            failed_at: DateTime::from_timestamp(1_700_000_000, 0).unwrap().into(),
        };
        let factory = PgUnitOfWorkFactory::new(pool);
        let mut uow = factory.begin().await.unwrap();
        PgCommandFailureOutboxEnqueuer::new()
            .enqueue_command_failure(&mut uow, &failure)
            .await
            .unwrap();
        uow.commit().await.unwrap();

        let limit = OutboxBatchSize::new(NonZeroU32::new(10).unwrap());
        let fetcher = PgCommandFailureOutboxFetcher::new();
        let writer = PgCommandFailureOutboxWriter::new();
        let mut uow = factory.begin().await.unwrap();
        let mut pending = fetcher.fetch_pending(&mut uow, limit).await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].failure, failure);
        pending[0]
            .nack(
                &PublishDispatchError::Permanent {
                    code: "invalid".to_owned(),
                    message: "delivery failed".to_owned(),
                },
                &OutboxRetryOptions {
                    backoff: OutboxRetryDelay::new(chrono::Duration::seconds(1)),
                    max_attempts: OutboxMaxAttempts::default(),
                },
            )
            .unwrap();
        writer.write_outbox(&mut uow, &pending).await.unwrap();
        uow.commit().await.unwrap();

        let mut uow = factory.begin().await.unwrap();
        assert!(
            fetcher
                .fetch_pending(&mut uow, limit)
                .await
                .unwrap()
                .is_empty()
        );
        let mut dead_letters = fetcher.fetch_dead_lettered(&mut uow, limit).await.unwrap();
        assert_eq!(dead_letters.len(), 1);
        assert_eq!(dead_letters[0].failure, failure);
        dead_letters[0].redrive().unwrap();
        writer.write_outbox(&mut uow, &dead_letters).await.unwrap();
        uow.commit().await.unwrap();

        let mut uow = factory.begin().await.unwrap();
        let redriven = fetcher.fetch_pending(&mut uow, limit).await.unwrap();
        assert_eq!(redriven.len(), 1);
        assert_eq!(redriven[0].failure, failure);
        assert!(
            fetcher
                .fetch_dead_lettered(&mut uow, limit)
                .await
                .unwrap()
                .is_empty()
        );
        uow.commit().await.unwrap();
    }
}

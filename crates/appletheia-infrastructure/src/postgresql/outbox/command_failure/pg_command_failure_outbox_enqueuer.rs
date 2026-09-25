use appletheia_application::command::{CommandFailureEnvelope, CommandTerminalReason};
use appletheia_application::outbox::command_failure::{
    CommandFailureOutboxEnqueueError, CommandFailureOutboxEnqueuer,
};
use chrono::{DateTime, Utc};

use crate::postgresql::unit_of_work::PgUnitOfWork;

/// Enqueues terminal command-failure notifications in PostgreSQL.
#[derive(Clone, Copy, Debug, Default)]
pub struct PgCommandFailureOutboxEnqueuer;

impl PgCommandFailureOutboxEnqueuer {
    pub fn new() -> Self {
        Self
    }
}

impl CommandFailureOutboxEnqueuer for PgCommandFailureOutboxEnqueuer {
    type Uow = PgUnitOfWork;

    async fn enqueue_command_failure(
        &self,
        uow: &mut Self::Uow,
        failure: &CommandFailureEnvelope,
    ) -> Result<(), CommandFailureOutboxEnqueueError> {
        let terminal_reason = match failure.terminal_reason {
            CommandTerminalReason::NonRetryable => "non_retryable",
            CommandTerminalReason::RetryExhausted => "retry_exhausted",
        };
        let transaction = uow.transaction_mut();
        sqlx::query(
            r#"
            INSERT INTO command_failure_outbox (
              id,
              failure_id,
              command_message_id,
              command_name,
              saga_name,
              saga_instance_id,
              saga_step,
              terminal_reason,
              command_attempt_count,
              correlation_id,
              causation_id,
              failed_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (command_message_id) DO NOTHING
            "#,
        )
        .bind(uuid::Uuid::now_v7())
        .bind(failure.failure_id.value())
        .bind(failure.command_message_id.value())
        .bind(failure.command_name.value())
        .bind(failure.origin.saga_name.value())
        .bind(failure.origin.saga_instance_id.value())
        .bind(failure.origin.step.value().clone())
        .bind(terminal_reason)
        .bind(i64::from(failure.attempt_count.value()))
        .bind(failure.correlation_id.value())
        .bind(failure.causation_id.value())
        .bind(DateTime::<Utc>::from(failure.failed_at))
        .execute(transaction.as_mut())
        .await
        .map_err(|source| CommandFailureOutboxEnqueueError::Persistence(Box::new(source)))?;

        Ok(())
    }
}

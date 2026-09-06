use appletheia_application::command::{
    CommandAttemptCount, CommandEnvelope, CommandExecutionFailureMarkResult, CommandExecutionId,
    CommandExecutionLeaseAcquisitionResult, CommandExecutionLeaseDuration,
    CommandExecutionLeaseReleaseResult, CommandExecutionStore, CommandExecutionStoreError,
    CommandFailedAt,
};
use appletheia_application::request_context::MessageId;
use chrono::{DateTime, Utc};

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::pg_command_execution_row::PgCommandExecutionRow;

/// Persists command execution fencing and terminality in PostgreSQL.
#[derive(Clone, Copy, Debug, Default)]
pub struct PgCommandExecutionStore;

impl PgCommandExecutionStore {
    pub fn new() -> Self {
        Self
    }
}

impl CommandExecutionStore for PgCommandExecutionStore {
    type Uow = PgUnitOfWork;

    async fn acquire_lease(
        &self,
        uow: &mut Self::Uow,
        command: &CommandEnvelope,
        lease_duration: CommandExecutionLeaseDuration,
    ) -> Result<CommandExecutionLeaseAcquisitionResult, CommandExecutionStoreError> {
        let execution_id = CommandExecutionId::new();
        let attempt_count = CommandAttemptCount::first();
        let lease_until = Utc::now() + lease_duration.value();
        let transaction = uow.transaction_mut();
        let insertion = sqlx::query(
            r#"
            INSERT INTO command_executions (
              id,
              message_id,
              command_name,
              attempt_count,
              lease_until
            ) VALUES (
              $1, $2, $3, $4, $5
            )
            ON CONFLICT (message_id) DO NOTHING
            "#,
        )
        .bind(execution_id.value())
        .bind(command.message_id.value())
        .bind(command.command_name.value())
        .bind(i64::from(attempt_count.value()))
        .bind(lease_until)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| CommandExecutionStoreError::Persistence(Box::new(source)))?;

        if insertion.rows_affected() == 1 {
            return Ok(CommandExecutionLeaseAcquisitionResult::Acquired { attempt_count });
        }

        let row = sqlx::query_as::<_, PgCommandExecutionRow>(
            r#"
            SELECT id,
                   message_id,
                   command_name,
                   attempt_count,
                   lease_until,
                   succeeded_at,
                   failed_at
            FROM command_executions
            WHERE message_id = $1
            FOR UPDATE
            "#,
        )
        .bind(command.message_id.value())
        .fetch_one(transaction.as_mut())
        .await
        .map_err(|source| CommandExecutionStoreError::Persistence(Box::new(source)))?;

        if row.succeeded_at.is_some() {
            return Ok(CommandExecutionLeaseAcquisitionResult::Succeeded);
        }
        if row.failed_at.is_some() {
            return Ok(CommandExecutionLeaseAcquisitionResult::Failed);
        }

        let now = Utc::now();
        if row.lease_until.is_some_and(|lease_until| lease_until > now) {
            return Ok(CommandExecutionLeaseAcquisitionResult::InProgress);
        }

        let attempt_count = CommandAttemptCount::try_from(row.attempt_count)
            .map_err(|source| CommandExecutionStoreError::Persistence(Box::new(source)))?
            .next();
        let lease_until = now + lease_duration.value();
        sqlx::query(
            r#"
            UPDATE command_executions
            SET attempt_count = $2,
                lease_until = $3
            WHERE message_id = $1
              AND succeeded_at IS NULL
              AND failed_at IS NULL
            "#,
        )
        .bind(command.message_id.value())
        .bind(i64::from(attempt_count.value()))
        .bind(lease_until)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| CommandExecutionStoreError::Persistence(Box::new(source)))?;

        Ok(CommandExecutionLeaseAcquisitionResult::Acquired { attempt_count })
    }

    async fn mark_succeeded(
        &self,
        uow: &mut Self::Uow,
        command_message_id: MessageId,
    ) -> Result<(), CommandExecutionStoreError> {
        let succeeded_at = Utc::now();
        let transaction = uow.transaction_mut();
        let result = sqlx::query(
            r#"
            UPDATE command_executions
            SET lease_until = NULL,
                succeeded_at = COALESCE(succeeded_at, $2)
            WHERE message_id = $1
              AND failed_at IS NULL
            "#,
        )
        .bind(command_message_id.value())
        .bind(succeeded_at)
        .execute(transaction.as_mut())
        .await
        .map_err(|source| CommandExecutionStoreError::Persistence(Box::new(source)))?;

        if result.rows_affected() == 0 {
            return Err(CommandExecutionStoreError::InvalidStateTransition);
        }
        Ok(())
    }

    async fn release_lease(
        &self,
        uow: &mut Self::Uow,
        command_message_id: MessageId,
        attempt_count: CommandAttemptCount,
    ) -> Result<CommandExecutionLeaseReleaseResult, CommandExecutionStoreError> {
        let transaction = uow.transaction_mut();
        let result = sqlx::query(
            r#"
            UPDATE command_executions
            SET lease_until = NULL
            WHERE message_id = $1
              AND attempt_count = $2
              AND lease_until IS NOT NULL
              AND succeeded_at IS NULL
              AND failed_at IS NULL
            "#,
        )
        .bind(command_message_id.value())
        .bind(i64::from(attempt_count.value()))
        .execute(transaction.as_mut())
        .await
        .map_err(|source| CommandExecutionStoreError::Persistence(Box::new(source)))?;

        if result.rows_affected() == 0 {
            return Ok(CommandExecutionLeaseReleaseResult::Stale);
        }
        Ok(CommandExecutionLeaseReleaseResult::Released)
    }

    async fn mark_failed(
        &self,
        uow: &mut Self::Uow,
        command_message_id: MessageId,
        attempt_count: CommandAttemptCount,
    ) -> Result<CommandExecutionFailureMarkResult, CommandExecutionStoreError> {
        let failed_at = CommandFailedAt::now();
        let transaction = uow.transaction_mut();
        let result = sqlx::query(
            r#"
            UPDATE command_executions
            SET lease_until = NULL,
                failed_at = $3
            WHERE message_id = $1
              AND attempt_count = $2
              AND lease_until IS NOT NULL
              AND succeeded_at IS NULL
              AND failed_at IS NULL
            "#,
        )
        .bind(command_message_id.value())
        .bind(i64::from(attempt_count.value()))
        .bind(DateTime::<Utc>::from(failed_at))
        .execute(transaction.as_mut())
        .await
        .map_err(|source| CommandExecutionStoreError::Persistence(Box::new(source)))?;

        if result.rows_affected() == 0 {
            return Ok(CommandExecutionFailureMarkResult::Stale);
        }
        Ok(CommandExecutionFailureMarkResult::Marked { failed_at })
    }
}

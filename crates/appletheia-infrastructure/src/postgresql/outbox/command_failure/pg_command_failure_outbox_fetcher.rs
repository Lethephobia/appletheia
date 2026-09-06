use appletheia_application::outbox::command_failure::CommandFailureOutbox;
use appletheia_application::outbox::{OutboxBatchSize, OutboxFetcher, OutboxFetcherError};
use chrono::Utc;
use sqlx::Postgres;

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::{PgCommandFailureOutboxRow, PgCommandFailureOutboxRowError};

/// Fetches terminal command-failure notifications for relay.
#[derive(Clone, Copy, Debug, Default)]
pub struct PgCommandFailureOutboxFetcher;

impl PgCommandFailureOutboxFetcher {
    pub fn new() -> Self {
        Self
    }

    async fn fetch(
        uow: &mut PgUnitOfWork,
        query: &'static str,
        pending: bool,
        limit: OutboxBatchSize,
    ) -> Result<Vec<CommandFailureOutbox>, OutboxFetcherError> {
        let database_query = sqlx::query_as::<Postgres, PgCommandFailureOutboxRow>(query);
        let rows = if pending {
            database_query
                .bind(Utc::now())
                .bind(limit.as_i64())
                .fetch_all(uow.transaction_mut().as_mut())
                .await
        } else {
            database_query
                .bind(limit.as_i64())
                .fetch_all(uow.transaction_mut().as_mut())
                .await
        }
        .map_err(|source| OutboxFetcherError::Persistence(Box::new(source)))?;

        rows.into_iter()
            .map(PgCommandFailureOutboxRow::try_into_outbox)
            .collect::<Result<Vec<_>, PgCommandFailureOutboxRowError>>()
            .map_err(|source| OutboxFetcherError::MappingFailed(Box::new(source)))
    }
}

impl OutboxFetcher for PgCommandFailureOutboxFetcher {
    type Uow = PgUnitOfWork;
    type Outbox = CommandFailureOutbox;

    async fn fetch_pending(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<Self::Outbox>, OutboxFetcherError> {
        Self::fetch(
            uow,
            r#"
            SELECT id, failure_sequence, failure_id, command_message_id, command_name,
                   saga_name, saga_instance_id, saga_step, terminal_reason,
                   command_attempt_count, correlation_id, causation_id, failed_at,
                   published_at, attempt_count, next_attempt_after, lease_owner,
                   lease_until, last_error, dead_lettered_at
            FROM command_failure_outbox
            WHERE published_at IS NULL
              AND dead_lettered_at IS NULL
              AND next_attempt_after <= $1
              AND (lease_owner IS NULL OR lease_until <= $1)
            ORDER BY next_attempt_after ASC, failure_sequence ASC
            LIMIT $2
            FOR UPDATE SKIP LOCKED
            "#,
            true,
            limit,
        )
        .await
    }

    async fn fetch_dead_lettered(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<Self::Outbox>, OutboxFetcherError> {
        Self::fetch(
            uow,
            r#"
            SELECT id, failure_sequence, failure_id, command_message_id, command_name,
                   saga_name, saga_instance_id, saga_step, terminal_reason,
                   command_attempt_count, correlation_id, causation_id, failed_at,
                   published_at, attempt_count, next_attempt_after, lease_owner,
                   lease_until, last_error, dead_lettered_at
            FROM command_failure_outbox
            WHERE dead_lettered_at IS NOT NULL
            ORDER BY dead_lettered_at ASC, id ASC
            LIMIT $1
            "#,
            false,
            limit,
        )
        .await
    }
}

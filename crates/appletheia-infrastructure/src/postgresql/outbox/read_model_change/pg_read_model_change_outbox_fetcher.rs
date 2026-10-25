use chrono::Utc;
use sqlx::Postgres;

use appletheia_application::outbox::{
    OutboxBatchSize, OutboxFetcher, OutboxFetcherError, read_model_change::ReadModelChangeOutbox,
};

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::{
    PgReadModelChangeOutboxDeadLetterRow, PgReadModelChangeOutboxDeadLetterRowError,
    PgReadModelChangeOutboxRow, PgReadModelChangeOutboxRowError,
};

pub struct PgReadModelChangeOutboxFetcher;

impl PgReadModelChangeOutboxFetcher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgReadModelChangeOutboxFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboxFetcher for PgReadModelChangeOutboxFetcher {
    type Uow = PgUnitOfWork;
    type Outbox = ReadModelChangeOutbox;

    async fn fetch_pending(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<ReadModelChangeOutbox>, OutboxFetcherError> {
        let now = Utc::now();

        let transaction = uow.transaction_mut();

        let outbox_rows = sqlx::query_as::<Postgres, PgReadModelChangeOutboxRow>(
            r#"
            SELECT
                id,
                change_id,
                source_event_id,
                changed_at,
                read_model_type,
                read_model_id,
                published_at,
                attempt_count,
                next_attempt_after,
                lease_owner,
                lease_until,
                last_error
            FROM read_model_change_outbox
            WHERE published_at IS NULL
              AND next_attempt_after <= $1
              AND (lease_owner IS NULL OR lease_until <= $1)
            ORDER BY next_attempt_after ASC, id ASC
            LIMIT $2
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(now)
        .bind(limit.as_i64())
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|e| OutboxFetcherError::Persistence(Box::new(e)))?;

        if outbox_rows.is_empty() {
            return Ok(Vec::new());
        }
        let outboxes = outbox_rows
            .into_iter()
            .map(PgReadModelChangeOutboxRow::try_into_outbox)
            .collect::<Result<Vec<ReadModelChangeOutbox>, PgReadModelChangeOutboxRowError>>()
            .map_err(|e| OutboxFetcherError::MappingFailed(Box::new(e)))?;

        Ok(outboxes)
    }

    async fn fetch_dead_lettered(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<ReadModelChangeOutbox>, OutboxFetcherError> {
        let transaction = uow.transaction_mut();

        let outbox_rows = sqlx::query_as::<Postgres, PgReadModelChangeOutboxDeadLetterRow>(
            r#"
            SELECT
                read_model_change_outbox_id,
                change_id,
                source_event_id,
                changed_at,
                read_model_type,
                read_model_id,
                published_at,
                attempt_count,
                next_attempt_after,
                lease_owner,
                lease_until,
                last_error,
                dead_lettered_at
            FROM read_model_change_dead_letters
            ORDER BY dead_lettered_at ASC, read_model_change_outbox_id ASC
            LIMIT $1
            "#,
        )
        .bind(limit.as_i64())
        .fetch_all(transaction.as_mut())
        .await
        .map_err(|e| OutboxFetcherError::Persistence(Box::new(e)))?;

        outbox_rows
            .into_iter()
            .map(
                PgReadModelChangeOutboxDeadLetterRow::try_into_outbox,
            )
            .collect::<Result<
                Vec<ReadModelChangeOutbox>,
                PgReadModelChangeOutboxDeadLetterRowError,
            >>()
            .map_err(|e| OutboxFetcherError::MappingFailed(Box::new(e)))
    }
}

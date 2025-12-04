use chrono::Utc;
use sqlx::{Postgres, Transaction};

use appletheia_application::outbox::{Outbox, OutboxBatchSize, OutboxFetcher, OutboxFetcherError};

use super::{PgOutboxRow, PgOutboxRowError};

pub struct PgOutboxFetcher<'c> {
    transaction: &'c mut Transaction<'static, Postgres>,
}

impl<'c> PgOutboxFetcher<'c> {
    pub fn new(transaction: &'c mut Transaction<'static, Postgres>) -> Self {
        Self { transaction }
    }
}

impl<'c> OutboxFetcher for PgOutboxFetcher<'c> {
    async fn fetch(&mut self, limit: OutboxBatchSize) -> Result<Vec<Outbox>, OutboxFetcherError> {
        let now = Utc::now();

        let outbox_rows = sqlx::query_as::<Postgres, PgOutboxRow>(
            r#"
            SELECT
                id,
                event_sequence,
                event_id,
                aggregate_type,
                aggregate_id,
                aggregate_version,
                payload,
                occurred_at,
                correlation_id,
                causation_id,
                context,
                published_at,
                attempt_count,
                next_attempt_after,
                lease_owner,
                lease_until,
                last_error
            FROM outbox
            WHERE published_at IS NULL
              AND next_attempt_after <= $1
              AND (lease_owner IS NULL OR lease_until <= $1)
            ORDER BY next_attempt_after ASC, event_sequence ASC
            LIMIT $2
            FOR UPDATE SKIP LOCKED
            "#,
        )
        .bind(now)
        .bind(limit.as_i64())
        .fetch_all(self.transaction.as_mut())
        .await
        .map_err(|e| OutboxFetcherError::Persistence(Box::new(e)))?;

        if outbox_rows.is_empty() {
            return Ok(Vec::new());
        }
        let outboxes = outbox_rows
            .into_iter()
            .map(PgOutboxRow::try_into_outbox)
            .collect::<Result<Vec<Outbox>, PgOutboxRowError>>()
            .map_err(|e| OutboxFetcherError::MappingFailed(Box::new(e)))?;

        Ok(outboxes)
    }
}

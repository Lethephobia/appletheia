use chrono::Utc;
use sqlx::Postgres;

use appletheia_application::outbox::{
    OutboxBatchSize, OutboxFetcher, OutboxFetcherError, event::EventOutbox,
};

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::{PgEventOutboxRow, PgEventOutboxRowError};

pub struct PgEventOutboxFetcher;

impl PgEventOutboxFetcher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgEventOutboxFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboxFetcher for PgEventOutboxFetcher {
    type Uow = PgUnitOfWork;
    type Outbox = EventOutbox;

    async fn fetch(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<EventOutbox>, OutboxFetcherError> {
        let now = Utc::now();

        let transaction = uow.transaction_mut();

        let outbox_rows = sqlx::query_as::<Postgres, PgEventOutboxRow>(
            r#"
            SELECT
                id,
                event_sequence,
                event_id,
                aggregate_type,
                aggregate_id,
                aggregate_version,
                event_name,
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
            FROM event_outbox eo
            WHERE published_at IS NULL
              AND next_attempt_after <= $1
              AND (lease_owner IS NULL OR lease_until <= $1)
              AND NOT EXISTS (
                SELECT 1
                FROM event_outbox eo2
                WHERE eo2.published_at IS NULL
                  AND eo2.aggregate_type = eo.aggregate_type
                  AND eo2.aggregate_id = eo.aggregate_id
                  AND eo2.aggregate_version < eo.aggregate_version
              )
            ORDER BY next_attempt_after ASC, event_sequence ASC
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
            .map(PgEventOutboxRow::try_into_outbox)
            .collect::<Result<Vec<EventOutbox>, PgEventOutboxRowError>>()
            .map_err(|e| OutboxFetcherError::MappingFailed(Box::new(e)))?;

        Ok(outboxes)
    }
}

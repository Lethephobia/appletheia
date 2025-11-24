use chrono::Utc;
use sqlx::{Postgres, QueryBuilder, Transaction};

use appletheia_application::outbox::{
    Outbox, OutboxBatchSize, OutboxFetcher, OutboxFetcherError, OutboxLeaseDuration,
    OutboxLeaseExpiresAt, OutboxRelayInstance,
};

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
    async fn fetch_and_acquire_outbox(
        &mut self,
        limit: OutboxBatchSize,
        owner: &OutboxRelayInstance,
        lease_for: OutboxLeaseDuration,
    ) -> Result<Vec<Outbox>, OutboxFetcherError> {
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
                lease_until
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

        let lease_until = OutboxLeaseExpiresAt::from_now(lease_for).value();

        let mut update_query = QueryBuilder::<Postgres>::new("UPDATE outbox SET lease_owner = ");
        update_query
            .push_bind(owner.to_string())
            .push(", lease_until = ")
            .push_bind(lease_until)
            .push(" WHERE id IN (");

        {
            let mut separated = update_query.separated(", ");
            for row in &outbox_rows {
                separated.push_bind(row.id);
            }
        }

        update_query.push(")");

        update_query
            .build()
            .execute(self.transaction.as_mut())
            .await
            .map_err(|e| OutboxFetcherError::Persistence(Box::new(e)))?;

        let outboxes = outbox_rows
            .into_iter()
            .map(PgOutboxRow::try_into_outbox)
            .collect::<Result<Vec<Outbox>, PgOutboxRowError>>()
            .map_err(|e| OutboxFetcherError::MappingFailed(Box::new(e)))?;

        Ok(outboxes)
    }
}

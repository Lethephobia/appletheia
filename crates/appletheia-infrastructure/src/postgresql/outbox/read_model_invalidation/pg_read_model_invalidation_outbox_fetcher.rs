use chrono::Utc;
use sqlx::Postgres;

use appletheia_application::outbox::read_model_invalidation::ReadModelInvalidationOutbox;
use appletheia_application::outbox::{OutboxBatchSize, OutboxFetcher, OutboxFetcherError};

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::{PgReadModelInvalidationOutboxRow, PgReadModelInvalidationOutboxRowError};

/// Fetches pending and dead-lettered invalidations for the generic relay.
pub struct PgReadModelInvalidationOutboxFetcher;

impl PgReadModelInvalidationOutboxFetcher {
    pub fn new() -> Self {
        Self
    }

    async fn fetch(
        uow: &mut PgUnitOfWork,
        query: &'static str,
        bind_current_time: bool,
        limit: OutboxBatchSize,
    ) -> Result<Vec<ReadModelInvalidationOutbox>, OutboxFetcherError> {
        let database_query = sqlx::query_as::<Postgres, PgReadModelInvalidationOutboxRow>(query);
        let transaction = uow.transaction_mut();
        let rows = if bind_current_time {
            database_query
                .bind(Utc::now())
                .bind(limit.as_i64())
                .fetch_all(transaction.as_mut())
                .await
        } else {
            database_query
                .bind(limit.as_i64())
                .fetch_all(transaction.as_mut())
                .await
        }
        .map_err(|source| OutboxFetcherError::Persistence(Box::new(source)))?;

        rows.into_iter()
            .map(PgReadModelInvalidationOutboxRow::try_into_outbox)
            .collect::<Result<Vec<_>, PgReadModelInvalidationOutboxRowError>>()
            .map_err(|source| OutboxFetcherError::MappingFailed(Box::new(source)))
    }
}

impl Default for PgReadModelInvalidationOutboxFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboxFetcher for PgReadModelInvalidationOutboxFetcher {
    type Uow = PgUnitOfWork;
    type Outbox = ReadModelInvalidationOutbox;

    async fn fetch_pending(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<Self::Outbox>, OutboxFetcherError> {
        Self::fetch(
            uow,
            r#"
            SELECT
                current_change.id,
                current_change.source_projector_name,
                current_change.source_event_sequence,
                current_change.source_event_id,
                current_change.occurred_at,
                current_change.correlation_id,
                current_change.causation_id,
                current_change.invalidated_dependencies,
                current_change.recorded_at,
                current_change.published_at,
                current_change.attempt_count,
                current_change.next_attempt_after,
                current_change.lease_owner,
                current_change.lease_until,
                current_change.last_error,
                current_change.dead_lettered_at
            FROM read_model_invalidation_outbox AS current_change
            WHERE current_change.published_at IS NULL
              AND current_change.dead_lettered_at IS NULL
              AND current_change.next_attempt_after <= $1
              AND (current_change.lease_owner IS NULL OR current_change.lease_until <= $1)
              AND NOT EXISTS (
                SELECT 1
                FROM read_model_invalidation_outbox earlier_change
                WHERE earlier_change.published_at IS NULL
                  AND earlier_change.source_projector_name = current_change.source_projector_name
                  AND earlier_change.source_event_sequence < current_change.source_event_sequence
              )
            ORDER BY
                current_change.next_attempt_after ASC,
                current_change.source_event_sequence ASC,
                current_change.id ASC
            LIMIT $2
            FOR UPDATE OF current_change SKIP LOCKED
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
            SELECT
                id, source_projector_name, source_event_sequence, source_event_id,
                occurred_at, correlation_id, causation_id, invalidated_dependencies,
                recorded_at, published_at,
                attempt_count, next_attempt_after, lease_owner, lease_until, last_error,
                dead_lettered_at
            FROM read_model_invalidation_outbox
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

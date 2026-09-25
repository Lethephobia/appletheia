use chrono::Utc;
use sqlx::Postgres;

use appletheia_application::outbox::read_model_invalidation::ReadModelInvalidationOutbox;
use appletheia_application::outbox::{OutboxBatchSize, OutboxFetcher, OutboxFetcherError};

use crate::postgresql::unit_of_work::PgUnitOfWork;

use super::{
    PgReadModelInvalidationOutboxDeadLetterRow, PgReadModelInvalidationOutboxDeadLetterRowError,
    PgReadModelInvalidationOutboxRow, PgReadModelInvalidationOutboxRowError,
};

/// Fetches pending and dead-lettered invalidations for the generic relay.
pub struct PgReadModelInvalidationOutboxFetcher;

impl PgReadModelInvalidationOutboxFetcher {
    pub fn new() -> Self {
        Self
    }

    async fn fetch(
        uow: &mut PgUnitOfWork,
        query: &'static str,
        limit: OutboxBatchSize,
    ) -> Result<Vec<ReadModelInvalidationOutbox>, OutboxFetcherError> {
        let database_query = sqlx::query_as::<Postgres, PgReadModelInvalidationOutboxRow>(query);
        let transaction = uow.transaction_mut();
        let rows = database_query
            .bind(Utc::now())
            .bind(limit.as_i64())
            .fetch_all(transaction.as_mut())
            .await
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
                current_invalidation.id,
                current_invalidation.invalidation_id,
                current_invalidation.source_projector_name,
                current_invalidation.source_event_sequence,
                current_invalidation.source_event_id,
                current_invalidation.source_event_occurred_at,
                current_invalidation.correlation_id,
                current_invalidation.causation_id,
                current_invalidation.invalidated_partitions,
                current_invalidation.recorded_at,
                current_invalidation.published_at,
                current_invalidation.attempt_count,
                current_invalidation.next_attempt_after,
                current_invalidation.lease_owner,
                current_invalidation.lease_until,
                current_invalidation.last_error
            FROM read_model_invalidation_outbox AS current_invalidation
            WHERE current_invalidation.published_at IS NULL
              AND current_invalidation.next_attempt_after <= $1
              AND (current_invalidation.lease_owner IS NULL OR current_invalidation.lease_until <= $1)
              AND NOT EXISTS (
                SELECT 1
                FROM read_model_invalidation_outbox earlier_invalidation
                WHERE earlier_invalidation.published_at IS NULL
                  AND earlier_invalidation.source_projector_name = current_invalidation.source_projector_name
                  AND earlier_invalidation.source_event_sequence < current_invalidation.source_event_sequence
              )
            ORDER BY
                current_invalidation.next_attempt_after ASC,
                current_invalidation.source_event_sequence ASC,
                current_invalidation.id ASC
            LIMIT $2
            FOR UPDATE OF current_invalidation SKIP LOCKED
            "#,
            limit,
        )
        .await
    }

    async fn fetch_dead_lettered(
        &self,
        uow: &mut Self::Uow,
        limit: OutboxBatchSize,
    ) -> Result<Vec<Self::Outbox>, OutboxFetcherError> {
        let rows = sqlx::query_as::<Postgres, PgReadModelInvalidationOutboxDeadLetterRow>(
            r#"
            SELECT
                read_model_invalidation_outbox_id, invalidation_id, source_projector_name, source_event_sequence, source_event_id,
                source_event_occurred_at, correlation_id, causation_id, invalidated_partitions,
                recorded_at, published_at,
                attempt_count, next_attempt_after, lease_owner, lease_until, last_error,
                dead_lettered_at
            FROM read_model_invalidation_dead_letters
            ORDER BY dead_lettered_at ASC, read_model_invalidation_outbox_id ASC
            LIMIT $1
            "#,
        )
        .bind(limit.as_i64())
        .fetch_all(uow.transaction_mut().as_mut())
        .await
        .map_err(|source| OutboxFetcherError::Persistence(Box::new(source)))?;
        rows.into_iter()
            .map(PgReadModelInvalidationOutboxDeadLetterRow::try_into_outbox)
            .collect::<Result<Vec<_>, PgReadModelInvalidationOutboxDeadLetterRowError>>()
            .map_err(|source| OutboxFetcherError::MappingFailed(Box::new(source)))
    }
}

use chrono::{DateTime, Utc};
use sqlx::{Postgres, QueryBuilder};

use appletheia_application::messaging::PublishDispatchError;
use appletheia_application::outbox::read_model_invalidation::ReadModelInvalidationOutbox;
use appletheia_application::outbox::{OutboxLifecycle, OutboxWriter, OutboxWriterError};

use crate::postgresql::unit_of_work::PgUnitOfWork;

/// Persists invalidation relay lifecycle state.
pub struct PgReadModelInvalidationOutboxWriter;

impl PgReadModelInvalidationOutboxWriter {
    pub fn new() -> Self {
        Self
    }

    async fn upsert_outbox_rows(
        uow: &mut PgUnitOfWork,
        outboxes: &[&ReadModelInvalidationOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO read_model_invalidation_outbox (
                id, source_projector_name, source_event_sequence, source_event_id,
                occurred_at, correlation_id, causation_id, invalidated_partitions,
                recorded_at, published_at,
                attempt_count, next_attempt_after, lease_owner, lease_until, last_error
            )
            "#,
        );

        let mut prepared_values = Vec::with_capacity(outboxes.len());
        for outbox in outboxes {
            let partitions = serde_json::to_value(&outbox.invalidation.invalidated_partitions)
                .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
            let last_error = outbox
                .last_error
                .as_ref()
                .map(|error: &PublishDispatchError| serde_json::to_value(error))
                .transpose()
                .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
            prepared_values.push((outbox, partitions, last_error));
        }

        query_builder.push_values(
            prepared_values,
            |mut separated, (outbox, partitions, last_error)| {
                let invalidation = &outbox.invalidation;
                separated
                    .push_bind(outbox.id.value())
                    .push_bind(invalidation.source_projector_name.value())
                    .push_bind(invalidation.source_event_sequence.value())
                    .push_bind(invalidation.source_event_id.value())
                    .push_bind(DateTime::<Utc>::from(invalidation.occurred_at))
                    .push_bind(invalidation.correlation_id.value())
                    .push_bind(invalidation.causation_id.value())
                    .push_bind(partitions)
                    .push("clock_timestamp()")
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
            },
        );

        query_builder.push(
            r#"
            ON CONFLICT (id) DO UPDATE
               SET published_at = EXCLUDED.published_at,
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
        outboxes: &[&ReadModelInvalidationOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO read_model_invalidation_dead_letters (
                read_model_invalidation_outbox_id, source_projector_name, source_event_sequence, source_event_id,
                occurred_at, correlation_id, causation_id, invalidated_partitions,
                recorded_at, published_at,
                attempt_count, next_attempt_after, lease_owner, lease_until, last_error,
                dead_lettered_at
            )
            "#,
        );

        let mut prepared_values = Vec::with_capacity(outboxes.len());
        for outbox in outboxes {
            let partitions = serde_json::to_value(&outbox.invalidation.invalidated_partitions)
                .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
            let last_error = outbox
                .last_error
                .as_ref()
                .map(|error: &PublishDispatchError| serde_json::to_value(error))
                .transpose()
                .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
            let dead_lettered_at = match outbox.lifecycle {
                OutboxLifecycle::Active => continue,
                OutboxLifecycle::DeadLettered { dead_lettered_at } => {
                    DateTime::<Utc>::from(dead_lettered_at)
                }
            };
            prepared_values.push((outbox, partitions, last_error, dead_lettered_at));
        }

        query_builder.push_values(
            prepared_values,
            |mut separated, (outbox, partitions, last_error, dead_lettered_at)| {
                let invalidation = &outbox.invalidation;
                separated
                    .push_bind(outbox.id.value())
                    .push_bind(invalidation.source_projector_name.value())
                    .push_bind(invalidation.source_event_sequence.value())
                    .push_bind(invalidation.source_event_id.value())
                    .push_bind(DateTime::<Utc>::from(invalidation.occurred_at))
                    .push_bind(invalidation.correlation_id.value())
                    .push_bind(invalidation.causation_id.value())
                    .push_bind(partitions)
                    .push("clock_timestamp()")
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
        dead_lettered_outboxes: &[&ReadModelInvalidationOutbox],
    ) -> Result<(), OutboxWriterError> {
        let mut query_builder = QueryBuilder::<Postgres>::new(
            "DELETE FROM read_model_invalidation_outbox WHERE id IN (",
        );

        {
            let mut separated = query_builder.separated(", ");
            for outbox in dead_lettered_outboxes {
                separated.push_bind(outbox.id.value());
            }
        }

        query_builder.push(")");

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }

    async fn delete_dead_letters(
        uow: &mut PgUnitOfWork,
        active_outboxes: &[&ReadModelInvalidationOutbox],
    ) -> Result<(), OutboxWriterError> {
        if active_outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            DELETE FROM read_model_invalidation_dead_letters
             WHERE read_model_invalidation_outbox_id IN (
            "#,
        );

        {
            let mut separated = query_builder.separated(", ");
            for outbox in active_outboxes {
                separated.push_bind(outbox.id.value());
            }
        }

        query_builder.push(")");

        let transaction = uow.transaction_mut();

        query_builder
            .build()
            .execute(transaction.as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }
}

impl Default for PgReadModelInvalidationOutboxWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboxWriter for PgReadModelInvalidationOutboxWriter {
    type Uow = PgUnitOfWork;
    type Outbox = ReadModelInvalidationOutbox;

    async fn write_outbox(
        &self,
        uow: &mut Self::Uow,
        outboxes: &[ReadModelInvalidationOutbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut active_outboxes: Vec<&ReadModelInvalidationOutbox> = Vec::new();
        let mut dead_lettered_outboxes: Vec<&ReadModelInvalidationOutbox> = Vec::new();
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
    use std::num::NonZeroU32;

    use appletheia_application::outbox::read_model_invalidation::ReadModelInvalidationOutboxEnqueuer;
    use appletheia_application::outbox::{OutboxBatchSize, OutboxDeadLetteredAt, OutboxFetcher};
    use appletheia_application::read_model::ReadModelInvalidationEnvelope;
    use appletheia_application::unit_of_work::{UnitOfWork, UnitOfWorkFactory};
    use serde_json::json;
    use sqlx::PgPool;
    use uuid::Uuid;

    use super::super::{
        PgReadModelInvalidationOutboxEnqueuer, PgReadModelInvalidationOutboxFetcher,
    };
    use super::*;
    use crate::postgresql::PgUnitOfWorkFactory;

    #[sqlx::test(migrations = "migrations/postgresql")]
    #[ignore = "requires PostgreSQL"]
    async fn dead_letter_transfer_and_redrive_are_transactional(pool: PgPool) {
        let factory = PgUnitOfWorkFactory::new(pool.clone());
        let fetcher = PgReadModelInvalidationOutboxFetcher;
        let writer = PgReadModelInvalidationOutboxWriter;
        let batch = OutboxBatchSize::new(NonZeroU32::new(10).unwrap());
        let envelope: ReadModelInvalidationEnvelope = serde_json::from_value(json!({
            "source_projector_name": "test_projector",
            "source_event_sequence": 1,
            "source_event_id": Uuid::now_v7(),
            "occurred_at": DateTime::<Utc>::from_timestamp_micros(Utc::now().timestamp_micros()).unwrap(),
            "correlation_id": Uuid::now_v7(),
            "causation_id": Uuid::now_v7(),
            "invalidated_partitions": [{"fragment_name": "test", "key": 1}],
        }))
        .unwrap();
        let mut setup = factory.begin().await.unwrap();
        PgReadModelInvalidationOutboxEnqueuer
            .enqueue_invalidation(&mut setup, &envelope)
            .await
            .unwrap();
        setup.commit().await.unwrap();

        let mut transition = factory.begin().await.unwrap();
        let mut pending = fetcher.fetch_pending(&mut transition, batch).await.unwrap();
        assert_eq!(pending.len(), 1);
        let id = pending[0].id;
        pending[0].lifecycle = OutboxLifecycle::DeadLettered {
            dead_lettered_at: OutboxDeadLetteredAt::from(
                DateTime::<Utc>::from_timestamp_micros(Utc::now().timestamp_micros()).unwrap(),
            ),
        };
        writer
            .write_outbox(&mut transition, &pending)
            .await
            .unwrap();
        assert!(
            fetcher
                .fetch_pending(&mut transition, batch)
                .await
                .unwrap()
                .is_empty()
        );
        let dead = fetcher
            .fetch_dead_lettered(&mut transition, batch)
            .await
            .unwrap();
        assert_eq!(dead, pending);
        transition.rollback().await.unwrap();

        let mut committed = factory.begin().await.unwrap();
        assert_eq!(
            fetcher
                .fetch_pending(&mut committed, batch)
                .await
                .unwrap()
                .len(),
            1
        );
        assert!(
            fetcher
                .fetch_dead_lettered(&mut committed, batch)
                .await
                .unwrap()
                .is_empty()
        );
        writer.write_outbox(&mut committed, &pending).await.unwrap();
        committed.commit().await.unwrap();

        let mut retry = factory.begin().await.unwrap();
        let mut dead = fetcher
            .fetch_dead_lettered(&mut retry, batch)
            .await
            .unwrap();
        dead[0].lifecycle = OutboxLifecycle::Active;
        writer.write_outbox(&mut retry, &dead).await.unwrap();
        assert!(
            fetcher
                .fetch_dead_lettered(&mut retry, batch)
                .await
                .unwrap()
                .is_empty()
        );
        let restored = fetcher.fetch_pending(&mut retry, batch).await.unwrap();
        assert_eq!(restored[0].id, id);
        assert_eq!(restored[0].invalidation, envelope);
        retry.rollback().await.unwrap();

        let mut final_retry = factory.begin().await.unwrap();
        assert_eq!(
            fetcher
                .fetch_dead_lettered(&mut final_retry, batch)
                .await
                .unwrap()
                .len(),
            1
        );
        assert!(
            fetcher
                .fetch_pending(&mut final_retry, batch)
                .await
                .unwrap()
                .is_empty()
        );
        writer.write_outbox(&mut final_retry, &dead).await.unwrap();
        final_retry.commit().await.unwrap();
    }
}

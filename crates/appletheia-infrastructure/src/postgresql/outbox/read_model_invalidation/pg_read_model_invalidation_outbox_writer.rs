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
        outboxes: &[Self::Outbox],
    ) -> Result<(), OutboxWriterError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO read_model_invalidation_outbox (
                id, source_projector_name, source_event_sequence, source_event_id,
                occurred_at, correlation_id, causation_id, invalidated_dependencies,
                recorded_at, published_at,
                attempt_count, next_attempt_after, lease_owner, lease_until, last_error,
                dead_lettered_at
            )
            "#,
        );

        let mut prepared_values = Vec::with_capacity(outboxes.len());
        for outbox in outboxes {
            let dependencies = serde_json::to_value(&outbox.invalidation.invalidated_dependencies)
                .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
            let last_error = outbox
                .last_error
                .as_ref()
                .map(|error: &PublishDispatchError| serde_json::to_value(error))
                .transpose()
                .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;
            let dead_lettered_at = match outbox.lifecycle {
                OutboxLifecycle::Active => None,
                OutboxLifecycle::DeadLettered { dead_lettered_at } => {
                    Some(DateTime::<Utc>::from(dead_lettered_at))
                }
            };
            prepared_values.push((outbox, dependencies, last_error, dead_lettered_at));
        }

        query_builder.push_values(
            prepared_values,
            |mut separated, (outbox, dependencies, last_error, dead_lettered_at)| {
                let invalidation = &outbox.invalidation;
                separated
                    .push_bind(invalidation.invalidation_id.value())
                    .push_bind(invalidation.source_projector_name.value())
                    .push_bind(invalidation.source_event_sequence.value())
                    .push_bind(invalidation.source_event_id.value())
                    .push_bind(DateTime::<Utc>::from(invalidation.occurred_at))
                    .push_bind(invalidation.correlation_id.value())
                    .push_bind(invalidation.causation_id.value())
                    .push_bind(dependencies)
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

        query_builder.push(
            r#"
            ON CONFLICT (id) DO UPDATE
               SET published_at = EXCLUDED.published_at,
                   attempt_count = EXCLUDED.attempt_count,
                   next_attempt_after = EXCLUDED.next_attempt_after,
                   lease_owner = EXCLUDED.lease_owner,
                   lease_until = EXCLUDED.lease_until,
                   last_error = EXCLUDED.last_error,
                   dead_lettered_at = EXCLUDED.dead_lettered_at
            "#,
        );

        query_builder
            .build()
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| OutboxWriterError::Persistence(Box::new(source)))?;

        Ok(())
    }
}

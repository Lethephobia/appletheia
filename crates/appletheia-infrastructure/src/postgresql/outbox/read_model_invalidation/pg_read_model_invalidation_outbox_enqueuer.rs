use appletheia_application::outbox::read_model_invalidation::{
    ReadModelInvalidationOutboxEnqueueError, ReadModelInvalidationOutboxEnqueuer,
};
use appletheia_application::read_model::ReadModelInvalidationEnvelope;

use crate::postgresql::unit_of_work::PgUnitOfWork;

/// Persists read-model invalidations in their projection transaction.
pub struct PgReadModelInvalidationOutboxEnqueuer;

impl PgReadModelInvalidationOutboxEnqueuer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgReadModelInvalidationOutboxEnqueuer {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadModelInvalidationOutboxEnqueuer for PgReadModelInvalidationOutboxEnqueuer {
    type Uow = PgUnitOfWork;

    async fn enqueue_invalidations(
        &self,
        uow: &mut Self::Uow,
        invalidations: &[ReadModelInvalidationEnvelope],
    ) -> Result<(), ReadModelInvalidationOutboxEnqueueError> {
        for invalidation in invalidations {
            let dependencies = serde_json::to_value(&invalidation.invalidated_dependencies)
                .map_err(|source| {
                    ReadModelInvalidationOutboxEnqueueError::Persistence(Box::new(source))
                })?;

            sqlx::query(
                r#"
                INSERT INTO read_model_invalidation_outbox (
                    id,
                    source_projector_name,
                    source_event_sequence,
                    source_event_id,
                    occurred_at,
                    correlation_id,
                    causation_id,
                    invalidated_dependencies
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT (source_projector_name, source_event_id) DO NOTHING
                "#,
            )
            .bind(invalidation.invalidation_id.value())
            .bind(invalidation.source_projector_name.value())
            .bind(invalidation.source_event_sequence.value())
            .bind(invalidation.source_event_id.value())
            .bind(chrono::DateTime::<chrono::Utc>::from(
                invalidation.occurred_at,
            ))
            .bind(invalidation.correlation_id.value())
            .bind(invalidation.causation_id.value())
            .bind(dependencies)
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| {
                ReadModelInvalidationOutboxEnqueueError::Persistence(Box::new(source))
            })?;
        }

        Ok(())
    }
}

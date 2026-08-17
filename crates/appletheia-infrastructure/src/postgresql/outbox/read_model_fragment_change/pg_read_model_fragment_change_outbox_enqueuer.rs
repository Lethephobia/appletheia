use appletheia_application::outbox::read_model_fragment_change::{
    ReadModelFragmentChangeOutboxEnqueueError, ReadModelFragmentChangeOutboxEnqueuer,
};
use appletheia_application::read_model::ReadModelFragmentChangeEnvelope;

use crate::postgresql::unit_of_work::PgUnitOfWork;

/// Persists projected source-fragment changes in their projection transaction.
pub struct PgReadModelFragmentChangeOutboxEnqueuer;

impl PgReadModelFragmentChangeOutboxEnqueuer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PgReadModelFragmentChangeOutboxEnqueuer {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadModelFragmentChangeOutboxEnqueuer for PgReadModelFragmentChangeOutboxEnqueuer {
    type Uow = PgUnitOfWork;

    async fn enqueue_fragment_changes(
        &self,
        uow: &mut Self::Uow,
        fragment_changes: &[ReadModelFragmentChangeEnvelope],
    ) -> Result<(), ReadModelFragmentChangeOutboxEnqueueError> {
        for change in fragment_changes {
            let partition = serde_json::to_value(&change.partition).map_err(|source| {
                ReadModelFragmentChangeOutboxEnqueueError::Persistence(Box::new(source))
            })?;
            let serialized_changes = serde_json::to_value(&change.changes).map_err(|source| {
                ReadModelFragmentChangeOutboxEnqueueError::Persistence(Box::new(source))
            })?;

            sqlx::query(
                r#"
                INSERT INTO read_model_fragment_change_outbox (
                    id,
                    partition,
                    source_projector_name,
                    source_event_sequence,
                    source_event_id,
                    source_aggregate_type,
                    source_aggregate_id,
                    occurred_at,
                    correlation_id,
                    causation_id,
                    changes
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (
                    partition,
                    source_projector_name,
                    source_event_id
                ) DO NOTHING
                "#,
            )
            .bind(change.change_id.value())
            .bind(partition)
            .bind(change.source_projector_name.value())
            .bind(change.source_event_sequence.value())
            .bind(change.source_event_id.value())
            .bind(change.source_aggregate_type.value())
            .bind(change.source_aggregate_id.value())
            .bind(chrono::DateTime::<chrono::Utc>::from(change.occurred_at))
            .bind(change.correlation_id.value())
            .bind(change.causation_id.value())
            .bind(serialized_changes)
            .execute(uow.transaction_mut().as_mut())
            .await
            .map_err(|source| {
                ReadModelFragmentChangeOutboxEnqueueError::Persistence(Box::new(source))
            })?;
        }

        Ok(())
    }
}

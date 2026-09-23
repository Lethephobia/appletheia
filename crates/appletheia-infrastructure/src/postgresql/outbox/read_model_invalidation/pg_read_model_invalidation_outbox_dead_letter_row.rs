use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::outbox::read_model_invalidation::ReadModelInvalidationOutbox;
use appletheia_application::outbox::{OutboxDeadLetteredAt, OutboxLifecycle};

use super::{PgReadModelInvalidationOutboxDeadLetterRowError, PgReadModelInvalidationOutboxRow};

#[derive(Clone, Debug, Eq, PartialEq, FromRow)]
pub struct PgReadModelInvalidationOutboxDeadLetterRow {
    pub read_model_invalidation_outbox_id: Uuid,
    pub invalidation_id: Uuid,
    pub source_projector_name: String,
    pub source_event_sequence: i64,
    pub source_event_id: Uuid,
    pub source_event_occurred_at: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub invalidated_partitions: serde_json::Value,
    pub recorded_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub attempt_count: i64,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
    pub last_error: Option<serde_json::Value>,
    pub dead_lettered_at: DateTime<Utc>,
}

impl PgReadModelInvalidationOutboxDeadLetterRow {
    pub fn try_into_outbox(
        self,
    ) -> Result<ReadModelInvalidationOutbox, PgReadModelInvalidationOutboxDeadLetterRowError> {
        let dead_lettered_at = OutboxDeadLetteredAt::from(self.dead_lettered_at);
        let row = PgReadModelInvalidationOutboxRow {
            id: self.read_model_invalidation_outbox_id,
            invalidation_id: self.invalidation_id,
            source_projector_name: self.source_projector_name,
            source_event_sequence: self.source_event_sequence,
            source_event_id: self.source_event_id,
            source_event_occurred_at: self.source_event_occurred_at,
            correlation_id: self.correlation_id,
            causation_id: self.causation_id,
            invalidated_partitions: self.invalidated_partitions,
            recorded_at: self.recorded_at,
            published_at: self.published_at,
            attempt_count: self.attempt_count,
            next_attempt_after: self.next_attempt_after,
            lease_owner: self.lease_owner,
            lease_until: self.lease_until,
            last_error: self.last_error,
        };
        let mut outbox = row.try_into_outbox()?;
        outbox.lifecycle = OutboxLifecycle::DeadLettered { dead_lettered_at };
        Ok(outbox)
    }
}

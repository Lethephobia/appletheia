use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::outbox::{OutboxDeadLetteredAt, OutboxLifecycle, event::EventOutbox};

use super::PgEventOutboxRow;
use super::pg_event_outbox_dead_letter_row_error::PgEventOutboxDeadLetterRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgEventOutboxDeadLetterRow {
    pub event_outbox_id: Uuid,
    pub event_sequence: i64,
    pub event_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_version: i64,
    pub event_name: String,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub context: serde_json::Value,
    pub published_at: Option<DateTime<Utc>>,
    pub attempt_count: i64,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
    pub last_error: Option<serde_json::Value>,
    pub dead_lettered_at: DateTime<Utc>,
}

impl PgEventOutboxDeadLetterRow {
    pub fn try_into_outbox(self) -> Result<EventOutbox, PgEventOutboxDeadLetterRowError> {
        let dead_lettered_at = OutboxDeadLetteredAt::from(self.dead_lettered_at);

        let outbox_row = PgEventOutboxRow {
            id: self.event_outbox_id,
            event_sequence: self.event_sequence,
            event_id: self.event_id,
            aggregate_type: self.aggregate_type,
            aggregate_id: self.aggregate_id,
            aggregate_version: self.aggregate_version,
            event_name: self.event_name,
            payload: self.payload,
            occurred_at: self.occurred_at,
            correlation_id: self.correlation_id,
            causation_id: self.causation_id,
            context: self.context,
            published_at: self.published_at,
            attempt_count: self.attempt_count,
            next_attempt_after: self.next_attempt_after,
            lease_owner: self.lease_owner,
            lease_until: self.lease_until,
            last_error: self.last_error,
        };

        let mut outbox = outbox_row.try_into_outbox()?;
        outbox.lifecycle = OutboxLifecycle::DeadLettered { dead_lettered_at };

        Ok(outbox)
    }
}

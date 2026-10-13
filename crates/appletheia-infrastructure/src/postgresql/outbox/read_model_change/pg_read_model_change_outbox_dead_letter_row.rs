use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::outbox::{
    OutboxDeadLetteredAt, OutboxLifecycle, read_model_change::ReadModelChangeOutbox,
};

use super::PgReadModelChangeOutboxRow;
use super::pg_read_model_change_outbox_dead_letter_row_error::PgReadModelChangeOutboxDeadLetterRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgReadModelChangeOutboxDeadLetterRow {
    pub read_model_change_outbox_id: Uuid,
    pub change_id: Uuid,
    pub source_event_id: Uuid,
    pub changed_at: DateTime<Utc>,
    pub read_model_type: String,
    pub read_model_id: String,
    pub published_at: Option<DateTime<Utc>>,
    pub attempt_count: i64,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
    pub last_error: Option<serde_json::Value>,
    pub dead_lettered_at: DateTime<Utc>,
}

impl PgReadModelChangeOutboxDeadLetterRow {
    pub fn try_into_outbox(
        self,
    ) -> Result<ReadModelChangeOutbox, PgReadModelChangeOutboxDeadLetterRowError> {
        let dead_lettered_at = OutboxDeadLetteredAt::from(self.dead_lettered_at);

        let outbox_row = PgReadModelChangeOutboxRow {
            id: self.read_model_change_outbox_id,
            change_id: self.change_id,
            source_event_id: self.source_event_id,
            changed_at: self.changed_at,
            read_model_type: self.read_model_type,
            read_model_id: self.read_model_id,
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

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::outbox::{
    OutboxDeadLetteredAt, OutboxLifecycle, command::CommandOutbox,
};

use super::PgCommandOutboxRow;
use super::pg_command_outbox_dead_letter_row_error::PgCommandOutboxDeadLetterRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgCommandOutboxDeadLetterRow {
    pub command_outbox_id: Uuid,
    pub command_sequence: i64,
    pub message_id: Uuid,
    pub command_name: String,
    pub payload: serde_json::Value,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub published_at: Option<DateTime<Utc>>,
    pub attempt_count: i64,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
    pub last_error: Option<serde_json::Value>,
    pub dead_lettered_at: DateTime<Utc>,
}

impl PgCommandOutboxDeadLetterRow {
    pub fn try_into_outbox(self) -> Result<CommandOutbox, PgCommandOutboxDeadLetterRowError> {
        let dead_lettered_at = OutboxDeadLetteredAt::from(self.dead_lettered_at);

        let outbox_row = PgCommandOutboxRow {
            id: self.command_outbox_id,
            command_sequence: self.command_sequence,
            message_id: self.message_id,
            command_name: self.command_name,
            payload: self.payload,
            correlation_id: self.correlation_id,
            causation_id: self.causation_id,
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

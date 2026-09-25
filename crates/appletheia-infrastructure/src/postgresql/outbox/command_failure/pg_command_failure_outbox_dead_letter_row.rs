use appletheia_application::outbox::command_failure::CommandFailureOutbox;
use appletheia_application::outbox::{OutboxDeadLetteredAt, OutboxLifecycle};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::{PgCommandFailureOutboxDeadLetterRowError, PgCommandFailureOutboxRow};

/// Maps a PostgreSQL command-failure dead-letter row to application types.
#[derive(Clone, Debug, FromRow)]
pub struct PgCommandFailureOutboxDeadLetterRow {
    pub command_failure_outbox_id: Uuid,
    pub failure_sequence: i64,
    pub failure_id: Uuid,
    pub command_message_id: Uuid,
    pub command_name: String,
    pub saga_name: String,
    pub saga_instance_id: Uuid,
    pub saga_step: serde_json::Value,
    pub terminal_reason: String,
    pub command_attempt_count: i64,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub failed_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub attempt_count: i64,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
    pub last_error: Option<serde_json::Value>,
    pub dead_lettered_at: DateTime<Utc>,
}

impl PgCommandFailureOutboxDeadLetterRow {
    pub fn try_into_outbox(
        self,
    ) -> Result<CommandFailureOutbox, PgCommandFailureOutboxDeadLetterRowError> {
        let dead_lettered_at = OutboxDeadLetteredAt::from(self.dead_lettered_at);
        let outbox_row = PgCommandFailureOutboxRow {
            id: self.command_failure_outbox_id,
            failure_sequence: self.failure_sequence,
            failure_id: self.failure_id,
            command_message_id: self.command_message_id,
            command_name: self.command_name,
            saga_name: self.saga_name,
            saga_instance_id: self.saga_instance_id,
            saga_step: self.saga_step,
            terminal_reason: self.terminal_reason,
            command_attempt_count: self.command_attempt_count,
            correlation_id: self.correlation_id,
            causation_id: self.causation_id,
            failed_at: self.failed_at,
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

use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::command::CommandNameOwned;
use appletheia_application::massaging::PublishDispatchError;
use appletheia_application::outbox::command::{CommandEnvelope, SerializedCommand};
use appletheia_application::outbox::{
    OutboxAttemptCount, OutboxLeaseExpiresAt, OutboxLifecycle, OutboxNextAttemptAt,
    OutboxPublishedAt, OutboxRelayInstance, OutboxState,
    command::{CommandOutbox, CommandOutboxId},
};
use appletheia_application::request_context::{CausationId, CorrelationId, MessageId};

use super::PgCommandOutboxRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgCommandOutboxRow {
    pub id: Uuid,
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
}

impl PgCommandOutboxRow {
    pub fn try_into_outbox(self) -> Result<CommandOutbox, PgCommandOutboxRowError> {
        let id = CommandOutboxId::try_from(self.id)?;

        let command_name_string = self.command_name;
        let command_name = match CommandNameOwned::new(command_name_string.clone()) {
            Ok(value) => value,
            Err(_) => return Err(PgCommandOutboxRowError::CommandName(command_name_string)),
        };
        let serialized_command = SerializedCommand::try_from(self.payload)?;

        let correlation_id = CorrelationId(self.correlation_id);
        let message_id = MessageId::from(self.message_id);
        let causation_id = CausationId::from(MessageId::from(self.causation_id));

        let command = CommandEnvelope {
            command_name,
            command: serialized_command,
            correlation_id,
            message_id,
            causation_id,
        };

        let attempt_count = OutboxAttemptCount::try_from(self.attempt_count)?;
        let next_attempt_after = OutboxNextAttemptAt::from(self.next_attempt_after);

        let lease_owner = match self.lease_owner {
            Some(owner) => Some(OutboxRelayInstance::from_str(&owner)?),
            None => None,
        };
        let lease_until = self.lease_until.map(OutboxLeaseExpiresAt::from);
        let published_at = self.published_at.map(OutboxPublishedAt::from);

        let last_error = match self.last_error {
            Some(value) => Some(serde_json::from_value::<PublishDispatchError>(value)?),
            None => None,
        };

        let state = match (published_at, lease_owner, lease_until) {
            (Some(published_at), _, _) => OutboxState::Published {
                published_at,
                attempt_count,
            },
            (None, Some(lease_owner), Some(lease_until)) => OutboxState::Leased {
                attempt_count,
                next_attempt_after,
                lease_owner,
                lease_until,
            },
            (None, None, _) => OutboxState::Pending {
                attempt_count,
                next_attempt_after,
            },
            (None, Some(_), None) => return Err(PgCommandOutboxRowError::InconsistentLeaseState),
        };

        Ok(CommandOutbox {
            id,
            sequence: self.command_sequence,
            command,
            state,
            last_error,
            lifecycle: OutboxLifecycle::Active,
        })
    }
}

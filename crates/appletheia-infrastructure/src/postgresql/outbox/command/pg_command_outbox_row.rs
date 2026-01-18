use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::command::{CommandHash, CommandNameOwned};
use appletheia_application::outbox::command::CommandEnvelope;
use appletheia_application::outbox::{
    OrderingKey, OutboxAttemptCount, OutboxDispatchError, OutboxLeaseExpiresAt, OutboxLifecycle,
    OutboxNextAttemptAt, OutboxPublishedAt, OutboxRelayInstance, OutboxState,
    command::{CommandOutbox, CommandOutboxId},
};
use appletheia_application::request_context::{
    CausationId, CorrelationId, MessageId, RequestContext,
};

use super::PgCommandOutboxRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgCommandOutboxRow {
    pub id: Uuid,
    pub command_sequence: i64,
    pub message_id: Uuid,
    pub command_name: String,
    pub command_hash: String,
    pub payload: serde_json::Value,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub context: serde_json::Value,
    pub ordering_key: String,
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

        let command_name = CommandNameOwned::from_str(&self.command_name)?;
        let command_hash = CommandHash::new(self.command_hash)?;
        let payload = self.payload;

        let _context = serde_json::from_value::<RequestContext>(self.context)?;

        // Preserve the DB columns as the source of truth for message/correlation ids, even if
        // context differs.
        let _correlation_id = CorrelationId(self.correlation_id);
        let message_id = MessageId::from(self.message_id);
        let causation_id = CausationId::from(MessageId::from(self.causation_id));

        let ordering_key = OrderingKey::new(self.ordering_key)?;

        let command = CommandEnvelope {
            command_name,
            command_hash,
            payload,
            context: RequestContext {
                correlation_id: _correlation_id,
                message_id,
            },
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
            Some(value) => Some(serde_json::from_value::<OutboxDispatchError>(value)?),
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
            ordering_key,
            command,
            state,
            last_error,
            lifecycle: OutboxLifecycle::Active,
        })
    }
}

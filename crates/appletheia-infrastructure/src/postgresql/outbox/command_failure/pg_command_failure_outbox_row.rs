use std::str::FromStr;

use appletheia_application::command::{
    CommandAttemptCount, CommandFailureEnvelope, CommandFailureId, CommandNameOwned,
    CommandTerminalReason,
};
use appletheia_application::messaging::PublishDispatchError;
use appletheia_application::outbox::command_failure::{
    CommandFailureOutbox, CommandFailureOutboxId,
};
use appletheia_application::outbox::{
    OutboxAttemptCount, OutboxLeaseExpiresAt, OutboxLifecycle, OutboxNextAttemptAt,
    OutboxPublishedAt, OutboxRelayInstance, OutboxState,
};
use appletheia_application::request_context::{CausationId, CorrelationId, MessageId};
use appletheia_application::saga::{
    SagaCommandOrigin, SagaInstanceId, SagaNameOwned, SerializedSagaStep,
};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use super::PgCommandFailureOutboxRowError;

/// Maps a PostgreSQL command-failure outbox row to application types.
#[derive(Clone, Debug, FromRow)]
pub struct PgCommandFailureOutboxRow {
    pub id: Uuid,
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
}

impl PgCommandFailureOutboxRow {
    pub fn try_into_outbox(self) -> Result<CommandFailureOutbox, PgCommandFailureOutboxRowError> {
        let id = CommandFailureOutboxId::try_from(self.id)?;
        let failure_id = CommandFailureId::try_from(self.failure_id)?;
        let command_name = CommandNameOwned::new(self.command_name)?;
        let saga_name = SagaNameOwned::new(self.saga_name)?;
        let saga_instance_id = SagaInstanceId::try_from(self.saga_instance_id)?;
        let terminal_reason = match self.terminal_reason.as_str() {
            "non_retryable" => CommandTerminalReason::NonRetryable,
            "retry_exhausted" => CommandTerminalReason::RetryExhausted,
            value => {
                return Err(PgCommandFailureOutboxRowError::TerminalReason(
                    value.to_owned(),
                ));
            }
        };
        let command_attempt_count = CommandAttemptCount::try_from(self.command_attempt_count)?;
        let failure = CommandFailureEnvelope {
            failure_id,
            command_message_id: MessageId::from(self.command_message_id),
            command_name,
            origin: SagaCommandOrigin {
                saga_name,
                saga_instance_id,
                step: SerializedSagaStep::try_from(self.saga_step)?,
            },
            terminal_reason,
            attempt_count: command_attempt_count,
            correlation_id: CorrelationId::from(self.correlation_id),
            causation_id: CausationId::from(MessageId::from(self.causation_id)),
            failed_at: self.failed_at.into(),
        };

        let attempt_count = OutboxAttemptCount::try_from(self.attempt_count)?;
        let next_attempt_after = OutboxNextAttemptAt::from(self.next_attempt_after);
        let lease_owner = self
            .lease_owner
            .map(|value| OutboxRelayInstance::from_str(&value))
            .transpose()?;
        let lease_until = self.lease_until.map(OutboxLeaseExpiresAt::from);
        let published_at = self.published_at.map(OutboxPublishedAt::from);
        let last_error = self
            .last_error
            .map(serde_json::from_value::<PublishDispatchError>)
            .transpose()?;
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
            (None, Some(_), None) => {
                return Err(PgCommandFailureOutboxRowError::InconsistentLeaseState);
            }
        };
        Ok(CommandFailureOutbox {
            id,
            sequence: self.failure_sequence,
            failure,
            state,
            last_error,
            lifecycle: OutboxLifecycle::Active,
        })
    }
}

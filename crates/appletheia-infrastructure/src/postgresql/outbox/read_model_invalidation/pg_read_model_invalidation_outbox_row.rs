use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::event::EventSequence;
use appletheia_application::messaging::PublishDispatchError;
use appletheia_application::outbox::read_model_invalidation::ReadModelInvalidationOutbox;
use appletheia_application::outbox::{
    OutboxAttemptCount, OutboxDeadLetteredAt, OutboxLeaseExpiresAt, OutboxLifecycle,
    OutboxNextAttemptAt, OutboxPublishedAt, OutboxRelayInstance, OutboxState,
};
use appletheia_application::projection::ProjectorNameOwned;
use appletheia_application::read_model::{
    ReadModelDependency, ReadModelInvalidationEnvelope, ReadModelInvalidationId,
};
use appletheia_application::request_context::{CausationId, CorrelationId, MessageId};
use appletheia_domain::{EventId, EventOccurredAt};

use super::PgReadModelInvalidationOutboxRowError;

/// Maps a PostgreSQL invalidation outbox row to application types.
#[derive(Clone, Debug, Eq, PartialEq, FromRow)]
pub struct PgReadModelInvalidationOutboxRow {
    pub id: Uuid,
    pub source_projector_name: String,
    pub source_event_sequence: i64,
    pub source_event_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub invalidated_dependencies: serde_json::Value,
    pub recorded_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub attempt_count: i64,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
    pub last_error: Option<serde_json::Value>,
    pub dead_lettered_at: Option<DateTime<Utc>>,
}

impl PgReadModelInvalidationOutboxRow {
    /// Validates and converts the storage row into a relayable outbox entry.
    pub fn try_into_outbox(
        self,
    ) -> Result<ReadModelInvalidationOutbox, PgReadModelInvalidationOutboxRowError> {
        let invalidation_id = ReadModelInvalidationId::try_from(self.id)?;
        let source_projector_name = ProjectorNameOwned::new(self.source_projector_name)?;
        let source_event_sequence = EventSequence::try_from(self.source_event_sequence)?;
        let source_event_id = EventId::try_from(self.source_event_id)?;
        let occurred_at = EventOccurredAt::from(self.occurred_at);
        let correlation_id = CorrelationId::from(self.correlation_id);
        let causation_id = CausationId::from(MessageId::from(self.causation_id));
        let invalidated_dependencies =
            serde_json::from_value::<Vec<ReadModelDependency>>(self.invalidated_dependencies)?;

        let invalidation =
            serde_json::from_value::<ReadModelInvalidationEnvelope>(serde_json::json!({
                "invalidation_id": invalidation_id,
                "source_projector_name": source_projector_name,
                "source_event_sequence": source_event_sequence,
                "source_event_id": source_event_id,
                "occurred_at": occurred_at,
                "correlation_id": correlation_id,
                "causation_id": causation_id,
                "invalidated_dependencies": invalidated_dependencies,
            }))?;

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
                return Err(PgReadModelInvalidationOutboxRowError::InconsistentLeaseState);
            }
        };

        let lifecycle = match self.dead_lettered_at {
            Some(dead_lettered_at) => OutboxLifecycle::DeadLettered {
                dead_lettered_at: OutboxDeadLetteredAt::from(dead_lettered_at),
            },
            None => OutboxLifecycle::Active,
        };

        Ok(ReadModelInvalidationOutbox {
            invalidation,
            state,
            last_error,
            lifecycle,
        })
    }
}

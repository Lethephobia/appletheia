use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::event::{AggregateIdValue, AggregateTypeOwned, EventSequence};
use appletheia_application::messaging::PublishDispatchError;
use appletheia_application::outbox::read_model_fragment_change::ReadModelFragmentChangeOutbox;
use appletheia_application::outbox::{
    OutboxAttemptCount, OutboxDeadLetteredAt, OutboxLeaseExpiresAt, OutboxLifecycle,
    OutboxNextAttemptAt, OutboxPublishedAt, OutboxRelayInstance, OutboxState,
};
use appletheia_application::projection::ProjectorNameOwned;
use appletheia_application::read_model::{
    ReadModelFragmentChangeEnvelope, ReadModelFragmentChangeId, SerializedPartition,
    SerializedReadModelFragmentChange,
};
use appletheia_application::request_context::{CausationId, CorrelationId, MessageId};
use appletheia_domain::{EventId, EventOccurredAt};

use super::PgReadModelFragmentChangeOutboxRowError;

/// Maps a PostgreSQL fragment-change outbox row to application types.
#[derive(Clone, Debug, Eq, PartialEq, FromRow)]
pub struct PgReadModelFragmentChangeOutboxRow {
    pub id: Uuid,
    pub partition: serde_json::Value,
    pub source_projector_name: String,
    pub source_event_sequence: i64,
    pub source_event_id: Uuid,
    pub source_aggregate_type: String,
    pub source_aggregate_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub changes: serde_json::Value,
    pub recorded_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub attempt_count: i64,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
    pub last_error: Option<serde_json::Value>,
    pub dead_lettered_at: Option<DateTime<Utc>>,
}

impl PgReadModelFragmentChangeOutboxRow {
    /// Validates and converts the storage row into a relayable outbox entry.
    pub fn try_into_outbox(
        self,
    ) -> Result<ReadModelFragmentChangeOutbox, PgReadModelFragmentChangeOutboxRowError> {
        let change_id = ReadModelFragmentChangeId::try_from(self.id)?;
        let partition = SerializedPartition::try_from(self.partition)?;
        let source_projector_name = ProjectorNameOwned::new(self.source_projector_name)?;
        let source_event_sequence = EventSequence::try_from(self.source_event_sequence)?;
        let source_event_id = EventId::try_from(self.source_event_id)?;
        let source_aggregate_type = AggregateTypeOwned::new(self.source_aggregate_type)?;
        let source_aggregate_id = AggregateIdValue::from(self.source_aggregate_id);
        let occurred_at = EventOccurredAt::from(self.occurred_at);
        let correlation_id = CorrelationId::from(self.correlation_id);
        let causation_id = CausationId::from(MessageId::from(self.causation_id));
        let changes =
            serde_json::from_value::<Vec<SerializedReadModelFragmentChange>>(self.changes)?;

        let change =
            serde_json::from_value::<ReadModelFragmentChangeEnvelope>(serde_json::json!({
                "change_id": change_id,
                "partition": partition,
                "source_projector_name": source_projector_name,
                "source_event_sequence": source_event_sequence,
                "source_event_id": source_event_id,
                "source_aggregate_type": source_aggregate_type,
                "source_aggregate_id": source_aggregate_id,
                "occurred_at": occurred_at,
                "correlation_id": correlation_id,
                "causation_id": causation_id,
                "changes": changes,
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
                return Err(PgReadModelFragmentChangeOutboxRowError::InconsistentLeaseState);
            }
        };

        let lifecycle = match self.dead_lettered_at {
            Some(dead_lettered_at) => OutboxLifecycle::DeadLettered {
                dead_lettered_at: OutboxDeadLetteredAt::from(dead_lettered_at),
            },
            None => OutboxLifecycle::Active,
        };

        Ok(ReadModelFragmentChangeOutbox {
            change,
            state,
            last_error,
            lifecycle,
        })
    }
}

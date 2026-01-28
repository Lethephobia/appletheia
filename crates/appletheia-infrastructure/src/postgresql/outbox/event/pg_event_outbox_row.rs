use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::event::{
    AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
    SerializedEventPayload,
};
use appletheia_application::outbox::{
    OutboxAttemptCount, OutboxLeaseExpiresAt, OutboxLifecycle,
    OutboxNextAttemptAt, OutboxPublishedAt, OutboxRelayInstance, OutboxState,
    event::{EventOutbox, EventOutboxId},
};
use appletheia_application::massaging::PublishDispatchError;
use appletheia_application::request_context::{
    CausationId, CorrelationId, MessageId, RequestContext,
};
use appletheia_domain::aggregate::AggregateVersion;
use appletheia_domain::event::{EventId, EventOccurredAt};

use super::pg_event_outbox_row_error::PgEventOutboxRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgEventOutboxRow {
    pub id: Uuid,
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
}

impl PgEventOutboxRow {
    pub fn try_into_outbox(self) -> Result<EventOutbox, PgEventOutboxRowError> {
        let id = EventOutboxId::try_from(self.id)?;
        let event_sequence = EventSequence::try_from(self.event_sequence)?;
        let event_id = EventId::try_from(self.event_id)?;

        let aggregate_type_string = self.aggregate_type;
        let aggregate_type = match AggregateTypeOwned::new(aggregate_type_string.clone()) {
            Ok(value) => value,
            Err(_) => return Err(PgEventOutboxRowError::AggregateType(aggregate_type_string)),
        };
        let aggregate_id = AggregateIdValue::from(self.aggregate_id);
        let aggregate_version = AggregateVersion::try_from(self.aggregate_version)?;
        let event_name_string = self.event_name;
        let event_name = match EventNameOwned::new(event_name_string.clone()) {
            Ok(value) => value,
            Err(_) => return Err(PgEventOutboxRowError::EventName(event_name_string)),
        };

        let payload = SerializedEventPayload::try_from(self.payload)?;

        let occurred_at = EventOccurredAt::from(self.occurred_at);

        let correlation_id = CorrelationId(self.correlation_id);
        let causation_message_id = MessageId::from(self.causation_id);
        let causation_id = CausationId::from(causation_message_id);
        let mut context = serde_json::from_value::<RequestContext>(self.context)?;
        context.correlation_id = correlation_id;
        context.message_id = causation_message_id;

        let attempt_count = OutboxAttemptCount::try_from(self.attempt_count)?;

        let next_attempt_after = OutboxNextAttemptAt::from(self.next_attempt_after);

        let lease_owner = match self.lease_owner {
            Some(owner) => {
                let parsed = OutboxRelayInstance::from_str(&owner)?;
                Some(parsed)
            }
            None => None,
        };

        let lease_until = self.lease_until.map(OutboxLeaseExpiresAt::from);

        let published_at = self.published_at.map(OutboxPublishedAt::from);

        let last_error = match self.last_error {
            Some(value) => {
                let deserialized = serde_json::from_value::<PublishDispatchError>(value)?;
                Some(deserialized)
            }
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
            (None, Some(_), None) => {
                return Err(PgEventOutboxRowError::InconsistentLeaseState);
            }
        };

        let event = EventEnvelope {
            event_sequence,
            event_id,
            aggregate_type,
            aggregate_id,
            aggregate_version,
            event_name,
            payload,
            occurred_at,
            correlation_id,
            causation_id,
            context,
        };

        Ok(EventOutbox {
            id,
            event,
            state,
            last_error,
            lifecycle: OutboxLifecycle::Active,
        })
    }
}

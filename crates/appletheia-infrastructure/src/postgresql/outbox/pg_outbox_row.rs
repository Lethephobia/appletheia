use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::event::{
    AggregateIdOwned, AggregateTypeOwned, EventPayloadOwned, EventSequence,
};
use appletheia_application::outbox::{
    Outbox, OutboxAttemptCount, OutboxId, OutboxLeaseExpiresAt, OutboxNextAttemptAt,
    OutboxPublishedAt, OutboxRelayInstance, OutboxState,
};
use appletheia_application::request_context::{CorrelationId, MessageId, RequestContext};
use appletheia_domain::aggregate::AggregateVersion;
use appletheia_domain::event::{EventId, EventOccurredAt};

use super::pg_outbox_row_error::PgOutboxRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgOutboxRow {
    pub id: Uuid,
    pub event_sequence: i64,
    pub event_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_version: i64,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub context: serde_json::Value,
    pub published_at: Option<DateTime<Utc>>,
    pub attempt_count: i32,
    pub next_attempt_after: DateTime<Utc>,
    pub lease_owner: Option<String>,
    pub lease_until: Option<DateTime<Utc>>,
}

impl PgOutboxRow {
    pub fn try_into_outbox(self) -> Result<Outbox, PgOutboxRowError> {
        let id = OutboxId::try_from(self.id)?;
        let event_sequence = EventSequence::try_from(self.event_sequence)?;
        let event_id = EventId::try_from(self.event_id)?;

        let aggregate_type = AggregateTypeOwned::try_from(self.aggregate_type)?;
        let aggregate_id = AggregateIdOwned::from(self.aggregate_id);
        let aggregate_version = AggregateVersion::try_from(self.aggregate_version)?;

        let payload = EventPayloadOwned::try_from(self.payload)?;

        let occurred_at = EventOccurredAt::from(self.occurred_at);

        let correlation_id = CorrelationId(self.correlation_id);
        let causation_id = MessageId::from(self.causation_id);
        let context: RequestContext = serde_json::from_value(self.context)?;

        let attempt_count_i64 = i64::from(self.attempt_count);
        let attempt_count = OutboxAttemptCount::try_from(attempt_count_i64)?;

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
                return Err(PgOutboxRowError::InconsistentLeaseState);
            }
        };

        Ok(Outbox {
            id,
            event_sequence,
            event_id,
            aggregate_type,
            aggregate_id,
            aggregate_version,
            payload,
            occurred_at,
            correlation_id,
            causation_id,
            context,
            state,
        })
    }
}

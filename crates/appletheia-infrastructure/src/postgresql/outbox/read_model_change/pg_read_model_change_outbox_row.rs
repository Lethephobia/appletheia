use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::messaging::PublishDispatchError;
use appletheia_application::outbox::{
    OutboxAttemptCount, OutboxLeaseExpiresAt, OutboxLifecycle, OutboxNextAttemptAt,
    OutboxPublishedAt, OutboxRelayInstance, OutboxState,
    read_model_change::{ReadModelChangeOutbox, ReadModelChangeOutboxId},
};
use appletheia_application::read_model::{
    ReadModelChange, ReadModelChangeEnvelope, ReadModelChangeId, ReadModelChangedAt, ReadModelRef,
};
use appletheia_domain::EventId;

use super::pg_read_model_change_outbox_row_error::PgReadModelChangeOutboxRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgReadModelChangeOutboxRow {
    pub id: Uuid,
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
}

impl PgReadModelChangeOutboxRow {
    pub fn try_into_outbox(self) -> Result<ReadModelChangeOutbox, PgReadModelChangeOutboxRowError> {
        let id = ReadModelChangeOutboxId::try_from(self.id)?;
        let change = ReadModelChangeEnvelope {
            change_id: ReadModelChangeId::try_from(self.change_id)?,
            source_event_id: EventId::try_from(self.source_event_id)?,
            changed_at: ReadModelChangedAt::from(self.changed_at),
            change: ReadModelChange::new(ReadModelRef {
                read_model_type: self.read_model_type.parse()?,
                id: self.read_model_id.into(),
            }),
        };

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
                return Err(PgReadModelChangeOutboxRowError::InconsistentLeaseState);
            }
        };

        Ok(ReadModelChangeOutbox {
            id,
            change,
            state,
            last_error,
            lifecycle: OutboxLifecycle::Active,
        })
    }
}

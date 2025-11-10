use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersion, Event, EventId, EventPayload, OccurredAt,
};

use super::pg_event_row_error::PgEventRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub(crate) struct PgEventRow {
    pub event_sequence: i64,
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_version: i64,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub recorded_at: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub context: serde_json::Value,
}

impl PgEventRow {
    pub fn to_event<A: Aggregate>(
        self,
    ) -> Result<Event<A::Id, A::EventPayload>, PgEventRowError<A::Id, A::EventPayload>> {
        let id = EventId::try_from(self.id).map_err(PgEventRowError::EventId)?;
        let aggregate_id =
            A::Id::try_from_uuid(self.aggregate_id).map_err(PgEventRowError::AggregateId)?;
        let aggregate_version = AggregateVersion::try_from(self.aggregate_version)
            .map_err(PgEventRowError::AggregateVersion)?;
        let payload = A::EventPayload::try_from_json_value(self.payload)
            .map_err(PgEventRowError::EventPayload)?;
        Ok(Event::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            payload,
            OccurredAt::from(self.occurred_at),
        ))
    }
}

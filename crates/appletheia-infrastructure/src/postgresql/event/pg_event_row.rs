use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::event::{
    AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
    SerializedEventPayload,
};
use appletheia_application::request_context::{
    CausationId, CorrelationId, MessageId, RequestContext,
};
use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersion, Event, EventId, EventOccurredAt, EventPayload,
};

use super::pg_event_row_error::PgEventRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgEventRow {
    pub event_sequence: i64,
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_version: i64,
    pub event_name: String,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub context: serde_json::Value,
}

impl PgEventRow {
    pub fn try_into_event<A: Aggregate>(
        self,
    ) -> Result<Event<A::Id, A::EventPayload>, PgEventRowError>
    where
        <A::Id as AggregateId>::Error: std::error::Error + Send + Sync + 'static,
        <A::EventPayload as EventPayload>::Error: std::error::Error + Send + Sync + 'static,
    {
        let id = EventId::try_from(self.id)?;
        let aggregate_id = A::Id::try_from_uuid(self.aggregate_id)
            .map_err(|source| PgEventRowError::AggregateId(Box::new(source)))?;
        let aggregate_version = AggregateVersion::try_from(self.aggregate_version)?;
        let payload = A::EventPayload::try_from_json_value(self.payload)
            .map_err(|source| PgEventRowError::EventPayload(Box::new(source)))?;
        Ok(Event::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            payload,
            EventOccurredAt::from(self.occurred_at),
        ))
    }

    pub fn try_into_event_envelope(self) -> Result<EventEnvelope, PgEventRowError> {
        let event_sequence = EventSequence::try_from(self.event_sequence)?;
        let event_id = EventId::try_from(self.id)?;

        let aggregate_type_string = self.aggregate_type;
        let aggregate_type = match AggregateTypeOwned::new(aggregate_type_string.clone()) {
            Ok(value) => value,
            Err(_) => return Err(PgEventRowError::AggregateType(aggregate_type_string)),
        };
        let aggregate_id = AggregateIdValue::from(self.aggregate_id);
        let aggregate_version = AggregateVersion::try_from(self.aggregate_version)?;

        let event_name_string = self.event_name;
        let event_name = match EventNameOwned::new(event_name_string.clone()) {
            Ok(value) => value,
            Err(_) => return Err(PgEventRowError::EventName(event_name_string)),
        };

        let payload = SerializedEventPayload::try_from(self.payload)?;
        let occurred_at = EventOccurredAt::from(self.occurred_at);

        let correlation_id = CorrelationId::from(self.correlation_id);
        let causation_message_id = MessageId::from(self.causation_id);
        let causation_id = CausationId::from(causation_message_id);

        let mut context = serde_json::from_value::<RequestContext>(self.context)?;
        context.correlation_id = correlation_id;
        context.message_id = causation_message_id;

        Ok(EventEnvelope {
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
        })
    }
}

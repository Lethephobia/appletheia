use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::event::{
    AggregateIdOwned, AggregateTypeOwned, AppEvent, EventPayloadOwned, EventSequence,
};
use appletheia_application::request_context::{
    CausationId, CorrelationId, MessageId, RequestContext,
};
use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersion, Event, EventId, EventOccurredAt, EventPayload,
};

use super::pg_event_row_app_event_error::PgEventRowAppEventError;
use super::pg_event_row_error::PgEventRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgEventRow {
    pub event_sequence: i64,
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_version: i64,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub context: serde_json::Value,
}

impl PgEventRow {
    pub fn try_into_event<A: Aggregate>(
        self,
    ) -> Result<Event<A::Id, A::EventPayload>, PgEventRowError<A>> {
        let id = EventId::try_from(self.id)?;
        let aggregate_id =
            A::Id::try_from_uuid(self.aggregate_id).map_err(PgEventRowError::AggregateId)?;
        let aggregate_version = AggregateVersion::try_from(self.aggregate_version)?;
        let payload = A::EventPayload::try_from_json_value(self.payload)
            .map_err(PgEventRowError::EventPayload)?;
        Ok(Event::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            payload,
            EventOccurredAt::from(self.occurred_at),
        ))
    }

    pub fn try_into_app_event(self) -> Result<AppEvent, PgEventRowAppEventError> {
        let event_sequence = EventSequence::try_from(self.event_sequence)?;
        let event_id = EventId::try_from(self.id)?;

        let aggregate_type = AggregateTypeOwned::try_from(self.aggregate_type)?;
        let aggregate_id = AggregateIdOwned::from(self.aggregate_id);
        let aggregate_version = AggregateVersion::try_from(self.aggregate_version)?;

        let payload = EventPayloadOwned::try_from(self.payload)?;
        let occurred_at = EventOccurredAt::from(self.occurred_at);

        let correlation_id = CorrelationId(self.correlation_id);
        let causation_message_id = MessageId::from(self.causation_id);
        let causation_id = CausationId::from(causation_message_id);

        let mut context = serde_json::from_value::<RequestContext>(self.context)?;
        context.correlation_id = correlation_id;
        context.message_id = causation_message_id;

        Ok(AppEvent {
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
        })
    }
}

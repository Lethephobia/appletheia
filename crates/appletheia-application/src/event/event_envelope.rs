use serde::{Deserialize, Serialize};

use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersion, Event, EventId, EventOccurredAt, EventPayload,
};

use crate::event::{AggregateIdValue, AggregateTypeOwned, EventSequence, SerializedEventPayload};
use crate::request_context::{CausationId, CorrelationId, RequestContext};

use super::EventEnvelopeError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_sequence: EventSequence,
    pub event_id: EventId,
    pub aggregate_type: AggregateTypeOwned,
    pub aggregate_id: AggregateIdValue,
    pub aggregate_version: AggregateVersion,
    pub payload: SerializedEventPayload,
    pub occurred_at: EventOccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub context: RequestContext,
}

impl EventEnvelope {
    pub fn try_into_domain_event<A>(
        &self,
    ) -> Result<Event<A::Id, A::EventPayload>, EventEnvelopeError>
    where
        A: Aggregate,
        <A::Id as AggregateId>::Error: std::error::Error + Send + Sync + 'static,
        <A::EventPayload as EventPayload>::Error: std::error::Error + Send + Sync + 'static,
    {
        if self.aggregate_type.value() != A::TYPE.value() {
            return Err(EventEnvelopeError::AggregateTypeMismatch {
                expected: A::TYPE.value(),
                actual: self.aggregate_type.value().to_owned(),
            });
        }

        let aggregate_id = A::Id::try_from_uuid(self.aggregate_id.value())
            .map_err(|source| EventEnvelopeError::AggregateId(Box::new(source)))?;

        let payload = A::EventPayload::try_from_json_value(self.payload.value().clone())
            .map_err(|source| EventEnvelopeError::EventPayload(Box::new(source)))?;

        Ok(Event::from_persisted(
            self.event_id,
            aggregate_id,
            self.aggregate_version,
            payload,
            self.occurred_at,
        ))
    }
}

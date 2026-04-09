use serde::{Deserialize, Serialize};

use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersion, Event, EventId, EventOccurredAt, EventPayload,
};

use crate::event::{
    AggregateIdValue, AggregateTypeOwned, EventNameOwned, EventSequence, SerializedEventPayload,
};
use crate::request_context::{CausationId, CorrelationId, RequestContext};

use super::EventEnvelopeError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_sequence: EventSequence,
    pub event_id: EventId,
    pub aggregate_type: AggregateTypeOwned,
    pub aggregate_id: AggregateIdValue,
    pub aggregate_version: AggregateVersion,
    pub event_name: EventNameOwned,
    pub payload: SerializedEventPayload,
    pub occurred_at: EventOccurredAt,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
    pub context: RequestContext,
}

impl EventEnvelope {
    pub fn is_for_aggregate<A>(&self) -> bool
    where
        A: Aggregate,
    {
        self.aggregate_type.value() == A::TYPE.value()
    }

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

#[cfg(test)]
mod tests {
    use std::fmt::{self, Display};

    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    use super::*;
    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventNameOwned, EventSequence, SerializedEventPayload,
    };
    use crate::request_context::{MessageId, Principal};
    use appletheia_domain::{
        AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState,
        AggregateStateError, AggregateType, EventName, UniqueConstraints,
    };

    #[derive(Debug, Error)]
    enum CounterIdError {
        #[error("nil uuid is not allowed")]
        NilUuid,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    struct CounterId(Uuid);

    impl AggregateId for CounterId {
        type Error = CounterIdError;

        fn value(&self) -> Uuid {
            self.0
        }

        fn try_from_uuid(value: Uuid) -> Result<Self, Self::Error> {
            if value.is_nil() {
                return Err(CounterIdError::NilUuid);
            }

            Ok(Self(value))
        }
    }

    impl Display for CounterId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Display::fmt(&self.0, f)
        }
    }

    #[derive(Debug, Error)]
    enum CounterStateError {
        #[error(transparent)]
        AggregateState(#[from] AggregateStateError),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct CounterState {
        id: CounterId,
    }

    impl UniqueConstraints<CounterStateError> for CounterState {}

    impl AggregateState for CounterState {
        type Id = CounterId;
        type Error = CounterStateError;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    #[derive(Debug, Error)]
    enum CounterEventPayloadError {
        #[error(transparent)]
        Serde(#[from] serde_json::Error),
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "snake_case")]
    enum CounterEventPayload {
        Opened,
    }

    impl EventPayload for CounterEventPayload {
        type Error = CounterEventPayloadError;

        fn name(&self) -> EventName {
            match self {
                Self::Opened => EventName::new("opened"),
            }
        }
    }

    #[derive(Debug, Error)]
    enum CounterError {
        #[error(transparent)]
        Aggregate(#[from] AggregateError<CounterId>),
    }

    #[derive(Clone, Debug, Default)]
    struct Counter {
        core: AggregateCore<CounterState, CounterEventPayload>,
    }

    impl AggregateApply<CounterEventPayload, CounterError> for Counter {
        fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
            match payload {
                CounterEventPayload::Opened => {
                    self.set_state(Some(CounterState {
                        id: CounterId::try_from_uuid(Uuid::now_v7())
                            .expect("generated uuid should be valid"),
                    }));
                }
            }

            Ok(())
        }
    }

    impl Aggregate for Counter {
        type Id = CounterId;
        type State = CounterState;
        type EventPayload = CounterEventPayload;
        type Error = CounterError;

        const TYPE: AggregateType = AggregateType::new("counter");

        fn core(&self) -> &AggregateCore<Self::State, Self::EventPayload> {
            &self.core
        }

        fn core_mut(&mut self) -> &mut AggregateCore<Self::State, Self::EventPayload> {
            &mut self.core
        }
    }

    #[derive(Clone, Debug, Default)]
    struct OtherCounter;

    impl AggregateApply<CounterEventPayload, CounterError> for OtherCounter {
        fn apply(&mut self, _payload: &CounterEventPayload) -> Result<(), CounterError> {
            Ok(())
        }
    }

    impl Aggregate for OtherCounter {
        type Id = CounterId;
        type State = CounterState;
        type EventPayload = CounterEventPayload;
        type Error = CounterError;

        const TYPE: AggregateType = AggregateType::new("other_counter");

        fn core(&self) -> &AggregateCore<Self::State, Self::EventPayload> {
            panic!("test aggregate should not access core")
        }

        fn core_mut(&mut self) -> &mut AggregateCore<Self::State, Self::EventPayload> {
            panic!("test aggregate should not access core")
        }
    }

    fn event_envelope() -> EventEnvelope {
        let payload = CounterEventPayload::Opened;
        let message_id = MessageId::new();

        EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::from(Counter::TYPE),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1).expect("version should be valid"),
            event_name: EventNameOwned::from(payload.name()),
            payload: SerializedEventPayload::try_from(
                payload.into_json_value().expect("payload should serialize"),
            )
            .expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id: CorrelationId::from(message_id.value()),
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(
                CorrelationId::from(MessageId::new().value()),
                MessageId::new(),
                Principal::System,
            )
            .expect("request context should be valid"),
        }
    }

    #[test]
    fn is_for_aggregate_returns_true_for_matching_aggregate_type() {
        let event = event_envelope();

        assert!(event.is_for_aggregate::<Counter>());
    }

    #[test]
    fn is_for_aggregate_returns_false_for_different_aggregate_type() {
        let event = event_envelope();

        assert!(!event.is_for_aggregate::<OtherCounter>());
    }
}

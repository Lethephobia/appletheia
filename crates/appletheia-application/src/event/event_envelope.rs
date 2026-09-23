use appletheia_domain::{
    Aggregate, AggregateId, AggregateVersion, Event, EventId, EventOccurredAt, EventPayload,
};
use serde::{Deserialize, Serialize};

use crate::aggregate::{AggregateIdValue, AggregateTypeOwned};
use crate::event::{EventNameOwned, EventSequence, SerializedEventPayload};
use crate::messaging::{
    CloudEvent, CloudEventSource, CloudEventTypePrefix, EventCloudEventCodec, PublishableMessage,
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

    pub fn try_to_domain_event<A>(
        &self,
    ) -> Result<Event<A::Id, A::EventPayload>, EventEnvelopeError>
    where
        A: Aggregate,
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

        if payload.name().value() != self.event_name.value() {
            return Err(EventEnvelopeError::EventNameMismatch {
                expected: self.event_name.value().to_owned(),
                actual: payload.name().value().to_owned(),
            });
        }

        Ok(Event::from_persisted(
            self.event_id,
            aggregate_id,
            self.aggregate_version,
            payload,
            self.occurred_at,
        ))
    }
}

impl PublishableMessage for EventEnvelope {
    type Error = EventEnvelopeError;

    fn try_to_cloud_event(
        &self,
        source: &CloudEventSource,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<CloudEvent, Self::Error> {
        Ok(EventCloudEventCodec::encode(self, source, type_prefix)?)
    }

    fn try_from_cloud_event(
        event: &CloudEvent,
        type_prefix: Option<&CloudEventTypePrefix>,
    ) -> Result<Self, Self::Error> {
        Ok(EventCloudEventCodec::decode(event, type_prefix)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::messaging::CloudEventAttributeValue;
    use std::fmt::{self, Display};

    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::Uuid;

    use super::*;
    use crate::aggregate::{AggregateIdValue, AggregateTypeOwned};
    use crate::event::{EventNameOwned, EventSequence, SerializedEventPayload};
    use crate::messaging::{CloudEventData, CloudEventDataContentType};
    use crate::request_context::{MessageId, Principal};
    use appletheia_domain::{
        AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState,
        AggregateStateError, AggregateType, EventName, ReferenceIndexes, UniqueConstraints,
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

        fn new() -> Self {
            Self(Uuid::now_v7())
        }

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
    impl ReferenceIndexes<CounterStateError> for CounterState {}

    impl AggregateState for CounterState {
        type Error = CounterStateError;
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

        #[error(transparent)]
        State(#[from] CounterStateError),
    }

    #[derive(Clone, Debug, Default)]
    struct Counter {
        core: AggregateCore<CounterId, CounterState, CounterEventPayload>,
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

        fn new() -> Self {
            Self {
                core: AggregateCore::new(),
            }
        }

        fn from_id(id: Self::Id) -> Self {
            Self {
                core: AggregateCore::from_id(id),
            }
        }

        fn core(&self) -> &AggregateCore<Self::Id, Self::State, Self::EventPayload> {
            &self.core
        }

        fn core_mut(&mut self) -> &mut AggregateCore<Self::Id, Self::State, Self::EventPayload> {
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

        fn new() -> Self {
            Self
        }

        fn from_id(_id: Self::Id) -> Self {
            Self
        }

        fn core(&self) -> &AggregateCore<Self::Id, Self::State, Self::EventPayload> {
            panic!("test aggregate should not access core")
        }

        fn core_mut(&mut self) -> &mut AggregateCore<Self::Id, Self::State, Self::EventPayload> {
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
                payload
                    .try_into_json_value()
                    .expect("payload should serialize"),
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

    #[test]
    fn try_to_domain_event_preserves_matching_event() {
        let envelope = event_envelope();

        let event = envelope
            .try_to_domain_event::<Counter>()
            .expect("valid event");

        assert_eq!(event.payload().name().value(), envelope.event_name.value());
    }

    #[test]
    fn try_to_domain_event_rejects_mismatched_event_name() {
        let mut envelope = event_envelope();
        let payload_name = envelope.event_name.value().to_owned();
        envelope.event_name = EventNameOwned::from(EventName::new("different_event"));

        assert!(matches!(
            envelope.try_to_domain_event::<Counter>(),
            Err(EventEnvelopeError::EventNameMismatch { expected, actual })
                if expected == "different_event" && actual == payload_name
        ));
    }

    #[test]
    fn cloud_event_round_trip_preserves_metadata_and_rejects_invalid_values() {
        let mut envelope = event_envelope();
        envelope.event_sequence = EventSequence::try_from(i64::MAX).unwrap();
        envelope.aggregate_version = AggregateVersion::try_from(i64::MAX).unwrap();
        let source = "urn:banking:events".parse().unwrap();
        let prefix = "example.banking".parse().unwrap();
        let event = envelope.try_to_cloud_event(&source, Some(&prefix)).unwrap();
        assert_eq!(
            event.event_type().as_str(),
            "example.banking.counter.opened"
        );
        assert_eq!(event.id().as_str(), envelope.event_id.to_string());
        let restored = EventEnvelope::try_from_cloud_event(&event, Some(&prefix)).unwrap();
        assert_eq!(
            serde_json::to_value(&restored).unwrap(),
            serde_json::to_value(&envelope).unwrap()
        );
        assert_eq!(restored.context.principal, Principal::Unavailable);
        assert!(
            EventEnvelope::try_from_cloud_event(&event, Some(&"wrong".parse().unwrap())).is_err()
        );
        let wrong_subject = event.clone().with_subject(
            "other/00000000-0000-0000-0000-000000000000"
                .parse()
                .unwrap(),
        );
        assert!(EventEnvelope::try_from_cloud_event(&wrong_subject, Some(&prefix)).is_err());
        let mut invalid_sequence = event.clone();
        invalid_sequence
            .insert_extension(
                "eventsequence".parse().unwrap(),
                CloudEventAttributeValue::String(("-1".to_owned()).parse().unwrap()),
            )
            .unwrap();
        assert!(EventEnvelope::try_from_cloud_event(&invalid_sequence, Some(&prefix)).is_err());
        let mut missing_context = event.clone();
        missing_context.remove_extension(&"context".parse().unwrap());
        assert!(EventEnvelope::try_from_cloud_event(&missing_context, Some(&prefix)).is_err());
        let null_payload = event
            .try_with_data(
                Some(CloudEventData::Json(serde_json::Value::Null)),
                Some(CloudEventDataContentType::json()),
            )
            .unwrap();
        assert!(EventEnvelope::try_from_cloud_event(&null_payload, Some(&prefix)).is_err());
    }
}

pub mod event_id;
pub mod event_id_error;
pub mod event_name;
pub mod event_occurred_at;
pub mod event_payload;

pub use event_id::EventId;
pub use event_id_error::EventIdError;
pub use event_name::EventName;
pub use event_occurred_at::EventOccurredAt;
pub use event_payload::EventPayload;

use crate::aggregate::{AggregateId, AggregateVersion};

/// Represents a persisted or newly produced domain event.
///
/// An event carries its own identifier, the target aggregate identifier, the
/// aggregate version at which it was produced, the domain payload, and the
/// timestamp at which it occurred.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Event<I: AggregateId, P: EventPayload> {
    id: EventId,
    aggregate_id: I,
    aggregate_version: AggregateVersion,
    payload: P,
    occurred_at: EventOccurredAt,
}

impl<I: AggregateId, P: EventPayload> Event<I, P> {
    /// Creates a new event with a fresh event ID and the current timestamp.
    pub fn new(aggregate_id: I, aggregate_version: AggregateVersion, payload: P) -> Self {
        Self {
            id: EventId::new(),
            aggregate_id,
            aggregate_version,
            payload,
            occurred_at: EventOccurredAt::now(),
        }
    }

    /// Rebuilds an event from already persisted values.
    pub fn from_persisted(
        id: EventId,
        aggregate_id: I,
        aggregate_version: AggregateVersion,
        payload: P,
        occurred_at: EventOccurredAt,
    ) -> Self {
        Self {
            id,
            aggregate_id,
            aggregate_version,
            payload,
            occurred_at,
        }
    }

    /// Returns the event identifier.
    pub fn id(&self) -> EventId {
        self.id
    }

    /// Returns the identifier of the aggregate that produced the event.
    pub fn aggregate_id(&self) -> I {
        self.aggregate_id
    }

    /// Returns the aggregate version carried by the event.
    pub fn aggregate_version(&self) -> AggregateVersion {
        self.aggregate_version
    }

    /// Returns the event payload.
    pub fn payload(&self) -> &P {
        &self.payload
    }

    /// Returns the timestamp at which the event occurred.
    pub fn occurred_at(&self) -> EventOccurredAt {
        self.occurred_at
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;
    use uuid::{Uuid, Version};

    use super::{Event, EventId, EventOccurredAt};
    use crate::aggregate::{AggregateId, AggregateVersion};
    use crate::event::{EventName, EventPayload};

    #[derive(Debug, Error, Eq, PartialEq)]
    enum CounterIdError {
        #[error("nil uuid is not allowed")]
        NilUuid,
    }

    fn validate_counter_id(value: Uuid) -> Result<(), CounterIdError> {
        if value.is_nil() {
            return Err(CounterIdError::NilUuid);
        }

        Ok(())
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
            validate_counter_id(value)?;
            Ok(Self(value))
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
        Incremented { amount: i32 },
    }

    impl EventPayload for CounterEventPayload {
        type Error = CounterEventPayloadError;

        fn name(&self) -> EventName {
            match self {
                Self::Opened => EventName::new("opened"),
                Self::Incremented { .. } => EventName::new("incremented"),
            }
        }
    }

    #[test]
    fn new_creates_event_with_generated_id_and_timestamp() {
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let aggregate_version = AggregateVersion::try_from(3).expect("version should be valid");
        let payload = CounterEventPayload::Incremented { amount: 2 };
        let before = Utc::now();

        let event = Event::new(aggregate_id, aggregate_version, payload.clone());

        let after = Utc::now();
        assert_eq!(event.id().value().get_version(), Some(Version::SortRand));
        assert_eq!(event.aggregate_id(), aggregate_id);
        assert_eq!(event.aggregate_version(), aggregate_version);
        assert_eq!(event.payload(), &payload);
        assert!(event.occurred_at().value() >= before);
        assert!(event.occurred_at().value() <= after);
    }

    #[test]
    fn from_persisted_preserves_all_fields() {
        let id = EventId::try_from(Uuid::now_v7()).expect("uuidv7 should be accepted");
        let aggregate_id =
            CounterId::try_from_uuid(Uuid::now_v7()).expect("valid uuid should be accepted");
        let aggregate_version = AggregateVersion::try_from(7).expect("version should be valid");
        let payload = CounterEventPayload::Opened;
        let occurred_at = EventOccurredAt::from(Utc::now());

        let event = Event::from_persisted(
            id,
            aggregate_id,
            aggregate_version,
            payload.clone(),
            occurred_at,
        );

        assert_eq!(event.id(), id);
        assert_eq!(event.aggregate_id(), aggregate_id);
        assert_eq!(event.aggregate_version(), aggregate_version);
        assert_eq!(event.payload(), &payload);
        assert_eq!(event.occurred_at(), occurred_at);
    }
}

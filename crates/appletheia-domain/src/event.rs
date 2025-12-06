pub mod event_id;
pub mod event_id_error;
pub mod event_occurred_at;
pub mod event_payload;

pub use event_id::EventId;
pub use event_id_error::EventIdError;
pub use event_occurred_at::EventOccurredAt;
pub use event_payload::EventPayload;

use crate::aggregate::{AggregateId, AggregateVersion};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Event<I: AggregateId, P: EventPayload> {
    id: EventId,
    aggregate_id: I,
    aggregate_version: AggregateVersion,
    payload: P,
    occurred_at: EventOccurredAt,
}

impl<I: AggregateId, P: EventPayload> Event<I, P> {
    pub fn new(aggregate_id: I, aggregate_version: AggregateVersion, payload: P) -> Self {
        Self {
            id: EventId::new(),
            aggregate_id,
            aggregate_version,
            payload,
            occurred_at: EventOccurredAt::now(),
        }
    }

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

    pub fn id(&self) -> EventId {
        self.id
    }

    pub fn aggregate_id(&self) -> I {
        self.aggregate_id
    }

    pub fn aggregate_version(&self) -> AggregateVersion {
        self.aggregate_version
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }

    pub fn occurred_at(&self) -> EventOccurredAt {
        self.occurred_at
    }
}

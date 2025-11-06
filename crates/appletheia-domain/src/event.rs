pub mod event_id;
pub mod event_id_error;
pub mod event_payload;
pub mod occurred_at;

pub use event_id::EventId;
pub use event_id_error::EventIdError;
pub use event_payload::EventPayload;
pub use occurred_at::OccurredAt;

use crate::aggregate::{AggregateId, AggregateVersion};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Event<A: AggregateId, P: EventPayload> {
    id: EventId,
    aggregate_id: A,
    aggregate_version: AggregateVersion,
    payload: P,
    occurred_at: OccurredAt,
}

impl<A: AggregateId, P: EventPayload> Event<A, P> {
    pub fn new(aggregate_id: A, aggregate_version: AggregateVersion, payload: P) -> Self {
        Self {
            id: EventId::new(),
            aggregate_id,
            aggregate_version,
            payload,
            occurred_at: OccurredAt::now(),
        }
    }

    pub fn from_persisted(
        id: EventId,
        aggregate_id: A,
        aggregate_version: AggregateVersion,
        payload: P,
        occurred_at: OccurredAt,
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

    pub fn aggregate_id(&self) -> A {
        self.aggregate_id
    }

    pub fn aggregate_version(&self) -> AggregateVersion {
        self.aggregate_version
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }

    pub fn occurred_at(&self) -> OccurredAt {
        self.occurred_at
    }
}

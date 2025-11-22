pub mod event_id;
pub mod event_id_error;
pub mod event_payload;
pub mod event_reader;
pub mod event_reader_error;
pub mod event_reader_provider;
pub mod occurred_at;

pub use event_id::EventId;
pub use event_id_error::EventIdError;
pub use event_payload::EventPayload;
pub use event_reader::EventReader;
pub use event_reader_error::EventReaderError;
pub use event_reader_provider::EventReaderProvider;
pub use occurred_at::OccurredAt;

use crate::aggregate::{AggregateId, AggregateVersion};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Event<I: AggregateId, P: EventPayload> {
    id: EventId,
    aggregate_id: I,
    aggregate_version: AggregateVersion,
    payload: P,
    occurred_at: OccurredAt,
}

impl<I: AggregateId, P: EventPayload> Event<I, P> {
    pub fn new(aggregate_id: I, aggregate_version: AggregateVersion, payload: P) -> Self {
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
        aggregate_id: I,
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

    pub fn aggregate_id(&self) -> I {
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

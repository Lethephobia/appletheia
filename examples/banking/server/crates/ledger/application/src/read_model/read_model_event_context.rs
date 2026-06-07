use appletheia::application::event::{EventEnvelope, EventSequence};
use appletheia::domain::{EventId, EventOccurredAt};

/// Event context used to update a read model row from a projected event.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ReadModelEventContext {
    pub event_id: EventId,
    pub event_sequence: EventSequence,
    pub occurred_at: EventOccurredAt,
}

impl ReadModelEventContext {
    pub fn new(
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Self {
        Self {
            event_id,
            event_sequence,
            occurred_at,
        }
    }
}

impl From<EventEnvelope> for ReadModelEventContext {
    fn from(event: EventEnvelope) -> Self {
        Self::new(event.event_id, event.event_sequence, event.occurred_at)
    }
}

impl From<&EventEnvelope> for ReadModelEventContext {
    fn from(event: &EventEnvelope) -> Self {
        Self::new(event.event_id, event.event_sequence, event.occurred_at)
    }
}

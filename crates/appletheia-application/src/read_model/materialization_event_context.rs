use appletheia_domain::{EventId, EventOccurredAt};

use crate::event::{EventEnvelope, EventSequence};

/// Carries source event coordinates used to order guarded materialization writes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MaterializationEventContext {
    pub event_id: EventId,
    pub event_sequence: EventSequence,
    pub occurred_at: EventOccurredAt,
}

impl MaterializationEventContext {
    /// Creates a materialization context from source event coordinates.
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

impl From<EventEnvelope> for MaterializationEventContext {
    fn from(event: EventEnvelope) -> Self {
        Self::new(event.event_id, event.event_sequence, event.occurred_at)
    }
}

impl From<&EventEnvelope> for MaterializationEventContext {
    fn from(event: &EventEnvelope) -> Self {
        Self::new(event.event_id, event.event_sequence, event.occurred_at)
    }
}

use appletheia_domain::{AggregateType, EventName};

use crate::event::EventEnvelope;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EventSelector {
    pub aggregate_type: AggregateType,
    pub event_name: EventName,
}

impl EventSelector {
    pub const fn new(aggregate_type: AggregateType, event_name: EventName) -> Self {
        Self {
            aggregate_type,
            event_name,
        }
    }

    pub fn matches(&self, event: &EventEnvelope) -> bool {
        event.aggregate_type.value() == self.aggregate_type.value()
            && event.event_name.value() == self.event_name.value()
    }
}


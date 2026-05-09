use appletheia_domain::{Aggregate, AggregateType, EventName};

use crate::event::EventEnvelope;
use crate::messaging::Selector as MessageSelector;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EventSelector {
    pub aggregate_type: AggregateType,
    pub event_name: EventName,
}

impl EventSelector {
    pub const fn new<A: Aggregate>(event_name: EventName) -> Self {
        Self {
            aggregate_type: A::TYPE,
            event_name,
        }
    }

    pub const fn from_parts(aggregate_type: AggregateType, event_name: EventName) -> Self {
        Self {
            aggregate_type,
            event_name,
        }
    }

    pub const fn is_same_as(&self, other: &Self) -> bool {
        Self::str_eq(self.aggregate_type.value(), other.aggregate_type.value())
            && Self::str_eq(self.event_name.value(), other.event_name.value())
    }

    pub fn matches(&self, event: &EventEnvelope) -> bool {
        event.aggregate_type.value() == self.aggregate_type.value()
            && event.event_name.value() == self.event_name.value()
    }

    const fn str_eq(left: &str, right: &str) -> bool {
        let left = left.as_bytes();
        let right = right.as_bytes();
        if left.len() != right.len() {
            return false;
        }

        let mut i = 0;
        while i < left.len() {
            if left[i] != right[i] {
                return false;
            }
            i += 1;
        }

        true
    }
}

impl MessageSelector<EventEnvelope> for EventSelector {
    fn matches(&self, message: &EventEnvelope) -> bool {
        message.aggregate_type.value() == self.aggregate_type.value()
            && message.event_name.value() == self.event_name.value()
    }
}

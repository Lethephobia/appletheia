use crate::event::{Event, EventPayload};

use super::AggregateId;

pub trait AggregateUncommittedEvents<I: AggregateId, E: EventPayload> {
    fn uncommitted_events(&self) -> &[Event<I, E>];
    fn record_uncommitted_event(&mut self, event: Event<I, E>);
    fn clear_uncommitted_events(&mut self);
}

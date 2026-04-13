use crate::event::EventSelector;

use super::{SagaName, SagaPredecessor};

/// Describes a saga's identity and source event.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SagaDescriptor {
    pub name: SagaName,
    pub trigger_event: EventSelector,
    pub predecessor: SagaPredecessor,
}

impl SagaDescriptor {
    /// Creates a new saga descriptor.
    pub const fn new(
        name: SagaName,
        trigger_event: EventSelector,
        predecessor: SagaPredecessor,
    ) -> Self {
        Self {
            name,
            trigger_event,
            predecessor,
        }
    }
}

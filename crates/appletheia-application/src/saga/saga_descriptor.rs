use crate::event::EventSelector;
use crate::messaging::Subscription;

use super::{SagaName, SagaStartEvents};

/// Describes a saga's identity and subscribed events.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SagaDescriptor {
    pub name: SagaName,
    pub start_events: SagaStartEvents,
    pub subscription: Subscription<'static, EventSelector>,
}

impl SagaDescriptor {
    /// Creates a new saga descriptor.
    pub const fn new(
        name: SagaName,
        start_events: SagaStartEvents,
        subscription: Subscription<'static, EventSelector>,
    ) -> Self {
        if !start_events.is_included_in_subscription(subscription) {
            panic!("saga start events must be included in subscription");
        }

        Self {
            name,
            start_events,
            subscription,
        }
    }
}

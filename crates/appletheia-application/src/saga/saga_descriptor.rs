use crate::event::EventSelector;
use crate::messaging::Subscription;

use super::SagaName;

/// Describes a saga's identity and subscribed events.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SagaDescriptor {
    pub name: SagaName,
    pub start_event: EventSelector,
    pub subscription: Subscription<'static, EventSelector>,
}

impl SagaDescriptor {
    /// Creates a new saga descriptor.
    pub const fn new(
        name: SagaName,
        start_event: EventSelector,
        subscription: Subscription<'static, EventSelector>,
    ) -> Self {
        if !Self::subscription_contains_start_event(subscription, start_event) {
            panic!("saga start event must be included in subscription");
        }

        Self {
            name,
            start_event,
            subscription,
        }
    }

    const fn subscription_contains_start_event(
        subscription: Subscription<'static, EventSelector>,
        start_event: EventSelector,
    ) -> bool {
        match subscription {
            Subscription::All => true,
            Subscription::One(selector) => selector.is_same_as(&start_event),
            Subscription::AnyOf(selectors) => {
                let mut i = 0;
                while i < selectors.len() {
                    if selectors[i].is_same_as(&start_event) {
                        return true;
                    }
                    i += 1;
                }
                false
            }
        }
    }
}

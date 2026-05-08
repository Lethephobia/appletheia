use crate::event::{EventEnvelope, EventSelector};
use crate::messaging::Subscription;

/// Declares the event selectors that can start a saga instance.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SagaStartEvents(&'static [EventSelector]);

impl SagaStartEvents {
    /// Creates saga start events from one or more selectors.
    pub const fn new(selectors: &'static [EventSelector]) -> Self {
        if selectors.is_empty() {
            panic!("saga start events must not be empty");
        }

        Self(selectors)
    }

    /// Returns whether the event starts a saga instance.
    pub fn matches(&self, event: &EventEnvelope) -> bool {
        self.0.iter().any(|selector| selector.matches(event))
    }

    pub(crate) const fn is_included_in_subscription(
        &self,
        subscription: Subscription<'static, EventSelector>,
    ) -> bool {
        let mut i = 0;
        while i < self.0.len() {
            if !Self::selector_is_included_in_subscription(self.0[i], subscription) {
                return false;
            }
            i += 1;
        }
        true
    }

    const fn selector_is_included_in_subscription(
        selector: EventSelector,
        subscription: Subscription<'static, EventSelector>,
    ) -> bool {
        match subscription {
            Subscription::All => true,
            Subscription::One(subscription_selector) => subscription_selector.is_same_as(&selector),
            Subscription::AnyOf(subscription_selectors) => {
                let mut i = 0;
                while i < subscription_selectors.len() {
                    if subscription_selectors[i].is_same_as(&selector) {
                        return true;
                    }
                    i += 1;
                }
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use appletheia_domain::{AggregateType, EventName};

    use super::SagaStartEvents;
    use crate::event::EventSelector;
    use crate::messaging::Subscription;

    const USER_DELETED: EventSelector =
        EventSelector::new(AggregateType::new("user"), EventName::new("user_deleted"));
    const ORGANIZATION_DELETED: EventSelector = EventSelector::new(
        AggregateType::new("organization"),
        EventName::new("organization_deleted"),
    );
    const ACCOUNT_CLOSED: EventSelector = EventSelector::new(
        AggregateType::new("account"),
        EventName::new("account_closed"),
    );

    #[test]
    fn single_selector_is_included_when_subscription_contains_selector() {
        let start_events = SagaStartEvents::new(&[USER_DELETED]);

        assert!(start_events.is_included_in_subscription(Subscription::One(&USER_DELETED)));
    }

    #[test]
    fn any_of_requires_all_selectors_to_be_in_subscription() {
        let start_events = SagaStartEvents::new(&[USER_DELETED, ORGANIZATION_DELETED]);

        assert!(
            start_events.is_included_in_subscription(Subscription::AnyOf(&[
                USER_DELETED,
                ORGANIZATION_DELETED,
                ACCOUNT_CLOSED,
            ]))
        );
        assert!(
            !start_events
                .is_included_in_subscription(Subscription::AnyOf(&[USER_DELETED, ACCOUNT_CLOSED]))
        );
    }

    #[test]
    #[should_panic(expected = "saga start events must not be empty")]
    fn new_rejects_empty_selector_list() {
        let _ = SagaStartEvents::new(&[]);
    }
}

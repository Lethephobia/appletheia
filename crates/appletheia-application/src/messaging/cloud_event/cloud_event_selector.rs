use crate::messaging::Selector;

use super::{CloudEvent, CloudEventSource, CloudEventSubject, CloudEventType};

/// Matches an event type and, when specified, its source and subject.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CloudEventSelector {
    pub event_type: CloudEventType,
    pub source: Option<CloudEventSource>,
    pub subject: Option<CloudEventSubject>,
}

impl CloudEventSelector {
    pub fn new(event_type: CloudEventType) -> Self {
        Self {
            event_type,
            source: None,
            subject: None,
        }
    }

    pub fn with_source(mut self, source: CloudEventSource) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_subject(mut self, subject: CloudEventSubject) -> Self {
        self.subject = Some(subject);
        self
    }
}

impl Selector<CloudEvent> for CloudEventSelector {
    fn matches(&self, message: &CloudEvent) -> bool {
        message.event_type() == &self.event_type
            && self
                .source
                .as_ref()
                .is_none_or(|source| message.source() == source)
            && self
                .subject
                .as_ref()
                .is_none_or(|subject| message.subject() == Some(subject))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messaging::Subscription;

    #[test]
    fn matches_optional_context_and_subscription_alternatives() {
        let event = CloudEvent::new(
            "id".parse().unwrap(),
            "/source".parse().unwrap(),
            "example.changed".parse().unwrap(),
        );
        let selector = CloudEventSelector::new(event.event_type().clone());
        assert!(selector.matches(&event));
        assert!(
            selector
                .clone()
                .with_source(event.source().clone())
                .matches(&event)
        );
        assert!(
            !selector
                .clone()
                .with_source("/other".parse().unwrap())
                .matches(&event)
        );
        let specific = selector.with_subject("item/1".parse().unwrap());
        assert!(!specific.matches(&event));
        let with_subject = event.with_subject("item/1".parse().unwrap());
        assert!(specific.matches(&with_subject));
        let other = CloudEventSelector::new("other.changed".parse().unwrap());
        assert!(!other.matches(&with_subject));
        assert!(Subscription::AnyOf(&[other, specific]).matches(&with_subject));
        assert!(!Subscription::<CloudEventSelector>::AnyOf(&[]).matches(&with_subject));
    }
}

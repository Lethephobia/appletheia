use std::collections::BTreeMap;

use crate::messaging::Selector;

use super::{
    CloudEvent, CloudEventAttributeString, CloudEventExtensionName, CloudEventSource,
    CloudEventSubject, CloudEventType,
};

/// Matches all specified context conditions; an empty selector matches every event.
/// Extension values are compared using their canonical string representations.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct CloudEventSelector {
    pub event_type: Option<CloudEventType>,
    pub source: Option<CloudEventSource>,
    pub subject: Option<CloudEventSubject>,
    pub extensions: BTreeMap<CloudEventExtensionName, CloudEventAttributeString>,
}

impl CloudEventSelector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_type(mut self, event_type: CloudEventType) -> Self {
        self.event_type = Some(event_type);
        self
    }

    pub fn with_extension(
        mut self,
        name: CloudEventExtensionName,
        value: CloudEventAttributeString,
    ) -> Self {
        self.extensions.insert(name, value);
        self
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
        self.event_type
            .as_ref()
            .is_none_or(|event_type| message.event_type() == event_type)
            && self
                .source
                .as_ref()
                .is_none_or(|source| message.source() == source)
            && self
                .subject
                .as_ref()
                .is_none_or(|subject| message.subject() == Some(subject))
            && self.extensions.iter().all(|(name, expected)| {
                message
                    .extensions()
                    .get(name)
                    .is_some_and(|actual| actual.to_string() == expected.as_str())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messaging::{CloudEventAttributeValue, Subscription};

    #[test]
    fn matches_optional_context_and_subscription_alternatives() {
        let event = CloudEvent::new(
            "id".parse().unwrap(),
            "/source".parse().unwrap(),
            "example.changed".parse().unwrap(),
        );
        let selector = CloudEventSelector::new().with_type(event.event_type().clone());
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
        let other = CloudEventSelector::new().with_type("other.changed".parse().unwrap());
        assert!(!other.matches(&with_subject));
        assert!(Subscription::AnyOf(&[other, specific]).matches(&with_subject));
        assert!(!Subscription::<CloudEventSelector>::AnyOf(&[]).matches(&with_subject));
    }

    #[test]
    fn matches_extensions_without_type_using_wire_values() {
        let mut event = CloudEvent::new(
            "id".parse().unwrap(),
            "/events".parse().unwrap(),
            "example.failed".parse().unwrap(),
        );
        let selector = CloudEventSelector::new()
            .with_extension("attempt".parse().unwrap(), "3".parse().unwrap())
            .with_extension("saganame".parse().unwrap(), "transfer".parse().unwrap());
        assert!(CloudEventSelector::new().matches(&event));
        assert!(!selector.matches(&event));
        event
            .insert_extension(
                "attempt".parse().unwrap(),
                CloudEventAttributeValue::Integer(3),
            )
            .unwrap();
        assert!(!selector.matches(&event));
        event
            .insert_extension(
                "saganame".parse().unwrap(),
                CloudEventAttributeValue::String("transfer".parse().unwrap()),
            )
            .unwrap();
        assert!(selector.matches(&event));
        event
            .insert_extension(
                "attempt".parse().unwrap(),
                CloudEventAttributeValue::String("3".parse().unwrap()),
            )
            .unwrap();
        assert!(selector.matches(&event));
        event
            .insert_extension(
                "attempt".parse().unwrap(),
                CloudEventAttributeValue::Integer(4),
            )
            .unwrap();
        assert!(!selector.matches(&event));
    }
}

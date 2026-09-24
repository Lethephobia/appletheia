use super::PubsubTopicName;
use appletheia_application::ConsumerGroup;

use super::PubsubSubscriptionPathPrefixError;

/// Represents the `projects/{project_id}` prefix used to build subscription names.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PubsubSubscriptionPathPrefix(String);

impl PubsubSubscriptionPathPrefix {
    pub fn new(value: String) -> Result<Self, PubsubSubscriptionPathPrefixError> {
        if value.is_empty() {
            return Err(PubsubSubscriptionPathPrefixError::Empty);
        }

        if !value.starts_with("projects/") {
            return Err(PubsubSubscriptionPathPrefixError::InvalidFormat);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn subscription_name(
        &self,
        topic_name: &PubsubTopicName,
        consumer_group: &ConsumerGroup,
    ) -> String {
        let topic_identifier = topic_name
            .value()
            .rsplit('/')
            .next()
            .unwrap_or(topic_name.value());
        format!(
            "{}/subscriptions/{}_{}",
            self.value(),
            topic_identifier,
            consumer_group.value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_topic_identifier_and_consumer_group() {
        let prefix = PubsubSubscriptionPathPrefix::new("projects/subscribers".to_owned()).unwrap();
        let group = ConsumerGroup::new("projector_transfer".to_owned()).unwrap();
        let events =
            PubsubTopicName::new("projects/publishers/topics/banking-events".to_owned()).unwrap();
        let other_events =
            PubsubTopicName::new("projects/publishers/topics/other-events".to_owned()).unwrap();

        assert_eq!(
            prefix.subscription_name(&events, &group),
            "projects/subscribers/subscriptions/banking-events_projector_transfer"
        );
        assert_ne!(
            prefix.subscription_name(&events, &group),
            prefix.subscription_name(&other_events, &group)
        );
    }
}

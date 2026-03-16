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

    pub fn subscription_name(&self, consumer_group: &ConsumerGroup) -> String {
        format!("{}/subscriptions/{}", self.value(), consumer_group.value())
    }
}

use appletheia_application::CloudEventSelector;
use appletheia_application::CloudEventSubscriber;
use appletheia_application::CloudEventSubscriberError;
use appletheia_application::ConsumerGroup;
use appletheia_application::messaging::Subscription;
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::{Subscriber as GoogleSubscriber, SubscriptionAdmin};
use google_cloud_pubsub::model::Subscription as PubsubSubscription;

use crate::google_cloud::pubsub::messaging::{PubsubSubscriptionPathPrefix, PubsubTopicName};

use super::PubsubCloudEventConsumer;

/// Subscribes to binary CloudEvents. Existing subscriptions must match the
/// requested topic, filter, and ordering configuration.
pub struct PubsubCloudEventSubscriber {
    subscriber: GoogleSubscriber,
    subscription_admin: SubscriptionAdmin,
    subscription_path_prefix: PubsubSubscriptionPathPrefix,
    topic_name: PubsubTopicName,
}

impl PubsubCloudEventSubscriber {
    const MAX_FILTER_BYTES: usize = 256;

    pub fn new(
        subscriber: GoogleSubscriber,
        subscription_admin: SubscriptionAdmin,
        subscription_path_prefix: PubsubSubscriptionPathPrefix,
        topic_name: PubsubTopicName,
    ) -> Self {
        Self {
            subscriber,
            subscription_admin,
            subscription_path_prefix,
            topic_name,
        }
    }

    fn filter_expression_for_selector(selector: &CloudEventSelector) -> String {
        let mut conditions = Vec::new();
        if let Some(event_type) = &selector.event_type {
            conditions.push(format!(
                "attributes.\"ce-type\" = {}",
                Self::quote(event_type.as_str())
            ));
        }
        if let Some(source) = &selector.source {
            conditions.push(format!(
                "attributes.\"ce-source\" = {}",
                Self::quote(source.as_str())
            ));
        }
        if let Some(subject) = &selector.subject {
            conditions.push(format!(
                "attributes.\"ce-subject\" = {}",
                Self::quote(subject.as_str())
            ));
        }
        for (name, value) in &selector.extensions {
            conditions.push(format!(
                "attributes.\"ce-{name}\" = {}",
                Self::quote(value.as_str())
            ));
        }
        if conditions.is_empty() {
            return String::new();
        }
        format!("({})", conditions.join(" AND "))
    }

    fn quote(value: &str) -> String {
        format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
    }

    fn filter_expression(
        subscription: Subscription<'_, CloudEventSelector>,
    ) -> Result<String, CloudEventSubscriberError> {
        let filter = match subscription {
            Subscription::All => String::new(),
            Subscription::AnyOf([]) => return Err(CloudEventSubscriberError::InvalidSubscription),
            Subscription::AnyOf(selectors) => {
                let mut expressions = selectors
                    .iter()
                    .map(Self::filter_expression_for_selector)
                    .collect::<Vec<_>>();
                if expressions.iter().any(String::is_empty) {
                    return Ok(String::new());
                }
                expressions.sort();
                expressions.dedup();
                expressions.join(" OR ")
            }
            Subscription::One(selector) => Self::filter_expression_for_selector(selector),
        };
        if filter.len() > Self::MAX_FILTER_BYTES {
            return Err(CloudEventSubscriberError::InvalidSubscription);
        }
        Ok(filter)
    }
}

impl CloudEventSubscriber for PubsubCloudEventSubscriber {
    type Consumer = PubsubCloudEventConsumer;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, CloudEventSelector>,
    ) -> Result<Self::Consumer, CloudEventSubscriberError> {
        let subscription_name = self
            .subscription_path_prefix
            .subscription_name(&self.topic_name, consumer_group);
        let filter = Self::filter_expression(subscription)?;

        let create_request = PubsubSubscription::new()
            .set_name(&subscription_name)
            .set_topic(self.topic_name.value())
            .set_enable_message_ordering(true)
            .set_filter(&filter);

        match self
            .subscription_admin
            .create_subscription()
            .with_request(create_request)
            .send()
            .await
        {
            Ok(_) => {}
            Err(error)
                if matches!(
                    error.status().map(|status| status.code),
                    Some(Code::AlreadyExists)
                ) =>
            {
                let existing = self
                    .subscription_admin
                    .get_subscription()
                    .set_subscription(&subscription_name)
                    .send()
                    .await
                    .map_err(|source| CloudEventSubscriberError::Subscribe(Box::new(source)))?;
                if existing.topic != self.topic_name.value()
                    || existing.filter != filter
                    || !existing.enable_message_ordering
                {
                    return Err(CloudEventSubscriberError::SubscriptionConflict);
                }
            }
            Err(error) => {
                return Err(CloudEventSubscriberError::Subscribe(Box::new(error)));
            }
        }

        let stream = self.subscriber.subscribe(subscription_name).build();
        Ok(PubsubCloudEventConsumer::new(stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_escaped_attribute_filters() {
        let selector = CloudEventSelector::new()
            .with_type("example.changed".parse().unwrap())
            .with_source("/events".parse().unwrap())
            .with_subject("item\"\\1".parse().unwrap());
        assert_eq!(
            PubsubCloudEventSubscriber::filter_expression(Subscription::One(&selector)).unwrap(),
            r#"(attributes."ce-type" = "example.changed" AND attributes."ce-source" = "/events" AND attributes."ce-subject" = "item\"\\1")"#
        );
    }

    #[test]
    fn subscriptions_are_canonical_and_respect_filter_limits() {
        let first = CloudEventSelector::new().with_type("a".parse().unwrap());
        let second = CloudEventSelector::new().with_type("b".parse().unwrap());
        assert_eq!(
            PubsubCloudEventSubscriber::filter_expression(Subscription::AnyOf(&[
                first.clone(),
                second.clone(),
                first.clone()
            ]))
            .unwrap(),
            PubsubCloudEventSubscriber::filter_expression(Subscription::AnyOf(&[second, first]))
                .unwrap(),
        );
        assert_eq!(
            PubsubCloudEventSubscriber::filter_expression(Subscription::All).unwrap(),
            ""
        );
        assert!(matches!(
            PubsubCloudEventSubscriber::filter_expression(Subscription::AnyOf(&[])),
            Err(CloudEventSubscriberError::InvalidSubscription)
        ));
        let long = CloudEventSelector::new().with_type("あ".repeat(90).parse().unwrap());
        assert!(matches!(
            PubsubCloudEventSubscriber::filter_expression(Subscription::One(&long)),
            Err(CloudEventSubscriberError::InvalidSubscription)
        ));
    }

    #[test]
    fn filters_extensions_without_type_and_handles_unrestricted_selectors() {
        let selector = CloudEventSelector::new().with_extension(
            "saganame".parse().unwrap(),
            "transfer\"\\1".parse().unwrap(),
        );
        assert_eq!(
            PubsubCloudEventSubscriber::filter_expression(Subscription::One(&selector)).unwrap(),
            r#"(attributes."ce-saganame" = "transfer\"\\1")"#
        );
        let unrestricted = CloudEventSelector::new();
        assert_eq!(
            PubsubCloudEventSubscriber::filter_expression(Subscription::One(&unrestricted))
                .unwrap(),
            ""
        );
        assert_eq!(
            PubsubCloudEventSubscriber::filter_expression(Subscription::AnyOf(&[
                selector,
                unrestricted
            ]))
            .unwrap(),
            ""
        );
    }
}

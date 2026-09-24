use super::PubsubTopicName;
use appletheia_application::ConsumerGroup;
use appletheia_application::Subscriber;
use appletheia_application::SubscriberError;
use appletheia_application::command::{CommandEnvelope, CommandSelector};
use appletheia_application::messaging::Subscription;
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::{Subscriber as GoogleSubscriber, SubscriptionAdmin};
use google_cloud_pubsub::model::Subscription as PubsubSubscription;

use super::PubsubSubscriptionPathPrefix;
use super::pubsub_consumer::PubsubConsumer;

pub struct PubsubCommandSubscriber {
    subscriber: GoogleSubscriber,
    subscription_admin: SubscriptionAdmin,
    subscription_path_prefix: PubsubSubscriptionPathPrefix,
    topic_name: PubsubTopicName,
}

impl PubsubCommandSubscriber {
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

    fn filter_expression_for_selector(selector: &CommandSelector) -> String {
        format!(
            "(attributes.command_name = \"{}\")",
            selector.command_name.value()
        )
    }

    fn filter_expression_for_selectors(selectors: &[CommandSelector]) -> String {
        selectors
            .iter()
            .map(Self::filter_expression_for_selector)
            .collect::<Vec<_>>()
            .join(" OR ")
    }
}

impl Subscriber<CommandEnvelope> for PubsubCommandSubscriber {
    type Consumer = PubsubConsumer<CommandEnvelope>;
    type Selector = CommandSelector;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, Self::Selector>,
    ) -> Result<Self::Consumer, SubscriberError> {
        let subscription_name = self
            .subscription_path_prefix
            .subscription_name(&self.topic_name, consumer_group);
        let filter = match subscription {
            Subscription::All => String::new(),
            Subscription::AnyOf([]) => {
                return Err(SubscriberError::InvalidSubscription);
            }
            Subscription::AnyOf(selectors) => Self::filter_expression_for_selectors(selectors),
            Subscription::One(selector) => Self::filter_expression_for_selector(selector),
        };

        let create_request = PubsubSubscription::new()
            .set_name(&subscription_name)
            .set_topic(self.topic_name.value())
            .set_enable_message_ordering(true)
            .set_filter(filter);

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
                    .map_err(|source| SubscriberError::Subscribe(Box::new(source)))?;
                if existing.topic != self.topic_name.value() {
                    return Err(SubscriberError::SubscriptionConflict);
                }
            }
            Err(error) => {
                return Err(SubscriberError::Subscribe(Box::new(error)));
            }
        }

        let stream = self.subscriber.subscribe(subscription_name).build();
        Ok(PubsubConsumer::new(stream))
    }
}

use appletheia_application::command::CommandFailureEnvelope;
use appletheia_application::messaging::{Subscription, TopicId};
use appletheia_application::saga::SagaName;
use appletheia_application::{ConsumerGroup, Subscriber, SubscriberError};
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::{Subscriber as GoogleSubscriber, SubscriptionAdmin};
use google_cloud_pubsub::model::Subscription as PubsubSubscription;

use super::PubsubSubscriptionPathPrefix;
use super::pubsub_consumer::PubsubConsumer;

/// Subscribes sagas to terminal command-failure notifications.
pub struct PubsubCommandFailureSubscriber {
    subscriber: GoogleSubscriber,
    subscription_admin: SubscriptionAdmin,
    subscription_path_prefix: PubsubSubscriptionPathPrefix,
    topic_id: TopicId,
}

impl PubsubCommandFailureSubscriber {
    pub fn new(
        subscriber: GoogleSubscriber,
        subscription_admin: SubscriptionAdmin,
        subscription_path_prefix: PubsubSubscriptionPathPrefix,
        topic_id: TopicId,
    ) -> Self {
        Self {
            subscriber,
            subscription_admin,
            subscription_path_prefix,
            topic_id,
        }
    }

    fn selector_filter(selector: &SagaName) -> String {
        format!("attributes.saga_name = \"{}\"", selector.value())
    }
}

impl Subscriber<CommandFailureEnvelope> for PubsubCommandFailureSubscriber {
    type Consumer = PubsubConsumer<CommandFailureEnvelope>;
    type Selector = SagaName;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, Self::Selector>,
    ) -> Result<Self::Consumer, SubscriberError> {
        let routes = match subscription {
            Subscription::All => String::new(),
            Subscription::AnyOf([]) => return Err(SubscriberError::InvalidSubscription),
            Subscription::AnyOf(selectors) => selectors
                .iter()
                .map(Self::selector_filter)
                .collect::<Vec<_>>()
                .join(" OR "),
            Subscription::One(selector) => Self::selector_filter(selector),
        };
        let failure_consumer_group =
            ConsumerGroup::new(format!("{}_command_failures", consumer_group.value()))
                .map_err(|source| SubscriberError::Subscribe(Box::new(source)))?;
        let subscription_name = self
            .subscription_path_prefix
            .subscription_name(&failure_consumer_group);
        let create_request = PubsubSubscription::new()
            .set_name(&subscription_name)
            .set_topic(self.topic_id.value())
            .set_enable_message_ordering(true)
            .set_filter(routes);
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
                ) => {}
            Err(error) => return Err(SubscriberError::Subscribe(Box::new(error))),
        }
        let stream = self.subscriber.subscribe(subscription_name).build();
        Ok(PubsubConsumer::new(stream))
    }
}

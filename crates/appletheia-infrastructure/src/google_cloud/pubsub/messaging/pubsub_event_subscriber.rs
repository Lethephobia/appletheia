use appletheia_application::ConsumerGroup;
use appletheia_application::Subscriber;
use appletheia_application::SubscriberError;
use appletheia_application::event::{EventEnvelope, EventSelector};
use appletheia_application::messaging::{Subscription, TopicId};
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::{Subscriber as GoogleSubscriber, SubscriptionAdmin};
use google_cloud_pubsub::model::Subscription as PubsubSubscription;

use super::PubsubSubscriptionPathPrefix;
use super::pubsub_consumer::PubsubConsumer;

pub struct PubsubEventSubscriber {
    subscriber: GoogleSubscriber,
    subscription_admin: SubscriptionAdmin,
    subscription_path_prefix: PubsubSubscriptionPathPrefix,
    topic_id: TopicId,
}

impl PubsubEventSubscriber {
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

    fn filter_expression(selectors: &[EventSelector]) -> String {
        selectors
            .iter()
            .map(|selector| {
                format!(
                    "(attributes.aggregate_type = \"{}\" AND attributes.event_name = \"{}\")",
                    selector.aggregate_type.value(),
                    selector.event_name.value()
                )
            })
            .collect::<Vec<_>>()
            .join(" OR ")
    }
}

impl Subscriber<EventEnvelope> for PubsubEventSubscriber {
    type Consumer = PubsubConsumer<EventEnvelope>;
    type Selector = EventSelector;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, Self::Selector>,
    ) -> Result<Self::Consumer, SubscriberError> {
        let subscription_name = self
            .subscription_path_prefix
            .subscription_name(consumer_group);
        let filter = match subscription {
            Subscription::All => String::new(),
            Subscription::Only([]) => {
                return Err(SubscriberError::InvalidSubscription);
            }
            Subscription::Only(selectors) => Self::filter_expression(selectors),
        };

        let create_request = PubsubSubscription::new()
            .set_name(&subscription_name)
            .set_topic(self.topic_id.value())
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
                ) => {}
            Err(error) => {
                return Err(SubscriberError::Subscribe(Box::new(error)));
            }
        }

        let stream = self.subscriber.subscribe(subscription_name).build();
        Ok(PubsubConsumer::new(stream))
    }
}

use appletheia_application::messaging::{
    ConsumerGroup, Subscriber, SubscriberError, Subscription, TopicId,
};
use appletheia_application::read_model::ReadModelInvalidationEnvelope;
use appletheia_application::read_model::watch::ReadModelInvalidationShard;
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::{Subscriber as GoogleSubscriber, SubscriptionAdmin};
use google_cloud_pubsub::model::Subscription as PubsubSubscription;

use super::{PubsubConsumer, PubsubSubscriptionPathPrefix};

/// Creates one ordered Google Cloud Pub/Sub subscription per fixed watch shard.
pub struct PubsubReadModelInvalidationSubscriber {
    subscriber: GoogleSubscriber,
    subscription_admin: SubscriptionAdmin,
    subscription_path_prefix: PubsubSubscriptionPathPrefix,
    topic_id: TopicId,
}

impl PubsubReadModelInvalidationSubscriber {
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

    fn subscription_name(
        &self,
        consumer_group: &ConsumerGroup,
        shard: ReadModelInvalidationShard,
    ) -> String {
        format!(
            "{}-shard-{}",
            self.subscription_path_prefix
                .subscription_name(consumer_group),
            shard.index()
        )
    }
}

impl Subscriber<ReadModelInvalidationEnvelope> for PubsubReadModelInvalidationSubscriber {
    type Consumer = PubsubConsumer<ReadModelInvalidationEnvelope>;
    type Selector = ReadModelInvalidationShard;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, Self::Selector>,
    ) -> Result<Self::Consumer, SubscriberError> {
        let Subscription::One(shard) = subscription else {
            return Err(SubscriberError::InvalidSubscription);
        };
        let subscription_name = self.subscription_name(consumer_group, *shard);
        let filter = format!(
            "attributes.{} = \"{}\"",
            ReadModelInvalidationShard::ATTRIBUTE_NAME,
            shard.attribute_value()
        );
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
            Err(error) => return Err(SubscriberError::Subscribe(Box::new(error))),
        }

        let stream = self.subscriber.subscribe(subscription_name).build();
        Ok(PubsubConsumer::new(stream))
    }
}

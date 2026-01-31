use appletheia_application::ConsumerGroup;
use appletheia_application::event::{EventEnvelope, EventSelector};
use appletheia_application::messaging::{Subscription, Topic, TopicError, TopicId, TopicIdAccess};
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::subscription::SubscriptionConfig;
use tonic::Code;

use super::pubsub_consumer::PubsubConsumer;
use super::pubsub_event_publisher::PubsubEventPublisher;

pub struct PubsubEventTopic {
    client: Client,
    topic_id: TopicId,
}

impl PubsubEventTopic {
    pub fn new(client: Client, topic_id: TopicId) -> Self {
        Self { client, topic_id }
    }

    fn subscription_id(&self, consumer_group: &ConsumerGroup) -> String {
        consumer_group.value().to_string()
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

impl TopicIdAccess for PubsubEventTopic {
    fn topic_id(&self) -> &TopicId {
        &self.topic_id
    }
}

impl Topic<EventEnvelope> for PubsubEventTopic {
    type Consumer = PubsubConsumer<EventEnvelope>;
    type Publisher = PubsubEventPublisher;
    type Selector = EventSelector;

    fn new_publisher(&self) -> Self::Publisher {
        let publisher = self.client.topic(self.topic_id.value()).new_publisher(None);
        PubsubEventPublisher::new(publisher)
    }

    async fn subscribe(
        &mut self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, Self::Selector>,
    ) -> Result<Self::Consumer, TopicError> {
        let subscription_id = self.subscription_id(consumer_group);
        let topic_id = self.topic_id.value();

        let mut config = SubscriptionConfig::default();
        config.enable_message_ordering = true;
        config.filter = match subscription {
            Subscription::All => String::new(),
            Subscription::Only(selectors) if selectors.is_empty() => {
                return Err(TopicError::InvalidSubscription);
            }
            Subscription::Only(selectors) => Self::filter_expression(selectors),
        };

        let subscription = match self
            .client
            .create_subscription(&subscription_id, topic_id, config, None)
            .await
        {
            Ok(subscription) => subscription,
            Err(status) if status.code() == Code::AlreadyExists => {
                self.client.subscription(&subscription_id)
            }
            Err(status) => {
                return Err(TopicError::Subscribe(Box::new(status)));
            }
        };

        let stream = subscription
            .subscribe(None)
            .await
            .map_err(|error| TopicError::Subscribe(Box::new(error)))?;

        Ok(PubsubConsumer::new(stream))
    }
}

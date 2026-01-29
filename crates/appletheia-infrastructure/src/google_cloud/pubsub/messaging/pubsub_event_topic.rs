use appletheia_application::ConsumerGroup;
use appletheia_application::event::{EventEnvelope, EventSelector};
use appletheia_application::messaging::{Topic, TopicError};
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::publisher::PublisherConfig;
use google_cloud_pubsub::subscription::{SubscribeConfig, SubscriptionConfig};
use google_cloud_pubsub::topic::Topic as PubsubTopic;
use tonic::Code;

use super::pubsub_consumer::PubsubConsumer;
use super::pubsub_event_publisher::PubsubEventPublisher;

pub struct PubsubEventTopic {
    client: Client,
    topic_id: String,
    topic: PubsubTopic,
    publisher: PubsubEventPublisher,
    subscription_config: SubscriptionConfig,
    subscribe_config: Option<SubscribeConfig>,
}

impl PubsubEventTopic {
    pub fn new(client: Client, topic_id: impl Into<String>) -> Self {
        let topic_id = topic_id.into();
        let topic = client.topic(&topic_id);
        let publisher = topic.new_publisher(None);

        Self {
            client,
            topic_id,
            topic,
            publisher: PubsubEventPublisher::new(publisher),
            subscription_config: SubscriptionConfig::default(),
            subscribe_config: None,
        }
    }

    pub fn with_subscription_config(mut self, config: SubscriptionConfig) -> Self {
        self.subscription_config = config;
        self
    }

    pub fn with_subscribe_config(mut self, config: SubscribeConfig) -> Self {
        self.subscribe_config = Some(config);
        self
    }

    pub fn with_publisher_config(mut self, config: PublisherConfig) -> Self {
        let publisher = self.topic.new_publisher(Some(config));
        self.publisher = PubsubEventPublisher::new(publisher);
        self
    }

    fn subscription_id(&self, consumer_group: &ConsumerGroup) -> String {
        consumer_group.value().to_string()
    }

    fn filter_expression(selectors: &[EventSelector]) -> String {
        if selectors.is_empty() {
            return String::new();
        }

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

impl Topic<EventEnvelope> for PubsubEventTopic {
    type Consumer = PubsubConsumer<EventEnvelope>;
    type Publisher = PubsubEventPublisher;
    type Selector = EventSelector;

    fn publisher(&self) -> &Self::Publisher {
        &self.publisher
    }

    async fn subscribe(
        &mut self,
        consumer_group: &ConsumerGroup,
        selectors: &[Self::Selector],
    ) -> Result<Self::Consumer, TopicError> {
        let subscription_id = self.subscription_id(consumer_group);

        let mut config = self.subscription_config.clone();
        config.enable_message_ordering = true;
        config.filter = Self::filter_expression(selectors);

        let subscription = match self
            .client
            .create_subscription(&subscription_id, &self.topic_id, config, None)
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
            .subscribe(self.subscribe_config.clone())
            .await
            .map_err(|error| TopicError::Subscribe(Box::new(error)))?;

        Ok(PubsubConsumer::new(stream))
    }
}

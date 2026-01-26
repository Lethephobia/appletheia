use appletheia_application::event::{EventEnvelope, EventSelector};
use appletheia_application::{ConsumerFactory, ConsumerFactoryError, ConsumerGroup};
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::subscription::{SubscribeConfig, SubscriptionConfig};
use tonic::Code;

use super::pubsub_consumer::PubsubConsumer;

pub struct PubsubEventConsumerFactory {
    client: Client,
    topic_id: String,
    subscription_config: SubscriptionConfig,
    subscribe_config: Option<SubscribeConfig>,
}

impl PubsubEventConsumerFactory {
    pub fn new(client: Client, topic_id: impl Into<String>) -> Self {
        Self {
            client,
            topic_id: topic_id.into(),
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

    fn subscription_id(&self, consumer_group: &str) -> String {
        consumer_group.to_string()
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

impl ConsumerFactory<EventEnvelope> for PubsubEventConsumerFactory {
    type Consumer = PubsubConsumer<EventEnvelope>;
    type Selector = EventSelector;

    async fn subscribe(
        &mut self,
        consumer_group: &ConsumerGroup,
        selectors: &[Self::Selector],
    ) -> Result<Self::Consumer, ConsumerFactoryError> {
        let subscription_id = self.subscription_id(consumer_group.value());

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
                return Err(ConsumerFactoryError::Subscribe(Box::new(status)));
            }
        };

        let stream = subscription
            .subscribe(self.subscribe_config.clone())
            .await
            .map_err(|error| ConsumerFactoryError::Subscribe(Box::new(error)))?;

        Ok(PubsubConsumer::new(stream))
    }
}

use std::collections::HashMap;

use appletheia_application::ConsumerGroup;
use appletheia_application::event::{EventEnvelope, EventSelector};
use appletheia_application::massaging::PublishDispatchError;
use appletheia_application::massaging::{
    PublishResult, Publisher, PublisherError, Topic, TopicError,
};
use appletheia_application::outbox::OrderingKey;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::publisher::PublisherConfig;
use google_cloud_pubsub::subscription::{SubscribeConfig, SubscriptionConfig};
use google_cloud_pubsub::topic::Topic as PubsubTopic;
use tonic::Code;

use super::pubsub_consumer::PubsubConsumer;

#[derive(Clone)]
pub struct PubsubEventPublisher {
    publisher: google_cloud_pubsub::publisher::Publisher,
}

impl PubsubEventPublisher {
    pub fn new(publisher: google_cloud_pubsub::publisher::Publisher) -> Self {
        Self { publisher }
    }

    fn build_message(event: &EventEnvelope) -> Result<PubsubMessage, PublisherError> {
        let mut attributes = HashMap::new();

        attributes.insert(
            "event_sequence".to_string(),
            event.event_sequence.to_string(),
        );
        attributes.insert("event_id".to_string(), event.event_id.to_string());
        attributes.insert(
            "aggregate_type".to_string(),
            event.aggregate_type.to_string(),
        );
        attributes.insert("aggregate_id".to_string(), event.aggregate_id.to_string());
        attributes.insert(
            "aggregate_version".to_string(),
            event.aggregate_version.to_string(),
        );
        attributes.insert("event_name".to_string(), event.event_name.to_string());
        attributes.insert("occurred_at".to_string(), event.occurred_at.to_string());
        attributes.insert(
            "correlation_id".to_string(),
            event.correlation_id.to_string(),
        );
        attributes.insert("causation_id".to_string(), event.causation_id.to_string());

        let data = serde_json::to_vec(event)
            .map_err(|source| PublisherError::Publish(Box::new(source)))?;

        let ordering_key =
            OrderingKey::from((&event.aggregate_type, &event.aggregate_id)).to_string();

        Ok(PubsubMessage {
            data,
            attributes,
            message_id: String::new(),
            publish_time: None,
            ordering_key,
        })
    }
}

impl Publisher<EventEnvelope> for PubsubEventPublisher {
    async fn publish<'a, I>(&self, messages: I) -> Result<Vec<PublishResult>, PublisherError>
    where
        I: IntoIterator<Item = &'a EventEnvelope>,
        EventEnvelope: 'a,
    {
        let pubsub_messages = messages
            .into_iter()
            .map(Self::build_message)
            .collect::<Result<Vec<_>, _>>()?;

        if pubsub_messages.is_empty() {
            return Ok(Vec::new());
        }

        let awaiters = self.publisher.publish_bulk(pubsub_messages).await;

        let mut results = Vec::with_capacity(awaiters.len());

        for (input_index, awaiter) in awaiters.into_iter().enumerate() {
            match awaiter.get().await {
                Ok(message_id) => {
                    results.push(PublishResult::Success {
                        input_index,
                        transport_message_id: Some(message_id),
                    });
                }
                Err(status) => {
                    let code = status.code().to_string();
                    let message = status.to_string();
                    let cause = match status.code() {
                        Code::Unavailable
                        | Code::DeadlineExceeded
                        | Code::ResourceExhausted
                        | Code::Aborted => PublishDispatchError::Transient {
                            code: code.clone(),
                            message,
                        },
                        _ => PublishDispatchError::Permanent { code, message },
                    };

                    results.push(PublishResult::Failed { input_index, cause });
                }
            }
        }

        Ok(results)
    }
}

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

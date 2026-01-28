use std::collections::HashMap;

use appletheia_application::massaging::{
    PublishDispatchError, PublishResult, Publisher, PublisherError, Topic, TopicError,
};
use appletheia_application::outbox::OrderingKey;
use appletheia_application::outbox::command::CommandEnvelope;
use appletheia_application::ConsumerGroup;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::publisher::PublisherConfig;
use google_cloud_pubsub::subscription::{SubscribeConfig, SubscriptionConfig};
use google_cloud_pubsub::topic::Topic as PubsubTopic;
use tonic::Code;

use super::pubsub_consumer::PubsubConsumer;

#[derive(Clone)]
pub struct PubsubCommandPublisher {
    publisher: google_cloud_pubsub::publisher::Publisher,
}

impl PubsubCommandPublisher {
    pub fn new(publisher: google_cloud_pubsub::publisher::Publisher) -> Self {
        Self { publisher }
    }

    fn build_message(command: &CommandEnvelope) -> Result<PubsubMessage, PublisherError> {
        let mut attributes = HashMap::new();

        attributes.insert("message_id".to_string(), command.message_id.to_string());
        attributes.insert("command_name".to_string(), command.command_name.to_string());
        attributes.insert("correlation_id".to_string(), command.correlation_id.to_string());
        attributes.insert("causation_id".to_string(), command.causation_id.to_string());

        let data = serde_json::to_vec(command)
            .map_err(|source| PublisherError::Publish(Box::new(source)))?;

        let ordering_key = OrderingKey::from(command.correlation_id).to_string();

        Ok(PubsubMessage {
            data,
            attributes,
            message_id: String::new(),
            publish_time: None,
            ordering_key,
        })
    }
}

impl Publisher<CommandEnvelope> for PubsubCommandPublisher {
    async fn publish<'a, I>(&self, messages: I) -> Result<Vec<PublishResult>, PublisherError>
    where
        I: IntoIterator<Item = &'a CommandEnvelope>,
        CommandEnvelope: 'a,
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

pub struct PubsubCommandTopic {
    client: Client,
    topic_id: String,
    topic: PubsubTopic,
    publisher: PubsubCommandPublisher,
    subscription_config: SubscriptionConfig,
    subscribe_config: Option<SubscribeConfig>,
}

impl PubsubCommandTopic {
    pub fn new(client: Client, topic_id: impl Into<String>) -> Self {
        let topic_id = topic_id.into();
        let topic = client.topic(&topic_id);
        let publisher = topic.new_publisher(None);

        Self {
            client,
            topic_id,
            topic,
            publisher: PubsubCommandPublisher::new(publisher),
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
        self.publisher = PubsubCommandPublisher::new(publisher);
        self
    }

    fn subscription_id(&self, consumer_group: &ConsumerGroup) -> String {
        consumer_group.value().to_string()
    }
}

impl Topic<CommandEnvelope> for PubsubCommandTopic {
    type Consumer = PubsubConsumer<CommandEnvelope>;
    type Publisher = PubsubCommandPublisher;
    type Selector = ();

    fn publisher(&self) -> &Self::Publisher {
        &self.publisher
    }

    async fn subscribe(
        &mut self,
        consumer_group: &ConsumerGroup,
        _selectors: &[Self::Selector],
    ) -> Result<Self::Consumer, TopicError> {
        let subscription_id = self.subscription_id(consumer_group);

        let mut config = self.subscription_config.clone();
        config.enable_message_ordering = true;

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

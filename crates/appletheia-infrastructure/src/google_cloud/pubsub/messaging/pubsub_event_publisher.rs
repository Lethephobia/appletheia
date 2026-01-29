use std::collections::HashMap;

use appletheia_application::event::EventEnvelope;
use appletheia_application::messaging::{
    PublishDispatchError, PublishResult, Publisher, PublisherError,
};
use appletheia_application::outbox::OrderingKey;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use tonic::Code;

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

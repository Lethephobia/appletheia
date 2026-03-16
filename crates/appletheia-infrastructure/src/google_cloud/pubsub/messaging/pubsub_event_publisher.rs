use std::collections::HashMap;

use appletheia_application::event::EventEnvelope;
use appletheia_application::messaging::{
    PublishDispatchError, PublishResult, Publisher, PublisherError,
};
use appletheia_application::outbox::OrderingKey;
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::Publisher as GooglePublisher;
use google_cloud_pubsub::error::PublishError;
use google_cloud_pubsub::model::Message;

#[derive(Clone)]
pub struct PubsubEventPublisher {
    publisher: GooglePublisher,
}

impl PubsubEventPublisher {
    pub fn new(publisher: GooglePublisher) -> Self {
        Self { publisher }
    }

    fn build_message(event: &EventEnvelope) -> Result<Message, PublisherError> {
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

        Ok(Message::new()
            .set_data(data)
            .set_attributes(attributes)
            .set_ordering_key(ordering_key))
    }

    fn dispatch_error(error: PublishError) -> PublishDispatchError {
        match error {
            PublishError::Rpc(source) => {
                let code = source
                    .status()
                    .map(|status| status.code.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                let message = source.to_string();

                match source.status().map(|status| status.code) {
                    Some(
                        Code::Unavailable
                        | Code::DeadlineExceeded
                        | Code::ResourceExhausted
                        | Code::Aborted,
                    ) => PublishDispatchError::Transient { code, message },
                    _ => PublishDispatchError::Permanent { code, message },
                }
            }
            other => PublishDispatchError::Permanent {
                code: "publish_error".to_string(),
                message: other.to_string(),
            },
        }
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

        let publish_futures = pubsub_messages
            .into_iter()
            .map(|message| self.publisher.publish(message))
            .collect::<Vec<_>>();

        let mut results = Vec::with_capacity(publish_futures.len());

        for (input_index, publish_future) in publish_futures.into_iter().enumerate() {
            match publish_future.await {
                Ok(message_id) => {
                    results.push(PublishResult::Success {
                        input_index,
                        transport_message_id: Some(message_id),
                    });
                }
                Err(error) => {
                    let cause = Self::dispatch_error(error);
                    results.push(PublishResult::Failed { input_index, cause });
                }
            }
        }

        Ok(results)
    }
}

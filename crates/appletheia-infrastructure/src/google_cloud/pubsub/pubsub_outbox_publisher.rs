use std::collections::HashMap;

use appletheia_application::outbox::{Outbox, OutboxPublisher, OutboxPublisherError};
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::publisher::Publisher;

use super::PubsubOutboxPublisherError;

pub struct PubsubOutboxPublisher {
    publisher: Publisher,
}

impl PubsubOutboxPublisher {
    pub fn new(publisher: Publisher) -> Self {
        Self { publisher }
    }

    fn build_message(outbox: &Outbox) -> Result<PubsubMessage, OutboxPublisherError> {
        let mut attributes = HashMap::new();

        attributes.insert("outbox_id".to_string(), outbox.id.to_string());
        attributes.insert("event_id".to_string(), outbox.event_id.to_string());
        attributes.insert(
            "event_sequence".to_string(),
            outbox.event_sequence.to_string(),
        );
        attributes.insert(
            "aggregate_type".to_string(),
            outbox.aggregate_type.to_string(),
        );
        attributes.insert("aggregate_id".to_string(), outbox.aggregate_id.to_string());
        attributes.insert(
            "aggregate_version".to_string(),
            outbox.aggregate_version.to_string(),
        );
        attributes.insert("occurred_at".to_string(), outbox.occurred_at.to_string());
        attributes.insert(
            "correlation_id".to_string(),
            outbox.correlation_id.to_string(),
        );
        attributes.insert("causation_id".to_string(), outbox.causation_id.to_string());

        let data = serde_json::to_vec(outbox.payload.value()).map_err(|err| {
            OutboxPublisherError::Publish(Box::new(PubsubOutboxPublisherError::BuildMessage(err)))
        })?;

        let ordering_key = outbox.ordering_key().to_string();

        Ok(PubsubMessage {
            data,
            attributes,
            message_id: String::new(),
            publish_time: None,
            ordering_key,
        })
    }
}

impl OutboxPublisher for PubsubOutboxPublisher {
    async fn publish_outbox(&self, outboxes: &[Outbox]) -> Result<(), OutboxPublisherError> {
        if outboxes.is_empty() {
            return Ok(());
        }

        let messages = outboxes
            .iter()
            .map(Self::build_message)
            .collect::<Result<Vec<_>, _>>()?;

        let awaiters = self.publisher.publish_bulk(messages).await;

        for awaiter in awaiters {
            if let Err(status) = awaiter.get().await {
                let err = PubsubOutboxPublisherError::Publish(status);
                return Err(OutboxPublisherError::Publish(Box::new(err)));
            }
        }

        Ok(())
    }
}

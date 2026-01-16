use std::collections::HashMap;

use appletheia_application::event::{
    EventOutbox, EventOutboxDispatchError, EventOutboxPublishResult, EventOutboxPublisher,
    EventOutboxPublisherError,
};
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::publisher::Publisher;
use tonic::Code;

pub struct PubsubEventOutboxPublisher {
    publisher: Publisher,
}

impl PubsubEventOutboxPublisher {
    pub fn new(publisher: Publisher) -> Self {
        Self { publisher }
    }

    fn build_message(outbox: &EventOutbox) -> Result<PubsubMessage, EventOutboxPublisherError> {
        let mut attributes = HashMap::new();

        attributes.insert("outbox_id".to_string(), outbox.id.to_string());
        attributes.insert("event_id".to_string(), outbox.event.event_id.to_string());
        attributes.insert(
            "event_sequence".to_string(),
            outbox.event.event_sequence.to_string(),
        );
        attributes.insert(
            "aggregate_type".to_string(),
            outbox.event.aggregate_type.to_string(),
        );
        attributes.insert(
            "aggregate_id".to_string(),
            outbox.event.aggregate_id.to_string(),
        );
        attributes.insert(
            "aggregate_version".to_string(),
            outbox.event.aggregate_version.to_string(),
        );
        attributes.insert(
            "occurred_at".to_string(),
            outbox.event.occurred_at.to_string(),
        );
        attributes.insert(
            "correlation_id".to_string(),
            outbox.event.correlation_id.to_string(),
        );
        attributes.insert(
            "causation_id".to_string(),
            outbox.event.causation_id.to_string(),
        );

        let data = serde_json::to_vec(&outbox.event)?;

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

impl EventOutboxPublisher for PubsubEventOutboxPublisher {
    async fn publish_outbox(
        &self,
        outboxes: &[EventOutbox],
    ) -> Result<Vec<EventOutboxPublishResult>, EventOutboxPublisherError> {
        if outboxes.is_empty() {
            return Ok(Vec::new());
        }

        let messages = outboxes
            .iter()
            .map(Self::build_message)
            .collect::<Result<Vec<_>, _>>()?;

        let awaiters = self.publisher.publish_bulk(messages).await;

        let mut results = Vec::with_capacity(outboxes.len());

        for (input_index, (outbox, awaiter)) in
            outboxes.iter().zip(awaiters.into_iter()).enumerate()
        {
            let outbox_id = outbox.id;
            match awaiter.get().await {
                Ok(message_id) => {
                    results.push(EventOutboxPublishResult::Success {
                        input_index,
                        outbox_id,
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
                        | Code::Aborted => EventOutboxDispatchError::Transient {
                            code: code.clone(),
                            message,
                        },
                        _ => EventOutboxDispatchError::Permanent { code, message },
                    };

                    results.push(EventOutboxPublishResult::Failed {
                        input_index,
                        outbox_id,
                        cause,
                    });
                }
            }
        }

        Ok(results)
    }
}

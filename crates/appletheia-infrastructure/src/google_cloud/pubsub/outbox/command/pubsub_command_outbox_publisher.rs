use std::collections::HashMap;

use appletheia_application::outbox::{OutboxDispatchError, OutboxPublisherError, command::*};
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::publisher::Publisher;
use tonic::Code;

pub struct PubsubCommandOutboxPublisher {
    publisher: Publisher,
}

impl PubsubCommandOutboxPublisher {
    pub fn new(publisher: Publisher) -> Self {
        Self { publisher }
    }

    fn build_message(outbox: &CommandOutbox) -> Result<PubsubMessage, OutboxPublisherError> {
        let mut attributes = HashMap::new();

        attributes.insert("command_outbox_id".to_string(), outbox.id.to_string());
        attributes.insert(
            "message_id".to_string(),
            outbox.command.message_id().to_string(),
        );
        attributes.insert(
            "causation_id".to_string(),
            outbox.command.causation_id().to_string(),
        );
        attributes.insert(
            "correlation_id".to_string(),
            outbox.command.correlation_id().to_string(),
        );
        attributes.insert(
            "command_name".to_string(),
            outbox.command.command_name.to_string(),
        );
        attributes.insert(
            "command_hash".to_string(),
            outbox.command.command_hash.to_string(),
        );

        let data = serde_json::to_vec(&outbox.command)?;

        let ordering_key = outbox.command.ordering_key.clone().unwrap_or_default();

        Ok(PubsubMessage {
            data,
            attributes,
            message_id: String::new(),
            publish_time: None,
            ordering_key,
        })
    }
}

impl CommandOutboxPublisher for PubsubCommandOutboxPublisher {
    async fn publish_outbox(
        &self,
        outboxes: &[CommandOutbox],
    ) -> Result<Vec<CommandOutboxPublishResult>, OutboxPublisherError> {
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
                    results.push(CommandOutboxPublishResult::Success {
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
                        | Code::Aborted => OutboxDispatchError::Transient {
                            code: code.clone(),
                            message,
                        },
                        _ => OutboxDispatchError::Permanent { code, message },
                    };

                    results.push(CommandOutboxPublishResult::Failed {
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

use std::collections::HashMap;

use appletheia_application::messaging::{
    PublishDispatchError, PublishResult, Publisher, PublisherError,
};
use appletheia_application::outbox::OrderingKey;
use appletheia_application::outbox::command::CommandEnvelope;
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::Publisher as GooglePublisher;
use google_cloud_pubsub::error::PublishError;
use google_cloud_pubsub::model::Message;

#[derive(Clone)]
pub struct PubsubCommandPublisher {
    publisher: GooglePublisher,
}

impl PubsubCommandPublisher {
    pub fn new(publisher: GooglePublisher) -> Self {
        Self { publisher }
    }

    fn build_message(command: &CommandEnvelope) -> Result<Message, PublisherError> {
        let mut attributes = HashMap::new();

        attributes.insert("message_id".to_string(), command.message_id.to_string());
        attributes.insert("command_name".to_string(), command.command_name.to_string());
        attributes.insert(
            "correlation_id".to_string(),
            command.correlation_id.to_string(),
        );
        attributes.insert("causation_id".to_string(), command.causation_id.to_string());

        let data = serde_json::to_vec(command)
            .map_err(|source| PublisherError::Publish(Box::new(source)))?;

        let ordering_key = OrderingKey::from(command.correlation_id).to_string();

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

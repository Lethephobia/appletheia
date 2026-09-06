use std::collections::HashMap;

use appletheia_application::command::CommandFailureEnvelope;
use appletheia_application::messaging::{
    PublishDispatchError, PublishResult, Publisher, PublisherError,
};
use appletheia_application::outbox::OrderingKey;
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::Publisher as GooglePublisher;
use google_cloud_pubsub::error::PublishError;
use google_cloud_pubsub::model::Message;

/// Publishes terminal command-failure notifications to Google Cloud Pub/Sub.
#[derive(Clone)]
pub struct PubsubCommandFailurePublisher {
    publisher: GooglePublisher,
}

impl PubsubCommandFailurePublisher {
    pub fn new(publisher: GooglePublisher) -> Self {
        Self { publisher }
    }

    fn build_message(failure: &CommandFailureEnvelope) -> Result<Message, PublisherError> {
        let mut attributes = HashMap::new();
        attributes.insert("failure_id".to_owned(), failure.failure_id.to_string());
        attributes.insert("command_name".to_owned(), failure.command_name.to_string());
        attributes.insert("saga_name".to_owned(), failure.origin.saga_name.to_string());
        let data = serde_json::to_vec(failure)
            .map_err(|source| PublisherError::Publish(Box::new(source)))?;
        let ordering_key = OrderingKey::from(failure.correlation_id).to_string();
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
                    .unwrap_or_else(|| "unknown".to_owned());
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
                code: "publish_error".to_owned(),
                message: other.to_string(),
            },
        }
    }
}

impl Publisher<CommandFailureEnvelope> for PubsubCommandFailurePublisher {
    async fn publish<'a, I>(&self, messages: I) -> Result<Vec<PublishResult>, PublisherError>
    where
        I: IntoIterator<Item = &'a CommandFailureEnvelope>,
        CommandFailureEnvelope: 'a,
    {
        let pubsub_messages = messages
            .into_iter()
            .map(Self::build_message)
            .collect::<Result<Vec<_>, _>>()?;
        let publish_futures = pubsub_messages
            .into_iter()
            .map(|message| self.publisher.publish(message))
            .collect::<Vec<_>>();
        let mut results = Vec::with_capacity(publish_futures.len());
        for (input_index, publish_future) in publish_futures.into_iter().enumerate() {
            match publish_future.await {
                Ok(message_id) => results.push(PublishResult::Success {
                    input_index,
                    transport_message_id: Some(message_id),
                }),
                Err(error) => results.push(PublishResult::Failed {
                    input_index,
                    cause: Self::dispatch_error(error),
                }),
            }
        }
        Ok(results)
    }
}
